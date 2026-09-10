# DEN-390 flags-2-env sidecar boundary

`flags2env-platform-sidecar` owns `.cli-flags.toml` as its public argv/environment contract.

The executable delegates parsing to the pinned `ores-otel-sidecar` runtime, which uses the bundled `flags2env` parser to audit the contract, reject unknown options/operands, resolve commands, and apply canonical defaults/environment/argv precedence before the sidecar binds a socket.

The generated `generated/rust/runtime.rs` file is no longer part of executable startup. It remains a derivative compatibility projection only. Runtime admission is the audited `.cli-flags.toml` boundary plus `SidecarConfig::from_bind`.

The distroless image carries both `/flags2env-platform-sidecar` and `/.cli-flags.toml`; kubelet-style `probe` and `probe-readyz` invocations therefore use the same repository-owned contract as source CI.

CI checks the exact pull-request head, Rust formatting/Clippy/tests, invalid raw environment rejection, argv-over-environment precedence, payload-free rejection diagnostics, the distroless `preflight` command, and live in-container health/readiness probes.
