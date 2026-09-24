use syscall::{CallFlags, NumaVerb};

use crate::Os;

pub struct Redox;

pub static OS: Redox = Redox;

impl Os for Redox {
    fn spawn(&self, program: &str, args: &Vec<String>, config: &crate::SpawnConfig) {
        if config.node_local_page_table {
            let thread_fd = redox_rt::RtTcb::current().thread_fd().as_raw_fd();
            let addrspace_fd = libredox::call::dup(thread_fd, b"addrspace").unwrap();
            let _ = libredox::call::call_wo(
                addrspace_fd,
                &[],
                CallFlags::WRITE,
                &[NumaVerb::ReplicatePageTables as u64],
            )
            .unwrap();
        }
        let mut handle = std::process::Command::new(program)
            .args(args)
            .spawn()
            .unwrap();
        handle.wait().unwrap();
    }
}
