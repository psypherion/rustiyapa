use std;
extern crate lazy_static;
extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process::exit;
use std::io::{self, Write};

lazy_static::lazy_static! {
    static ref SUMMED: std::sync::Mutex<i32> = std::sync::Mutex::new(0);
}
fn input(content:String) -> String{
    print!("{}", content);
    io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is printed
    let mut buff: String = String::new();
    io::stdin().read_line(&mut buff).expect("Failed to read line");
    // _ = std::io::stdin().read_line(&mut buff).expect("Couldn't read the input...");
    return buff.trim().to_string();
}

fn sum(num:i32) -> i32{
    let mut summed = SUMMED.lock().unwrap();
    *summed += num;
    return *summed;
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\nCtrl+C pressed! Exiting...");
        r.store(false, Ordering::SeqCst); 
    }).expect("Error setting Ctrl+C handler");

    println!("Program started. Press Ctrl+C to exit.");
    let mut count:i8 = 0;
    while running.load(Ordering::SeqCst) {
        count += 1;
        let content:String = format!("Enter the value of no. {}: ", count);
        let input_str = input(content);
        let num: i32 = match input_str.parse() {
            Ok(n)=>n,
            Err(_)=>{
                println!("No New values are added...");
                continue;
            }
        };
        let x:i32 = sum(num);
        println!("Current Summed Value : {}", x);
    }
    println!("Exiting...");
    exit(0);
}
