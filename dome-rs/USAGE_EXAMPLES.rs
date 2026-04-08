// EJEMPLOS DE USO - Cómo utilizar los módulos principales

// ============================================
// 1. CIFRADO Y DESCIFRADO (crypto)
// ============================================

use dome::crypto::{encrypt, decrypt};

fn crypto_example() -> Result<()> {
    let plaintext = b"Mi contraseña secreta: P@ssw0rd!";
    let password = "mi_password_maestro";
    
    // Encriptar
    let encrypted = encrypt(plaintext, password)?;
    println!("Encrypted bytes: {} bytes", encrypted.len());
    
    // Desencriptar
    let decrypted = decrypt(&encrypted, password)?;
    assert_eq!(plaintext.to_vec(), decrypted);
    
    println!("✓ Encryption/decryption working correctly");
    Ok(())
}

// ============================================
// 2. GESTIÓN DE BÓVEDA (vault)
// ============================================

use dome::vault::{Vault, VaultFile, VaultManager};
use std::path::PathBuf;

fn vault_example() -> Result<()> {
    let vault_path = PathBuf::from("/tmp/my.vault");
    let config_path = PathBuf::from("/tmp/config.json");
    
    let mut manager = VaultManager::new(vault_path.clone(), config_path);
    
    // Agregar archivo a la bóveda
    manager.add_file(VaultFile {
        title: "Gmail".to_string(),
        file_type: "password".to_string(),
        content: "user@gmail.com\nMyPassword123".to_string(),
        encryption: "normal".to_string(),
    });
    
    manager.add_file(VaultFile {
        title: "AWS Keys".to_string(),
        file_type: "password".to_string(),
        content: "AKIAIOSFODNN7EXAMPLE\nwJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
        encryption: "normal".to_string(),
    });
    
    // Guardar bóveda encriptada
    let vault_password = "MyVaultMasterPassword";
    manager.save_vault(vault_password)?;
    println!("✓ Vault saved and encrypted");
    
    // Cargar bóveda
    let mut manager2 = VaultManager::new(vault_path, config_path);
    manager2.load_vault(vault_password)?;
    
    println!("Files in vault:");
    for file in manager2.list_files() {
        println!("  - {}: {}", file.title, file.file_type);
    }
    
    Ok(())
}

// ============================================
// 3. BÚSQUEDA (search)
// ============================================

fn search_example() -> Result<()> {
    let vault_path = PathBuf::from("/tmp/my.vault");
    let config_path = PathBuf::from("/tmp/config.json");
    
    let mut manager = VaultManager::new(vault_path, config_path);
    manager.load_vault("MyVaultMasterPassword")?;
    
    // Buscar por título
    let results = manager.search("Gmail");
    println!("Search results for 'Gmail':");
    for file in results {
        println!("  - {} ({})", file.title, file.file_type);
    }
    
    Ok(())
}

// ============================================
// 4. EDITOR TUI (editor)
// ============================================

use dome::editor::Editor;

fn editor_example() {
    let mut editor = Editor::new();
    
    // Cargar contenido
    editor.load("Line 1\nLine 2\nLine 3");
    
    // Simular edición
    editor.move_down();
    editor.move_right();
    editor.insert_char('X');  // Inserta X
    
    println!("Edited content:\n{}", editor.content());
    // Output:
    // Line 1
    // LXine 2
    // Line 3
}

// ============================================
// 5. COMANDOS CLI (cli)
// ============================================

use dome::cli::Command;

fn cli_example() -> Result<()> {
    // Parse de comandos
    let cmd1 = Command::parse("help").unwrap();
    match cmd1.execute() {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    let cmd2 = Command::parse("list").unwrap();
    println!("Command: {}", cmd2.name);
    println!("Args: {:?}", cmd2.args);
    
    let cmd3 = Command::parse("remove mypassword").unwrap();
    println!("Remove file: {}", cmd3.args.join(" "));
    
    Ok(())
}

// ============================================
// 6. UTILIDADES (utils)
// ============================================

use dome::utils::{format_size, compare_versions};

fn utils_example() {
    // Formateo de tamaño
    println!("{}", format_size(512));              // "512 B"
    println!("{}", format_size(1024 * 1024));      // "1.00 MB"
    println!("{}", format_size(5 * 1024 * 1024 * 1024));  // "5.00 GB"
    
    // Comparación de versiones
    let cmp = compare_versions("1.0.0", "2.0.0");
    println!("1.0.0 < 2.0.0: {}", cmp == -1);      // true
    
    let cmp = compare_versions("1.5.0", "1.5.0");
    println!("1.5.0 == 1.5.0: {}", cmp == 0);      // true
}

// ============================================
// PROGRAMA COMPLETO: REPL INTERACTIVO
// ============================================

#[tokio::main]
async fn main() -> Result<()> {
    use rustyline::DefaultEditor;
    use dome::cli::CommandCompleter;
    
    println!("Welcome to DOME - Secure Password Vault");
    println!("Type 'help' for commands");
    
    let mut manager = VaultManager::new(
        PathBuf::from(std::env::var("HOME")? + "/.dome/vault.vault"),
        PathBuf::from(std::env::var("HOME")? + "/.dome/config.json"),
    );
    
    // Intentar cargar bóveda existente
    let vault_password = rpassword::prompt_password("Vault password: ")?;
    if manager.load_vault(&vault_password).is_err() {
        println!("Creating new vault...");
    }
    
    // REPL Loop
    let mut rl = DefaultEditor::new()?;
    let completer = CommandCompleter::new();
    
    loop {
        match rl.readline("vault> ") {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                
                if let Some(cmd) = Command::parse(&line) {
                    match cmd.name.as_str() {
                        "help" => {
                            println!("Commands:");
                            println!("  list, ls         - List all files");
                            println!("  new <title>      - Create new entry");
                            println!("  remove <title>   - Delete entry");
                            println!("  search <term>    - Search entries");
                            println!("  save             - Save vault");
                            println!("  exit, quit       - Exit");
                        },
                        "list" | "ls" => {
                            for file in manager.list_files() {
                                println!("  {} [{}]", file.title, file.file_type);
                            }
                        },
                        "new" => {
                            if !cmd.args.is_empty() {
                                let title = cmd.args.join(" ");
                                manager.add_file(VaultFile {
                                    title,
                                    file_type: "normal".to_string(),
                                    content: String::new(),
                                    encryption: "normal".to_string(),
                                });
                                println!("✓ File created");
                            }
                        },
                        "save" => {
                            manager.save_vault(&vault_password)?;
                            println!("✓ Vault saved");
                        },
                        "search" => {
                            let term = cmd.args.join(" ");
                            let results = manager.search(&term);
                            for file in results {
                                println!("  {}", file.title);
                            }
                        },
                        "exit" | "quit" => break,
                        _ => println!("Unknown command. Type 'help' for commands."),
                    }
                }
            },
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("^C");
                break;
            },
            Err(_) => break,
        }
    }
    
    Ok(())
}
