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

use clap::Parser;

use crate::os::OS;

trait Os {
    fn spawn(&self, program: &str, args: &Vec<String>, config: &SpawnConfig);
}

struct SpawnConfig {
    node_local_page_table: bool,
}

#[cfg(target_os = "linux")]
#[path = "./linux.rs"]
mod os;

#[cfg(target_os = "redox")]
#[path = "./redox.rs"]
mod os;

#[derive(clap::Subcommand, Debug)]
enum SubCommand {
    #[clap(about("Launch a program with the specified NUMA behaviour"))]
    Spawn {
        #[arg(help = "Path of the program to run")]
        path: String,
        #[clap(
            long,
            action,
            help("Make the spawned process use node-local page tables")
        )]
        local_pgtbl: bool,
        #[arg(long("arg"), help = "Arguments to the spawned process")]
        args: Vec<String>,
    },
    #[clap(about("Display information about system NUMA"))]
    Show,
}

#[derive(clap::Parser, Debug)]
#[clap(version, about("Manipulate NUMA behaviour"))]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        SubCommand::Spawn {
            path,
            local_pgtbl,
            args,
        } => OS.spawn(
            &path,
            &args,
            &SpawnConfig {
                node_local_page_table: local_pgtbl,
            },
        ),
        SubCommand::Show => todo!(),
    }
}
