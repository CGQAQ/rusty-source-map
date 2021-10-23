#![allow(dead_code)]

use crate::array_set::ArraySet;
use crate::base64_vlq::base64vlq_encode;
use crate::mapping::Mapping;
use crate::mapping_list::MappingList;
use crate::source_map::SourceMapJson;
use crate::util;
use serde_json;
use std::collections::hash_map::HashMap;

pub struct SourceMapGenerator {
    pub(crate) file: Option<String>,
    pub(crate) source_root: Option<String>,
    pub(crate) skip_validation: bool,

    pub(crate) sources: ArraySet,
    pub(crate) names: ArraySet,
    pub(crate) mappings: MappingList,
    pub(crate) source_contents: HashMap<String, String>,
}

impl SourceMapGenerator {
    pub fn from_source_map() {
        unimplemented!()
    }

    pub fn add_mapping(&mut self, mapping: Mapping) {
        // if (!this._skipValidation) {
        // 	this._validateMapping(generated, original, source, name);
        // }
        let source: Option<String> = mapping.source.clone();
        let name: Option<String> = mapping.name.clone();

        if let Some(ref s) = source {
            if !self.sources.has(s.clone()) {
                self.sources.add(s.clone(), false)
            }
        }

        if let Some(ref n) = name {
            if !self.names.has(n.clone()) {
                self.names.add(n.clone(), false);
            }
        }

        self.mappings.add(mapping);
    }

    pub fn set_source_content(&mut self, source_file: String, source_content: Option<String>) {
        let mut source = source_file;
        source = match self.source_root {
            Some(ref s) => util::relative(s.clone(), source.clone()),
            None => source,
        };

        if let Some(content) = source_content {
            self.source_contents.insert(source, content);
        } else {
            self.source_contents.remove(&source);
        }
    }

    pub fn apply_sourcemap() {
        unimplemented!();
    }

    fn validate_mapping(mapping: &Mapping) -> Result<(), ()> {
        if (mapping.generated.column >= 0
            && mapping.generated.line > 0
            && mapping.original.is_none()
            && mapping.source.is_none()
            && mapping.name.is_none()) // case 1
            || (mapping.original.is_some()
                && mapping.generated.line > 0
                && mapping.generated.column >= 0
                && mapping.original.as_ref().unwrap().line > 0
                && mapping.original.as_ref().unwrap().column >= 0
                && mapping.source.is_some())
        // case 2, 3
        {
            // case 1 2 3
            Ok(())
        } else {
            Err(())
        }
    }

    fn serialize_mappings(&mut self) -> String {
        let mut previous_generated_column = 0;
        let mut previous_generated_line = 1;
        let mut previous_original_column = 0;
        let mut previous_original_line = 0;
        let mut previous_name = 0;
        let mut previous_source = 0;
        let mut result = "".to_string();
        let mut name_idx;
        let mut source_idx;

        let mappings = self.mappings.to_array();

        let mappings_len = mappings.len();
        for i in 0..mappings_len {
            let mapping = mappings[i].clone();
            let mut next = "".to_string();

            if mapping.generated.line != previous_generated_line {
                previous_generated_column = 0;
                while mapping.generated.line != previous_generated_line {
                    next += ";";
                    previous_generated_line += 1;
                }
            } else if i > 0 {
                if util::compare_by_generated_pos_inflated(&mapping, &mappings[i - 1]) == 0 {
                    continue;
                }
                next += ",";
            }

            next += &base64vlq_encode(mapping.generated.column - previous_generated_column);
            previous_generated_column = mapping.generated.column;

            if let Some(mapping_source) = mapping.source {
                source_idx = self.sources.index_of(mapping_source.clone()).unwrap() as i32;
                next += &base64vlq_encode(source_idx - previous_source);
                previous_source = source_idx;

                // lines are stored 0-based in SourceMap spec version 3
                next += &base64vlq_encode(
                    mapping.original.as_ref().unwrap().line - 1 - previous_original_line,
                );
                previous_original_line = mapping.original.as_ref().unwrap().line - 1;

                next += &base64vlq_encode(
                    mapping.original.as_ref().unwrap().column - previous_original_column,
                );
                previous_original_column = mapping.original.as_ref().unwrap().column;

                if let Some(mapping_name) = mapping.name {
                    name_idx = self.names.index_of(mapping_name).unwrap() as i32;
                    next += &base64vlq_encode(name_idx - previous_name);
                    previous_name = name_idx;
                }
            }
            result += &next;
        }

        result
    }

    fn generate_sources_contents(
        &self,
        sources: Vec<String>,
        source_root: Option<String>,
    ) -> Vec<Option<String>> {
        sources
            .into_iter()
            .map(|mut source| -> Option<String> {
                if let Some(ref root) = source_root {
                    source = util::relative(root.clone(), source);
                }

                if self.source_contents.contains_key(&source) {
                    Some(self.source_contents[&source].clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn as_json(&mut self) -> SourceMapJson {
        let sources_vec = self.sources.to_vec();
        let mut sources_content: Option<Vec<String>> = None;
        if self.source_contents.len() > 0 {
            sources_content = Some(
                self.generate_sources_contents(sources_vec.clone(), self.source_root.clone())
                    .into_iter()
                    .flatten()
                    .collect(),
            );
        }
        SourceMapJson {
            version: 3,
            sources: sources_vec,
            names: self.names.to_vec(),
            mappings: self.serialize_mappings(),
            file: self.file.clone(),
            source_root: self.source_root.clone(),
            sources_content,
            sections: None,
        }
    }

    pub fn as_string(&mut self) -> String {
        serde_json::to_string(&self.as_json()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::array_set::ArraySet;
    use crate::mapping_list::MappingList;
    use crate::source_map::Position;

    #[test]
    fn simple() {
        let map = SourceMapGenerator {
            file: Some("foo.js".to_string()),
            source_root: Some(".".to_string()),
            skip_validation: false,
            sources: ArraySet::new(),
            names: ArraySet::new(),
            mappings: MappingList::new(),
            source_contents: Default::default(),
        }
        .as_json();
        assert!(map.file.is_some());
        assert!(map.source_root.is_some());
    }

    #[test]
    fn simple_json() {
        let map = SourceMapGenerator {
            file: Some("foo.js".to_string()),
            source_root: Some(".".to_string()),
            skip_validation: false,
            sources: ArraySet::new(),
            names: ArraySet::new(),
            mappings: MappingList::new(),
            source_contents: Default::default(),
        }
        .as_string();
        assert_eq!(map, r#"{"version":3,"sources":[],"names":[],"mappings":"","file":"foo.js","sourceRoot":".","sourcesContent":[]}"#.to_string());
    }

    #[test]
    fn mappings() {
        let mut map = SourceMapGenerator {
            file: Some("min.js".to_string()),
            source_root: Some("/the/root".to_string()),
            skip_validation: false,
            sources: ArraySet::new(),
            names: ArraySet::new(),
            mappings: MappingList::new(),
            source_contents: Default::default(),
        };

        map.add_mapping(Mapping {
            generated: Position { line: 1, column: 1 },
            original: Some(Position { line: 1, column: 1 }),
            source: Some("one.js".to_string()),
            name: None,
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position { line: 1, column: 5 },
            original: Some(Position { line: 1, column: 5 }),
            source: Some("one.js".to_string()),
            name: None,
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position { line: 1, column: 9 },
            original: Some(Position {
                line: 1,
                column: 11,
            }),
            source: Some("one.js".to_string()),
            name: None,
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position {
                line: 1,
                column: 18,
            },
            original: Some(Position {
                line: 1,
                column: 21,
            }),
            source: Some("one.js".to_string()),
            name: Some("bar".to_string()),
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position {
                line: 1,
                column: 21,
            },
            original: Some(Position { line: 2, column: 3 }),
            source: Some("one.js".to_string()),
            name: None,
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position {
                line: 1,
                column: 28,
            },
            original: Some(Position {
                line: 2,
                column: 10,
            }),
            source: Some("one.js".to_string()),
            name: Some("baz".to_string()),
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position {
                line: 1,
                column: 32,
            },
            original: Some(Position {
                line: 2,
                column: 14,
            }),
            source: Some("one.js".to_string()),
            name: Some("bar".to_string()),
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position { line: 2, column: 1 },
            original: Some(Position { line: 1, column: 1 }),
            source: Some("two.js".to_string()),
            name: None,
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position { line: 2, column: 5 },
            original: Some(Position { line: 1, column: 5 }),
            source: Some("two.js".to_string()),
            name: None,
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position { line: 2, column: 9 },
            original: Some(Position {
                line: 1,
                column: 11,
            }),
            source: Some("two.js".to_string()),
            name: None,
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position {
                line: 2,
                column: 18,
            },
            original: Some(Position {
                line: 1,
                column: 21,
            }),
            source: Some("two.js".to_string()),
            name: Some("n".to_string()),
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position {
                line: 2,
                column: 21,
            },
            original: Some(Position { line: 2, column: 3 }),
            source: Some("two.js".to_string()),
            name: None,
            last_generated_column: None,
        });

        map.add_mapping(Mapping {
            generated: Position {
                line: 2,
                column: 28,
            },
            original: Some(Position {
                line: 2,
                column: 10,
            }),
            source: Some("two.js".to_string()),
            name: Some("n".to_string()),
            last_generated_column: None,
        });

        assert_eq!(map.as_string(), r#"{"version":3,"sources":["one.js","two.js"],"names":["bar","baz","n"],"mappings":"CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA","file":"min.js","sourceRoot":"/the/root","sourcesContent":null}"#.to_string())
    }
}
