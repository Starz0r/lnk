extern crate simplelog;
extern crate winapi;

use std::env;
use std::path::Path;
use std::fs;
use std::os::windows::fs as winfs;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let path = env::home_dir();

    match path
    {
        Some(p) =>
        {
            if (argv.len() == 4)
            {
                match argv[1].to_lowercase().as_str()
                {
                    // Symbolic Link
                    "--soft" =>
                    {
                        match winfs::symlink_file(Path::new(&argv[2]), Path::new(&argv[3]))
                        {
                            Ok(_) => println!("Symbolic Link created at {}, with destination {}", &argv[2], &argv[3]),
                            Err(_) => panic!("The given paths were not files or non-existent."),
                        }
                    },

                    // Hard Link
                    "--hard" =>
                    {
                        match fs::hard_link(&argv[2], &argv[3])
                        {
                            Ok(_) => println!("Hard Link created at {}, with destination {}", &argv[2], &argv[3]),
                            Err(_) => panic!("The given paths were not files or non-existent."),
                        }
                    },

                    // Soft Link
                    "--junction" =>
                    {
                        match winfs::symlink_dir(Path::new(&argv[2]), Path::new(&argv[3]))
                        {
                                Ok(_) => println!("Junction created at {}, with destination {}", &argv[2], &argv[3]),
                                Err(_) => panic!("The given paths were not directories or were already existing."),
                        }
                    },

                    // Invalid Link
                    _ =>
                    {
                        panic!("Link type was not specified or invalid, aborting.")
                    },
                }
            }
            else
            {
                panic!("Not enough arguments.");
            }
        },
        None => panic!("Could not get home directory, aborting."),
    }
}
