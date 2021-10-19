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

	pub fn add_mapping(&mut self, mapping: Mapping) {}
}
