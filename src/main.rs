use clap::{CommandFactory, Parser};

use crate::{app::run, cli::{Cli, maybe_page_output}};

mod model;
mod api;
mod error;
mod cli;
mod date;
mod validation;
mod storage;
mod app;

#[tokio::main]
async fn main() {
    match Cli::try_parse() {
        Ok(cli) => run(cli).await,
        Err(e) => {
            use clap::error::ErrorKind;

            match e.kind() {
                ErrorKind::DisplayHelp => {
                    let mut cmd = Cli::command();
                    let help = cmd.render_long_help().to_string();
                    maybe_page_output(&help);
                    std::process::exit(0);
                }
                ErrorKind::DisplayVersion => {
                    e.print().unwrap();
                    std::process::exit(0);
                }
                _ => {
                    e.print().unwrap();
                    std::process::exit(1);
                }
            }
        }
    }
}
