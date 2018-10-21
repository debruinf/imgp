extern crate imgp;

use std::env;
use std::process;

use imgp::Input;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if let Err(err) = imgp::check_min_length(&args) {
        println!("Something went wrong: {}", err);
        process::exit(1);
    }

    imgp::check_if_help(&args);

    let img_path = args[1].clone();

    let input = Input::new(args).unwrap_or_else(|err| {
        println!("Something went wrong: {}", err);
        process::exit(1);
    });

    println!("{:?}", input);
    imgp::run(&img_path, input)
}
