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
