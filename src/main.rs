#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

#[entry]
fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut st).unwrap();
    info!("Hello World!");
    st.boot_services().stall(10_000_000);
    Status::SUCCESS
}
