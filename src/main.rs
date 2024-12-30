#![no_std]
#![no_main]

mod fmt;
use oled::OLED;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut oled = OLED::new(p.PC14, p.PB5, p.PB4, p.PB3);

    loop {
        oled.show_string(1, 1, "Hello, World!").ok();
        oled.show_number(1, 15, 1000, 4, 16).ok();
        oled.refresh().ok();
        
        // 添加一个简单的延时
        Timer::after_millis(500).await;
    }
}