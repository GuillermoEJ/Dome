/// Command auto-completion for rustyline
/// 
/// Provides command and file name completion

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::context::Context;
use rustyline::error::ReadlineError;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::highlight::Highlighter;
use rustyline::Helper;

use super::COMMANDS;

#[derive(Helper)]
pub struct CommandCompleter {
    filename_completer: FilenameCompleter,
}

impl CommandCompleter {
    pub fn new() -> Self {
        Self {
            filename_completer: FilenameCompleter::new(),
        }
    }
}

impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let parts: Vec<&str> = line[..pos].split_whitespace().collect();

        // Complete command names
        if parts.is_empty() || (parts.len() == 1 && !line.ends_with(' ')) {
            let prefix = if parts.is_empty() { "" } else { parts[0] };
            let matches: Vec<Pair> = COMMANDS
                .iter()
                .filter(|cmd| cmd.starts_with(prefix))
                .map(|cmd| Pair {
                    display: cmd.to_string(),
                    replacement: cmd.to_string(),
                })
                .collect();

            return Ok((line.len() - prefix.len(), matches));
        }

        // For other completions, use filename
        self.filename_completer.complete(line, pos, _ctx)
    }
}

impl Hinter for CommandCompleter {
    type Hint = String;
}

impl Validator for CommandCompleter {}

impl Highlighter for CommandCompleter {}
