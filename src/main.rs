use std::env;
use std::error::Error;
use std::fs;
use std::os::windows::fs as winfs;
use std::path::{Path, PathBuf};
use std::process::exit;

use color_eyre::eyre::{eyre, Result};
use owo_colors::OwoColorize;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mklink")]
struct CliOpts {
    #[structopt(short = "o", long = "target")]
    src: PathBuf,

    #[structopt(short = "t", long = "link")]
    dst: PathBuf,

    #[structopt(short = "j", long = "junction")]
    soft: bool,

    #[structopt(short, long)]
    hard: bool,

    #[structopt(short = "d", long = "directory")]
    symbolic: bool,
}

const FILE_ERR: &str = "File was a path or non-existent.";
const PATH_ERR: &str = "Paths were not directories or were already existing.";

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let argv = CliOpts::from_args();

    let cmd = (argv.soft as u8 * 1) + (argv.hard as u8 * 4) + (argv.symbolic as u8 * 8);

    match cmd {
        // Symbolic Link
        8 => {
            winfs::symlink_file(&argv.src, &argv.dst).or_else(|_| Err(eyre!("{}", FILE_ERR)))?;
            println!(
                "{} {:?}, {} {:?}",
                "Symbolic Link created at the destination".bright_green(),
                argv.dst.bright_green(),
                "with the source path of".bright_green(),
                argv.src.bright_green()
            )
        }

        // Hard Link
        4 => {
            fs::hard_link(&argv.src, &argv.dst).or_else(|_| Err(eyre!("{}", FILE_ERR)))?;
            println!(
                "{} {:?}, {} {:?}",
                "Hard Link created at the destination".bright_green(),
                argv.dst.bright_green(),
                "with the source path of".bright_green(),
                argv.src.bright_green()
            )
        }

        // Soft Link
        1 => {
            winfs::symlink_dir(&argv.src, &argv.dst).or_else(|_| Err(eyre!("{}", PATH_ERR)))?;
            println!(
                "{} {:?}, {} {:?}",
                "Junction created at the destination".bright_green(),
                argv.dst.bright_green(),
                "with the source path of".bright_green(),
                argv.src.bright_green()
            )
        }

        // Invalid Link
        _ => Err(eyre!(
            "Multiple or no link type(s) were specified or invalid, aborting."
        ))?,
    }

    Ok(())
}
