use std::fs;
use std::path::PathBuf;

pub fn create(project_path: &PathBuf, project_name: &str) -> std::io::Result<()> {
    let path = project_path.join("settings.gradle");
    let content = format!("rootProject.name = '{}'", project_name);
    
    fs::write(path, content)
}