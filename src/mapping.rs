use crate::source_map::Position;

#[derive(Debug, Clone)]
pub struct Mapping {
    pub generated: Position,
    pub original: Option<Position>,
    pub source: Option<String>,
    pub name: Option<String>,
    pub last_generated_column: Option<i32>,
}
