use containerd_shim::Config;
use containerd_shim_wasm::{
    container::Instance,
    sandbox::cli::{revision, shim_main, version},
};

pub mod engine;
pub mod loader;

fn main() {
    // Configure the shim to have only error level logging for performance improvements.
    let shim_config = Config {
        ..Default::default()
    };
    shim_main::<Instance<engine::SpinEngine>>(
        "spin",
        version!(),
        revision!(),
        "v2",
        Some(shim_config),
    );
}
