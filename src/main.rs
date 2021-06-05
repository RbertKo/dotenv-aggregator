// use env_text::EnvText;;

mod env_text;

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

    // if let Ok(env) = test {
    //     let (mut test1, test2) = env;

    //     {
    //         test1.convert();
    //     }
        
    //     println!("{:?}", test1.parsed_text);

    //     test1.update_text(test2.text);

    //     println!("{:?}", test1);
    // }
}







