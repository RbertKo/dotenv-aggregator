pub mod path_args;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Element {
    value: String,
    line_num: usize,
    is_comment: bool,
}

#[derive(Debug)]
pub struct EnvText {
    comment_idx: usize,
    line_idx: usize,
    text: String,
    pub parsed_text: Option<HashMap<String, Element>>,
}

impl EnvText {
    pub fn new(text: String) -> EnvText {
        EnvText {
            comment_idx: 0,
            line_idx: 0,
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
            let len = text_line.len();
            let index = if let Some(_index) = text_line.find('=') {
                _index
            } else {
                0
            };

            let is_comment: bool = 
                index == 0
                || len < 3
                || &text_line[0..1] == "#" 
                || &text_line[0..2] == "//";

            if is_comment {
                _parsed_text.insert(
                    String::from(format!("da_cmt_{}", self.comment_idx)), 
                    Element {
                        value: String::from(text_line),
                        line_num: self.line_idx,
                        is_comment,
                    }
                );
                self.comment_idx += 1;
                self.line_idx += 1;
                return;
            }

            let key = &text_line[0 .. index];
            let value = &text_line[index+1 .. text_line.len()];

            if key == "" {
                return;
            }

            _parsed_text.insert(
                String::from(key.trim()), 
                Element {
                    value: String::from(value.trim()),
                    line_num: self.line_idx,
                    is_comment: false,
                }
            ); 
            self.line_idx += 1;
        }
    }

    pub fn update_text(&mut self, text: String) {
        self.line_idx = 0;
        self.comment_idx = 0;
        self.text = text;
        self.parsed_text = None;
    }

    pub fn migrate_from(&mut self, from: &EnvText) -> Result<(), &str> {
        if let (Some(target_map), Some(from_map)) = (&mut self.parsed_text, &from.parsed_text) {
            for (key, element) in from_map {
                if (key.contains("da_cmt")) {
                    continue;
                }

                if let Some(target_value) = target_map.get_mut(key) {
                    target_value.value = element.value.to_string();
                }
            }

            return Ok(());
        } 
            
        return Result::Err("This instance isn't converted yet.");
    }
}