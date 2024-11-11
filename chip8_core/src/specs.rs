pub struct Specs {
    pub ram_size: usize,
    pub screen_w: usize,
    pub screen_h: usize,
    pub registers_count: usize,
    pub stack_size: usize,
    pub keys_count: usize,
    pub start_addr: u16,
}

pub const SPECS: Specs = Specs {
    ram_size: 4096,
    screen_w: 64,
    screen_h: 32,
    registers_count: 16,
    stack_size: 16,
    keys_count: 16,
    start_addr: 0x200,
};
