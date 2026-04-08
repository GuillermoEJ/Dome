/// DOME - Secure Password Vault
/// Rust port from original Python implementation
///
/// Usage: dome [VAULT_NAME]

mod crypto;
mod vault;
mod editor;
mod cli;
mod utils;

use anyhow::Result;
use colored::*;
use rustyline::DefaultEditor;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};
use vault::{Vault, VaultFile, VaultManager};
use cli::commands::{Command, CommandType, HELP_TEXT};
use rpassword::prompt_password;
use rand::Rng;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const LOGO: &str = r#"
▓█████▄  ▒█████   ███▄ ▄███▓▓█████ 
▒██▀ ██▌▒██▒  ██▒▓██▒▀█▀ ██▒▓█   ▀ 
░██   █▌▒██░  ██▒▓██    ▓██░▒███   
░▓█▄   ▌▒██   ██░▒██    ▒██ ▒▓█  ▄ 
░▒████▓ ░ ████▓▒░▒██▒   ░██▒░▒████▒
 ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ░  ░░░ ▒░ ░
 ░ ▒  ▒   ░ ▒ ▒░ ░  ░      ░ ░ ░  ░
 ░ ░  ░ ░ ░ ░ ▒  ░      ░      ░   
   ░        ░ ░         ░      ░  ░
"#;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("{}", LOGO);
    println!("{}", format!("Vault version v{}\n", VERSION).cyan());

    // Initialize vault directories
    let vault_dir = get_vault_dir()?;
    std::fs::create_dir_all(&vault_dir)?;

    // Get vault name from args or use default
    let vault_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "default".to_string());

    let vault_path = vault_dir.join(format!("{}.vault", vault_name));

    // Initialize vault manager
    let mut manager = VaultManager::new(vault_path.clone(), vault_dir.join("config.json"));

    // Try to load existing vault, or create new one
    let password = if vault_path.exists() {
        println!("{}", "Enter vault password: ".cyan());
        prompt_password("  ")?
    } else {
        println!("{}", format!("Creating new vault: {}", vault_name).yellow());
        let pwd = prompt_password("  Enter new password: ")?;
        let pwd_confirm = prompt_password("  Confirm password: ")?;

        if pwd != pwd_confirm {
            eprintln!("{}", "Passwords do not match!".red());
            std::process::exit(1);
        }
        pwd
    };

    // Load vault
    match manager.load_vault(&password) {
        Ok(_) => {
            println!("{}", format!("✓ Vault '{}' unlocked", vault_name).green());
        }
        Err(e) if !vault_path.exists() => {
            // First time - save empty vault
            manager.save_vault(&password)?;
            println!("{}", format!("✓ Vault '{}' created", vault_name).green());
        }
        Err(_) => {
            eprintln!("{}", "Invalid password!".red());
            std::process::exit(1);
        }
    }

    println!("{}", "Type 'help' for commands\n".cyan());

    // REPL Loop
    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline(&format!("{}", "vault> ".green())) {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }

                if let Some(cmd) = Command::parse(&line) {
                    // Handle exit
                    if cmd.is_exit() {
                        println!("{}", "Saving and exiting...".cyan());
                        manager.save_vault(&password)?;
                        println!("{}", "Goodbye!".green());
                        break;
                    }

                    // Execute command
                    match execute_command(&mut manager, &cmd, &password, &vault_dir).await {
                        Ok(output) => {
                            if !output.is_empty() {
                                println!("{}", output);
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", format!("Error: {}", e).red());
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!();
                println!("{}", "^C - Type 'exit' to quit".yellow());
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!();
                break;
            }
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                break;
            }
        }
    }

    Ok(())
}

/// Execute a parsed command
async fn execute_command(
    manager: &mut VaultManager,
    cmd: &Command,
    password: &str,
    vault_dir: &Path,
) -> Result<String> {
    match cmd.command_type() {
        CommandType::Help => {
            Ok(HELP_TEXT.to_string())
        }

        CommandType::List | CommandType::Ls => {
            let files = manager.list_files();
            if files.is_empty() {
                Ok("No entries stored.".to_string())
            } else {
                let mut output = String::new();
                for (i, file) in files.iter().enumerate() {
                    output.push_str(&format!(
                        "{:2}. {} {}\n",
                        i + 1,
                        file.title.cyan(),
                        format!("[{}]", file.file_type).dimmed()
                    ));
                }
                Ok(output)
            }
        }

        CommandType::New | CommandType::Create => {
            if cmd.args.is_empty() {
                return Err(anyhow::anyhow!("Usage: new <title>"));
            }
            let title = cmd.args.join(" ");

            // Check if already exists
            if manager.list_files().iter().any(|f| f.title == title) {
                return Err(anyhow::anyhow!("Entry '{}' already exists", title));
            }

            // Create new file
            manager.add_file(VaultFile {
                title: title.clone(),
                file_type: "normal".to_string(),
                content: String::new(),
                encryption: "normal".to_string(),
            });

            manager.save_vault(password)?;
            Ok(format!("{}  Created '{}'", "✓".green(), title.cyan()))
        }

        CommandType::Remove | CommandType::Delete => {
            if cmd.args.is_empty() {
                return Err(anyhow::anyhow!("Usage: remove <title>"));
            }
            let title = cmd.args.join(" ");

            manager.remove_file(&title)?;
            manager.save_vault(password)?;
            Ok(format!("{}  Deleted '{}'", "✓".green(), title.cyan()))
        }

        CommandType::Edit => {
            if cmd.args.is_empty() {
                return Err(anyhow::anyhow!("Usage: edit <title>"));
            }
            let title = cmd.args.join(" ");

            // Find file
            let file = manager
                .list_files()
                .iter()
                .find(|f| f.title == title)
                .ok_or_else(|| anyhow::anyhow!("Entry not found: {}", title))?
                .clone();

            // Get new content from user
            println!("{}", "Enter content (empty line to save):".cyan());
            println!("{}", "(Current content below:)".dimmed());
            println!("{}", file.content);
            println!();

            let mut new_content = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            loop {
                let mut line = String::new();
                handle.read_line(&mut line)?;
                if line.trim().is_empty() && !new_content.is_empty() {
                    break;
                }
                new_content.push_str(&line);
            }

            // Update file
            manager.remove_file(&title)?;
            manager.add_file(VaultFile {
                title: title.clone(),
                file_type: file.file_type,
                content: new_content,
                encryption: "normal".to_string(),
            });

            manager.save_vault(password)?;
            Ok(format!("{}  Updated '{}'", "✓".green(), title.cyan()))
        }

        CommandType::Search => {
            if cmd.args.is_empty() {
                return Err(anyhow::anyhow!("Usage: search <term>"));
            }
            let term = cmd.args.join(" ");

            let results = manager.search(&term);
            if results.is_empty() {
                Ok(format!("No results for '{}'", term))
            } else {
                let mut output = format!("Found {} entries:\n", results.len());
                for file in results {
                    output.push_str(&format!("  • {} [{}]\n", file.title.cyan(), file.file_type));
                }
                Ok(output)
            }
        }

        CommandType::SearchContents => {
            if cmd.args.is_empty() {
                return Err(anyhow::anyhow!("Usage: searchcontents <term>"));
            }
            let term = cmd.args.join(" ");

            let results = manager.search(&term);
            if results.is_empty() {
                Ok(format!("No results for '{}'", term))
            } else {
                let mut output = String::new();
                for file in results {
                    output.push_str(&format!(
                        "{}\n  {}\n\n",
                        file.title.cyan(),
                        file.content.dimmed()
                    ));
                }
                Ok(output)
            }
        }

        CommandType::PassCreate => {
            let pwd = generate_password(16);
            Ok(format!("Generated password: {}", pwd.yellow()))
        }

        CommandType::PassRefresh => {
            if cmd.args.is_empty() {
                return Err(anyhow::anyhow!("Usage: passrefresh <title>"));
            }
            let title = cmd.args.join(" ");

            let file = manager
                .list_files()
                .iter()
                .find(|f| f.title == title)
                .ok_or_else(|| anyhow::anyhow!("Entry not found: {}", title))?
                .clone();

            let new_pwd = generate_password(16);

            manager.remove_file(&title)?;
            manager.add_file(VaultFile {
                title: title.clone(),
                file_type: file.file_type,
                content: new_pwd.clone(),
                encryption: "normal".to_string(),
            });

            manager.save_vault(password)?;
            Ok(format!(
                "{}  New password for '{}': {}",
                "✓".green(),
                title.cyan(),
                new_pwd.yellow()
            ))
        }

        CommandType::Append => {
            if cmd.args.len() < 2 {
                return Err(anyhow::anyhow!("Usage: append <title> <content>"));
            }

            let title = cmd.args[0].clone();
            let text = cmd.args[1..].join(" ");

            let file = manager
                .list_files()
                .iter()
                .find(|f| f.title == title)
                .ok_or_else(|| anyhow::anyhow!("Entry not found: {}", title))?
                .clone();

            let mut new_content = file.content;
            new_content.push('\n');
            new_content.push_str(&text);

            manager.remove_file(&title)?;
            manager.add_file(VaultFile {
                title: title.clone(),
                file_type: file.file_type,
                content: new_content,
                encryption: "normal".to_string(),
            });

            manager.save_vault(password)?;
            Ok(format!("{}  Appended to '{}'", "✓".green(), title.cyan()))
        }

        CommandType::PrintEverything => {
            let mut output = format!("{}\n", "=== VAULT CONTENTS ===".yellow());
            for file in manager.list_files() {
                output.push_str(&format!(
                    "\n{} [{}]\n{}\n",
                    file.title.cyan(),
                    file.file_type,
                    "─".repeat(40)
                ));
                output.push_str(&file.content);
                output.push('\n');
            }
            Ok(output)
        }

        CommandType::Rename => {
            if cmd.args.len() < 2 {
                return Err(anyhow::anyhow!("Usage: rename <old_title> <new_title>"));
            }

            let old_title = cmd.args[0].clone();
            let new_title = cmd.args[1..].join(" ");

            let file = manager
                .list_files()
                .iter()
                .find(|f| f.title == old_title)
                .ok_or_else(|| anyhow::anyhow!("Entry not found: {}", old_title))?
                .clone();

            manager.remove_file(&old_title)?;
            manager.add_file(VaultFile {
                title: new_title.clone(),
                file_type: file.file_type,
                content: file.content,
                encryption: "normal".to_string(),
            });

            manager.save_vault(password)?;
            Ok(format!(
                "{}  Renamed '{}' to '{}'",
                "✓".green(),
                old_title.cyan(),
                new_title.cyan()
            ))
        }

        CommandType::Clear | CommandType::Cls => {
            println!("{}", "\x1B[2J\x1B[1;1H");
            Ok(String::new())
        }

        CommandType::NewVault => {
            Ok("Multi-vault support coming soon!".to_string())
        }

        CommandType::DbEncrypt => {
            // Re-save vault (already encrypted)
            manager.save_vault(password)?;
            Ok(format!("{}  Vault re-encrypted", "✓".green()))
        }

        CommandType::ImportCsv => {
            Ok("CSV import coming soon!".to_string())
        }

        CommandType::AddFile => {
            Ok("File attachment coming soon!".to_string())
        }

        CommandType::Unknown => {
            Err(anyhow::anyhow!("Unknown command: '{}'. Type 'help' for available commands.", cmd.name))
        }

        CommandType::Exit | CommandType::Quit => {
            unreachable!() // Already handled above
        }
    }
}

/// Get the vault directory (~/.dome)
fn get_vault_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    Ok(PathBuf::from(home).join(".dome"))
}

/// Generate a random password
fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
