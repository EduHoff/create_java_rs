use std::fs;
use std::path::PathBuf;

pub fn create(src_path: &PathBuf) -> std::io::Result<()> {
    let file_path = src_path.join("Program.java");
    let content = 
r#"package application;

public class Program {

    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}"#;
    fs::write(file_path, content)
}