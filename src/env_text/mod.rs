pub mod path_args;

use std::collections::HashMap;

#[derive(Debug)]
pub struct EnvText {
    comment_idx: usize,
    text: String,
    pub parsed_text: Option<HashMap<String, String>>,
}

impl EnvText {
    pub fn new(text: String) -> EnvText {
        EnvText {
            comment_idx: 0,
            text,
            parsed_text: None,
        }
    }

    pub fn parse(&mut self) {
        let texts: Vec<String> = self.text.split('\n').map(|x| String::from(x)).collect();
        for text in texts {
            self.parse_line(text.as_str());
        }
    }

    fn parse_line(&mut self, text_line: &str) {
        if let None = self.parsed_text {
            self.parsed_text = Some(HashMap::new());
        }

        if let Some(_parsed_text) = &mut self.parsed_text {
            let index = if let Some(_index) = text_line.find('=') {
                _index
            } else {
                0
            };

            let is_comment: bool = 
                index == 0
                || &text_line[0..1] == "#" 
                || &text_line[0..2] == "//";

            if is_comment {
                _parsed_text.insert(
                    String::from(format!("cmt_{}", self.comment_idx)), 
                    String::from(text_line));
                self.comment_idx += 1;
            }

            let key = &text_line[0 .. index];
            let value = &text_line[index+1 .. text_line.len()];

            if key == "" {
                return;
            }

            _parsed_text.insert(String::from(key.trim()), String::from(value.trim()));
        }
    }

    pub fn update_text(&mut self, text: String) {
        self.comment_idx = 0;
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