/// # The Ailis CPU
/// 
/// Structure is of a register-based vm.
struct AilisCPU {
    registers: [i32;4],
    pc: usize,
    halted: bool,
}

pub struct OPCODE(u16);