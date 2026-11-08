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

use clap::{Parser, value_parser};

use crate::os::OS;

trait Os {
    fn spawn(&self, program: &str, args: &Vec<String>, config: &SpawnConfig);
}

enum Policy {
    Local,
    Preferred(()),
}

struct SpawnConfig {
    strict: bool,
}

// #[cfg(target_os = "linux")]
// #[path = "./linux.rs"]
// mod os;

// #[cfg(target_os = "redox")]
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
            help("Specifies whether locality should be strictly followed"),
            default_value("false")
        )]
        strict: bool,
        #[arg(long("arg"), help = "Arguments to the spawned process")]
        args: Vec<String>,
    },
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
        SubCommand::Spawn { path, strict, args } => OS.spawn(&path, &args, &SpawnConfig { strict }),
    }
}
