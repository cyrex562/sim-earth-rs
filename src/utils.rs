pub fn CONCAT22(a: u16, b: u16) -> u32 {
    ((a << 16) | b) as u32
}

pub fn CONCAT11(a: u8, b: u8) -> u16 {
    ((a << 8) | b) as u16
}

pub fn CONCAT13(a: u8, b: u16) -> u32 {
    unimplemented!()
}

pub fn CONCAT12(a: u8, b: u16) -> u32 {
    unimplemented!()
}

pub fn make_ptr_from_u16(addr: u16) -> *mut u16 {
    unimplemented!()
}
