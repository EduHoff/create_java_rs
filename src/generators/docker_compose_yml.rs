use std::fs;
use std::path::PathBuf;

pub fn create(project_path: &PathBuf, java_project_name: &str, build_tool: &str, spring_boot: bool) -> std::io::Result<()> {
    let path = project_path.join("docker-compose.yml");
    let name_low = java_project_name.to_lowercase();

    let docker_command = match build_tool {
        "vanilla" => "sh -c 'javac -d bin $(find src -name \"*.java\") && java -cp bin application.Program'",
        "maven" => "mvn -q compile exec:java -Dexec.mainClass='application.Program'",
        "gradle" => if spring_boot { "gradle bootRun -q --console=plain" } else { "gradle run -q --console=plain" },
        _ => "",
    };

    let content = format!(
r#"services:
  {name_low}:
    build: .
    volumes:
      - .:/app
    stdin_open: true
    tty: true
    command: {docker_command}"#);

    fs::write(path, content)
}