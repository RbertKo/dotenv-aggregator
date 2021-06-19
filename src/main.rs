// use env_text::EnvText;;
mod env_text;

use std::env;
use env_text::path_args::PathArgs;

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
        let (mut test1, mut test2) = env;

        {
            test1.parse();
        }

        test2.parse();
        
        println!("{:?}", test1.text);

        test1.migrate_from(&test2);

        println!("{:?}", test1.text);

        test1.export("test");
    }
}







