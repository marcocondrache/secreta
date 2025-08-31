use anyhow::Result;
use clap::Args;
use tokio::process::Command;

#[derive(Debug, Args)]
pub struct RunCommandArguments {
    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

pub async fn init(mut args: RunCommandArguments) -> Result<()> {
    if args.command.is_empty() {
        return Err(anyhow::anyhow!("No command provided"));
    }

    let _status = Command::new(args.command.remove(0))
        .args(args.command)
        .kill_on_drop(true)
        .status()
        .await?;

    Ok(())
}
