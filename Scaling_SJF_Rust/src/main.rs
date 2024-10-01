// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
mod queue;
use queue::*;
use random_string::generate;

fn main() {
    let mut _total_exec_time: i32;
    let mut _total_await_time: i32;
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

    entry_time_sort(&mut proc);
    sjf_sort(&mut proc);

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
    println!("Process Name\tEntry Time\tProccess Time\n");
    for val in p {
        println!(
            "{}\t{: >10}\t{: >10}\n",
            val.name, val.entry_time, val.execution_time
        );
    }
}

fn sjf_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.execution_time.cmp(&b.execution_time));
}

fn entry_time_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.entry_time.cmp(&b.entry_time));
}
