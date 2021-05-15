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

    PathArgs {
        from: _args[1].clone(),
        target: _args[2].clone()
    }
}