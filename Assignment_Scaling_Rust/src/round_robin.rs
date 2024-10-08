// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
use crate::proccess::*;
use random_string::generate;
use std::io;

enum MENU {
    _OptDesselect = 0,
    OptAdd = 1,
    OptShow = 2,
    OptExecute = 3,
    OptExit = 4,
}

/// Funçao principal Round Robin
///
/// Realiza o loop de acordo com o menu, chamando as funções conforme necessário.
pub fn main_rrobin() {
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
                println!("Going back to the main menu");
                break;
            }
            _ => println!("Couldn't find the given option"),
        }
    }
}

/// Função do menu que retornará um inteiro sem sinal de 8 bits.
pub fn menu() -> u8 {
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

/// Função para adicionar o processo
///
/// Recebe o endereço do vetor de processos e insere as informações dadas pelo usuário através da
/// função `push()` do vetor.
///
/// Ao fim de cada inserção, chama a função para ordenar por tempo de entrada.
///
/// * p: Endereço mutável de um vetor de Processos
pub fn add_proccess(p: &mut Vec<Proccess>) {
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

/// Listar os processos adicionados
///
/// Caso o vetor recebido através do parâmetro `p` não esteja vazio, exibe na tela o nome do
/// processo, seu tempo de entrada, tempo de execução, tempo de espera e tempo de turnaround.
///
/// Obs: Os dois últimos tempos (Espera e Turnaround) só serão inicializados a partir da execução
/// dos processos.
///
/// * p: Endereço de um vetor de Processos
pub fn list_proccesses(p: &Vec<Proccess>) {
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

/// Ordenar pelo tempo de entrada
///
/// Ordena o vetor pelo tempo de entrada de cada item contido nele.
///
/// * p: Endereço mutável de um vetor de Processos
/// * sort_by(|a, b|): Ordena através de uma verificação de dois itens do vetor sequenciais (a ->
/// b) onde o valor do tempo de entrada de a comparado ao tempo de entrada de b sejam do menor
/// para o maior.
///
/// Note: nós passamos o endereço de b na comparação pois, no momento da execução do programa, não temos como
/// saber o valor em si de b, mas sim de a (que é o primeiro valor a ser checado). Basicamente,
/// realiza um `bubble_sort` a partir da variável em questão.
pub fn entry_time_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.entry_time.cmp(&b.entry_time));
}

/// Executar os processos
///
/// Executa os processos a partir de um `quantum` de tempo (2 unidades de tempo) para ir retirando
/// da execução dos processos. Sendo preemptivo, ele sempre ordenará o valor de entrada do menor
/// para o maior, e checando se o tempo de execução de todos os processos já superou o tempo de
/// entrada do próximo processo.
///
/// * p: Endereço mutável de um vetor de Processos
/// * medium_return_time: Tempo Médio de Retorno
/// * medium_await_time: Tempo Médio de Espera
/// * _popped_proc: Processo que foi removido #variável não utilizada#
/// * response: tempo total de todos os processos
/// * proccess_cl_push: Variável que mantém o controle de quantos processos já foram clonados
/// * check_proccessed: Variável que mantém o controle de quantos processos já foram executados
/// * clone: Vetor de Processos clone do vetor principal.
/// * count: Valor máximo de tempo de execução
pub fn execute_proccesses(p: &mut Vec<Proccess>) {
    let mut response: i32 = 0;
    let mut _medium_return_time: i32 = 0;
    let mut _medium_await_time: i32 = 0;
    let mut _popped_proc: Option<Proccess>;
    let mut i: usize = 0;
    let quantum: i32 = 2;
    let mut count = 0;
    let mut proccess_cl_push: usize = 1;
    let mut check_proccessed = 0;

    let mut clone: Vec<Proccess> = Vec::new();
    clone.push(p[i].clone());

    for i in &mut *p {
        count += i.execution_time;
    }

    while count != 0 || check_proccessed == p.len() {
        if !clone[0].proccessed {
            if clone[0].execution_time >= quantum {
                clone[0].execution_time -= quantum;
                clone[0].turnaround_time = response + quantum;
                count -= quantum;
                response += quantum;
            } else if clone[0].execution_time == 1 {
                clone[0].execution_time -= 1;
                clone[0].turnaround_time = response + 1;
                count -= 1;
                response += 1;
            }
            if clone[0].execution_time == 0 {
                clone[0].await_time = response - clone[0].entry_time;
                clone[0].proccessed = true;
                check_proccessed += 1;
                _popped_proc = Some(clone.remove(0));
                clone.push(Option::expect(_popped_proc, "Failed"));
            }
        }
        if proccess_cl_push < p.len() {
            if response >= p[proccess_cl_push].entry_time {
                clone.push(Proccess::new(
                    p[proccess_cl_push].name.clone(),
                    p[proccess_cl_push].entry_time,
                    p[proccess_cl_push].execution_time,
                ));
                proccess_cl_push += 1;
            }
        }
        clone.rotate_left(1);
    }
    entry_time_sort(&mut clone);
    loop {
        _medium_return_time += clone[i].turnaround_time;
        p[i].turnaround_time = clone[i].turnaround_time;
        p[i].await_time = clone[i].await_time - p[i].execution_time;
        _medium_await_time += p[i].await_time;

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

/// Armazenar número do input
///
/// Retorna um valor inteiro de 32bits a partir do valor inserido pelo usuário. A princípio começa
/// como String, e mudamos o valor para inteiro.
///
/// * num: String -> i32 via .parse();
pub fn get_number() -> i32 {
    let mut num: String = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Couldn't read a number");
    let num: i32 = num.trim().parse().expect("Error while parsing");

    num
}
