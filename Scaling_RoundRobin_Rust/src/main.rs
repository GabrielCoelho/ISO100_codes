// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.

use random_string::generate;
use std::io;

struct Proccess {
    name: String,
    entry_time: u8,
    execution_time: u32,
    await_time: u32,
    turnaround_time: u32,
}

impl Proccess {
    fn new(p_name: String, p_entime: u8, p_exect: u32) -> Self {
        Proccess {
            name: p_name,
            entry_time: p_entime,
            execution_time: p_exect,
            await_time: 0,
            turnaround_time: 0,
        }
    }
}

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
    println!("Round Robin Scaling in Rust");
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
    p.push(Proccess::new(
        generate(5, random_string::charsets::ALPHA_LOWER),
        fastrand::u8(0..20),
        fastrand::u32(1..10),
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

fn entry_time_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.entry_time.cmp(&b.entry_time));
}

fn execute_proccesses(p: &mut Vec<Proccess>) {
    let mut response: u32 = 0;
    let mut waiting: u32 = 0;
    let mut _medium_return_time: u32 = 0;
    let mut _medium_await_time: u32 = 0;
    let mut _popped_proc: Option<Proccess>;
}
