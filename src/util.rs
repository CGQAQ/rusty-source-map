#![allow(dead_code)]

use crate::mapping::Mapping;
use crate::util::UrlType::{Absolute, PathAbsolute, SchemeRelative};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::LinkedList;
use std::sync::Arc;
use url::{ParseError, Url};

pub fn strcmp(a: Option<String>, b: Option<String>) -> i32 {
    if a == b {
        return 0;
    }

    if a.is_none() {
        return 1;
    }

    if b.is_none() {
        return -1;
    }

    if a.unwrap() > b.unwrap() {
        return 1;
    }

    -1
}

pub fn compare_by_generated_pos_inflated(a: &Mapping, b: &Mapping) -> i32 {
    let mut cmp = a.generated.line - b.generated.line;
    if cmp != 0 {
        return cmp;
    }

    cmp = a.generated.column - b.generated.column;
    if cmp != 0 {
        return cmp;
    }

    cmp = strcmp(a.source.clone(), b.source.clone());
    if cmp != 0 {
        return cmp;
    }
    if a.original.is_some() && b.original.is_some() {
        cmp = a.original.as_ref().unwrap().line - b.original.as_ref().unwrap().line;
        if cmp != 0 {
            return cmp;
        }

        cmp = a.original.as_ref().unwrap().column - b.original.as_ref().unwrap().column;
        if cmp != 0 {
            return cmp;
        }
    }

    return strcmp(a.name.clone(), b.name.clone());
}

// We use 'http' as the base here because we want URLs processed relative
// to the safe base to be treated as "special" URLs during parsing using
// the WHATWG URL parsing. This ensures that backslash normalization
// applies to the path and such.
const PROTOCOL: &str = "http:";
const ABSOLUTE_SCHEME: &str = r"^[A-Za-z0-9\+\-\.]+:/";
lazy_static! {
    static ref ABSOLUTE_SCHEME_REGEXP: Regex = Regex::new(ABSOLUTE_SCHEME).unwrap();
    static ref PROTOCOL_AND_HOST: String = format!("{}//host", PROTOCOL);
}

#[derive(Debug, Clone, PartialEq)]
enum UrlType {
    Absolute,
    PathAbsolute,
    PathRelative,
    SchemeRelative,
}

fn get_url_type(input: &str) -> UrlType {
    let first = input.chars().nth(0).unwrap();
    let second = input.chars().nth(1).unwrap();
    if first == '/' {
        if second == '/' {
            return SchemeRelative;
        }
        return PathAbsolute;
    }
    return if ABSOLUTE_SCHEME_REGEXP.is_match(input) {
        Absolute
    } else {
        PathAbsolute
    };
}

fn build_unique_segment(prefix: &str, input: &str) -> String {
    let mut id = 0;
    loop {
        let ident = format!("{}{}", prefix, id);
        id += 1;
        if input.find(ident.as_str()).is_none() {
            return ident;
        }
    }
}

fn build_safe_base(input: &str) -> String {
    let max_dot_parts = input.split("..").count() - 1;

    // If we used a segment that also existed in `str`, then we would be unable
    // to compute relative paths. For example, if `segment` were just "a":
    //
    //   const url = "../../a/"
    //   const base = buildSafeBase(url); // http://host/a/a/
    //   const joined = "http://host/a/";
    //   const result = relative(base, joined);
    //
    // Expected: "../../a/";
    // Actual: "a/"
    //

    let segment = build_unique_segment("p", input);

    let mut base = format!("{}/", *PROTOCOL_AND_HOST);
    for _ in 0..max_dot_parts {
        base.push_str(&segment);
        base.push('/');
    }

    base
}

fn compute_relative_url(root_url: &str, target_url: &str) -> String {
    let root_url = Url::parse(root_url).unwrap();
    let target_url = Url::parse(target_url).unwrap();

    let mut target_parts = target_url.path().split("/").collect::<LinkedList<_>>();
    let mut root_parts = root_url.path().split("/").collect::<LinkedList<_>>();

    if root_parts.len() > 0 && root_parts.back().unwrap().len() == 0 {
        root_parts.pop_back();
    }

    while target_parts.len() > 0
        && root_parts.len() > 0
        && target_parts.front().unwrap() == root_parts.front().unwrap()
    {
        target_parts.pop_front();
        root_parts.pop_front();
    }

    let mut relative_path: String = root_parts
        .iter()
        .map(|_| "..")
        .chain(target_parts)
        .collect::<Vec<_>>()
        .join("/");

    if let Some(query) = target_url.query() {
        relative_path.push_str(query);
    }

    if let Some(frag) = target_url.fragment() {
        relative_path.push_str(frag);
    }
    relative_path
}

fn create_safe_handler(cb: Box<dyn Fn(Arc<Url>) + Sync>) -> Box<dyn Fn(String) -> String + Sync> {
    Box::new(move |input: String| -> String {
        let t = get_url_type(input.as_str());
        let base = build_safe_base(input.as_str());
        let urlx = Url::parse(base.as_str()).unwrap();
        let mut urlx = Arc::new(urlx.join(input.as_str()).unwrap());

        cb(urlx.clone());

        let result = urlx.to_string();

        match t {
            Absolute => result,
            SchemeRelative => result.chars().skip(PROTOCOL.len()).collect(),
            PathAbsolute => result.chars().skip(PROTOCOL_AND_HOST.len()).collect(),
            _ => compute_relative_url(base.as_str(), result.as_str()),
        }
    })
}

type UtilityFn = Box<dyn Fn(String) -> String + Sync>;

lazy_static! {
    static ref ensureDirectory: UtilityFn = create_safe_handler(Box::new(|url| {
        // replace(/\/?$/, "/");
        let reg = Regex::new("/?$").unwrap();

        url.set_path(&*reg.replace(url.path(), "/"));
    }));

    static ref trimFilename: UtilityFn = create_safe_handler(Box::new(|url| {
        let mut path = url.path().split("/").collect::<Vec<_>>();

        if path.last().unwrap().len() != 0 {
            path.pop();
        }

        url.set_path(&path.join(""));
    }));

    static ref normalize: UtilityFn = create_safe_handler(Box::new(|url|{}));
}

fn replace_if_possible(root: &str, target: &str) -> Option<String> {
    let url_type = get_url_type(root);
    if url_type != get_url_type(target) {
        return None;
    }

    let base = build_safe_base(&format!("{}{}", root, target));
    let root_url = Url::parse(root).unwrap();
    let target_url = Url::parse(target).unwrap();
    match target_url.join("") {
        Ok(_) => {}
        Err(_) => return None,
    };

    if target_url.scheme() != root_url.scheme()
        || target_url.username() != root_url.username()
        || target_url.password() != root_url.password()
        || target_url.host() != root_url.host()
        || target_url.port() != root_url.port()
    {
        return None;
    }

    Some(compute_relative_url(root, target))
}

pub fn relative(root: String, target: String) -> String {
    match replace_if_possible(root.as_str(), target.as_str()) {
        None => normalize(target),
        Some(r) => r,
    }
}
