use std::{env, fs, process::exit};

use clap::{Parser, Subcommand};
use colored::Colorize;
use file_searcher::FileSearcher;
use serde::Deserialize;

use util::{print_banner, print_error, print_warning};

mod file_searcher;
mod lexer;
mod syntax_tree;
mod tests;
mod util;

#[derive(Parser, Debug)]
#[command(name = "rstl")]
#[command(about = "rstl", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Builds the project.")]
    Build {},
}

pub const RSTL_FILE_EXTENSION: &str = "rstl";

fn main() {
    let args = Args::parse();

    print_banner();

    let project_file = "project.toml";
    let current_dir = env::current_dir();

    if current_dir.is_err() {
        print_error("Failed to find current directory", 0);
        exit(1);
    }

    let current_dir = current_dir.unwrap();

    let project_file_path = current_dir.join(project_file);
    let project_file_contents = fs::read_to_string(project_file_path);

    if project_file_contents.is_err() {
        print_error("Failed to find project.toml", 0);
        exit(1);
    }

    let project_file_contents = project_file_contents.unwrap();

    let config = check_malformed_project_file(project_file_contents);

    if config.is_err() {
        print_error("Invalid projcet.toml", 0);
        exit(1);
    }

    let config = config.unwrap();

    let result = command(args, config);

    match result {
        Ok(_) => (),
        Err(e) => {
            print_error(e.as_str(), 0);
        }
    }
}

#[derive(Deserialize)]
struct Config {
    project: Project,
    build: Build,
}

#[derive(Deserialize)]
struct Project {
    name: String,
}

#[derive(Deserialize)]
struct Build {
    source_dir: String,
}

fn check_malformed_project_file(project_file_contents: String) -> Result<Config, String> {
    let table: Result<Config, _> = toml::from_str::<Config>(project_file_contents.as_str());

    if table.is_err() {
        return Err(String::from("Failed to deserialize project.toml"));
    }

    let table = table.unwrap();

    Ok(table)
}

fn command(args: Args, config: Config) -> Result<(), String> {
    match args.command {
        Command::Build {} => {
            println!(
                "Building project `{}`...",
                config.project.name.bold().green()
            );

            let current_dir = env::current_dir();

            if current_dir.is_err() {
                print_error("Failed to find current directory", 0);
                exit(1);
            }

            let current_dir = current_dir.unwrap();

            let file_searcher = FileSearcher::new(&config.build.source_dir);

            let files = file_searcher.search(RSTL_FILE_EXTENSION);

            if files.len() == 0 {
                print_warning(
                    format!(
                        "No input files provided in directory `{}`!",
                        current_dir.join(config.build.source_dir).to_str().unwrap()
                    )
                    .as_str(),
                    0,
                );
                exit(1);
            }

            println!(
                "Building `{}` file(s)...",
                files.len().to_string().bold().green()
            );

            Ok(())
        }
    }
}
