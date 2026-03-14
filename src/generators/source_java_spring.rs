use std::fs;
use std::path::PathBuf;

pub fn create(src_path: &PathBuf) -> std::io::Result<()> {
    let file_path = src_path.join("Program.java");
    let content = 
r#"package application;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.ApplicationContext;

@SpringBootApplication
public class Program {

    public static void main(String[] args) {

        ApplicationContext context = SpringApplication.run(Program.class, args);

        System.out.println("--- Spring Boot Test ---");
        System.out.println("Spring Boot is running!");

        int beanCount = context.getBeanDefinitionCount();
        System.out.println("Loaded components (beans): " + beanCount);
    }
}"#;
    fs::write(file_path, content)
}