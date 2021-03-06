use crate::mapping::Mapping;
use crate::source_map::Position;
use crate::util;

fn generated_position_after(a: &Mapping, b: &Mapping) -> bool {
    let line_a = a.generated.line;
    let line_b = b.generated.line;
    let column_a = a.generated.column;
    let column_b = b.generated.column;
    line_b > line_a
        || (line_b == line_a && column_b >= column_a)
        || util::compare_by_generated_pos_inflated(a, b) <= 0
}
pub struct MappingList {
    array: Vec<Mapping>,
    sorted: bool,
    last: Option<Mapping>,
}

impl Default for MappingList {
    fn default() -> Self {
        MappingList {
            array: Vec::new(),
            sorted: true,
            last: None,
        }
    }
}

impl MappingList {
    pub fn new() -> Self {
        Default::default()
    }

    /// Iterate through internal items. This method takes the same arguments that
    /// `Array.prototype.forEach` takes.

    /// NOTE: The order of the mappings is NOT guaranteed.
    pub fn unsorted_for_each(&self, callback: impl Fn(&Mapping, usize)) {
        for (index, mapping) in self.array.iter().enumerate() {
            callback(mapping, index);
        }
    }

    pub fn add(&mut self, mapping: Mapping) {
        if generated_position_after(
            &self.last.clone().unwrap_or(Mapping {
                name: None,
                source: None,
                generated: Position {
                    column: -1,
                    line: -1,
                },
                original: None,
                last_generated_column: None,
            }),
            &mapping,
        ) {
            self.last = Some(mapping.clone());
            self.array.push(mapping);
        } else {
            self.sorted = false;
            self.array.push(mapping)
        }
    }

    pub fn to_array(&mut self) -> Vec<Mapping> {
        if !self.sorted {
            self.array.sort_by(|a, b| {
                let cmp = util::compare_by_generated_pos_inflated(a, b);
                match cmp {
                    d if d > 0 => std::cmp::Ordering::Greater,
                    d if d == 0 => std::cmp::Ordering::Equal,
                    _ => std::cmp::Ordering::Less,
                }
            });
            self.sorted = true;
        }

        self.array.clone()
    }
}
