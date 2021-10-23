use crate::source_map::SourceMapJson;

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
    source_map: SourceMapJson,
}
impl BasicConsumer {
    fn new(source_map_raw: &str) -> Self {
        let source_map = serde_json::from_str::<SourceMapJson>(source_map_raw).unwrap();
        BasicConsumer { source_map }
    }

    fn from_source_map(source_map: SourceMapJson) -> Self {
        BasicConsumer { source_map }
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

    fn from_source_map(source_map: SourceMapJson) -> Self {
        IndexedConsumer { source_map }
    }
}

pub fn create_consumer(source_map_raw: &str) -> Result<Consumer, ()> {
    let source_map = serde_json::from_str::<SourceMapJson>(source_map_raw).map_err(|_| ())?;
    if source_map.sections.is_some() {
        Ok(Consumer::IndexedConsumer(IndexedConsumer::from_source_map(
            source_map,
        )))
    } else {
        Ok(Consumer::BasicConsumer(BasicConsumer::from_source_map(
            source_map,
        )))
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
