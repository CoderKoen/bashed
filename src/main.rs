use std::io::{self, Write};

fn main() {
    println!("Bashed v0.0.1, by Koen Rivers");
    let mut user = String::from("root");
    let mut location = String::from("/");

    loop {
        let mut command = String::new();

        if user == "root" {
            print!("[{}]# ",location);
        } else {
            print!("[{}]$ ",location);
        }

        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut command).expect("Failed to read line");
    }
}
