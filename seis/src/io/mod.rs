pub mod grd;
pub mod seg;


use std::io::prelude::*;

fn u8_arr_to_string(u8_arr: Vec<u8>) -> String {
    let mut rs = String::new();
    for c in &u8_arr {
        let ch = *c as char;
        rs.push(ch);
    }
    rs
}

pub trait ReadEx {
    fn read_str(&mut self, n: usize) -> String;
}

impl<T: Read> ReadEx for T {
    fn read_str(&mut self, n: usize) -> String {
        let mut u8_arr_buf = vec![0u8; n];
        self.read_exact(&mut u8_arr_buf[..])
            .expect("Error in read buffer");
        u8_arr_to_string(u8_arr_buf)
    }
}
