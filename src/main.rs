//! RUMINATE - COPYRIGHT 2026 EuclidDivisionLemma

//    /                       \
//  /X/                       \X\
// |XX\         _____         /XX|
// |XXX\     _/       \_     /XXX|___________
//  \XXXXXXX             XXXXXXX/            \\\
//    \XXXX    /     \    XXXXX/                \\\
//         |   0     0   |                         \
//          |           |                           \
//           \         /                            |______//
//            \       /                             |
//             | O_O | \                            |
//              \ _ /   \________________           |
//                         | |  | |      \         /
//                         / |  / |       \______/
//                         \ |  \ |        \ |  \ |
//                       __| |__| |      __| |__| |
//                       |___||___|      |___||___|

use std::unimplemented;

use clap::{ArgAction, Parser};

trait Os {
    fn spawn(&self, program: &str, args: &str, config: &SpawnConfig) {
        unimplemented!()
    }
}

struct SpawnConfig {
    node_local_page_table: bool,
}

// #[cfg(target_os = "linux")]
// #[path = "./linux.rs"]
// mod os;

// #[cfg(target_os = "redox")]
#[path = "./redox.rs"]
mod os;

#[derive(clap::Subcommand, Debug)]
enum SubCommands {
    Spawn {
        #[arg(help = "Path of the program to run")]
        path: String,
        #[clap(long, action=ArgAction::SetFalse, help("Make the spawned process use node-local page tables"))]
        local_pgtbl: bool,
        #[arg(long, help = "Arguments to the spawned process")]
        args: String,
    },
    Show,
}

#[derive(clap::Parser, Debug)]
#[clap(version, about("Manipulate NUMA behaviour"))]
struct Cli {
    #[command(subcommand)]
    commands: SubCommands,
}

fn main() {
    let x = Cli::parse();
}
