pub mod path_args;

use std::collections::HashMap;

#[derive(Debug)]
pub struct EnvText<'a> {
    text: String,
    parsed_text: Option<HashMap<&'a str, String>>,
}

impl<'a> EnvText<'a> {
    pub fn new(text: String) -> EnvText<'a> {
        EnvText {
            text,
            parsed_text: None,
        }
    }

    pub fn convert(&mut self) {
        let texts: Vec<&str> = self.text.split('\n').collect();
        for text in texts {
            self.parse_line(text);
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

    pub fn update_text(&mut self, text: String) {
        self.text = text;
        self.parsed_text = None;
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