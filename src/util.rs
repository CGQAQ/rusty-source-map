use crate::mapping::Mapping;

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

	cmp = a.original.line - b.original.line;
	if cmp != 0 {
		return cmp;
	}

	cmp = a.original.column - b.original.column;
	if cmp != 0 {
		return cmp;
	}

	return strcmp(a.name.clone(), b.name.clone());
}
