// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
mod queue;
use std::io;

use queue::*;
use random_string::generate;

enum MENU {
    _OptDesselect = 0,
    OptAdd = 1,
    OptShow = 2,
    OptExecute = 3,
    OptExit = 4,
}

fn main() {
    let mut proc: Vec<Proccess> = Vec::new();
    let mut option: u8 = 0;
    println!("SJF Scaling in Rust");
    while option != MENU::OptExit as u8 {
        option = menu();
        match option {
            1 if 1 == MENU::OptAdd as u8 => add_proccess(&mut proc),
            2 if 2 == MENU::OptShow as u8 => list_proccesses(&proc),
            3 if 3 == MENU::OptExecute as u8 => execute_proccesses(&mut proc),
            4 if 4 == MENU::OptExit as u8 => {
                println!("Exiting the program!");
                break;
            }
            _ => println!("Couldn't find the given option"),
        }
    }
}

fn menu() -> u8 {
    let mut op = String::new();
    println!(
        "Choose an option\n{} - Add a Proccess\t{: >9} - List all proccesses\n{} - Execute all proccesses \t{} - Exit program\n\n",
        MENU::OptAdd as u8,
        MENU::OptShow as u8,
        MENU::OptExecute as u8,
        MENU::OptExit as u8
    );
    io::stdin().read_line(&mut op).unwrap();

    let op: u8 = op.trim().parse().expect("Error while parsing");
    op
}

fn add_proccess(p: &mut Vec<Proccess>) {
    println!("Insert a number to be the Entry Time of the proccess: ");
    let pentry = get_number();
    println!("Now insert the Execution time: ");
    let pexecute = get_number();
    p.push(Proccess::new(
        generate(5, random_string::charsets::ALPHA_LOWER),
        pentry,
        pexecute,
    ));
    entry_time_sort(p);
}

fn list_proccesses(p: &Vec<Proccess>) {
    if !p.is_empty() {
        println!("Process Name\tEntry Time\tProccess Time\tAwait Time\tTurnaround Time\n");
        for val in p {
            println!(
                "{}\t{: >10}\t{: >10}\t{: >10}\t{: >10}\n",
                val.name, val.entry_time, val.execution_time, val.await_time, val.turnaround_time
            );
        }
    } else {
        println!("There is no proccess to show");
    }
}

fn sjf_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.execution_time.cmp(&b.execution_time));
}

fn entry_time_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.entry_time.cmp(&b.entry_time));
}

fn execute_proccesses(p: &mut Vec<Proccess>) {
    let mut response: u32 = 0;
    let mut waiting: u32 = 0;
    let mut _medium_return_time: u32 = 0;
    let mut _medium_await_time: u32 = 0;
    let mut _popped_proc: Option<Proccess>;

    sjf_sort(&mut *p);

    for proccess in &mut *p {
        proccess.turnaround_time = response + proccess.execution_time;
        proccess.await_time = waiting;
        _medium_await_time += proccess.await_time;
        _medium_return_time += proccess.turnaround_time;
        waiting += proccess.execution_time;
        response += proccess.execution_time;
    }

    list_proccesses(&p);
    println!(
        "Await: {}\tResponse: {}\n",
        _medium_await_time, _medium_return_time
    );
    println!(
        "Medium Return Time: {:.4}",
        (_medium_return_time as f32 / (p.len()) as f32) as f32
    );
    println!(
        "Medium Await Time: {:.4}",
        (_medium_await_time as f32 / (p.len()) as f32) as f32
    );
    while !p.is_empty() {
        _popped_proc = p.pop();
    }
}

fn get_number() -> u32 {
    let mut num: String = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let num: u32 = num.trim().parse().expect("Error while parsing");

    num
}
