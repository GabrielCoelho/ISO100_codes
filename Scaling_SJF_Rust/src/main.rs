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
        add_proccess(&mut proc);
        i += 1;
        if i == 3 {
            break;
        }
    }

    for val in &proc {
        println!("{}\n{}\n{}\n", val.name, val.entry_time, val.execution_time);
    }

    proc.sort_by(|a, b| a.execution_time.cmp(&b.execution_time));

    list_proccesses(&proc);
}

fn add_proccess(p: &mut Vec<Proccess>) {
    p.push(Proccess::new(
        generate(5, random_string::charsets::ALPHA_LOWER),
        fastrand::u8(0..20),
        fastrand::u32(1..10),
    ));
}

fn list_proccesses(p: &Vec<Proccess>) {
    for val in p {
        println!(
            "Process Name: {}\nEntry Time: {}\n Time Proccessing: {}\n",
            val.name, val.entry_time, val.execution_time
        );
    }
}
