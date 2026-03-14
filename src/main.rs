mod generators;
use clearscreen::clear;
use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

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

    // PROJECT NAME
    clear().expect("Fail to clear screen");
    let (java_project_name, project_path) = loop {
    let java_project_name = get_input("Enter the Java project name: ");

    if !java_project_name_pattern.is_match(&java_project_name) {
        clear().expect("Fail to clear screen");
        println!("Failure! Invalid or unconventional name");
        continue;
    }

    let project_path = base_dir.join(&java_project_name);
    if project_path.exists() {
        clear().expect("Fail to clear screen");
        println!("Failure! Name is already in use");
        continue;
    }

    fs::create_dir_all(&project_path).expect("Failed to create directory");
    
    break (java_project_name, project_path); 
};

    // BUILD TOOL
    clear().expect("Fail to clear screen");
    let build_tool = loop {
    println!("1 | Vanilla \n2 | Maven \n3 | Gradle \n");
    let choice = get_input("|| ");

    match choice.as_str() {
        "1" => break "vanilla".to_string(),
        "2" => break "maven".to_string(),
        "3" => break "gradle".to_string(),
        _ => {
            clear().expect("Fail to clear screen");
            println!("Invalid option!");
        }
    }
};

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
                _ => {
                    clear().expect("Fail to clear screen");
                    println!("Invalid option!"); 
                },
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

    let mut docker_image = String::new();
    generators::docker_file::create(&project_path, &build_tool, spring_boot, &mut docker_image).expect("Fail to create Dockerfile");
    generators::docker_compose_yml::create(&project_path, &java_project_name, &build_tool, spring_boot).expect("Fail to create docker-compose.yml");
    


    clear().expect("Fail to clear screen");
    println!("--- Project Successfully Generated ---");
    println!("Operating System: {}", env::consts::OS);
    println!("Project name: {}", java_project_name);
    println!("Build Tool: {}", build_tool.to_uppercase());
    println!("Imagem Docker: {docker_image}");
    if spring_boot {
        println!("Spring Boot: {}", SPRING_VERSION);
        if build_tool == "gradle"{
            println!("Gradle Dep. Management: {GRADLE_DEP_MANAGEMENT_VERSION}")
        }
    }
    println!("----------------------------------");


    loop {
        println!("Would you like to start the project?");
        let choice = get_input("1 | Yes \n2 | No \n\n|| ");

        match choice.as_str() {
            "1" => {
                println!("Trying to start Docker Compose...");
                
                let status = Command::new("docker")
                    .arg("compose")
                    .arg("up")
                    .arg("--build")
                    .current_dir(&project_path)
                    .status();

                match status {
                    Ok(s) if s.success() => println!("Project successfully completed."),
                    _ => println!("Failed to execute Docker. Is the daemon running?"),
                }
                break;
            }
            "2" => break,
            _ => {
                clear().expect("Fail to clear screen");
                println!("Invalid option!");
            }
        }
    }

    println!("Project created on: {:?}", project_path);
}