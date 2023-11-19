use serde::Serialize;


#[derive(Debug, Serialize)]
pub(crate) struct DisplayDirectory {
    pub name: String,
    pub files: Vec<DFile>,
}

#[derive(Debug, Serialize)]
pub(crate) struct DFile {
    pub name: String,
    pub extension: String,
    pub url: String,
}