use std::fs;
use std::path::PathBuf;

pub fn create(project_path: &PathBuf, spring_boot: bool, spring_version: &str, gradle_dep_mgmt_version: &str) -> std::io::Result<()> {
    let path = project_path.join("build.gradle");

    let plugins_section = if spring_boot {
        format!(
r#"    id 'java'
    id 'org.springframework.boot' version '{spring_version}'
    id 'io.spring.dependency-management' version '{gradle_dep_mgmt_version}'"#)
    } else {
        "    id 'java'\n    id 'application'".to_string()
    };


    let dependencies_section = if spring_boot {
        r#"
dependencies {
    implementation 'org.springframework.boot:spring-boot-starter'
}"#
    } else {
        ""
    };

    let main_class_config = if spring_boot {
        ""
    } else {
        r#"
application {
    mainClass = 'application.Program'
}"#
    };


    let content = format!(
r#"plugins {{
{plugins_section}
}}

group = 'com.example'
version = '1.0'
sourceCompatibility = '21'

repositories {{
    mavenCentral()
}}
{dependencies_section}
{main_class_config}
"#,
        plugins_section = plugins_section,
        dependencies_section = dependencies_section,
        main_class_config = main_class_config
    );

    fs::write(path, content)
}