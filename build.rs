use std::process::{exit, Command};

fn main() {
    match Command::new("./tailwindcss")
        .arg("-c")
        .arg("tailwind.config.js")
        .arg("-o")
        .arg("styles/tailwind.css")
        .arg("--minify")
        .output() {
        Ok(_) => {
            println!("Tailwind CSS build successful!");
            exit(0);
        },
        Err(error) => {
            println!("Tailwind CSS build failed!");
            println!("error: {}", error);
            exit(1);
        }
    }
}