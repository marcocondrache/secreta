use std::process::ExitCode;

mod cli;
mod cnf;
mod env;

#[tokio::main]
async fn main() -> ExitCode {
    cli::init().await
}
