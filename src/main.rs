#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
// There are way too many warnings during the development, and I got tired of them.
// Will be removed after major development is done.
use std::{path::PathBuf, fs::File};
use clap::Parser;
use anyhow::Result;

mod scene_parser;
mod renderer;
mod camera;
mod integrator;
mod shape;
mod material;
mod medium;
mod env_map;
mod ray;
mod utils;
mod intersection;
mod acceleration;

use scene_parser::{SceneParser, ParseScene};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Scene file to render, can be json or toml format.
    pub scene: PathBuf,
    /// Location to output the rendered image, extension specified.
    pub output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let scene_file = File::open(cli.scene.as_path())?;
    let parser = SceneParser::new(cli.scene.extension())?;
    let renderer = parser.parse(&scene_file)?;
    renderer.render(&cli.output)?;
    Ok(())
}
