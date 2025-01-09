use std::{ env, fs::{ self, read_dir, File }, io, path::PathBuf };

pub struct Cmd {
    command: String,
    args: Option<Vec<String>>,
}

impl Cmd {
    pub fn new(command: &str) -> Cmd {
        Cmd { command: command.to_string(), args: None }
    }
    pub fn args(&self, args: &[&str]) -> Cmd {
        Cmd {
            command: self.command.clone(),
            args: Some(
                args
                    .iter()
                    .map(|&s| s.to_string())
                    .collect()
            ),
        }
    }
    pub fn run(&self) {
        match self.command.as_str() {
            "ls" => self.peak_directory(),
            "pwd" => {
                let path: PathBuf = Self::get_current_dir();
                match path.to_str() {
                    Some(path) => println!("{}", path),
                    None => println!("Unknown path"),
                }

                //    println!("{:#?}", path.to_str())
            }
            "torch" => self.create_file(),
            "rm" => self.delete_file(),
            "mkdir" => self.create_directory(),
            "rmdir" => self.delete_directory(),
            _ => println!("Unknown command"),
        }
    }

    pub fn get_current_dir() -> PathBuf {
        match env::current_dir() {
            Ok(path) => { path }
            Err(err) => {
                println!("Error getting current directory");
                PathBuf::new()
            }
        }
    }
    fn peak_directory(&self) {
        let mut files: Vec<PathBuf> = Vec::new();
        let path = Self::get_current_dir();
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                files.push(path);
            }
        }
        for file in files {
            println!(
                "{}",
                file
                    .components()
                    .last()
                    .map(|component| component.as_os_str().to_str())
                    .flatten()
                    .unwrap()
            );
        }

        // Ok(())
    }

    fn create_file(&self) {
        match &self.args {
            Some(arg) => {
                if arg.is_empty() {
                    println!("Enter a valid filename");
                    return;
                }
                match File::create(arg[0].clone()) {
                    Ok(file) => (),
                    Err(err) => println!("enter a valid file name"),
                }
            }
            None => { println!("File not found") }
        }
    }

    fn delete_file(&self) {
        match &self.args {
            Some(arg) => {
                if arg.is_empty() {
                    println!("Enter a valid filename");
                    return;
                }
                match fs::remove_file(arg[0].clone()) {
                    Ok(file) => (),
                    Err(err) => println!("enter a valid file name"),
                }
            }
            None => { println!("File not found") }
        }
    }
    fn create_directory(&self) {
        match &self.args {
            Some(arg) => {
                if arg.is_empty() {
                    println!("Enter a valid directory name");
                    return;
                }
                match fs::create_dir(arg[0].clone()) {
                    Ok(file) => (),
                    Err(err) => println!("enter a valid directory name"),
                }
            }
            None => { println!("File not found") }
        }
    }
    fn delete_directory(&self) {
        match &self.args {
            Some(arg) => {
                if arg.is_empty() {
                    println!("Enter a valid directory name");
                    return;
                }
                match fs::remove_dir(arg[0].clone()) {
                    Ok(file) => (),
                    Err(err) => println!("enter a valid directory name"),
                }
            }
            None => { println!("File not found") }
        }
    }
}
