extern crate simplelog;
extern crate winapi;

use std::env;
use std::fs;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let path = env::home_dir();

    match path
    {
        Some(p) =>
        {
            if (argv.len() == 4)
            {
                let allocated_string = String::from("/d");

                match argv[1].to_lowercase().as_str()
                {
                    // Symbolic Link
                    "/d" =>
                    {

                    },

                    // Hard Link
                    "/h" =>
                    {
                        match fs::hard_link(&argv[2], &argv[3])
                        {
                            Ok(_) => println!("Hard Link created at {}, with destination {}", &argv[2], &argv[3]),
                            Err(_) => println!("The given paths were not files or non-existant."),
                        }
                    },

                    // Soft Link
                    "/j" =>
                    {

                    },

                    // Invalid Link
                    _ => {},
                }
            }
        },
        None => panic!("Could not get home directory, aborting."),
    }

    println!("Hello, world!");
}
