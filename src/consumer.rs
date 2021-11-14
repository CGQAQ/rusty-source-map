use crate::array_set::ArraySet;
use crate::generator::SourceMapGenerator;
use crate::mapping::Mapping;
use crate::source_map::{Position, SourceMapJson};
use crate::util;
use rayon::prelude::*;
use std::collections::HashMap;
use std::panic;
use std::sync::{Arc, Mutex};

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

impl Consumer {
    pub fn as_basic_consumer(&self) -> &BasicConsumer {
        if let Self::BasicConsumer(ref consumer) = self {
            consumer
        } else {
            panic!("The consumer is not a basic consumer");
        }
    }

    pub fn as_indexed_consumer(&self) -> &IndexedConsumer {
        if let Self::IndexedConsumer(ref consumer) = self {
            consumer
        } else {
            panic!("The consumer is not a basic consumer");
        }
    }

    pub fn as_basic_consumer_mut(&mut self) -> &mut BasicConsumer {
        if let Self::BasicConsumer(ref mut consumer) = self {
            consumer
        } else {
            panic!("The consumer is not a basic consumer");
        }
    }

    pub fn as_indexed_consumer_mut(&mut self) -> &mut IndexedConsumer {
        if let Self::IndexedConsumer(ref mut consumer) = self {
            consumer
        } else {
            panic!("The consumer is not a basic consumer");
        }
    }

    pub fn try_as_basic_consumer(&self) -> Option<&BasicConsumer> {
        if let Self::BasicConsumer(ref consumer) = self {
            Some(consumer)
        } else {
            None
        }
    }

    pub fn try_as_indexed_consumer(&self) -> Option<&IndexedConsumer> {
        if let Self::IndexedConsumer(ref consumer) = self {
            Some(consumer)
        } else {
            None
        }
    }

    pub fn try_as_basic_consumer_mut(&mut self) -> Option<&mut BasicConsumer> {
        if let Self::BasicConsumer(ref mut consumer) = self {
            Some(consumer)
        } else {
            None
        }
    }

    pub fn try_as_indexed_consumer_mut(&mut self) -> Option<&mut IndexedConsumer> {
        if let Self::IndexedConsumer(ref mut consumer) = self {
            Some(consumer)
        } else {
            None
        }
    }
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
    pub(crate) source_lookup_cache: HashMap<String, i32>,
    pub(crate) absolute_sources: ArraySet,
    pub(crate) source_map_url: Option<String>,
    pub(crate) mappings: Option<source_map_mappings::Mappings>,
    pub(crate) computed_column_spans: bool,
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
            .cloned()
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
        self.source_map.sources_content.as_ref()?;

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

pub struct Section {
    generated_offset: Position,
    consumer: BasicConsumer,
}

pub struct IndexedConsumer {
    pub source_map: SourceMapJson,
    pub(crate) source_lookup_cache: HashMap<String, i32>,
    pub(crate) absolute_sources: ArraySet,
    pub(crate) source_map_url: Option<String>,
    pub(crate) mappings: Option<source_map_mappings::Mappings>,
    pub(crate) computed_column_spans: bool,
    pub(crate) sections: Vec<Section>,
}

const SUPPORTED_SOURCE_MAP_VERSION: i32 = 3;

impl IndexedConsumer {
    pub fn new(source_map_raw: &str, source_map_url: Option<&str>) -> Self {
        let source_map = serde_json::from_str::<SourceMapJson>(source_map_raw).unwrap();

        let version = source_map.version;

        // Once again, Sass deviates from the spec and supplies the version as a
        // string rather than a number, so we use loose equality checking here.
        if version != SUPPORTED_SOURCE_MAP_VERSION {
            panic!("Unsupported version: {}", version);
        }

        let mut last_offset = Arc::new(Mutex::new(Position {
            line: -1,
            column: 0,
        }));

        IndexedConsumer {
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
            sections: source_map
                .sections
                .unwrap()
                .par_iter()
                .map({
                    let last_offset = last_offset.clone();
                    move |section| {
                        if section.url.is_some() {
                            panic!("Section with url is not supported.");
                        }

                        let line = section.offset.line;
                        let colum = section.offset.column;

                        let mut last_offset = last_offset.lock().unwrap();

                        if line < last_offset.line
                            || (line == last_offset.line && colum < last_offset.column)
                        {
                            panic!("Section offsets must be ordered and non-overlapping.")
                        }

                        *last_offset = section.offset.clone();

                        Section {
                            generated_offset: Position {
                                // The offset fields are 0-based, but we use 1-based indices when
                                // encoding/decoding from VLQ.
                                line: line + 1,
                                column: colum + 1,
                            },
                            consumer: BasicConsumer::from_source_map_json(*section.map.clone()),
                        }
                    }
                })
                .collect(),
        }
    }

    pub fn from_source_map_json(source_map: SourceMapJson) -> Self {
        IndexedConsumer {
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
            sections: Vec::new(),
        }
    }

    // pub fn get_sources(&self) -> {
    //
    // }
}
