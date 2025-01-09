use std::io::{self, Write};

use rust_shell::Cmd;

fn main() {
        loop {
            let  path = Cmd::get_current_dir();
          let path =  path.components().last().map(|component| component.as_os_str().to_str())
          .flatten();
          if path.is_none(){
            println!("Path is empty");
            break;
          }
            print!("{}: ",path.unwrap());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Enter a command");
            let inputs:Vec<&str> = input.split_whitespace().collect();
            if inputs[0] == "exit" {
                break;
            }
            let cmd = Cmd::new(&inputs[0]).args(&inputs[1..]);
            cmd.run();
        }


}
