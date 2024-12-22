#[derive(Debug, Default)]
pub struct State {
    pub pc: usize,
    pub registers: RegisterState,
    pub out: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct RegisterState {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}
