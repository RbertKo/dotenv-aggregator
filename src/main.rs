use std::env;

fn main() {
    let test = env::args();

    println!("{:?}", test);
}

struct PathArgs {
    from: String;
    target: String;
}

fn get_paths_from_args(args: env::Args) -> PathArgs {
    let _args = args.collect();

    PathArgs {
        from: _args[1].clone(),
        target: _args[2].clone()
    }
}