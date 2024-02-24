#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
use {defmt_rtt as _, panic_probe as _};

#[cfg(target_os = "none")]
use defmt::info;

fn main() {
    reset_ctrl::run();
}
