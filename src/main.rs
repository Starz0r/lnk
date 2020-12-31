use std::env;
use std::fs;
use std::os::windows::fs as winfs;
use std::path::{Path, PathBuf};
use std::process::exit;

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

fn main() {
    let argv = CliOpts::from_args();

    const FILEERR: &str = "The given paths were not files or non-existent.";
    const PATHERR: &str = "The given paths were not directories or were already existing.";

    let cmd = (argv.soft as u8 * 1) + (argv.hard as u8 * 4) + (argv.symbolic as u8 * 8);

    match cmd {
        // Symbolic Link
        8 => match winfs::symlink_file(&argv.src, &argv.dst) {
            Ok(_) => println!(
                "Symbolic Link created at the destination {:?}, with the source path of {:?}",
                argv.dst, argv.src
            ),
            Err(_) => {
                println!("{}", FILEERR);
                exit(2)
            }
        },

        // Hard Link
        4 => match fs::hard_link(&argv.src, &argv.dst) {
            Ok(_) => println!(
                "Hard Link created at the destination {:?}, with the source path of {:?}",
                argv.dst, argv.src
            ),
            Err(_) => {
                println!("{}", FILEERR);
                exit(2)
            }
        },

        // Soft Link
        1 => match winfs::symlink_dir(&argv.src, &argv.dst) {
            Ok(_) => println!(
                "Junction created at the destination {:?}, with the source path of {:?}",
                argv.dst, argv.src
            ),
            Err(_) => {
                println!("{}", PATHERR);
                exit(2)
            }
        },

        // Invalid Link
        _ => panic!("Multiple or no link type(s) were specified or invalid, aborting."),
    }

    exit(0)
}
