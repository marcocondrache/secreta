use std::process::ExitCode;

mod cli;
mod cnf;
mod env;
mod pvd;
mod sec;

#[tokio::main]
async fn main() -> ExitCode {
    match cli::init().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("secreta: {e:?}");

            ExitCode::FAILURE
        }
    }
}
