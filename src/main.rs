use std::env;
use std::path::Path;
use std::fs;
use std::os::windows::fs as winfs;

fn main() {
    let argv: Vec<String> = env::args().collect();

    match env::home_dir()
    {
        Some(_) =>
        {
            if (argv.len() == 4)
            {
                match argv[1].to_lowercase().as_str()
                {
                    // Symbolic Link
                    "--soft" =>
                    {
                        match winfs::symlink_file(Path::new(&argv[3]), Path::new(&argv[2]))
                        {
                            Ok(_) => println!("Symbolic Link created at destination {}, with source path {}", &argv[3], &argv[2]),
                            Err(_) => panic!("The given paths were not files or non-existent."),
                        }
                    },

                    // Hard Link
                    "--hard" =>
                    {
                        match fs::hard_link(&argv[3], &argv[2])
                        {
                            Ok(_) => println!("Hard Link created at destination {}, with source path {}", &argv[3], &argv[2]),
                            Err(_) => panic!("The given paths were not files or non-existent."),
                        }
                    },

                    // Soft Link
                    "--junction" =>
                    {
                        match winfs::symlink_dir(Path::new(&argv[3]), Path::new(&argv[2]))
                        {
                                Ok(_) => println!("Junction created at destination {}, with source path {}", &argv[3], &argv[2]),
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
