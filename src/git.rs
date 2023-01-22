use crate::GitCommand;

pub struct Git {
    command: Box<dyn GitCommand>,
}

impl Git {
    pub fn new(command: Box<dyn GitCommand>) -> Self {
        Self { command }
    }

    pub fn execute_command(&self, args: &Vec<String>) {
        self.command.execute(args);
    }
}
