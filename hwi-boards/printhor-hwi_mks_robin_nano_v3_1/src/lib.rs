#![no_std]
#![feature(trait_alias)]
#![feature(type_alias_impl_trait)]
#![allow(stable_features)]
use embassy_stm32::interrupt;
use embassy_stm32::interrupt::Priority;
use embassy_stm32::interrupt::InterruptExt;
use embassy_executor::InterruptExecutor;
use embassy_executor::SendSpawner;

pub use defmt::{trace,debug,info,warn, error};
pub use defmt;

mod board;

pub use board::device;
pub use board::consts;
pub use board::IODevices;
pub use board::Controllers;
pub use board::MotionDevices;
pub use board::PwmDevices;

pub use board::init;
pub use board::setup;
pub use board::heap_current_size;
pub use board::heap_current_usage_percentage;
pub use board::stack_reservation_current_size;
pub use board::MACHINE_BOARD;
pub use board::MACHINE_TYPE;
pub use board::MACHINE_PROCESSOR;
pub use board::HEAP_SIZE_BYTES;
pub use board::MAX_STATIC_MEMORY;
pub use board::VREF_SAMPLE;
#[cfg(feature = "with-sdcard")]
pub use board::SDCARD_PARTITION;
#[cfg(feature = "with-usbserial")]
const USBSERIAL_BUFFER_SIZE: usize = 32;
#[cfg(feature = "with-uart-port-1")]
const UART_PORT1_BUFFER_SIZE: usize = 32;
#[cfg(feature = "with-uart-port-1")]
const UART_PORT1_BAUD_RATE: u32 = 115200;

pub static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();

#[interrupt]
unsafe fn RTC_ALARM() {
    EXECUTOR_HIGH.on_interrupt()
}

#[inline]
pub fn get_stepper_spawner() -> SendSpawner {
    interrupt::RTC_ALARM.set_priority(Priority::P5);
    interrupt::OTG_FS.set_priority(Priority::P6);
    EXECUTOR_HIGH.start(interrupt::RTC_ALARM)
}

#[inline]
pub fn launch_high_priotity<S: 'static + Send>(token: embassy_executor::SpawnToken<S>) -> Result<(),()> {
    let spawner = EXECUTOR_HIGH.start(interrupt::RTC_ALARM);
    spawner.spawn(token).map_err(|_| ())
}

#[inline]
pub fn init_logger() {
}

#[inline]
pub fn sys_reset() {
    cortex_m::peripheral::SCB::sys_reset();
}
