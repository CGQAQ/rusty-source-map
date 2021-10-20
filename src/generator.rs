#![allow(dead_code)]

use crate::array_set::ArraySet;
use crate::mapping::Mapping;
use crate::mapping_list::MappingList;
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

    pub fn set_source_content(&mut self) {}
}
