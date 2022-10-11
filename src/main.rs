use std::{path::PathBuf, fs::File};
use clap::Parser;
use anyhow::Result;

mod scene_parser;
mod renderer;
mod camera;
mod integrator;
mod shape;
mod material;
mod media;
mod env_map;

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
