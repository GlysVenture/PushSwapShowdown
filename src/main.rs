
use std::{env, process};
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Invalid args");
        process::exit(1);
    } //todo usage

    let progs = &args[1..2];
    let size: i32 = get_size(&args[3]);
    let input: String = gen_input(size as u32);
    println!("{}", input);
}

fn get_size(str: &String) -> i32 {
    let size = str.parse()
        .unwrap_or_else(|_| {
            println!("Invalid size");
            process::exit(1);
        });
    if size <= 0 {
        println!("Invalid size");
        process::exit(1);
    }
    size
}

fn gen_input(size: u32) -> String {
    let mut list : Vec<u32> = (0..size).collect();
    list.shuffle(&mut thread_rng());
    let input: Vec<String> = list.into_iter().map(|i| i.to_string()).collect();
    input.join(" ")
}
