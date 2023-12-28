use std::io::{self, BufRead};
use std::process::Command;

use chrono::Utc;
fn main() {
    Command::new("src/open-media.appl")
        .output()
        .expect("Failed to execute command");
    let stdin = io::stdin();
    let mut stdin = stdin.lock(); // locking is optional

    let mut line = String::new();

    while let Ok(n_bytes) = stdin.read_line(&mut line) {
        if n_bytes == 0 {
            break;
        }

        print!("{}", line);
        Command::new("src/write-log.appl")
            .arg(format!(
                "{} {}",
                Utc::now().format("%Y-%m-%dT%H:%M:%S"),
                line.to_string()
            ))
            .output()
            .expect("Failed to execute command");
        line.clear()
    }
}
