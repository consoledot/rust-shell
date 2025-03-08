use std::{ env, fs::{ self, read_dir, File }, io, path::{ PathBuf, Path } };

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
            // "rmdir" => self.delete_directory(),
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
    // fn peak_directory(&self) {
    //     let mut files: Vec<PathBuf> = Vec::new();
    //     let path = Self::get_current_dir();
    //     for entry in read_dir(path).unwrap() {
    //         let entry = entry.unwrap();
    //         let path = entry.path();

    //         if path.is_file() {
    //             files.push(path);
    //         }
    //     }
    //     for file in files {
    //         println!(
    //             "{}",
    //             file
    //                 .components()
    //                 .last()
    //                 .map(|component| component.as_os_str().to_str())
    //                 .flatten()
    //                 .unwrap()
    //         );
    //     }

    //     // Ok(())
    // }

    fn peak_directory(&self) {
        let path = Self::get_current_dir();
        Self::peak_directory_recursive(&path);
    }

    fn peak_directory_recursive(path: &Path) {
        if let Ok(entries) = read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let file_name = entry.file_name().to_string_lossy().into_owned();

                println!("{}", file_name);
            }
        } else {
            eprintln!("Failed to read directory: {:?}", path);
        }
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
    // fn delete_directory(&self) {
    //     self.handle_file_operation(fs::read_dir, "directory");
    // }
    // fn handle_files<F>(&self, handler: F, msg: &str) -> io::Result<()>
    //     where F: FnOnce(String) -> io::Result<String>
    // {
    //     match &self.args {
    //         Some(arg) => {
    //             if arg.is_empty() {
    //                 println!("{}", msg);
    //                 return Ok(());
    //             }
    //             match handler(arg[0].clone()) {
    //                 Ok(_) => Ok(()),
    //                 Err(_) => {
    //                     println!("{}", msg);
    //                     Err(_)
    //                 }
    //             }
    //         }
    //         None => {
    //             println!(" not found");
    //             Ok(())
    //         }
    //     }
    // }

    // fn handle_file_operation(&self, operation: fn(&str) -> io::Result<()>, entity_type: &str) {
    //     match &self.args {
    //         Some(args) => {
    //             if args.is_empty() {
    //                 println!("Enter a valid {} name", entity_type);
    //                 return;
    //             }
    //             match operation(&args[0]) {
    //                 Ok(_) => (),
    //                 Err(_) => {
    //                     println!("Enter a valid {} name", entity_type);
    //                 }
    //             }
    //         }
    //         None => {
    //             println!("{} not found", entity_type);
    //         }
    //     }
    // }
}
