#![forbid(unsafe_code)]

use ores_otel_sidecar::{runtime, SidecarConfig, SidecarIdentity};

fn main() {
    let cfg = SidecarConfig::from_env(SidecarIdentity::new(
        "flags-2-env-sidecar",
        "FLAGS_2_ENV_SIDECAR_BIND",
    ));
    runtime::run(&cfg);
}
