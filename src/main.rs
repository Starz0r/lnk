use std::env;
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;
use std::os::windows::fs as winfs;
use std::io::Write;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let path = env::home_dir();

    match path
    {
        Some(mut p) =>
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

                // Keep a record of all symlinked locations
                p.push(".mklink");
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(p)
                    .unwrap();
                writeln!(file, "Link Type: {} ~~~~~~~~ Working Directory: {:?} ~~~~~~~~ Source: {} ~~~~~~~~ Destination: {}", &argv[1], env::current_dir().unwrap(), &argv[3], &argv[2]);
            }
            else
            {
                panic!("Not enough arguments.");
            }
        },
        None => panic!("Could not get home directory, aborting."),
    }
}
