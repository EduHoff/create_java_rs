use std::io::{self, Write};
use std::path::PathBuf;
use std::fs;
use clearscreen::clear;
use regex::Regex;
mod generators;

const SPRING_VERSION: &str = "3.2.0";
const GRADLE_DEP_MANAGEMENT_VERSION: &str = "1.1.4";

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
    clear().expect("Fail to clear screen");
    let mut build_tool = String::new();
    loop {
        println!("1 | Vanilla \n2 | Maven \n3 | Gradle \n");
        let choice = get_input("|| ");

        match choice.as_str() {
            "1" => { build_tool = "vanilla".to_string(); break; }
            "2" => { build_tool = "maven".to_string(); break; }
            "3" => { build_tool = "gradle".to_string(); break; }
            _ => {clear().expect("Fail to clear screen"); println!("Invalid option!"); },
        }
    }

    // SPRING BOOT
    clear().expect("Fail to clear screen");
    let mut spring_boot = false;
    if build_tool != "vanilla" {
        loop {
            clear().expect("Fail to clear screen");
            println!("Would you like to use Spring Boot?");
            let choice = get_input("1 | Yes \n2 | No \n\n|| ");

            match choice.as_str() {
                "1" => { spring_boot = true; break; }
                "2" => { spring_boot = false; break; }
                _ => {clear().expect("Fail to clear screen"); println!("Invalid option!"); },
            }
        }
    }


    let src_path = if build_tool == "vanilla" {
        project_path.join("src").join("application")
    } else {
        project_path.join("src").join("main").join("java").join("application")
    };
    fs::create_dir_all(&src_path).expect("Failed to create source directories");


    if spring_boot {
        generators::source_java_spring::create(&src_path).expect("Failed to generate Java Spring Boot source file");
    } else {
        generators::source_java_standard::create(&src_path).expect("Failed to generate Standard Java source file");
    };

    generators::gitignore::create(&project_path).expect("Fail to create .gitignore");
    generators::dockerignore::create(&project_path).expect("Fail to create .dockerignore");
    generators::license::create(&project_path).expect("Fail to create LICENSE");
    generators::readme_md::create(&project_path, &java_project_name).expect("Fail to create LICENSE");

    if build_tool == "maven" {
        generators::pom_xml::create(&project_path, &java_project_name, spring_boot, &SPRING_VERSION).expect("Fail to create pom.xml");
    }

    if build_tool == "gradle" {
        generators::settings_gradle::create(&project_path, &java_project_name).expect("Fail to create settings.gradle");
        generators::build_gradle::create(&project_path, spring_boot, &SPRING_VERSION, &GRADLE_DEP_MANAGEMENT_VERSION).expect("Fail to create build.gradle");
    }


    println!("Projeto: {} | Build: {} | Spring Boot: {}", java_project_name, build_tool, spring_boot); // CÓDIGO DE TESTE AQUI 
}
