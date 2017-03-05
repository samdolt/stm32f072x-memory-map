//! Memory map for STM32F30X microcontrollers

#![deny(missing_docs)]
#![no_std]

extern crate volatile_register;

#[allow(missing_docs)]
pub mod btim;
#[allow(missing_docs)]
pub mod dbgmcu;
#[allow(missing_docs)]
pub mod gpio;
#[allow(missing_docs)]
pub mod gptim;
#[allow(missing_docs)]
pub mod i2c;
#[allow(missing_docs)]
pub mod rcc;
#[allow(missing_docs)]
pub mod spi;
#[allow(missing_docs)]
pub mod usart;

use btim::BTim;
use dbgmcu::Dbgmcu;
use gpio::Gpio;
use gptim::GpTim;
use i2c::I2c;
use rcc::Rcc;
use spi::Spi;
use usart::Usart;

const CRC: usize = 0x40023000;
const GPIOF: usize = 0x48001400;
const GPIOD: usize = 0x48000c00;
const GPIOC: usize = 0x48000800;
const GPIOB: usize = 0x48000400;
const GPIOE: usize = 0x48001000;
const GPIOA: usize = 0x48000000;
const SPI1: usize = 0x40013000;
const SPI2: usize = 0x40003800;
const DAC: usize = 0x40007400;
const PWR: usize = 0x40007000;
const I2C1: usize = 0x40005400;
const I2C2: usize = 0x40005800;
const IWDG: usize = 0x40003000;
const WWDG: usize = 0x40002c00;
const TIM1: usize = 0x40012c00;
const TIM2: usize = 0x40000000;
const TIM3: usize = 0x40000400;
const TIM14: usize = 0x40002000;
const TIM6: usize = 0x40001000;
const TIM7: usize = 0x40001400;
const EXTI: usize = 0x40010400;
const NVIC: usize = 0xe000e100;
const DMA: usize = 0x40020000;
const RCC: usize = 0x40021000;
const SYSCFG: usize = 0x40010000;
const ADC: usize = 0x40012400;
const USART1: usize = 0x40013800;
const USART2: usize = 0x40004400;
const USART3: usize = 0x40004800;
const USART4: usize = 0x40004c00;
const COMP: usize = 0x4001001c;
const RTC: usize = 0x40002800;
const TIM15: usize = 0x40014000;
const TIM16: usize = 0x40014400;
const TIM17: usize = 0x40014800;
const TSC: usize = 0x40024000;
const CEC: usize = 0x40007800;
const FLASH: usize = 0x40022000;
const DBGMCU: usize = 0x40015800;
const USB: usize = 0x40005c00;
const CRS: usize = 0x40006c00;
const CAN: usize = 0x40006400;



/// DBGMCU register block (&'static)
pub fn dbgmcu() -> &'static Dbgmcu {
    unsafe { deref(DBGMCU) }
}

/// DBGMCU register block (&'static mut)
pub unsafe fn dbgmcu_mut() -> &'static mut Dbgmcu {
    deref_mut(DBGMCU)
}

/// GPIOA register block (&'static)
pub fn gpioa() -> &'static Gpio {
    unsafe { deref(GPIOA) }
}

/// GPIOA register block (&'static mut)
pub unsafe fn gpioa_mut() -> &'static mut Gpio {
    deref_mut(GPIOA)
}

/// GPIOB register block (&'static)
pub fn gpiob() -> &'static Gpio {
    unsafe { deref(GPIOB) }
}

/// GPIOB register block (&'static mut)
pub unsafe fn gpiob_mut() -> &'static mut Gpio {
    deref_mut(GPIOB)
}

/// GPIOC register block (&'static)
pub fn gpioc() -> &'static Gpio {
    unsafe { deref(GPIOC) }
}

/// GPIOC register block (&'static mut)
pub unsafe fn gpioc_mut() -> &'static mut Gpio {
    deref_mut(GPIOC)
}

/// GPIOD register block (&'static)
pub fn gpiod() -> &'static Gpio {
    unsafe { deref(GPIOD) }
}

/// GPIOD register block (&'static mut)
pub unsafe fn gpiod_mut() -> &'static mut Gpio {
    deref_mut(GPIOD)
}

/// GPIOE register block (&'static)
pub fn gpioe() -> &'static Gpio {
    unsafe { deref(GPIOE) }
}

/// GPIOE register block (&'static mut)
pub unsafe fn gpioe_mut() -> &'static mut Gpio {
    deref_mut(GPIOE)
}

/// GPIOF register block (&'static)
pub fn gpiof() -> &'static Gpio {
    unsafe { deref(GPIOF) }
}

/// GPIOF register block (&'static mut)
pub unsafe fn gpiof_mut() -> &'static mut Gpio {
    deref_mut(GPIOF)
}

/// I2C1 register block (&'static)
pub fn i2c1() -> &'static I2c {
    unsafe { deref(I2C1) }
}

/// I2C1 register block (&'static mut)
pub unsafe fn i2c1_mut() -> &'static mut I2c {
    deref_mut(I2C1)
}

/// RCC register block (&'static)
pub fn rcc() -> &'static Rcc {
    unsafe { deref(RCC) }
}

/// RCC register block (&'static mut)
pub unsafe fn rcc_mut() -> &'static mut Rcc {
    deref_mut(RCC)
}

/// SPI1 register block (&'static)
pub fn spi1() -> &'static Spi {
    unsafe { deref(SPI1) }
}

/// SPI1 register block (&'static mut)
pub unsafe fn spi1_mut() -> &'static mut Spi {
    deref_mut(SPI1)
}

/// TIM2 register block (&'static)
pub fn tim2() -> &'static GpTim {
    unsafe { deref(TIM2) }
}

/// TIM2 register block (&'static mut)
pub unsafe fn tim2_mut() -> &'static mut GpTim {
    deref_mut(TIM2)
}

/// TIM3 register block (&'static)
pub fn tim3() -> &'static GpTim {
    unsafe { deref(TIM3) }
}

/// TIM3 register block (&'static mut)
pub unsafe fn tim3_mut() -> &'static mut GpTim {
    deref_mut(TIM3)
}


/// TIM6 register block (&'static)
pub fn tim6() -> &'static BTim {
    unsafe { deref(TIM6) }
}

/// TIM6 register block (&'static mut)
pub unsafe fn tim6_mut() -> &'static mut BTim {
    deref_mut(TIM6)
}

/// TIM7 register block (&'static)
pub fn tim7() -> &'static BTim {
    unsafe { deref(TIM7) }
}

/// TIM7 register block (&'static mut)
pub unsafe fn tim7_mut() -> &'static mut BTim {
    deref_mut(TIM7)
}

/// USART1 register block (&'static)
pub fn usart1() -> &'static Usart {
    unsafe { deref(USART1) }
}

/// USART1 register block (&'static mut)
pub unsafe fn usart1_mut() -> &'static mut Usart {
    deref_mut(USART1)
}

/// USART2 register block (&'static)
pub fn usart2() -> &'static Usart {
    unsafe { deref(USART2) }
}

/// USART2 register block (&'static mut)
pub unsafe fn usart2_mut() -> &'static mut Usart {
    deref_mut(USART2)
}

/// USART3 register block (&'static)
pub fn usart3() -> &'static Usart {
    unsafe { deref(USART3) }
}

/// USART3 register block (&'static mut)
pub unsafe fn usart3_mut() -> &'static mut Usart {
    deref_mut(USART3)
}

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}

// Here we extend the peripheral API -- AKA ~~svd2rust is~~ SVD files are great
// but not perfect
use core::ptr;

impl spi::Dr {
    /// Reads a byte (`u8`) from this register
    pub fn read_u8(&self) -> u8 {
        unsafe { ptr::read_volatile(self as *const _ as *const u8) }
    }

    /// Writes a byte (`u8`) to this register
    pub fn write_u8(&mut self, value: u8) {
        unsafe { ptr::write_volatile(self as *mut _ as *mut u8, value) }
    }
}
