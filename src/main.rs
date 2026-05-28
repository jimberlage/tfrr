use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(name = "tfrr")]
#[command(about = "Terraform wrapper that uses directory name for var-file naming")]
struct Cli {
    /// Directory to run terraform commands in (defaults to current directory)
    #[arg(long, global = true)]
    dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run terraform import with the appropriate var-file
    Import {
        /// Arguments to pass to terraform import
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Run terraform init with the appropriate var-file
    Init,
    /// Run terraform plan with the appropriate var-file
    Plan,
    /// Run terraform apply with the generated plan file
    Apply,
}

fn get_env_name(dir: &PathBuf) -> Result<String> {
    dir.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .context("Could not determine directory name")
}

fn run_terraform(dir: &PathBuf, args: &[&str]) -> Result<()> {
    let status = Command::new("terraform")
        .current_dir(dir)
        .args(args)
        .status()
        .context("Failed to execute terraform")?;

    if !status.success() {
        bail!(
            "terraform exited with status: {}",
            status.code().unwrap_or(-1)
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let dir = match cli.dir {
        Some(d) => d.canonicalize().context("Invalid directory path")?,
        None => std::env::current_dir().context("Could not determine current directory")?,
    };

    let env_name = get_env_name(&dir)?;
    let var_file = format!("{}.tfvars", env_name);
    let plan_file = format!("{}.tfplan", env_name);

    match cli.command {
        Commands::Import { args } => {
            let var_file_arg = format!("-var-file={}", var_file);
            let mut tf_args: Vec<&str> = vec!["import", &var_file_arg];
            tf_args.extend(args.iter().map(|s| s.as_str()));
            run_terraform(&dir, &tf_args)?;
        }
        Commands::Init => {
            run_terraform(&dir, &["init", &format!("-var-file={}", var_file)])?;
        }
        Commands::Plan => {
            run_terraform(
                &dir,
                &[
                    "plan",
                    &format!("-var-file={}", var_file),
                    "-out",
                    &plan_file,
                ],
            )?;
        }
        Commands::Apply => {
            run_terraform(&dir, &["apply", &plan_file])?;
        }
    }

    Ok(())
}
