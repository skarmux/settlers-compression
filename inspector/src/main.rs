mod bindings;
use bindings::s4::files::map::load_map_info;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "inspector", version = env!("CARGO_PKG_VERSION"))]
struct Inspector {
    /// Path to the input file
    path: PathBuf,
}

impl Inspector {
    fn run(self) {
        let res = load_map_info("./Aeneas.map").unwrap();
        dbg!(res);
    }
}

fn main() {
    Inspector::parse().run();
}
