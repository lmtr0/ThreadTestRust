use std::{sync::{Arc, Mutex}, usize};

#[macro_use]
extern crate lazy_static;

pub struct Data {
    pub lenght: usize
}
impl Data {
    pub fn new() -> Data {
        Data {
            lenght: 0
        }
    }

    pub fn modifie_len(&mut self) -> &usize {
        self.lenght = 15;
        &self.lenght
    }
}

lazy_static! {
    static ref  DATA: Arc<Mutex<Data>>  = Arc::new(Mutex::new(Data::new()));
}

fn main() {
    println!("Hello, world!");
    println!("lenght is {}", DATA.lock().unwrap().lenght);
    println!("lenght modified to {}", DATA.lock().unwrap().modifie_len());
    println!("lenght is {}", DATA.lock().unwrap().lenght);
}
