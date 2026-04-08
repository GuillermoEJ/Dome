/// Command parsing and definitions
/// 
/// Parse user input and categorize commands

use anyhow::Result;
use std::fmt;

/// A parsed command with name and arguments
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

/// Command categories for routing
#[derive(Debug, Clone)]
pub enum CommandType {
    Help,
    Exit,
    Quit,
    List,
    Ls,
    New,
    Create,
    Edit,
    Remove,
    Delete,
    Search,
    SearchContents,
    Append,
    AddFile,
    PassCreate,
    PassRefresh,
    PrintEverything,
    NewVault,
    Clear,
    Cls,
    Rename,
    DbEncrypt,
    ImportCsv,
    Unknown,
}

impl Command {
    /// Parse a command string
    pub fn parse(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        Some(Command {
            name: parts[0].to_string(),
            args: parts[1..].iter().map(|s| s.to_string()).collect(),
        })
    }

    /// Get command type
    pub fn command_type(&self) -> CommandType {
        match self.name.as_str() {
            "help" => CommandType::Help,
            "exit" => CommandType::Exit,
            "quit" => CommandType::Quit,
            "list" => CommandType::List,
            "ls" => CommandType::Ls,
            "new" => CommandType::New,
            "create" => CommandType::Create,
            "edit" => CommandType::Edit,
            "remove" => CommandType::Remove,
            "delete" => CommandType::Delete,
            "search" => CommandType::Search,
            "searchcontents" => CommandType::SearchContents,
            "append" => CommandType::Append,
            "addfile" => CommandType::AddFile,
            "passcreate" => CommandType::PassCreate,
            "passrefresh" => CommandType::PassRefresh,
            "printeverything" => CommandType::PrintEverything,
            "newvault" => CommandType::NewVault,
            "clear" => CommandType::Clear,
            "cls" => CommandType::Cls,
            "rename" => CommandType::Rename,
            "dbencrypt" => CommandType::DbEncrypt,
            "importcsv" => CommandType::ImportCsv,
            _ => CommandType::Unknown,
        }
    }

    /// Check if this is a help request
    pub fn is_help(&self) -> bool {
        matches!(self.command_type(), CommandType::Help)
    }

    /// Check if this is an exit request
    pub fn is_exit(&self) -> bool {
        matches!(
            self.command_type(),
            CommandType::Exit | CommandType::Quit
        )
    }
}

// Help text
pub const HELP_TEXT: &str = r#"
╔═══════════════════════════════════════════════════════════╗
║                 DOME - Password Vault                     ║
║                    Available Commands                     ║
╚═══════════════════════════════════════════════════════════╝

 File Operations:
   list, ls              List all stored entries
   new <title>           Create new entry
   edit <title>          Edit entry content
   remove, delete        Delete entry
   rename <old> <new>    Rename entry
   append <title>        Append to entry content

 Search & View:
   search <term>         Search by title or content
   searchcontents        Full text search
   printeverything       Show all entries (unencrypted!)

 Password Generation:
   passcreate            Generate random password
   passrefresh <title>   Refresh password for entry

 Vault Operations:
   newvault <name>       Create new vault
   dbencrypt             Re-encrypt vault database
   importcsv <file>      Import passwords from CSV

 Other:
   clear, cls            Clear screen
   help                  Show this help
   exit, quit            Exit vault

"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command() {
        let cmd = Command::parse("help").unwrap();
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.args.len(), 0);
    }

    #[test]
    fn test_parse_command_with_args() {
        let cmd = Command::parse("remove myfile").unwrap();
        assert_eq!(cmd.name, "remove");
        assert_eq!(cmd.args.len(), 1);
        assert_eq!(cmd.args[0], "myfile");
    }

    #[test]
    fn test_command_type() {
        let cmd = Command::parse("new test").unwrap();
        assert!(matches!(cmd.command_type(), CommandType::New));

        let cmd = Command::parse("exit").unwrap();
        assert!(cmd.is_exit());
    }
}
