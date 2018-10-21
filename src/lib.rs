extern crate image;

use std::error::Error;
use std::process;

#[derive(Debug)]
pub struct Input {
    flags: Vec<Config>,
}

impl Input {
    pub fn new(mut args: Vec<String>) -> Result<Input, &'static str> {
        args.remove(0);
        args.remove(0);
        let mut flags: Vec<Config> = Vec::new();
        while args.len() > 0 {
            let d = args.get(0..2).unwrap().to_vec();
            args.remove(0);
            args.remove(0);
            let c = Config::new(&d).unwrap();
            let mut d = vec![c];
            flags.append(&mut d)
        }
        Ok(Input {flags})
    }
}

#[derive(Debug)]
pub struct Config {
    command: String,
    instruction: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str>  {
        if args.len() < 2 {
            return Err("Missing argument(s), please check your input")
        }
        let command = args[0].clone();
        let instruction = args[1].clone();
        Ok(Config { command, instruction })
    }
}

pub fn run(img_path: &String, input: Input) {
    let source_img = image::open(img_path).unwrap_or_else(|_err| {
        println!("No image at path '{}'", img_path);
        process::exit(1);
    });

    if &input.flags[0].command[..] != "-d" {
        println!("Error, no destination");
        process::exit(1);
    };

    let destination: &str = &input.flags[0].instruction;
    copy_img(&source_img, destination);

    let copied_img = image::open(destination).unwrap_or_else(|_err| {
        println!("Something went wrong");
        process::exit(1);
    });

    for c in &input.flags {
        match &c.command[..] {
            "-r" => {
                rotate(&copied_img, &c.instruction, &destination);
                println!("Rotated!")
            },
            "-s" => println!("Resized"),
            "-sf" => println!("Resize forced"),
            // "-r" => rotate(&img, &c.instruction, &img_path),
        //     "-s" => size(img, &config.instruction, &img_path),
        //     "-sf" => size_forced(img, &config.instruction, &img_path),
            _ => {
                println!("Unknown flag");
            }
        };
    };
    println!("Hellow orld!")
}

pub fn check_min_length(args: &Vec<String>) -> Result<(), &'static str> {
    if args.len() < 2 {
        return Err("Too few arguments")
    }
    Ok(())
}

pub fn check_if_help(args: &Vec<String>) {
    if &args[1][..] == "-h" {
        let _ = print_help();
        process::exit(0);
    }
}

fn print_help() -> Result<(), Box<Error>> {
    println!("Description for IMGP, a command line tool for simple operations on images, written in Rust");
    println!("For now, only performs one operation at the time");
    println!("");
    println!("For help: img -h");
    println!("");
    println!("Syntax: img <img_path> -flag <instruction>");
    println!("Options:");
    println!("-c\tCopies the image as-is to path given in <instruction>");
    println!("-r\tRotates the image according to the <instructions> and saves in the original location:");
    println!("\t\t[cw] for clockwise operation and [ccw] for counter clockwise operation");
    println!("-s\tSizes the image to dimensions as specified in <instructions> and saves in the orignal location. Preserves the image dimensions ad fits largest size. <instruction> for dimension takes the for of [<width>x<height>]");
    println!("-sf\tSizes the image to dimensions as specified in <instructions> and saves in the orignal location.  Does NOT preserves the image dimensions. <instruction> for dimension takes the for of [<width>x<height>]");

    process::exit(1);
}

fn copy_img(img: &image::DynamicImage, copy_path: &str) -> Result<(), Box<Error>> {
    img.save(copy_path)?;
    println!("Image copied to {}", copy_path);
    Ok(())
}

fn rotate(img: &image::DynamicImage, direction: &str, destination: &str) -> Result<(), Box<Error>> {
    let mut new_img = img.clone();
    new_img = match &direction[..] {
        "cw" => new_img.rotate90(),
        "ccw" => new_img.rotate270(),
        _ =>  {
            println!("Direction not correctly specified");
            new_img
        }
    };

    new_img.save(destination)?;
    println!("Image copied to {}", destination);
    Ok(())
}
