// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
mod queue;
use queue::*;
use random_string::generate;

fn main() {
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

    // "executing the proccesses"
    let mut response: u32 = 0;
    let mut waiting: u32 = 0;
    let mut _medium_return_time: u32 = 0;
    let mut _medium_await_time: u32 = 0;
    i = 0;

    loop {
        proc[i].turnaround_time = response + proc[i].execution_time;
        proc[i].await_time = waiting;
        _medium_await_time += proc[i].await_time;
        _medium_return_time += proc[i].turnaround_time;
        waiting += proc[i].execution_time;
        response += proc[i].execution_time;
        i += 1;
        if i == 3 {
            break;
        }
    }
    list_proccesses(&proc);

    println!(
        "Await: {}\tResponse: {}\n",
        _medium_await_time, _medium_return_time
    );
    println!(
        "Medium Return Time: {:.4}",
        (_medium_return_time as f32 / (proc.len()) as f32) as f32
    );
    println!(
        "Medium Await Time: {:.4}",
        (_medium_await_time as f32 / (proc.len()) as f32) as f32
    );
}

fn add_proccess(p: &mut Vec<Proccess>) {
    p.push(Proccess::new(
        generate(5, random_string::charsets::ALPHA_LOWER),
        fastrand::u8(0..20),
        fastrand::u32(1..10),
    ));
}

fn list_proccesses(p: &Vec<Proccess>) {
    println!("Process Name\tEntry Time\tProccess Time\tAwait Time\tTurnaround Time\n");
    for val in p {
        println!(
            "{}\t{: >10}\t{: >10}\t{: >10}\t{: >10}\n",
            val.name, val.entry_time, val.execution_time, val.await_time, val.turnaround_time
        );
    }
}

fn sjf_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.execution_time.cmp(&b.execution_time));
}

fn entry_time_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.entry_time.cmp(&b.entry_time));
}
