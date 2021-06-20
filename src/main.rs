use std::{fs, os::windows::fs as winfs, path::PathBuf, process::exit};

use {owo_colors::OwoColorize, structopt::StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(name = "mklink")]
struct CliOpts {
    #[structopt(short = "t", long = "link")]
    src: PathBuf,

    #[structopt(short = "o", long = "target")]
    dst: PathBuf,

    #[structopt(short = "s", long = "soft")]
    soft: bool,

    #[structopt(short, long)]
    hard: bool,

    #[structopt(short = "d", long = "directory")]
    symbolic: bool,

    #[structopt(short = "j", long = "junction")]
    junction: bool,
}

const FILE_ERR: &str = "File was a path or non-existent.";
const PATH_ERR: &str = "Paths were not directories or were already existing.";
const LINK_TYPE_ERR: &str = "Multiple or no link type(s) were specified or invalid, aborting.";

fn main() {
    let argv = CliOpts::from_args();

    let cmd = (argv.soft as u8 * 1)
        + (argv.hard as u8 * 4)
        + (argv.symbolic as u8 * 8)
        + (argv.junction as u8 * 16);

    match cmd {
        // Junction
        16 => match junction::create(&argv.dst, &argv.src) {
            Ok(()) => println!(
                "{} {:?}, {} {:?}",
                "Junction created at source".bright_green(),
                argv.src.bright_green(),
                "with the destination of",
                argv.dst.bright_green()
            ),
            Err(_) => abort(PATH_ERR, 2),
        },
        // Symbolic Link
        8 => match winfs::symlink_file(&argv.dst, &argv.src) {
            Ok(_) => println!(
                "{} {:?}, {} {:?}",
                "Symbolic Link created at source".bright_green(),
                argv.src.bright_green(),
                "with the destination of".bright_green(),
                argv.dst.bright_green()
            ),
            Err(_) => abort(FILE_ERR, 3),
        },

        // Hard Link
        4 => match fs::hard_link(&argv.dst, &argv.src) {
            Ok(_) => println!(
                "{} {:?}, {} {:?}",
                "Hard Link created at source".bright_green(),
                argv.src.bright_green(),
                "with the destination of".bright_green(),
                argv.dst.bright_green()
            ),
            Err(_) => abort(FILE_ERR, 4),
        },

        // Soft Link
        1 => match winfs::symlink_dir(&argv.dst, &argv.src) {
            Ok(_) => println!(
                "{} {:?}, {} {:?}",
                "Soft Link created at source".bright_green(),
                argv.src.bright_green(),
                "with the destination of".bright_green(),
                argv.dst.bright_green()
            ),
            Err(_) => abort(PATH_ERR, 5),
        },

        // Invalid Link
        _ => abort(LINK_TYPE_ERR, 6),
    }
}

fn abort(e: &str, c: i32) -> ! {
    println!("{}", e.bright_red());
    exit(c);
}
