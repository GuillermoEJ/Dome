/// Command-line interface and REPL
/// 
/// Handles command parsing and execution

pub mod commands;
pub mod completer;

pub use commands::Command;
pub use completer::CommandCompleter;

/// Available commands in the vault
pub const COMMANDS: &[&str] = &[
    "encrypt", "decrypt", "exit", "quit", "list", "ls",
    "new", "create", "append", "remove", "addfile",
    "passrefresh", "passcreate", "printeverything",
    "newvault", "edit", "clear", "cls", "help",
    "rename", "dbencrypt", "importcsv", "search", "searchcontents",
];
