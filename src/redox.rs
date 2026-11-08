use std::ffi::{CStr, c_char};

use syscall::{CallFlags, NumaMemoryPolicy, NumaVerb};

use crate::Os;

pub struct Redox;

pub static OS: Redox = Redox;

impl Os for Redox {
    fn spawn(&self, mut program: String, args: &mut Vec<String>, config: &crate::SpawnConfig) {
        let pid = unsafe { nix::libc::fork() };

        if pid == 0 {
            if config.strict {
                let thread_fd = unsafe { libredox::call::current_thread_fd_unstable() };
                let addrspace_fd = libredox::call::dup(thread_fd, b"addrspace").unwrap();
                let _ = libredox::call::call_wo(
                    addrspace_fd,
                    [(NumaMemoryPolicy::NodeLocalStrict as u64).to_ne_bytes()].as_flattened(),
                    CallFlags::WRITE,
                    &[NumaVerb::MemPolicy as u64],
                )
                .unwrap();
                program.push('\0');
                args.iter_mut().for_each(|e| e.push('\0'));
                args.push('\0'.into());
                let c_args = args
                    .iter()
                    .map(|e| {
                        CStr::from_bytes_with_nul(e.as_bytes()).unwrap().as_ptr() as *const c_char
                    })
                    .collect::<Vec<_>>();

                let program = CStr::from_bytes_with_nul(program.as_bytes()).unwrap();

                unsafe {
                    nix::libc::execvp(
                        program.as_ptr() as *const c_char,
                        c_args.as_ptr() as *const *const c_char,
                    );
                }
            }
        } else {
            let mut status = 0;
            libredox::call::waitpid(pid as usize, &mut status, 0).unwrap();
        }
    }
}
