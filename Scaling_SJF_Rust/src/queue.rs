pub struct Proccess {
    pub name: String,
    pub entry_time: u8,
    pub execution_time: u32,
}

impl Proccess {
    pub fn new(p_name: String, p_entime: u8, p_exect: u32) -> Self {
        Proccess {
            name: p_name,
            entry_time: p_entime,
            execution_time: p_exect,
        }
    }
}
