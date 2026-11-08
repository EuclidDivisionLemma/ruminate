use crate::Os;

pub struct Linux;

pub static OS: Linux = Linux;

impl Os for Linux {
    fn spawn(&self, program: &str, args: &Vec<String>, config: &crate::SpawnConfig) {
        colour::yellow_ln_bold!("WARNING: Policy setting not supported");
        let mut handle = std::process::Command::new(program)
            .args(args)
            .spawn()
            .unwrap();
        handle.wait().unwrap();
    }
}
