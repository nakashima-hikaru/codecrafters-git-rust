use std::{fs, path::PathBuf};

use super::GitCommand;

pub struct InitCommand;
impl GitCommand for InitCommand {
    fn execute(&self, _args: &Vec<String>) {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
        println!("Initialized git directory")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_command() {
        let temp_path = format!("temp");
        std::env::set_current_dir(&temp_path).unwrap();
        let command = InitCommand {};
        let args: Vec<String> = Vec::new();
        command.execute(&args);
        let paths = fs::read_dir(".git").unwrap();
        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        assert!(std::path::Path::new(&format!(".git")).exists());
        assert!(std::path::Path::new(&format!(".git/objects")).exists());
        assert!(std::path::Path::new(&format!(".git/refs")).exists());
        assert!(std::path::Path::new(&format!(".git/HEAD")).exists());
        std::fs::remove_dir_all(".git").expect("Failed to remove .git directory");
    }
}
