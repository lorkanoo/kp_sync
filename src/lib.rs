mod addon;
mod api;
mod config;
mod context;
mod render;
mod thread;

use crate::addon::Addon;
use nexus::{AddonFlags, UpdateProvider};

nexus::export! {
    name: "KP Sync",
    signature: -0xc347f82,
    flags: AddonFlags::None,
    load: Addon::load,
    unload: Addon::unload,
    provider: UpdateProvider::GitHub,
    update_link: env!("CARGO_PKG_REPOSITORY"),
}
