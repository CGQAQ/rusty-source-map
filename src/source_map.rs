use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SourceMapJson {
    pub version: i32,
    pub sources: Vec<String>,
    pub names: Vec<String>,
    pub mappings: String,
    pub file: Option<String>,
    #[serde(rename = "sourceRoot")]
    pub source_root: Option<String>,
    #[serde(rename = "sourcesContent")]
    pub sources_content: Option<Vec<String>>,
    pub sections: Option<Section>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Section {
    offset: Position,
    map: Box<SourceMapJson>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    pub line: i32,
    pub column: i32,
}
