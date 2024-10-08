/// Estrutura do Processo
///
/// Deriva a função clone para o uso no Round Robin
#[derive(Clone)]
pub struct Proccess {
    pub name: String,
    pub entry_time: i32,
    pub execution_time: i32,
    pub await_time: i32,
    pub turnaround_time: i32,
    pub proccessed: bool,
}

impl Proccess {
    /// Criar novo Processo
    ///
    /// Cria um novo processo (retornando o próprio Processo) a partir de alguns parâmetros de
    /// entrada. Inicia o tempo de espera, o tempo de retorno (turnaround) em zero, e o valor
    /// `proccessed` como falso.
    ///
    /// * p_name: Nome do processo
    /// * p_entime: Valor em Unidade de Tempo da entrada do processo
    /// * p_exect: Valor em Unidade de Tempo da execução do processo
    pub fn new(p_name: String, p_entime: i32, p_exect: i32) -> Self {
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
