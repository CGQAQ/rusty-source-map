use crate::array_set::ArraySet;
use crate::generator::SourceMapGenerator;
use crate::source_map::SourceMapJson;
use crate::util;
use std::collections::HashMap;

pub enum Consumer {
    BasicConsumer(BasicConsumer),
    IndexedConsumer(IndexedConsumer),
}

pub enum IterOrd {
    GeneratedOrd,
    OriginalOrd,
}

pub trait ConsumerTrait: Sized {
    fn consume(source_map_raw: String, source_map_url: String, f: impl FnOnce(Self));
    fn each_mapping(f: impl Fn(&source_map_mappings::Mapping), bias: source_map_mappings::Bias);
}

pub struct BasicConsumer {
    pub source_map: SourceMapJson,
    source_lookup_cache: HashMap<String, i32>,
    source_map_url: Option<String>,
    absolute_sources: ArraySet,
}
impl BasicConsumer {
    pub fn new(source_map_raw: &str, source_map_url: Option<&str>) -> Self {
        let source_map = serde_json::from_str::<SourceMapJson>(source_map_raw).unwrap();
        BasicConsumer {
            source_map,
            source_lookup_cache: Default::default(),
            source_map_url: source_map_url.map(|it| it.to_string()),
            absolute_sources: ArraySet::new(),
        }
    }

    pub fn from_source_map_json(source_map: SourceMapJson) -> Self {
        BasicConsumer {
            source_map,
            source_lookup_cache: Default::default(),
            source_map_url: None,
            absolute_sources: ArraySet::new(),
        }
    }

    pub fn from_source_map(
        source_map: &mut SourceMapGenerator,
        source_map_url: Option<&str>,
    ) -> Self {
        BasicConsumer::new(source_map.as_string().as_str(), source_map_url)
    }

    fn find_source_index(&mut self, source: &str) -> Option<i32> {
        let cached_index = self.source_lookup_cache.get(source);
        if let Some(&index) = cached_index {
            return Some(index);
        }

        // Treat the source as map-relative overall by default.
        let source_as_map_relative =
            util::compute_source_url(None, source, self.source_map_url.map(|it| it.as_str()));
        if self.absolute_sources.has(source_as_map_relative.clone()) {
            let index = self
                .absolute_sources
                .index_of(source_as_map_relative.clone())
                .unwrap();

            self.source_lookup_cache
                .insert(source.to_string(), index as i32);
            return Some(index as i32);
        }

        // Fall back to treating the source as sourceRoot-relative.
        let source_as_source_root_relative = util::compute_source_url(
            self.source_map.source_root.map(|it| it.as_str()),
            source,
            self.source_map_url.map(|it| it.as_str()),
        );
        if self.absolute_sources.has(source_as_source_root_relative) {
            let index = self
                .absolute_sources
                .index_of(source_as_source_root_relative.clone())
                .unwrap();
            self.source_lookup_cache
                .insert(source.to_string(), index as i32);
            return Some(index as i32);
        }

        None
    }

    pub fn get_sources(&self) -> Vec<String> {
        self.absolute_sources.to_vec()
    }
}

pub struct IndexedConsumer {
    source_map: SourceMapJson,
}

impl IndexedConsumer {
    fn new(source_map_raw: &str) -> Self {
        let source_map = serde_json::from_str::<SourceMapJson>(source_map_raw).unwrap();
        IndexedConsumer { source_map }
    }

    fn from_source_map_json(source_map: SourceMapJson) -> Self {
        IndexedConsumer { source_map }
    }
}

pub fn create_consumer(source_map_raw: &str) -> Result<Consumer, serde_json::Error> {
    let source_map = serde_json::from_str::<SourceMapJson>(source_map_raw)?;
    if source_map.sections.is_some() {
        Ok(Consumer::IndexedConsumer(
            IndexedConsumer::from_source_map_json(source_map),
        ))
    } else {
        Ok(Consumer::BasicConsumer(
            BasicConsumer::from_source_map_json(source_map),
        ))
    }
}

#[cfg(test)]
mod tests {
    const TEST_MAP: &str = r#"{
  "version": 3,
  "file": "min.js",
  "names": ["bar", "baz", "n"],
  "sources": ["one.js", "two.js"],
  "sourceRoot": "/the/root",
  "mappings":
  "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
}"#;

    use super::*;
    #[test]
    fn test_sources() {
        let map = create_consumer(TEST_MAP).unwrap();
        if let Consumer::BasicConsumer(consumer) = map {
            let sources = consumer.source_map.sources;
            assert_eq!(sources[0], "/the/root/one.js");
            assert_eq!(sources[1], "/the/root/two.js");
            assert_eq!(sources.len(), 2);
        }
        panic!("Not ok");
    }
}
