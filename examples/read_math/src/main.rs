use std::{env, fs, path::PathBuf};

use math_fs::read_math;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("This example requires file path in first argument")
    }

    let relative = args[1].clone();
    let src = PathBuf::from(relative);

    println!("Reading file: {:?}", fs::canonicalize(&src).unwrap());

    let content = read_math(args[1].clone());
    println!("{}", content);
}
