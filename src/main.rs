use anyhow::{Context, Result};
use clap::Parser;
use path_clean::clean;
use std::{
    fs::{self, File, create_dir_all},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file_path: PathBuf,
    #[arg(short, long, default_value_t = false)]
    overwrite: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let absolute_path = if args.file_path.is_absolute() {
        args.file_path.clone()
    } else {
        std::env::current_dir()
            .context("Failed to get current directory")?
            .join(&args.file_path)
    };
    let absolute_path = clean(absolute_path);

    if absolute_path.exists() && !args.overwrite {
        anyhow::bail!(
            "'{}' already exists. Use --overwrite to overwrite it.",
            absolute_path.display()
        );
    }

    let parent = absolute_path.parent().context("Failed to get parent directory")?;

    let mut path = PathBuf::new();
    for component in parent.components() {
        path.push(component);
        if path.is_file() {
            if !args.overwrite {
                anyhow::bail!(
                    "'{}' already exists as a file. Use --overwrite to overwrite it.",
                    path.display()
                );
            } else {
                fs::remove_file(path.clone()).context("Failed to remove file")?;
            }
        }
    }

    create_dir_all(parent).context("Failed to create directories")?;

    if absolute_path.is_dir() {
        fs::remove_dir_all(absolute_path.clone())
            .context("Failed to remove directory")?;
    }

    File::create(absolute_path).context("Failed to create file")?;

    Ok(())
}
