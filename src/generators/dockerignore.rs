use std::fs;
use std::path::PathBuf;

pub fn create(project_path: &PathBuf) -> std::io::Result<()> {
    let file_path = project_path.join(".dockerignore");
    let content = 
r#"# Git
.git
.gitignore

# IDEs
.metadata
.idea
.vscode
.settings
nbproject
.project
.classpath
*.iml
*.iws

# Build output
target
build
out
nbbuild
dist
nbdist
.gradle

# Logs
*.log

# OS files
.DS_Store
Thumbs.db

# Temporary files
tmp
*.tmp
*.bak
*.swp
*~
.cache-main
.scala_dependencies
.worksheet
.history
.ionide

# Heap dump
*.hprof

# VSCode extensions
*.vsix

# Maven timing
.mvn/timing.properties"#;
    fs::write(file_path, content)
}