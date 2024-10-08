// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
pub mod proccess;
mod round_robin;
mod sjf;
use std::io;

fn main() {
    loop {
        println!("SJF and Round Robin Scaling\n\n1. SJF\t2. Round Robin\n3. Exit");
        let mut num: String = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Couldn't read a number");
        let num: i32 = num.trim().parse().expect("Error while parsing");
        match num {
            1 => sjf::main_sjf(),
            2 => round_robin::main_rrobin(),
            3 => {
                println!("Exiting the program");
                break;
            }
            _ => println!("Couldn't find this option, try again"),
        }
    }
}
