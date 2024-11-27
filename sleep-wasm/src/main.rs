use std::{thread, time::Duration};

fn main() {
    println!("wasm: sleep start");
    thread::sleep(Duration::from_millis(600000));
    println!("wasm: sleep end");
}
