use std::io::{self, Write};

use rust_shell::Cmd;

fn main() {
    // print!("Hello, world!: ");
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Input something");
    // let command: Vec<&str> = input.split_whitespace().collect(); 
    // println!("val: {}", input.trim());
    // println!("{:?}",command);
    
        loop {
            print!("Abimbola/Oladel: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Enter a command");
            let inputs:Vec<&str> = input.split_whitespace().collect();
            if inputs[0] == "exit" {
                break;
            }
            let cmd = Cmd::new(&inputs[0]);
            cmd.run();
        }


}
