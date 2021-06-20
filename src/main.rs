use std::env;
use std::error::Error;
use std::fs;
use std::os::windows::fs as winfs;
use std::path::{Path, PathBuf};
use std::process::exit;

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
const LINK_TYPE_ERR: &str = "Multiple or no link type(s) were specified or invalid, aborting.";

fn main() {
    let argv = CliOpts::from_args();

    let cmd = (argv.soft as u8 * 1) + (argv.hard as u8 * 4) + (argv.symbolic as u8 * 8);

    match cmd {
        // Symbolic Link
        8 => match winfs::symlink_file(&argv.src, &argv.dst) {
            Ok(_) => println!(
                "{} {:?}, {} {:?}",
                "Symbolic Link created at the destination".bright_green(),
                argv.dst.bright_green(),
                "with the source path of".bright_green(),
                argv.src.bright_green()
            ),
            Err(_) => abort(FILE_ERR, 2),
        },

        // Hard Link
        4 => match fs::hard_link(&argv.src, &argv.dst) {
            Ok(_) => println!(
                "{} {:?}, {} {:?}",
                "Hard Link created at the destination".bright_green(),
                argv.dst.bright_green(),
                "with the source path of".bright_green(),
                argv.src.bright_green()
            ),
            Err(_) => abort(FILE_ERR, 3),
        },

        // Soft Link
        1 => match winfs::symlink_dir(&argv.src, &argv.dst) {
            Ok(_) => println!(
                "{} {:?}, {} {:?}",
                "Soft Link created at the destination".bright_green(),
                argv.dst.bright_green(),
                "with the source path of".bright_green(),
                argv.src.bright_green()
            ),
            Err(_) => abort(PATH_ERR, 4),
        },

        // Invalid Link
        _ => abort(LINK_TYPE_ERR, 4),
    }
}

fn abort(e: &str, c: i32) -> ! {
    println!("{}", e.bright_red());
    exit(c);
}
