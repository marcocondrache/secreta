use std::process::ExitCode;

mod cli;
mod cnf;
mod env;
mod pvd;
mod sec;

#[tokio::main]
async fn main() -> ExitCode {
    cli::init().await
}
