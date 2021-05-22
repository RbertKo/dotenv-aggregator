use std::env;
use std::fs::File;

use std::io;
use std::io::prelude::*;

fn main() {
    let test = PathArgs::new(env::args());

    println!("{:?}", test.to_env_text());
}

#[derive(Debug)]
struct PathArgs {
    from: String,
    target: String,
}

#[derive(Debug)]
struct EnvText {
    text: String,
    parsed_text: Option<String>,
}

impl PathArgs {
    fn new(args: env::Args) -> PathArgs {
        let _args: Vec<String> = args.collect();
        let from: String;
        let target: String;
    
        if let (Some(_from), Some(_target)) = (_args.get(1), _args.get(2)) {
            from = String::from(_from);
            target = String::from(_target);
        } else {
            panic!("You've to send arguments \"from\", \"target\"")
        }
    
        PathArgs {
            from,
            target,
        }
    }

    fn read_env_to_string(&self, path: &String) -> Result<String, io::Error> {
        let mut f = File::open(path)?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn to_env_text(&self) -> Result<(EnvText, EnvText), io::Error> {
        let from_file = self.read_env_to_string(&self.from)?;
        let target_file = self.read_env_to_string(&self.target)?;
        
        Ok((
            EnvText::new(from_file),
            EnvText::new(target_file),
        ))
    }
}

impl EnvText {
    fn new(text: String) -> EnvText {
        EnvText {
            text,
            parsed_text: None,
        }
    }
}