#![allow(dead_code)]

use crate::mapping::Mapping;
use crate::util::UrlType::{Absolute, PathAbsolute, PathRelative, SchemeRelative};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::LinkedList;
use url::Url;

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

    strcmp(a.name.clone(), b.name.clone())
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
    let mut iter = input.chars();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    if first == '/' {
        if second == '/' {
            return SchemeRelative;
        }
        return PathAbsolute;
    }
    if ABSOLUTE_SCHEME_REGEXP.is_match(input) {
        Absolute
    } else {
        PathRelative
    }
}

fn build_unique_segment(prefix: &str, input: &str) -> String {
    let mut id = 0;
    loop {
        let ident = format!("{}{}", prefix, id);
        id += 1;
        if !input.contains(ident.as_str()) {
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

    if !root_parts.is_empty() && root_parts.back().unwrap().is_empty() {
        root_parts.pop_back();
    }

    while !target_parts.is_empty()
        && !root_parts.is_empty()
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

fn create_safe_handler(cb: Box<dyn Fn(&mut Url) + Sync>) -> Box<dyn Fn(String) -> String + Sync> {
    Box::new(move |input: String| -> String {
        let t = get_url_type(input.as_str());
        let base = build_safe_base(input.as_str());
        let urlx = Url::parse(base.as_str()).unwrap();
        let mut urlx = urlx.join(input.as_str()).unwrap();

        cb(&mut urlx);

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
    static ref ENSURE_DIRECTORY: UtilityFn = create_safe_handler(Box::new(|url| {
        // replace(/\/?$/, "/");
        let reg = Regex::new("/?$").unwrap();

        let path = reg.replace(url.path(), "/").to_string();
        url.set_path(&path);
    }));

    static ref TRIM_FILENAME: UtilityFn = create_safe_handler(Box::new(|url| {
        let path = url.path().to_string();
        let mut path = path.split('/').collect::<Vec<_>>();

        if !path.last().unwrap().is_empty() {
            path.pop();
        }

        url.set_path(&path.join(""));
    }));

    static ref NORMALIZE: UtilityFn = create_safe_handler(Box::new(|_url|{}));
}

fn replace_if_possible(root: &str, target: &str) -> Option<String> {
    let url_type = get_url_type(root);
    if url_type != get_url_type(target) {
        return None;
    }

    let base = build_safe_base(&format!("{}{}", root, target));
    let root_url = Url::parse(&base).unwrap();
    let root_url = root_url.join(root).unwrap();
    let target_url = Url::parse(&base).unwrap();
    let target_url = target_url.join(target).unwrap();
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
        None => NORMALIZE(target),
        Some(r) => r,
    }
}

fn with_base(url: &str, base: Option<&str>) -> String {
    match base {
        Some(base) => Url::parse(base)
            .unwrap()
            .join(url)
            .unwrap()
            .as_str()
            .to_string(),
        None => url.to_string(),
    }
}

pub fn join(root: &str, path: &str) -> String {
    let path_t = get_url_type(path);
    let root_t = get_url_type(root);

    let root = ENSURE_DIRECTORY(root.to_owned());

    if path_t == Absolute {
        return with_base(path, None);
    }

    if root_t == Absolute {
        return with_base(path, Some(root.as_str()));
    }

    if path_t == SchemeRelative {
        return NORMALIZE(path.to_string());
    }

    if root_t == SchemeRelative {
        return with_base(
            path,
            Some(with_base(root.as_str(), Some(PROTOCOL_AND_HOST.as_str())).as_str()),
        )
        .chars()
        .skip(PROTOCOL.len())
        .collect();
    }

    if path_t == PathAbsolute {
        return NORMALIZE(path.to_string());
    }

    if root_t == PathAbsolute {
        return with_base(
            path,
            Some(with_base(root.as_str(), Some(PROTOCOL_AND_HOST.as_str())).as_str()),
        )
        .chars()
        .skip(PROTOCOL_AND_HOST.len())
        .collect();
    }

    let base = build_safe_base(format!("{}{}", path, root).as_str());
    let new_path = with_base(
        path,
        Some(with_base(root.as_str(), Some(base.as_str())).as_str()),
    );
    compute_relative_url(base.as_str(), new_path.as_str())
}

pub fn compute_source_url(
    source_root: Option<&str>,
    source_url: &str,
    source_map_url: Option<&str>,
) -> String {
    // The source map spec states that "sourceRoot" and "sources" entries are to be appended. While
    // that is a little vague, implementations have generally interpreted that as joining the
    // URLs with a `/` between then, assuming the "sourceRoot" doesn't already end with one.
    // For example,
    //
    //   sourceRoot: "some-dir",
    //   sources: ["/some-path.js"]
    //
    // and
    //
    //   sourceRoot: "some-dir/",
    //   sources: ["/some-path.js"]
    //
    // must behave as "some-dir/some-path.js".
    //
    // With this library's the transition to a more URL-focused implementation, that behavior is
    // preserved here. To acheive that, we trim the "/" from absolute-path when a sourceRoot value
    // is present in order to make the sources entries behave as if they are relative to the
    // "sourceRoot", as they would have if the two strings were simply concated.

    let mut source_url = source_url;
    let after = source_url.replacen("/", "", 1);

    if source_root.is_some() && get_url_type(source_url) == PathAbsolute {
        // sourceURL = sourceURL.replace(/^\//, "");
        source_url = after.as_str();
    }

    let mut url = NORMALIZE(source_url.to_string());

    if let Some(source_root) = source_root {
        url = join(source_root, url.as_str());
    }

    if let Some(source_map_url) = source_map_url {
        url = join(
            TRIM_FILENAME(source_map_url.to_string()).as_str(),
            url.as_str(),
        );
    }

    url
}
