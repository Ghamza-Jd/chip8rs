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
        let mut emu = Self {
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
        };

        emu.ram[..SPECS.fontset_size].copy_from_slice(&SPECS.fontset);

        emu
    }

    pub fn reset(&mut self) {
        self.pc = SPECS.start_addr;
        self.ram = [0; SPECS.ram_size];
        self.screen = [false; SPECS.screen_w * SPECS.screen_h];
        self.v_reg = [0; SPECS.registers_count];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; SPECS.stack_size];
        self.keys = [false; SPECS.keys_count];
        self.dt = 0;
        self.st = 0;
        self.ram[..SPECS.fontset_size].copy_from_slice(&SPECS.fontset)
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        // decode
        // exec
    }

    pub fn tick_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // beep
            }
            self.st -= 1;
        }
    }

    fn fetch(&mut self) -> u16 {
        let high_byte = self.ram[self.pc as usize] as u16;
        let low_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (high_byte << 8) | low_byte;
        self.pc += 2;
        op
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
