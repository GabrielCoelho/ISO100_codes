pub struct Proccess {
    pub name: String,
    pub entry_time: i32,
    pub execution_time: i32,
    pub await_time: i32,
    pub turnaround_time: i32,
}

impl Proccess {
    pub fn new(p_name: String, p_entime: i32, p_exect: i32) -> Self {
        Proccess {
            name: p_name,
            entry_time: p_entime,
            execution_time: p_exect,
            await_time: 0,
            turnaround_time: 0,
        }
    }
}
