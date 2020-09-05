use clap::{ ArgMatches };
use crate::utils::*;
use std::thread;
use std::time::Duration;

#[allow(unused_variables)]
pub fn run_command(sub_m: &ArgMatches) {
    let system = load_todolst();
    system.add_callback(|item| {
        let item = item.upgrade().unwrap();
        let item = item.lock().unwrap();
        println!("Item coming: id: {}, msg: {}", item.id(), item.message());
    });
    loop {
        thread::sleep(Duration::from_micros(1000));
    }
}