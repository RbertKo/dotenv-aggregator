pub mod path_args;

use std::collections::HashMap;

#[derive(Debug)]
pub struct EnvText {
    pub text: String,
    pub parsed_text: Option<HashMap<String, String>>,
}

impl EnvText {
    pub fn new(text: String) -> EnvText {
        EnvText {
            text,
            parsed_text: None,
        }
    }

    pub fn convert(&mut self) {
        let texts: Vec<String> = self.text.split('\n').map(|x| String::from(x)).collect();
        for text in texts {
            self.parse_line(text.as_str());
        }
    }

    fn parse_line(&mut self, text_line: &str) {
        let index = text_line.find('=');

        if let None = self.parsed_text {
            self.parsed_text = Some(HashMap::new());
        }

        if let Some(_parsed_text) = &mut self.parsed_text {
            if let Some(_index) = index {
                let key = &text_line[0 .. _index];
                let value = &text_line[_index+1 .. text_line.len()];

                if key == "" {
                    return;
                }

                _parsed_text.insert(String::from(key.trim()), String::from(value.trim()));
            } else {
                // _parsed_text.push(Item::Comment(text_line));
            }
        }
    }

    pub fn update_text(&mut self, text: String) {
        self.text = text;
        self.parsed_text = None;
    }

    pub fn migrate_from(&mut self, from: &EnvText) {
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