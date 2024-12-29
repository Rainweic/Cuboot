
use embassy_stm32::{mode::Blocking, peripherals::{PC12, PD2, UART5}, usart::{Config, Parity, Uart}};

use crate::{common::LidarConfig, errors::DriverErrors};


#[allow(unused)]
pub struct Lidar {

    // uart5串口
    uart: Uart<'static, Blocking>,
    // 雷达配置
    config: LidarConfig,
}


#[allow(unused)]
impl Lidar {

    pub fn new(
        uart: UART5,
        tx: PC12,
        rx: PD2
    ) -> Self {

        let mut config = Config::default();
        config.baudrate = 115_200;
        config.parity = Parity::ParityNone;
        config.stop_bits = embassy_stm32::usart::StopBits::STOP1;
        config.data_bits = embassy_stm32::usart::DataBits::DataBits8;

        let usart = Uart::new_blocking(uart, rx, tx, config).unwrap();

        Self { 
            uart: usart, 
            config: Default::default() 
        }
    }

}


// IO
#[allow(unused)]
impl Lidar {
    pub fn read(&mut self, buffer: &mut [u8]) {
        self.uart.blocking_read(buffer).ok();
    }
}


// Main Func
impl Lidar {

    /**
     * 是否启用雷达的噪点滤波
     */
    pub fn set_noise_filter(&mut self, enable: bool) {
        self.config.use_noise_filter = enable;
    }

    /**
     * 分析单个雷达数据包
     */
    pub fn analysis_one_pkg(&self, byte: u8) -> Result<(), DriverErrors> {
        Ok(())
    }


}