use std::env;
use std::io::{ self, Write };
use std::io::prelude::*;
use std::path::Path;
use std::fs::{ self, File, OpenOptions };

struct Command;

impl Command {
    fn touch(path: &Path) -> io::Result<()> {
        match OpenOptions::new()
            .write(true)
            .create(true) 
            .open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn echo(text: &String, path: &Path) -> io::Result<()> {
        match File::create(path)?.write_all(text.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn cat(path: &Path) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut content: String  = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(e) => Err(e),
        }
    }

    fn mkdir(path: &Path) -> io::Result<()> {
        match fs::create_dir(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn rm(path: &Path) -> io::Result<()> {
        match fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn rd(path: &Path) -> io::Result<()> {
        match fs::remove_dir(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn rdf(path: &Path) -> io::Result<()> {
        match fs::remove_dir_all(&path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn info() -> String {
        format!("Usage: rutils <command> [<args>] <path> [<flag>]")
    }
}

fn get_args() -> Option<Vec<String>> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() == 0 { return None }
    Some(args)
}

fn parse_args(args: &Vec<String>) {
    let command = &args[0];
    match command.as_str() {
        "touch" => {
            let path = Path::new(&args[1]);
            match Command::touch(path) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e.kind()),
            }
        },

        "echo" => {
            let text = &args[1];
            let path = Path::new(&args[2]);
            match Command::echo(&text, &path) {
                Ok(()) => (),
                Err(e) => eprintln!("Error: {}", e.kind()),
            }
        },

        "cat" => {
            let path = Path::new(&args[1]);
            match Command::cat(&path) {
                Ok(content) => println!("{}", content),
                Err(e) => eprintln!("{}", e.kind()),
            }
        },

        "mkdir" => {
            let path = Path::new(&args[1]);
            match Command::mkdir(&path) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e.kind()),
            }
        },

        "rm" => {
            let path = Path::new(&args[1]);
            match Command::rm(&path) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e.kind()),
            }
        },

        "rd" => {
            let path = Path::new(&args[1]);
            match Command::rd(&path) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e.kind()),
            }
        },

        "rdf" => {
            let path = Path::new(&args[1]);
            match Command::rdf(&path) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e.kind()),
            }
        },

        _ => {
            eprintln!("{}", Command::info());
            std::process::exit(0);
        },
    }
}


fn main() {
    if let Some(args) = get_args() {
        parse_args(&args);
    } else {
        eprintln!("{}", Command::info());
        std::process::exit(0);
    }
}
