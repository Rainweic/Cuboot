use core::fmt::Error;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_stm32::{gpio::{Level, Output, Speed}, peripherals::*};
use embassy_time::Delay;
use crate::font;


#[derive(Debug)]
pub enum OLEDError {
    OutOfBounds,
    // 可以添加其他错误类型...
}


#[allow(unused)]
pub struct OLED {
    sclk: Output<'static, PC14>,
    sdin: Output<'static, PB5>,
    rst: Output<'static, PB4>,
    rs: Output<'static, PB3>,
    gram: [[u8; 8]; 128],
}


/**
 * Constructor
 */
#[allow(unused)]
impl OLED {

    pub fn new(pc14: PC14, pb5: PB5, pb4: PB4, pb3: PB3) -> Self {

        // 初始化引脚
        let mut sclk = Output::new(pc14, Level::Low, Speed::VeryHigh);
        let mut sdin = Output::new(pb5, Level::Low, Speed::VeryHigh);
        let mut rst = Output::new(pb4, Level::Low, Speed::VeryHigh);
        let mut rs = Output::new(pb3, Level::Low, Speed::VeryHigh);

        // rst复位
        rst.set_low();
        Delay.delay_ms(100_u32);
        rst.set_high();

        let mut oled = Self {
            sclk,
            sdin,
            rst,
            rs,
            gram: [[0x00; 8]; 128],
        };

        oled.write_cmd(0xAE);   // 关闭显示
        oled.write_cmd(0xD5);   // 设置时钟分频因子,震荡频率
        oled.write_cmd(80);     // [3:0],分频因子;[7:4],震荡频率
        oled.write_cmd(0xA8);   // 设置驱动路数
        oled.write_cmd(0x3F);   // 默认0X3F(1/64) 
        oled.write_cmd(0xD3);   // 设置显示偏移
        oled.write_cmd(0x00);   // 默认为0

        oled.write_cmd(0x40);   // 设置显示开始行 [5:0],行数.
														
        oled.write_cmd(0x8D);   // 电荷泵设置
        oled.write_cmd(0x14);   // bit2，开启/关闭
        oled.write_cmd(0x20);   // 设置内存地址模式
        oled.write_cmd(0x02);   // [1:0],00，列地址模式;01，行地址模式;10,页地址模式;默认10;
        oled.write_cmd(0xA1);   // 段重定义设置,bit0:0,0->0;1,0->127;
        oled.write_cmd(0xC0);   // 设置COM扫描方向;bit3:0,普通模式;1,重定义模式 COM[N-1]->COM0;N:驱动路数
        oled.write_cmd(0xDA);   // 设置COM硬件引脚配置
        oled.write_cmd(0x12);   // [5:4]配置
		 
        oled.write_cmd(0x81);   // 对比度设置
        oled.write_cmd(0xEF);   // 1~255;默认0X7F (亮度设置,越大越亮)
        oled.write_cmd(0xD9);   // 设置预充电周期
        oled.write_cmd(0xf1);   // [3:0],PHASE 1;[7:4],PHASE 2;
        oled.write_cmd(0xDB);   // 设置VCOMH 电压倍率
        oled.write_cmd(0x30);   // [6:4] 000,0.65*vcc;001,0.77*vcc;011,0.83*vcc;
        
        oled.write_cmd(0xA4);   // 全局显示开启;bit0:1,开启;0,关闭;(白屏/黑屏)
        oled.write_cmd(0xA6);   // 设置显示方式;bit0:1,反相显示;0,正常显示	    						   
        oled.write_cmd(0xAF);   // 开启显示	

        oled.clear(); 

        oled
    }

}


/**
 * I/O module
 */
#[allow(unused)]
impl OLED {

    fn write_byte(&mut self, mut data: u8, is_cmd: bool) -> Result<(), Error> {
        if is_cmd {
            self.rs.set_high();
        } else {
            self.rs.set_low();
        }

        for _ in 0..8 {
            self.sclk.set_low();
            if data & 0x80 != 0 {
                self.sdin.set_high();
            } else {
                self.sdin.set_low();
            }
            self.sclk.set_high();
            data <<= 1;
        }

        Ok(())
    }


    fn write_cmd(&mut self, cmd: u8) -> Result<(), Error> {
        self.write_byte(cmd, true)
    }

    fn write_data(&mut self, data: u8) -> Result<(), Error> {
        self.write_byte(data, false)
    }
    
}


/**
 * Function module
 */
#[allow(unused)]
impl OLED {

    /**
     * 开启显示
     */
    pub fn display_on(&mut self) -> Result<(), Error> {
        self.write_cmd(0x8D);       // SET DCDC命令
        self.write_cmd(0x14);       // 开启DCDC
        self.write_cmd(0xAF);       // 开启显示
        Ok(())
    }
    
    /**
     * 关闭显示
     */
    pub fn display_off(&mut self) -> Result<(), Error> {
        self.write_cmd(0x8D);       // SET DCDC命令
        self.write_cmd(0x10);       // 关闭DCDC
        self.write_cmd(0xAE);       // 关闭显示
        Ok(())
    }

    /**
     * 清屏，将屏幕全部设置为黑色
     */
    pub fn clear(&mut self) -> Result<(), Error> {
        for i in 0..8 {
            for j in 0..128 {
                self.gram[j][i] = 0x00;
            }
        }
        self.refresh();
        Ok(())
    }

    /**
     * 刷新屏幕
     */
    pub fn refresh(&mut self) -> Result<(), Error> {
        for i in 0..8 {
            self.write_cmd(0xB0 + i as u8);             //设置页地址（0~7)
            self.write_cmd(0x00);                       //设置显示位置—列低地址
            self.write_cmd(0x10);                       //设置显示位置—列高地址
            for j in 0..128 {
                self.write_data(self.gram[j][i]);
            }
        }
        Ok(())
    }

    /**
     * 画点
     * x: 列
     * y: 行
     * fill: 是否填充
     */
    pub fn draw_point(&mut self, x: u8, y: u8, fill: bool) -> Result<(), OLEDError> {

        // 检查坐标是否越界
        if x > 127 || y > 63 {
            return Err(OLEDError::OutOfBounds);
        }

        let pos = 7 - y / 8;
        let bx = y % 8;
        let tmp = 1 << (7 - bx);
        if fill {
            self.gram[x as usize][pos as usize] |= tmp;
        } else {
            self.gram[x as usize][pos as usize] &= !tmp;
        }

        Ok(())
    }

    /**
     * 显示字符
     * x: 列
     * y: 行
     * chr: 字符
     * size: 字体大小
     * normal_mode: 是否正常模式, true: 正常模式, false: 反相模式
     */
    pub fn show_char(&mut self, mut x: u8, mut y: u8, mut chr: u8, size: u8, normal_mode: bool) -> Result<(), OLEDError> {
        
        let mut tmp;
        let y0 = y;
        chr = chr - ' ' as u8;

        for t in 0..size {

            if size == 12 {
                tmp = font::ASCII_1206[chr as usize][t as usize];
            } else {
                tmp = font::ASCII_1608[chr as usize][t as usize];
            }

            for _ in 0..8 {
                if (tmp & 0x80) != 0 {
                    self.draw_point(x, y, normal_mode);
                } else {
                    self.draw_point(x, y, !normal_mode);
                }
                tmp <<= 1;
                y += 1;
                if (y - y0) == size {
                    y = y0;
                    x += 1;
                    break;
                }
            }
        }

        Ok(())
    }


    /**
     * 显示数字
     * x: 列
     * y: 行
     * num: 数字 0~4294967295
     * len: 数字长度
     * size: 字体大小
     */
    pub fn show_number(&mut self, x: u8, y: u8, num: u32, len: u32, size: u8) -> Result<(), OLEDError> {


        let mut tmp;
        let mut enshow = false; 

        for t in 0..len {
            tmp = (num / 10_u32.pow(len - t - 1)) % 10;
            if enshow && t < (len - 1) {
                if (tmp == 0) {
                    self.show_char(x + (size / 2) * t as u8, y, ' ' as u8, size, true);
                    continue;
                } else {
                    enshow = true;
                }
            }
            self.show_char(x + (size / 2) * t as u8, y, tmp as u8 + '0' as u8, size, true);
        }

        Ok(())

    }


    /**
     * 显示字符串
     * x: 列
     * y: 行
     * str: 字符串
     */
    pub fn show_string(&mut self, mut x: u8, mut y: u8, str: &str) -> Result<(), OLEDError> {

        for c in str.chars() {
            if c == '\0' {
                break;
            }
            if x > 122 {
                x = 0;
                y += 16;
            }
            if y > 58 {
                y = 0;
                x = 0;
                self.clear();
            }
            self.show_char(x, y, c as u8, 12, true);
            x += 8;
        }

        Ok(())
    }

}
