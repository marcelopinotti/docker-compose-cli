use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("create") => init(),

        Some(command) => {
            println!("❌ Comando desconhecido: {}", command);
            println!();
            println!("Uso: docker-gen create");
        }

        None => {
            println!("🐳 Docker Compose Generator");
            println!();
            println!("Uso:");
            println!("  docker-gen create");
        }
    }
}

fn init() {
    println!("🐳 Docker Compose Generator\n");

    println!("1. PostgreSQL");
    println!("2. MySQL");
    println!("3. MongoDB");

    let choice = input("\nEscolha o banco: ");

    let database = match choice.as_str() {
        "1" => "postgres",
        "2" => "mysql",
        "3" => "mongo",
        _ => {
            println!("❌ Opção inválida.");
            return;
        }
    };

    let name = input("Nome do banco: ");
    let username = input("Usuário: ");
    let password = input("Senha: ");

    let default_port = match database {
        "postgres" => "5432",
        "mysql" => "3306",
        "mongo" => "27017",
        _ => unreachable!(),
    };

    let port_input = input(&format!("Porta [{}]: ", default_port));

    let port = if port_input.is_empty() {
        default_port.to_string()
    } else {
        port_input
    };

    let content = match database {
        "postgres" => postgres_compose(
            &name,
            &username,
            &password,
            &port,
        ),

        "mysql" => mysql_compose(
            &name,
            &username,
            &password,
            &port,
        ),

        "mongo" => mongo_compose(
            &name,
            &username,
            &password,
            &port,
        ),

        _ => {
            println!("❌ Banco inválido.");
            return;
        }
    };

    let path = Path::new("docker-compose.yml");

    if path.exists() {
        let overwrite = input(
            "\n⚠️ docker-compose.yml já existe. Deseja substituir? (s/n): ",
        );

        if overwrite.to_lowercase() != "s" {
            println!("❌ Operação cancelada.");
            return;
        }
    }

    match fs::write(path, content) {
        Ok(_) => {
            println!();
            println!("✅ docker-compose.yml criado com sucesso!");
            println!("📁 Local: {}", path.display());
        }

        Err(error) => {
            println!("❌ Erro ao criar o arquivo: {}", error);
        }
    }
}

fn input(message: &str) -> String {
    let mut value = String::new();

    print!("{}", message);

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut value)
        .unwrap();

    value.trim().to_string()
}

fn postgres_compose(
    name: &str,
    username: &str,
    password: &str,
    port: &str,
) -> String {
    format!(
        r#"services:
  postgres:
    image: postgres:17
    container_name: postgres
    restart: unless-stopped
    environment:
      POSTGRES_DB: {}
      POSTGRES_USER: {}
      POSTGRES_PASSWORD: {}
    ports:
      - "{}:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
"#,
        name, username, password, port
    )
}

fn mysql_compose(
    name: &str,
    username: &str,
    password: &str,
    port: &str,
) -> String {
    format!(
        r#"services:
  mysql:
    image: mysql:8.4
    container_name: mysql
    restart: unless-stopped
    environment:
      MYSQL_DATABASE: {}
      MYSQL_USER: {}
      MYSQL_PASSWORD: {}
      MYSQL_ROOT_PASSWORD: {}
    ports:
      - "{}:3306"
    volumes:
      - mysql_data:/var/lib/mysql

volumes:
  mysql_data:
"#,
        name, username, password, password, port
    )
}

fn mongo_compose(
    name: &str,
    username: &str,
    password: &str,
    port: &str,
) -> String {
    format!(
        r#"services:
  mongodb:
    image: mongo:8
    container_name: mongodb
    restart: unless-stopped
    environment:
      MONGO_INITDB_DATABASE: {}
      MONGO_INITDB_ROOT_USERNAME: {}
      MONGO_INITDB_ROOT_PASSWORD: {}
    ports:
      - "{}:27017"
    volumes:
      - mongo_data:/data/db

volumes:
  mongo_data:
"#,
        name, username, password, port
    )
}