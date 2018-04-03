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
                println!("Argument 1: {}", &argv[0]);
                println!("Argument 2: {}", &argv[1]);
                println!("Argument 3: {}", &argv[2]);
                println!("Argument 4: {}", &argv[3]);

                match argv[1].to_lowercase().as_str()
                {
                    // Symbolic Link
                    "--soft" =>
                    {

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

                    },

                    // Invalid Link
                    _ => {},
                }
            }
            else
            {
                panic!("Not enough arguments.");
            }
        },
        None => panic!("Could not get home directory, aborting."),
    }

    println!("Hello, world!");
}
