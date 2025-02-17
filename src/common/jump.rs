use std::process::Command;
pub enum JumpTerm {
    Error,
    Warning,
}

impl JumpTerm {
    pub fn to_str(&self) -> &'static str {
        match self {
            JumpTerm::Error => " error: ",
            JumpTerm::Warning => " warning: ",
        }
    }
}

pub fn jump_on_term(line: &str, jump_term: &JumpTerm) -> Option<()> {
    if !line.contains(jump_term.to_str()) {
        return None;
    }
    let error_path_with_line_and_col = line.split(' ').next()?;
    for i in 0..error_path_with_line_and_col.len() {
        let n = error_path_with_line_and_col.len() - i;
        if std::path::Path::new(&error_path_with_line_and_col[0..n]).exists() {
            Command::new("code.cmd")
                .arg("-g")
                .arg(error_path_with_line_and_col)
                .spawn()
                .ok()?;
            return Some(());
        }
    }
    None
}
