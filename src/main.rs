use std::env;
use std::fs::File;

use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

fn main() {
    let test;

    let _args: Vec<String> = env::args().collect();

    if let (Some(_from), Some(_target)) = (_args.get(1), _args.get(2)) {
        test = PathArgs::new(_from, _target);
    } else {
        panic!("You've to send arguments \"from\", \"target\"")
    }

    let test = test.to_env_text();

    if let Ok(env) = test {
        let (mut test1, test2) = env;

        {
            test1.convert();
        }
        
        println!("{:?}", test1.parsed_text);

        test1.update_text(test2.text);

        println!("{:?}", test1);
    }
}

#[derive(Debug)]
struct PathArgs<'a> {
    from: &'a String,
    target: &'a String,
}

#[derive(Debug)]
struct EnvText<'a> {
    text: &'a String,
    parsed_text: Option<HashMap<&'a str, String>>,
}

// #[derive(Debug)]
// enum Item {
//     Comment(String),
//     Env(String, String),
// }

impl<'a> PathArgs<'a> {
    pub fn new(from: &'a String, target: &'a String) -> PathArgs<'a> {
        // if let (Some(_from), Some(_target)) = (_args.get(1), _args.get(2)) {
        //     from = _from;
        //     target = _target;
        // } else {
        //     panic!("You've to send arguments \"from\", \"target\"")
        // }
    
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

    pub fn to_env_text(&self) -> Result<(EnvText, EnvText), io::Error> {
        let from_file = self.read_env_to_string(&self.from)?;
        let target_file = self.read_env_to_string(&self.target)?;
        
        Ok((
            EnvText::new(&from_file),
            EnvText::new(&target_file),
        ))
    }
}

impl<'a> EnvText<'a> {
    pub fn new(text: &'a String) -> EnvText<'a> {
        EnvText {
            text,
            parsed_text: None,
        }
    }

    fn parse_line(&mut self, text_line: &'a str) {
        let index = text_line.find('=');

        if let None = self.parsed_text {
            self.parsed_text = Some(HashMap::new());
        }

        if let Some(_parsed_text) = &mut self.parsed_text {
            if let Some(_index) = index {
                let key = &text_line[0 .. _index];
                let value = &text_line[_index+1 .. text_line.len()];

                _parsed_text.insert(key.trim(), String::from(value.trim()));
            } else {
                // _parsed_text.push(Item::Comment(text_line));
            }
        }
    }

    pub fn update_text(&mut self, text: &'a String) {
        self.text = text;
        self.parsed_text = None;
    }

    pub fn convert(&'a mut self) {
        for text in self.text.split('\n') {
            self.parse_line(text);
        }
    }

    pub fn migrate_from(&mut self, from: &'a EnvText) {
        if self.parsed_text == None {
            // return Result::Err("This instance isn't converted yet.");
        }

        if let (Some(target_map), Some(from_map)) = (&mut self.parsed_text, &from.parsed_text) {
            for (key, value) in from_map {
                if let Some(target_value) = target_map.get_mut(key) {
                    *target_value = value.to_string();
                }
            }
        }
    }
}