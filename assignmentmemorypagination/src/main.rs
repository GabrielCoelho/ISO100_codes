use std::io;

fn main() {
    loop {
        println!("Memory Pagination\n\n1. FIFO\n2. Optimal\n3. Exit");
        let mut num: String = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Couldn't read a number");
        let num: i32 = num.trim().parse().expect("Error while parsing");
        match num {
            1 => fifo(),
            2 => optimal(),
            3 => {
                println!("Exiting the program");
                break;
            }
            _ => println!("Couldn't find this option, try again"),
        }
    }
}

fn fifo() {}

fn optimal() {}
