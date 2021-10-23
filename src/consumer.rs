use crate::array_set::ArraySet;
use crate::generator::SourceMapGenerator;
use crate::mapping::Mapping;
use crate::source_map::{Position, SourceMapJson};
use crate::util;
use std::collections::HashMap;

///
/// You should always use this function to create consumer
///
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
    fn each_mapping(&mut self, f: impl Fn(&source_map_mappings::Mapping), ord: IterOrd);
}

pub struct BasicConsumer {
    pub source_map: SourceMapJson,
    source_lookup_cache: HashMap<String, i32>,
    source_map_url: Option<String>,
    absolute_sources: ArraySet,
    mappings: Option<source_map_mappings::Mappings>,
    computed_column_spans: bool,
}
impl BasicConsumer {
    pub fn new(source_map_raw: &str, source_map_url: Option<&str>) -> Self {
        let source_map = serde_json::from_str::<SourceMapJson>(source_map_raw).unwrap();
        BasicConsumer {
            source_map: source_map.clone(),
            source_lookup_cache: Default::default(),
            source_map_url: source_map_url.map(|it| it.to_string()),
            absolute_sources: ArraySet::from_array(
                source_map
                    .sources
                    .iter()
                    .map(|it| {
                        util::compute_source_url(
                            source_map.source_root.as_deref(),
                            it,
                            source_map_url,
                        )
                    })
                    .collect(),
                true,
            ),
            mappings: None,
            computed_column_spans: false,
        }
    }

    pub fn from_source_map_json(source_map: SourceMapJson) -> Self {
        BasicConsumer {
            source_map: source_map.clone(),
            source_lookup_cache: Default::default(),
            source_map_url: None,
            absolute_sources: ArraySet::from_array(
                source_map
                    .sources
                    .iter()
                    .map(|it| util::compute_source_url(source_map.source_root.as_deref(), it, None))
                    .collect(),
                true,
            ),
            mappings: None,
            computed_column_spans: false,
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
            util::compute_source_url(None, source, self.source_map_url.as_deref());
        if self.absolute_sources.has(source_as_map_relative.clone()) {
            let index = self
                .absolute_sources
                .index_of(source_as_map_relative)
                .unwrap();

            self.source_lookup_cache
                .insert(source.to_string(), index as i32);
            return Some(index as i32);
        }

        // Fall back to treating the source as sourceRoot-relative.
        let source_as_source_root_relative = util::compute_source_url(
            self.source_map.source_root.as_ref().map(|it| it.as_str()),
            source,
            self.source_map_url.as_ref().map(|it| it.as_str()),
        );
        if self
            .absolute_sources
            .has(source_as_source_root_relative.clone())
        {
            let index = self
                .absolute_sources
                .index_of(source_as_source_root_relative)
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

    fn parse_mappings(&self) -> Result<source_map_mappings::Mappings, source_map_mappings::Error> {
        source_map_mappings::parse_mappings::<()>(self.source_map.mappings.as_bytes())
    }

    pub fn all_generated_position_for(
        &mut self,
        source: &str,
        original_line: i32,
        original_column: Option<i32>,
    ) -> Vec<source_map_mappings::Mapping> {
        let original_column = if let Some(r) = original_column { r } else { 0 };
        let source = self.find_source_index(source);
        if source.is_none() {
            return vec![];
        }

        let source = source.unwrap();

        if source < 0 {
            return vec![];
        }

        if original_line < 1 {
            panic!("Line numbers must be >= 1");
        }

        if original_column < 0 {
            panic!("Column numbers must be >= 0");
        }

        if self.mappings.is_none() {
            self.mappings = self.parse_mappings().ok();
        }

        let mappings = self.mappings.as_mut().unwrap();

        mappings
            .all_generated_locations_for(
                source as u32,
                original_line as u32,
                Some(original_column as u32),
            )
            .map(|it| it.clone())
            .collect()
    }

    pub fn compute_column_spans(&mut self) {
        if self.computed_column_spans {
            return;
        }

        if self.mappings.is_none() {
            self.mappings = self.parse_mappings().ok();
        }

        let mappings = self.mappings.as_mut().unwrap();
        mappings.compute_column_spans();
        self.computed_column_spans = true;
    }

    pub fn original_position_for(
        &mut self,
        generated: Position,
        bias: Option<source_map_mappings::Bias>,
    ) -> Option<Mapping> {
        let generated_line = generated.line;
        let generated_column = generated.column;
        if generated_line < 1 {
            panic!("Line numbers must be >= 1");
        }

        if generated_column < 0 {
            panic!("Column numbers must be >= 0");
        }

        let bias = bias.unwrap_or(source_map_mappings::Bias::GreatestLowerBound);

        if self.mappings.is_none() {
            self.mappings = self.parse_mappings().ok();
        }

        let mappings = self.mappings.as_mut().unwrap();

        let mapping = mappings
            .original_location_for((generated_line - 1) as u32, generated_column as u32, bias)
            .cloned();

        match mapping {
            Some(mapping) => {
                if mapping.generated_line as i32 == generated_line {
                    mapping.original.clone().map(|original| Mapping {
                        name: original
                            .name
                            .map(|it| self.source_map.names[it as usize].clone()),
                        source: self.absolute_sources.at(original.source as i32),
                        original: Some(Position {
                            line: (original.original_line + 1) as i32,
                            column: original.original_column as i32,
                        }),
                        generated: Position {
                            line: (mapping.generated_line + 1) as i32,
                            column: mapping.generated_column as i32,
                        },
                        last_generated_column: mapping.last_generated_column.map(|it| it as i32),
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn has_contents_of_all_sources(&self) -> bool {
        match self.source_map.sources_content {
            Some(ref s) => s.len() >= self.source_map.sources.len(),
            None => false,
        }
    }

    pub fn source_content_for(
        &mut self,
        source: &str,
        panic_on_missing: Option<bool>,
    ) -> Option<String> {
        if self.source_map.sources_content.is_none() {
            return None;
        }

        let sources_content = self.source_map.sources_content.clone().unwrap();
        let panic_on_missing = panic_on_missing.unwrap_or(true);

        let index = self.find_source_index(source);
        return match index {
            Some(i) => Some(sources_content[i as usize].clone()),
            None => {
                if panic_on_missing {
                    panic!(r#""{}" is not in the SourceMap."#, source);
                } else {
                    None
                }
            }
        };
    }

    pub fn generated_position_for(
        &mut self,
        source: &str,
        original_line: i32,
        original_column: i32,
        bias: Option<source_map_mappings::Bias>,
    ) -> Option<Mapping> {
        let source = match self.find_source_index(source) {
            Some(s) => s,
            None => return None,
        };

        if original_line < 1 {
            panic!("Line numbers must be >= 1")
        }

        if original_column < 0 {
            panic!("Column numbers must be >= 0")
        }

        let bias = bias.unwrap_or(source_map_mappings::Bias::GreatestLowerBound);

        if self.mappings.is_none() {
            self.mappings = self.parse_mappings().ok();
        }

        let mappings = self.mappings.as_mut().unwrap();

        let mapping = mappings
            .generated_location_for(
                source as u32,
                (original_line - 1) as u32,
                original_column as u32,
                bias,
            )
            .cloned();

        match mapping {
            Some(mapping) => {
                if mapping.original.as_ref().unwrap().source as i32 == source {
                    let last_column = mapping.last_generated_column;
                    let last_column = if self.computed_column_spans && last_column.is_none() {
                        Some(-1)
                    } else {
                        last_column.map(|it| it as i32)
                    };

                    Some(Mapping {
                        generated: Position {
                            line: (mapping.generated_line + 1) as i32,
                            column: mapping.generated_column as i32,
                        },
                        original: mapping.original.as_ref().map(|it| Position {
                            line: it.original_line as i32,
                            column: it.original_column as i32,
                        }),
                        source: mapping
                            .original
                            .as_ref()
                            .map(|it| self.absolute_sources.at(it.source as i32))
                            .flatten(),
                        name: mapping
                            .original
                            .as_ref()
                            .map(|it| self.source_map.names[it.source as usize].clone()),
                        last_generated_column: last_column,
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl ConsumerTrait for BasicConsumer {
    fn consume(source_map_raw: String, source_map_url: String, f: impl FnOnce(Self)) {
        let consumer = BasicConsumer::new(source_map_raw.as_str(), Some(source_map_url.as_str()));
        f(consumer);
    }

    fn each_mapping(&mut self, f: impl Fn(&source_map_mappings::Mapping), ord: IterOrd) {
        if self.mappings.is_none() {
            match self.parse_mappings() {
                Ok(mappings) => self.mappings = Some(mappings),
                Err(_) => return,
            }
        }
        let mappings = self.mappings.as_mut().unwrap();

        match ord {
            IterOrd::OriginalOrd => mappings
                .by_original_location()
                .for_each(|mapping| f(mapping)),
            IterOrd::GeneratedOrd => mappings
                .by_generated_location()
                .iter()
                .for_each(|mapping| f(mapping)),
        }
    }
}

pub struct IndexedConsumer {
    pub source_map: SourceMapJson,
}

impl IndexedConsumer {
    pub fn new(source_map_raw: &str) -> Self {
        let source_map = serde_json::from_str::<SourceMapJson>(source_map_raw).unwrap();
        IndexedConsumer { source_map }
    }

    pub fn from_source_map_json(source_map: SourceMapJson) -> Self {
        IndexedConsumer { source_map }
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
            let sources = consumer.absolute_sources.to_vec();
            assert_eq!(sources[0], "/the/root/one.js");
            assert_eq!(sources[1], "/the/root/two.js");
            assert_eq!(sources.len(), 2);
            return;
        }
        panic!("Not ok");
    }
}
