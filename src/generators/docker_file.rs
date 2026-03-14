use std::fs;
use std::path::PathBuf;

pub fn create(project_path: &PathBuf, build_tool: &str, spring_boot: bool, docker_image_external: &mut String) -> std::io::Result<()> {
    let path = project_path.join("Dockerfile");
    
    let (docker_image, docker_command, docker_copy) = match build_tool {
        "vanilla" => (
            "eclipse-temurin:21-jdk-jammy".to_string(),
            "sh -c 'javac -d bin $(find src -name \"*.java\") && java -cp bin application.Program'".to_string(),
            "COPY src ./src".to_string(),
        ),
        "maven" => (
            "maven:3.9.6-eclipse-temurin-21".to_string(),
            "mvn -q compile exec:java -Dexec.mainClass='application.Program'".to_string(),
            "COPY pom.xml .\nCOPY src ./src".to_string(),
        ),
        "gradle" => {
            let run_cmd = if spring_boot { "bootRun" } else { "run" };
            (
                "gradle:8.6-jdk21".to_string(),
                format!("gradle {} -q --console=plain", run_cmd),
                "COPY build.gradle settings.gradle .\nCOPY src ./src".to_string(),
            )
        },
        _ => (String::new(), String::new(), String::new()),
    };

    *docker_image_external = docker_image.clone(); // OR: docker_image_external.push_str(&docker_image);

    let content = format!(
r#"FROM {docker_image}

WORKDIR /app

{docker_copy}

CMD {docker_command}"#);

    fs::write(path, content)
}