#![forbid(unsafe_code)]

use flags_2_env_sidecar::{config::SidecarConfig, runtime};

fn main() {
    let cfg = SidecarConfig::from_env();
    runtime::run(&cfg);
}

