use std::env;
mod git;
mod git_commands;

use git::Git;
use git_commands::{init_command::InitCommand, GitCommand};

pub enum GitCommandEnum {
    Init,
}

pub fn create_git_command(command: GitCommandEnum) -> Box<dyn GitCommand> {
    match command {
        GitCommandEnum::Init => Box::new(InitCommand {}),
    }
}

fn main() {
    let command_name = env::args().nth(1).expect("Command name is not provided");
    let args: Vec<String> = env::args().skip(2).collect();

    let git_command = match command_name.as_str() {
        "init" => GitCommandEnum::Init,
        _ => panic!("Invalid command"),
    };

    let git = Git::new(create_git_command(git_command));
    git.execute_command(&args);
}
