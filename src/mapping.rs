#[derive(Debug, Clone)]
pub struct Position {
	pub line: i32,
	pub column: i32,
}

#[derive(Debug, Clone)]
pub struct Mapping {
	pub generated: Position,
	pub original: Option<Position>,
	pub source: Option<String>,
	pub name: Option<String>,
}
