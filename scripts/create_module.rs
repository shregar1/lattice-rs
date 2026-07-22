// Lattice-Rs CLI Module Generator
// Usage: cargo run --example create_module Product

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("❌ Error: Please provide a module name.");
        eprintln!("Example: cargo run --example create_module Product");
        std::process::exit(1);
    }

    let raw = &args[1];
    let pascal = format!("{}{}", &raw[..1].to_uppercase(), &raw[1..]);
    let snake = raw.to_lowercase();

    println!("🚀 Scaffolding Lattice-Rs Clean-Architecture Module for: {}...\n", pascal);

    // 1. Model
    write_file(&format!("src/models/{}.rs", snake), &format!(r#"
use serde::{{Serialize, Deserialize}};
use chrono::{{DateTime, Utc}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct {}Model {{
    pub id: String,
    pub urn: String,
    pub name: String,
    pub owner_id: String,
    pub tenant_id: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}}
"#, pascal));

    // 2. DTO
    write_file(&format!("src/dto/requests/{}.rs", snake), &format!(r#"
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Create{}Request {{
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}}
"#, pascal));

    // 3. Repository
    write_file(&format!("src/repositories/{}.rs", snake), &format!(r#"
use async_trait::async_trait;
use crate::models::{}::{}Model;

#[async_trait]
pub trait {}RepositoryTrait: Send + Sync {{
    async fn find_by_id(&self, id: &str) -> Result<Option<{}Model>, String>;
}}
"#, snake, pascal, pascal, pascal));

    println!("\n✨ Scaffolding complete for lattice-rs module '{}'!", pascal);
}

fn write_file(path_str: &str, content: &str) {
    let path = Path::new(path_str);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    if path.exists() {
        println!("  ⚠️ Skipping existing file: {}", path_str);
        return;
    }
    fs::write(path, content.trim()).unwrap();
    println!("  ✓ Created: {}", path_str);
}
