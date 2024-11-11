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
        self.exec(op);
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

    fn exec(&mut self, op: u16) {
        // h -> high, l -> low, b -> byte
        // e.g: hbln -> high byte low nibble
        let hbhn = (op & 0xF000) >> 12;
        let hbln = (op & 0x0F00) >> 8;
        let lbhn = (op & 0x00F0) >> 4;
        let lbln = op & 0x000F;

        let x = hbln as usize;
        let y = lbhn as usize;
        let nn = (op & 0xFF) as u8;
        let nnn = op & 0xFFF;

        match (hbhn, hbln, lbhn, lbln) {
            /* Noop */
            (0, 0, 0, 0) => return,
            /* CLS */
            (0, 0, 0xE, 0) => self.screen = [false; SPECS.screen_w * SPECS.screen_h],
            /* RET */
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }
            /* JMP nnn */
            (1, _, _, _) => {
                self.pc = nnn;
            }
            /* CALL nnn */
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = nnn;
            }
            /* SKIP vx == nn */
            (3, _, _, _) => {
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            }
            /* SKIP vx != nn */
            (4, _, _, _) => {
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            }
            /* SKIP vx == vy */
            (5, _, _, 0) => {
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }
            /* vx == nn */
            (6, _, _, _) => {
                self.v_reg[x] = nn;
            }
            /* vx += nn */
            (7, _, _, _) => {
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            }
            /* vx = vy */
            (8, _, _, 0) => {
                self.v_reg[x] = self.v_reg[y];
            }
            /* vx |= vy */
            (8, _, _, 1) => {
                self.v_reg[x] |= self.v_reg[y];
            }
            /* vx &= vy */
            (8, _, _, 2) => {
                self.v_reg[x] &= self.v_reg[y];
            }
            /* vx ^= vy */
            (8, _, _, 3) => {
                self.v_reg[x] ^= self.v_reg[y];
            }
            /* vx += vy */
            (8, _, _, 4) => {
                let (new_vx, has_carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if has_carry { 1 } else { 0 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            /* vx -= vy */
            (8, _, _, 5) => {
                let (new_vx, has_borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if has_borrow { 0 } else { 1 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            /* vx >>= 1 */
            (8, _, _, 6) => {
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }
            /* vx = vy - vx */
            (8, _, _, 7) => {
                let (new_vx, has_borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if has_borrow { 0 } else { 1 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            /* vx <<= 1 */
            (8, _, _, 0xE) => {
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }
            /* vx != vy */
            (9, _, _, 0) => {
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }
            /* I = nnn */
            (0xA, _, _, _) => {
                self.i_reg = nnn;
            }
            /* JMP v0 + nnn */
            (0xB, _, _, _) => {
                self.pc = (self.v_reg[0] as u16) + nnn;
            }
            /* vx = rand() & nn */
            (0xC, _, _, _) => {
                self.v_reg[x] = rand::random::<u8>() & nn;
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {op}"),
        }
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
