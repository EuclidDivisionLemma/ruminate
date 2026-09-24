use crate::Os;

pub struct Linux;

pub static OS: Linux = Linux;

impl Os for Linux {
    fn spawn(&self, program: &str, args: &Vec<String>, config: &crate::SpawnConfig) {
        if config.node_local_page_table {
            colour::e_yellow_ln!("WARNING: Linux does not support node-local page tables");
        }
        let mut handle = std::process::Command::new(program)
            .args(args)
            .spawn()
            .unwrap();
        handle.wait().unwrap();
    }
}
