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

trait Os {}

// #[cfg(target_os = "linux")]
// #[path = "./linux.rs"]
// mod os;

// #[cfg(target_os = "redox")]
#[path = "./redox.rs"]
mod os;

fn main() {
    println!("Hello, world!");
}
