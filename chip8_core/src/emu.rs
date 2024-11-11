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
            /* JMP NNN */
            (1, _, _, _) => self.pc = nnn,
            /* CALL NNN */
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = nnn;
            }
            /* SKIP VX == NN */
            (3, _, _, _) => {
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            }
            /* SKIP VX != NN */
            (4, _, _, _) => {
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            }
            /* SKIP VX == VY */
            (5, _, _, 0) => {
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }
            /* VX == NN */
            (6, _, _, _) => self.v_reg[x] = nn,
            /* VX += NN */
            (7, _, _, _) => self.v_reg[x] = self.v_reg[x].wrapping_add(nn),
            /* VX = VY */
            (8, _, _, 0) => self.v_reg[x] = self.v_reg[y],
            /* VX |= VY */
            (8, _, _, 1) => self.v_reg[x] |= self.v_reg[y],
            /* VX &= VY */
            (8, _, _, 2) => self.v_reg[x] &= self.v_reg[y],
            /* VX ^= VY */
            (8, _, _, 3) => self.v_reg[x] ^= self.v_reg[y],
            /* VX += VY */
            (8, _, _, 4) => {
                let (new_vx, has_carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if has_carry { 1 } else { 0 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            /* VX -= VY */
            (8, _, _, 5) => {
                let (new_vx, has_borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if has_borrow { 0 } else { 1 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            /* VX >>= 1 */
            (8, _, _, 6) => {
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }
            /* VX = VY - VX */
            (8, _, _, 7) => {
                let (new_vx, has_borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if has_borrow { 0 } else { 1 };
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            /* VX <<= 1 */
            (8, _, _, 0xE) => {
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }
            /* VX != VY */
            (9, _, _, 0) => {
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }
            /* I = NNN */
            (0xA, _, _, _) => self.i_reg = nnn,
            /* JMP V0 + NNN */
            (0xB, _, _, _) => self.pc = (self.v_reg[0] as u16) + nnn,
            /* VX = rand() & NN */
            (0xC, _, _, _) => self.v_reg[x] = rand::random::<u8>() & nn,
            /* Draw */
            (0xD, _, _, _) => self.draw(hbln, lbhn, lbln),
            /* SKIP KEY PRESS */
            (0xE, _, 9, 0xE) => {
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            }
            /* SKIP KEY RELEASE */
            (0xE, _, 0xA, 1) => {
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            }
            /* VX = DT */
            (0xF, _, 0, 7) => self.v_reg[x] = self.dt,
            /* WAIT KEY */
            (0xF, _, 0, 0xA) => {
                let mut pressed = false;
                for (idx, key) in self.keys.iter().enumerate() {
                    if *key {
                        self.v_reg[x] = idx as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // Redo opcode
                    self.pc -= 2;
                }
            }
            /* DT = VX */
            (0xF, _, 1, 5) => self.dt = self.v_reg[x],
            /* ST = VX */
            (0xF, _, 1, 8) => self.st = self.v_reg[x],
            (0xF, _, 1, 0xE) => {
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            }
            /* I = FONT */
            (0xF, _, 2, 9) => {
                let c = self.v_reg[x] as u16;
                self.i_reg = c * 5;
            }
            /* BCD */
            (0xF, _, 3, 3) => self.bcd(hbln),
            /* STORE V0 -> VX (inclusive) into I */
            (0xF, _, 5, 5) => {
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            }
            /* LOAD V0 -> VX (inclusive) from I */
            (0xF, _, 6, 5) => {
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {op}"),
        }
    }

    fn bcd(&mut self, hbln: u16) {
        let x = hbln as usize;
        let vx = self.v_reg[x] as f32;

        let hundreds = (vx / 100.0).floor() as u8;
        let tens = ((vx / 10.0) % 10.0).floor() as u8;
        let ones = (vx % 10.0) as u8;

        self.ram[self.i_reg as usize] = hundreds;
        self.ram[(self.i_reg + 1) as usize] = tens;
        self.ram[(self.i_reg + 2) as usize] = ones;
    }

    fn draw(&mut self, hbln: u16, lbhn: u16, lbln: u16) {
        let x = hbln as usize;
        let y = lbhn as usize;

        let x_coord = self.v_reg[x] as u16;
        let y_coord = self.v_reg[y] as u16;

        // The last nibble determins how many rows high our sprite is
        let num_rows = lbln;
        // keep track if any pixels were flipped
        let mut flipped = false;

        // Iterate over each row of our sprite
        for y_line in 0..num_rows {
            // Determine which memory address our row's data is stored
            // i_reg contains the start address of the sprite to draw
            let addr = self.i_reg + y_line as u16;
            let pixels = self.ram[addr as usize];
            // Iterate over each column in our row
            for x_line in 0..8 {
                // Use a mask to fetch current pixel's bit. Only flip if a 1
                if (pixels & (0b1000_0000 >> x_line)) != 0 {
                    // Sprites should wrap around screen, so apply modulo
                    let x = (x_coord + x_line) as usize % SPECS.screen_w;
                    let y = (y_coord + y_line) as usize % SPECS.screen_h;

                    // Get our pixel's index for our 1D screen array
                    let idx = x + SPECS.screen_w * y;
                    // Check if we're about to flip the pixel and set
                    flipped |= self.screen[idx];
                    self.screen[idx] ^= true;
                }
            }
        }

        // Populate VF register
        if flipped {
            self.v_reg[0xF] = 1;
        } else {
            self.v_reg[0xF] = 0;
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
