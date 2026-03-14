use std::fs;
use std::path::PathBuf;

pub fn create(project_path: &PathBuf, project_name: &str, spring_boot: bool, spring_version: &str) -> std::io::Result<()> {
    let path = project_path.join("pom.xml");


    let parent_section = if spring_boot {
        format!(
r#"    <parent>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-starter-parent</artifactId>
        <version>{spring_version}</version>
    </parent>"#)
    } else {
        String::new()
    };


    let dependencies_section = if spring_boot {
        r#"    <dependencies>
        <dependency>
            <groupId>org.springframework.boot</groupId>
            <artifactId>spring-boot-starter</artifactId>
        </dependency>
    </dependencies>"#
    } else {
        ""
    };


    let content = format!(
r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
    <modelVersion>4.0.0</modelVersion>
{parent_section}

    <groupId>com.example</groupId>
    <artifactId>{project_name}</artifactId>
    <version>1.0</version>

    <properties>
        <maven.compiler.source>21</maven.compiler.source>
        <maven.compiler.target>21</maven.compiler.target>
    </properties>
{dependencies_section}
</project>"#,
        parent_section = parent_section,
        project_name = project_name,
        dependencies_section = dependencies_section
    );

    fs::write(path, content)
}