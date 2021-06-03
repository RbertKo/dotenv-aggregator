#[derive(Debug)]
pub struct PathArgs<'a> {
    from: &'a str,
    target: &'a str,
}

pub impl<'a> PathArgs<'a> {
    pub fn new(from: &'a str, target: &'a str) -> PathArgs<'a> {
        PathArgs {
            from,
            target,
        }
    }

    fn read_env_to_string(&self, path: &str) -> Result<String, io::Error> {
        let mut f = File::open(path)?;
        let mut contents = String::new();

        f.read_to_string(&mut contents)?;

        Ok(contents)
    }

    pub fn to_env_text(&self) -> Result<(EnvText, EnvText), io::Error> {
        let from_file = self.read_env_to_string(&self.from)?;
        let target_file = self.read_env_to_string(&self.target)?;
        
        Ok((
            EnvText::new(from_file),
            EnvText::new(target_file),
        ))
    }
}