// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
mod queue;
use queue::*;
use random_string::generate;

fn main() {
    let mut total_exec_time: i32;
    let mut total_await_time: i32;
    let mut i = 0;
    let mut proc: Vec<Proccess> = Vec::new();
    println!("SJF Scaling in Rust");

    loop {
        proc.push(Proccess::new(
            generate(5, random_string::charsets::ALPHA_LOWER),
            fastrand::u8(0..20),
            fastrand::u32(1..10),
        ));
        i += 1;
        if i == 5 {
            break;
        }
    }

    for val in proc {
        println!("{}\n{}\n{}\n", val.name, val.entry_time, val.execution_time);
    }
}
