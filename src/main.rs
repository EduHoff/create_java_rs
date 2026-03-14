use std::io::{self, Write};
use std::path::PathBuf;
use std::fs;
use clearscreen::clear;
use regex::Regex;


fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {

    let base_dir = std::env::current_dir().expect("Failed to get current directory");
    
    let java_project_name_pattern = Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$").unwrap();
    let mut project_path = PathBuf::new();
    let mut java_project_name = String::new();

    // PROJECT NAME
    clear().expect("Fail to clear screen");
    loop {
        java_project_name = get_input("Enter the Java project name: ");

        if !java_project_name_pattern.is_match(&java_project_name) {
            clear().expect("Fail to clear screen");
            println!("Failure! Invalid or unconventional name");
            continue;
        }

        project_path = base_dir.join(&java_project_name);
        if project_path.exists() {
            clear().expect("Fail to clear screen");
            println!("Failure! Name is already in use");
            continue;
        }

        fs::create_dir_all(&project_path).expect("Failed to create directory");
        break;
    }

    // BUILD TOOL
    let mut build_tool = String::new();
    loop {
        clear().expect("Fail to clear screen");
        println!("1 | Vanilla \n2 | Maven \n3 | Gradle \n");
        let choice = get_input("|| ");

        match choice.as_str() {
            "1" => { build_tool = "vanilla".to_string(); break; }
            "2" => { build_tool = "maven".to_string(); break; }
            "3" => { build_tool = "gradle".to_string(); break; }
            _ => println!("Invalid option!"),
        }
    }

    // SPRING BOOT
    let mut spring_boot = false;
    if build_tool != "vanilla" {
        loop {
            clear().expect("Fail to clear screen");
            println!("Would you like to use Spring Boot?");
            let choice = get_input("1 | Yes \n2 | No \n\n|| ");

            match choice.as_str() {
                "1" => { spring_boot = true; break; }
                "2" => { spring_boot = false; break; }
                _ => println!("Invalid option!"),
            }
        }
    }

    println!("Projeto: {} | Build: {} | Spring Boot: {}", java_project_name, build_tool, spring_boot); // CÓDIGO DE TESTE AQUI 
}
