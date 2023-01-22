use std::fs;

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
        let current_dir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let temp_path = format!("{}/temp", &current_dir);
        std::env::set_current_dir(&temp_path).unwrap();
        let command = InitCommand {};
        let args: Vec<String> = Vec::new();
        command.execute(&args);
        std::fs::remove_dir_all(".git").expect("Failed to remove .git directory");
    }
}
