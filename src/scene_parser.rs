use std::{fs::File, ffi::OsStr};
use crate::renderer::Renderer;
use anyhow::{Result, bail};

pub enum SceneParser {
    Json(JsonParser),
    Toml(TomlParser),
}

impl SceneParser {
    pub fn new(extension: Option<&OsStr>) -> Result<Self> {
        match extension.and_then(|ext| ext.to_str()) {
            Some("json") => Ok(Self::Json(JsonParser {})),
            Some("toml") => Ok(Self::Toml(TomlParser {})),
            _ => bail!("Unsupported scene format.")
        }
    }
}

impl ParseScene for SceneParser {
    fn parse(&self, file: &File) -> Result<Renderer> {
        match &self {
            Self::Json(parser) => parser.parse(file),
            Self::Toml(parser) => parser.parse(file),
        }
    }
}

pub trait ParseScene {
    fn parse(&self, file: &File) -> Result<Renderer>;
}

pub struct JsonParser {

}

impl JsonParser {

}

impl ParseScene for JsonParser {
    fn parse(&self, _file: &File) -> Result<Renderer> {
        todo!()
    }
}

pub struct TomlParser {

}

impl TomlParser {

}

impl ParseScene for TomlParser {
    fn parse(&self, _file: &File) -> Result<Renderer> {
        todo!()
    }
}