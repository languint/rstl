use std::{env::current_dir, fs, path::PathBuf};

use clap::{Parser, Subcommand};
use color_eyre::{
    config::HookBuilder,
    eyre::{Context, Ok, Result},
    owo_colors::OwoColorize,
};

use errors::Errors;
use file_searcher::FileSearcher;
use lexer::Token;
use logos::Logos;
use serde::Deserialize;
use tracing::instrument;

mod ast;
mod errors;
mod file_searcher;
mod lexer;
mod util;

#[derive(Parser, Debug)]
#[command(name = "rstl", about = "rstl")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Build the project.")]
    Build,
}

#[derive(Deserialize)]
struct Config {
    project: Project,
}

#[derive(Deserialize)]
struct Project {
    name: String,
    source_dir: String,
}

#[instrument]
fn main() -> Result<()> {
    HookBuilder::default().install()?;

    util::print_banner();
    let args = Args::parse();

    let cwd: PathBuf = current_dir().map_err(|_| Errors::CWDError())?;

    let project_file_path = cwd.join("project.toml");

    let file_contents = fs::read_to_string(&project_file_path)
        .map_err(|_| Errors::ProjectFileError(cwd.clone()))?;

    let config = parse_project_toml(file_contents)?;

    run(&cwd, config, args)?;

    Ok(())
}

fn parse_project_toml(file_contents: String) -> Result<Config> {
    let config = toml::from_str::<Config>(file_contents.as_str())
        .map_err(|_| Errors::InvalidProjectFile())?;

    Ok(config)
}

fn run(cwd: &PathBuf, config: Config, args: Args) -> Result<()> {
    match args.command {
        Command::Build => {
            println!("Building project {}...", config.project.name.bold().green());

            let source_dir = cwd.join(config.project.source_dir);

            let file_searcher = FileSearcher::new(source_dir);

            let source_files = file_searcher.search(".rstl")?;

            if source_files.is_empty() {
                return Err(Errors::NoInputFiles().into());
            }

            for file in source_files {
                let source = fs::read_to_string(&file)
                    .wrap_err_with(|| Errors::ReadFileError(format!("{}", file.display())))?;

                let lexer = Token::lexer(source.as_str());
            }
        }
    }

    Ok(())
}
