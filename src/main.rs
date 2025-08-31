use std::process::ExitCode;

mod cli;
mod cnf;
mod env;
mod pvd;

#[tokio::main]
async fn main() -> ExitCode {
    cli::init().await
}
