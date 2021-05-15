use std::env;

fn main() {
    let test = get_paths_from_args(env::args());

    println!("{:?}", test);
}

#[derive(Debug)]
struct PathArgs {
    from: String,
    target: String,
}

fn get_paths_from_args(args: env::Args) -> PathArgs {
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