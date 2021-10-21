#![allow(dead_code)]

use crate::array_set::ArraySet;
use crate::base64_vlq::base64vlq_encode;
use crate::mapping::Mapping;
use crate::mapping_list::MappingList;
use crate::util;
use std::collections::hash_map::HashMap;

pub struct SourceMapGenerator {
    file: Option<String>,
    source_root: Option<String>,
    skip_validation: bool,

    sources: ArraySet,
    names: ArraySet,
    mappings: MappingList,
    source_contents: HashMap<String, String>,
}

impl SourceMapGenerator {
    pub fn from_source_map() {
        unimplemented!()
    }

    pub fn add_mapping(&mut self, mapping: Mapping, source: Option<String>, name: Option<String>) {
        // if (!this._skipValidation) {
        // 	this._validateMapping(generated, original, source, name);
        // }

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

        if source_content.is_some() {
            self.source_contents
                .insert(source.clone(), source_content.unwrap());
        } else {
            self.source_contents.remove(&source.clone());
        }
    }

    pub fn apply_sourcemap() {
        unimplemented!();
    }

    fn validate_mapping(mapping: &Mapping) -> Result<(), ()> {
        if mapping.generated.column >= 0
            && mapping.generated.line > 0
            && mapping.original.is_none()
            && mapping.source.is_none()
            && mapping.name.is_none()
        {
            // Case 1.
            Ok(())
        } else if mapping.original.is_some()
            && mapping.generated.line > 0
            && mapping.generated.column >= 0
            && mapping.original.as_ref().unwrap().line > 0
            && mapping.original.as_ref().unwrap().column >= 0
            && mapping.source.is_some()
        {
            // case 2 and 3
            Ok(())
        } else {
            Err(())
        }
    }

    fn serialize_mappings(&mut self) -> String {
        let mut previousGeneratedColumn = 0;
        let mut previousGeneratedLine = 1;
        let mut previousOriginalColumn = 0;
        let mut previousOriginalLine = 0;
        let mut previousName = 0;
        let mut previousSource = 0;
        let mut result = "".to_string();
        let mut nameIdx;
        let sourceIdx;

        let mappings = self.mappings.to_array();

        let mappings_len = mappings.len();
        for i in 0..mappings_len {
            let mapping = mappings[i].clone();
            let mut next = "".to_string();

            if mapping.generated.line != previousGeneratedLine {
                previousGeneratedColumn = 0;
                while mapping.generated.line != previousGeneratedLine {
                    next += ";";
                    previousGeneratedLine += 1;
                }
            } else if i > 0 {
                if util::compare_by_generated_pos_inflated(&mapping, &mappings[i - 1]) == 0 {
                    continue;
                }
                next += ",";
            }

            next += &base64vlq_encode(mapping.generated.column - previousGeneratedColumn);
            previousGeneratedColumn = mapping.generated.column;

            if let Some(mapping_source) = mapping.source {
                sourceIdx = self.sources.index_of(mapping_source.clone()).unwrap() as i32;
                next += &base64vlq_encode(sourceIdx - previousSource);
                previousSource = sourceIdx;

                // lines are stored 0-based in SourceMap spec version 3
                next += &base64vlq_encode(
                    mapping.original.as_ref().unwrap().line - 1 - previousOriginalLine,
                );
                previousOriginalLine = mapping.original.as_ref().unwrap().line - 1;

                next += &base64vlq_encode(
                    mapping.original.as_ref().unwrap().column - previousOriginalColumn,
                );
                previousOriginalColumn = mapping.original.as_ref().unwrap().column;

                if let Some(mapping_name) = mapping.name {
                    nameIdx = self.names.index_of(mapping_name).unwrap() as i32;
                    next += &base64vlq_encode(nameIdx - previousName);
                    previousName = nameIdx;
                }
            }
            result += &next;
        }

        result
    }
}
