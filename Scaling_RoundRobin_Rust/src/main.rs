// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
use random_string::generate;
use std::io;

// Structures
#[derive(Clone)]
struct Proccess {
    name: String,
    entry_time: i32,
    execution_time: i32,
    await_time: i32,
    turnaround_time: i32,
    proccessed: bool,
}

impl Proccess {
    fn new(p_name: String, p_entime: i32, p_exect: i32) -> Self {
        Proccess {
            name: p_name,
            entry_time: p_entime,
            execution_time: p_exect,
            await_time: 0,
            turnaround_time: 0,
            proccessed: false,
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

// main
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

fn entry_time_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.entry_time.cmp(&b.entry_time));
}

fn execute_proccesses(p: &mut Vec<Proccess>) {
    let mut response: i32 = 0;
    let mut waiting: i32 = 0;
    let mut _medium_return_time: i32 = 0;
    let mut _medium_await_time: i32 = 0;
    let mut _popped_proc: Option<Proccess>;
    let mut i: usize = 0;
    let quantum: i32 = 2;
    let mut count = 0;

    let mut clone = p.clone();

    for i in &mut *p {
        count += i.execution_time;
    }

    while count != 0 {
        println!("{count} times //  {response} \n");
        println!("Process Name\tEntry Time\tProccess Time\tAwait Time\tTurnaround Time\n");
        for val in &clone {
            println!(
                "{}\t{: >10}\t{: >10}\t{: >10}\t{: >10}\n",
                val.name, val.entry_time, val.execution_time, val.await_time, val.turnaround_time
            );
        }
        if response >= clone[i].entry_time && !clone[i].proccessed {
            if clone[i].execution_time >= quantum {
                clone[i].execution_time -= quantum;
                clone[i].turnaround_time = response + quantum;
                count -= quantum;
                response += quantum;
            } else if clone[i].execution_time == 1 {
                clone[i].execution_time -= 1;
                clone[i].turnaround_time = response + 1;
                count -= 1;
                response += 1;
            }
            if clone[i].execution_time == 0 {
                clone[i].await_time = response - clone[i].entry_time - p[i].execution_time;
                clone[i].proccessed = true;
            }
        }
        i += 1;
        if i == clone.len() {
            i = 0;
        }
    }

    i = 0;
    loop {
        _medium_await_time += clone[i].await_time;
        _medium_return_time += clone[i].turnaround_time;
        p[i].turnaround_time = clone[i].turnaround_time;
        p[i].await_time = clone[i].await_time;

        i += 1;
        if i == p.len() {
            break;
        }
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

fn get_number() -> i32 {
    let mut num: String = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Couldn't read a number");
    let num: i32 = num.trim().parse().expect("Error while parsing");

    num
}
