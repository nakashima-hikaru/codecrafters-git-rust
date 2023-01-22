pub mod init_command;
pub trait GitCommand {
    fn execute(&self, args: &Vec<String>);
}
