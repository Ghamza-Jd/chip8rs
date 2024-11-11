use crate::specs::SPECS;

pub struct Emu {
    /// Program counter
    pc: u16,
    ram: [u8; SPECS.ram_size],
    screen: [bool; SPECS.screen_w * SPECS.screen_h],
    /// Registers
    v_reg: [u8; SPECS.registers_count],
    /// Ram index register
    i_reg: u16,
    /// Stack pointer
    sp: u16,
    stack: [u16; SPECS.stack_size],
    keys: [bool; SPECS.keys_count],
    /// Delay timer
    dt: u8,
    /// Sound timer
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            pc: SPECS.start_addr,
            ram: [0; SPECS.ram_size],
            screen: [false; SPECS.screen_w * SPECS.screen_h],
            v_reg: [0; SPECS.registers_count],
            i_reg: 0,
            sp: 0,
            stack: [0; SPECS.stack_size],
            keys: [false; SPECS.keys_count],
            dt: 0,
            st: 0,
        }
    }
}
