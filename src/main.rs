extern crate core;

mod proc;
mod display;
mod stack;

use display::visualize;
use std::{env, process};
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Invalid args");
        eprintln!("usage: p_swap_showdown [program1] [program2] [min] [max]");
        process::exit(1);
    }

    let progs = &args[1..3];
    let mut size: [i32; 2] = [0, 0];
    size[0] = get_size(&args[3]);
    size[1] = get_size(&args[4]);
    if size[0] >= size[1] || size[1] - size[0] > 500 {
        eprintln!("Invalid size");
        process::exit(1);
    }

    let input = gen_input(size);
    let input_string = string_input(&input);
    showdown(progs, input_string, input);
}

fn check_output(output: &String) -> bool {
    let split = output.split("\n");
    for line in split {
        match line {
            "ra" | "rra" | "rb" | "rrb" | "sa" | "sb" | "pa" | "pb" | "" | "ss" | "rr" | "rrr" => {},
            _=> return true,
        };
    };
    false
}

fn showdown(progs: &[String], input_str: String, input: Vec<i32>){
    let mut process1 = proc::spawn_process(&progs[0], &input_str);
    let mut process2 = proc::spawn_process(&progs[1], &input_str);

    let output1 = proc::process_output(&mut process1, &progs[0]);
    let output2 = proc::process_output(&mut process2, &progs[1]);

    process1.wait().unwrap_or_else(|e| {
        eprintln!("{}: Error {}", progs[0], e);
        std::process::exit(1);
    });
    process2.wait().unwrap_or_else(|e| {
        eprintln!("{}: Error {}", progs[0], e);
        std::process::exit(1);
    });

    if check_output(&output1) { eprintln!("{}: Invalid output", progs[0]); }
    if check_output(&output2) { eprintln!("{}: Invalid output", progs[1]); }

    visualize(progs,
              output1.split("\n").map(|s| s.to_string()).collect(),
              output2.split("\n").map(|s| s.to_string()).collect(),
              input);
}

//stuff

fn get_size(str: &String) -> i32 {
    let size = str.parse()
        .unwrap_or_else(|_| {
            eprintln!("Invalid size");
            process::exit(1);
        });
    size
}

fn gen_input(size: [i32; 2]) -> Vec<i32> {
    let mut list: Vec<i32> = (size[0]..size[1]).collect();
    list.shuffle(&mut thread_rng());
    list
}

fn string_input(list: &Vec<i32>) -> String {
    list.into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
