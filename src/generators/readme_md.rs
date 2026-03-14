use std::fs;
use std::path::PathBuf;

pub fn create(project_path: &PathBuf, java_project_name: &str) -> std::io::Result<()> {
    let file_path = project_path.join("README.md");

    let name = java_project_name;
    let name_low = java_project_name.to_lowercase();

    let content = format!(
r#"# {name}

## Primeira execução / Rebuild
```
docker compose up --build
```

## Iniciar
```
docker compose run --rm {name_low}
```

## Encerrar
```
docker compose down"#);
    fs::write(file_path, content)
}