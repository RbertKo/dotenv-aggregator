// use env_text::EnvText;;
mod lib;

use std::env;

use lib::env_text::EnvText;

fn main() {
    let mut target_env;
    let mut from_env;

    let _args: Vec<String> = env::args().collect();

    if let (Some(_target), Some(_from)) = (_args.get(1), _args.get(2)) {
        target_env = EnvText::new(_target).unwrap();
        from_env = EnvText::new(_from).unwrap();
    } else {
        panic!("You've to send arguments \"from\", \"target\"")
    }

    // target_env.parse();

    // from_env.parse();

    target_env.migrate_from(&from_env).unwrap();

    target_env.export("test").unwrap();
}







