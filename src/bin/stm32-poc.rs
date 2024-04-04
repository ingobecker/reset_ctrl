#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let _pp = embassy_stm32::init(config);

    reset_ctrl::run();
    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}
