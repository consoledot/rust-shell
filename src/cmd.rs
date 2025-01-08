use std::{env, path::PathBuf};



pub struct Cmd {
    command: String
}

impl Cmd {
    pub fn new(command: &str) -> Cmd{
        Cmd{command:command.to_string()}
    }
    pub fn run(&self){

    
            match self.command.as_str() {
                "ls"=> self.peak_directory(),
                "pwd"=> {
                   let path: PathBuf = self.get_current_dir();
                   println!("{:?}", path)
                },
                _ => println!("Unkwon commad")
                   
                

                
            }
        
    }

    fn get_current_dir(&self)-> PathBuf{
        match env::current_dir() {
            Ok(path)=> {
                
                path
            },
            Err(err)=>{
                println!("Error getting current directory");
                PathBuf::new()
            }
            
        }
           
    }
    fn peak_directory(&self){
        println!("Ls baby")
    }
}