mod tokenize;

mod commands;




use std::io::{self, Write};

// fn run_command(main_command: String, args: Vec<String>)
// {

// }

fn main() {
    println!("Bashed v0.0.1, by Koen Rivers");
    let user = String::from("root");
    let location = String::from("/");
    // let files: Vec<Vec<String>> = vec![
    //     vec!["None","/","Folder","rw-"].iter().map(|s| s.to_string()).collect()
    // ];

    loop {
        let mut command = String::new();
        //let (main, arguments) = tokenize(&command);
    
        if user == "root" {
            print!("[{}]# ",location);
        } else {
            print!("[{}]$ ",location);
        }

        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut command).expect("Failed to read line");

    }
}
