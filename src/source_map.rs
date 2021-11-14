use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SourceMapJson {
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<String>,
    pub file: Option<String>,
    #[serde(rename = "sourceRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_root: Option<String>,
    #[serde(rename = "sourcesContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources_content: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<Section>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Section {
    pub offset: Position,
    pub map: Box<SourceMapJson>,

    // ref: https://github.com/mozilla/source-map/issues/437
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    pub line: i32,
    pub column: i32,
}
