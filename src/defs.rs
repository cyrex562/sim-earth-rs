
pub struct Address {
    pub lower: u16,
    pub upper: u16,
}

pub struct Code {
    pub address: Address,
}

impl Code {
    pub fn set_addr_from_u32(&mut self, a: u32) {
        todo!()
    }
}
