const FONTSET_SIZE: usize = 80;

pub struct Specs {
    pub ram_size: usize,
    pub screen_w: usize,
    pub screen_h: usize,
    pub registers_count: usize,
    pub stack_size: usize,
    pub keys_count: usize,
    pub start_addr: u16,
    pub fontset_size: usize,
    pub fontset: [u8; FONTSET_SIZE],
}

pub const SPECS: Specs = Specs {
    ram_size: 4096,
    screen_w: 64,
    screen_h: 32,
    registers_count: 16,
    stack_size: 16,
    keys_count: 16,
    start_addr: 0x200,
    fontset_size: FONTSET_SIZE,
    fontset: [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ],
};
