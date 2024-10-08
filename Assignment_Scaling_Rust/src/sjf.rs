// Copyright (c) 2024 Gabriel Coelho Soares. All Rights Reserved.
/// Imports
use crate::proccess::*;
use random_string::generate;
use std::io;

/// Enumeração do menu
enum MENU {
    _OptDesselect = 0,
    OptAdd = 1,
    OptShow = 2,
    OptExecute = 3,
    OptExit = 4,
}

/// Função SJF
///
/// Realiza o loop de acordo com o menu, chamando as funções conforme necessário.
pub fn main_sjf() {
    // Criação do Vetor de Processos e inicializando o vetor.
    let mut proc: Vec<Proccess> = Vec::new();
    // Criação da variável que pegará o valor do menu.
    let mut option: u8 = 0;
    println!("SJF Scaling in Rust");
    while option != MENU::OptExit as u8 {
        option = menu();
        match option {
            // Retorno 1 se a enumeração do menu for igual e já chamo as funções
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
    // Criação de uma variável do tipo String.
    let mut op = String::new();
    println!(
        "Choose an option\n{} - Add a Proccess\t{: >9} - List all proccesses\n{} - Execute all proccesses \t{} - Exit program\n\n",
        MENU::OptAdd as u8,
        MENU::OptShow as u8,
        MENU::OptExecute as u8,
        MENU::OptExit as u8
    );
    // Leitura do input do usuário
    io::stdin().read_line(&mut op).unwrap();

    // Shadowing (Sombreamento) da variável op para uma nova variável de tipo inteiro sem sinal de
    // 8 bites.
    let op: u8 = op.trim().parse().expect("Error while parsing");
    op // retorna o valor de op
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
        generate(5, random_string::charsets::ALPHA_LOWER), // gera uma string de 5 caracteres
        // randômicos e minúsculos
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

/// Ordenar pelo trabalho mais curto (Shortest Job First)
///
/// Ordena o vetor pelo tempo de execução de cada item contido nele.
///
/// * p: Endereço mutável de um vetor de Processos
/// * sort_by(|a, b|): Ordena através de uma verificação de dois itens do vetor sequenciais (a ->
/// b) onde o valor do tempo de execução de a comparado ao tempo de execução de b sejam do menor
/// para o maior.
///
/// Note: nós passamos o endereço de b na comparação pois, no momento da execução do programa, não temos como
/// saber o valor em si de b, mas sim de a (que é o primeiro valor a ser checado). Basicamente,
/// realiza um `bubble_sort` a partir da variável em questão.
pub fn sjf_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.execution_time.cmp(&b.execution_time));
}

/// Ordenar pelo menor tempo de entrada
///
/// * p: Endereço mutável de um vetor de Processos
pub fn entry_time_sort(p: &mut Vec<Proccess>) {
    p.sort_by(|a, b| a.entry_time.cmp(&b.entry_time));
}

/// Executar os processos
///
/// Ordena o vetor pelo menor trabalho, e inicializa um loop para executar os processos. Conforme
/// ocorre a execução, o cálculo de Tempo de Espera, Tempo de Turnaround, Tempo médio de espera e
/// Tempo médio de retorno é realizado. Ao fim, imprime todos os processos, juntamente com o
/// resultado dos cálculos e libera a memória executando um `pop()` de todo o vetor.
///
/// * p: Endereço mutável de um vetor de Processos
/// * medium_return_time: Tempo Médio de Retorno
/// * medium_await_time: Tempo Médio de Espera
/// * _popped_proc: Processo que foi removido #variável não utilizada#
/// * total_time: Tempo total decorrido durante a execução em `u.t.`
pub fn execute_proccesses(p: &mut Vec<Proccess>) {
    let mut medium_return_time: i32 = 0;
    let mut medium_await_time: i32 = 0;
    let mut _popped_proc: Option<Proccess>;
    let mut total_time: i32;
    let mut i = 0;

    sjf_sort(&mut *p);
    total_time = p[i].entry_time;
    loop {
        let mut j = i + 1;
        if j < p.len() {
            if p[i].entry_time > p[j].entry_time
                && total_time < p[i].entry_time
                && total_time >= p[j].entry_time
            {
                j = i;
                i = i + 1;
            }
        }
        if !p[i].proccessed {
            total_time += p[i].execution_time;
            p[i].turnaround_time = total_time - p[i].entry_time;
            p[i].await_time = total_time - p[i].execution_time - p[i].entry_time;

            medium_return_time += p[i].turnaround_time;
            medium_await_time += p[i].await_time;
            p[i].proccessed = true;
        }
        i = j;
        if i == p.len() {
            break;
        }
    }

    list_proccesses(&p);
    println!(
        "Await: {}\tResponse: {}\n",
        medium_await_time, medium_return_time
    );
    println!(
        "Medium Return Time: {:.4}",
        (medium_return_time as f32 / (p.len()) as f32) as f32
    );
    println!(
        "Medium Await Time: {:.4}",
        (medium_await_time as f32 / (p.len()) as f32) as f32
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
        .expect("Couldn't read any number");
    let num: i32 = num.trim().parse().expect("Error while parsing");

    num
}
