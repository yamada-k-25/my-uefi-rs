#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::device_path::text::{
  AllowShortcuts, DevicePathToText, DisplayOnly
};
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::SearchType;
use uefi::{Identify, Result};

#[entry]
fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut st).unwrap();
    let boot_services = st.boot_services();

    print_image_path(boot_services).unwrap();

    st.boot_services().stall(10_000_000);
    Status::SUCCESS
}

fn print_image_path(boot_services: &BootServices) -> Result {
  // Open a protocol interface for a handle.
  let loaded_image = boot_services
    .open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())?;
  // Return requested protocol in buffer.
  let device_path_to_text_handle = *boot_services
    .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
    .first()
    .expect("DevicePathToText is missing");
  // DevicePath to text utility.
  let device_path_to_text = boot_services
    .open_protocol_exclusive::<DevicePathToText>(
      device_path_to_text_handle,
    )?;

  // Return image decvice path.
  let image_device_path = 
    loaded_image.file_path().expect("File path is not set.");

  // Return the image path text.
  // if DisplayOnly is true, display shorter text representation.
  let image_device_path_text = device_path_to_text
    .convert_device_path_to_text(boot_services, image_device_path, DisplayOnly(true), AllowShortcuts(false))
    .expect("convert_device_path_to_text failed");

  info!("Image path* {}", &*image_device_path_text);
  Ok(())
}