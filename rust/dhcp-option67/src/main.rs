#![no_main]
#![no_std]

use core::fmt::Write;
use core::str;
use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::network::pxe::{BaseCode, DhcpV4Packet};
use uefi::table::system_table_boot as pointer;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    // https://uefi.org/specs/UEFI/2.10/09_Protocols_EFI_Loaded_Image.html#protocols-efi-loaded-image
    let loaded_image = system_table
        .boot_services()
        .open_protocol_exclusive::<LoadedImage>(image_handle)
        .unwrap();

    // https://uefi.org/specs/UEFI/2.10/24_Network_Protocols_SNP_PXE_BIS.html#pxe-base-code-protocol
    let pxe = system_table
        .boot_services()
        .open_protocol_exclusive::<BaseCode>(loaded_image.device().unwrap())
        .unwrap();

    // https://uefi.org/specs/UEFI/2.10/24_Network_Protocols_SNP_PXE_BIS.html#dhcp-packet-data-types
    let ack: &DhcpV4Packet = pxe.mode().dhcp_ack.as_ref();

    // Remove trailing null character.
    let mut boot_file_slice = ack.bootp_boot_file.as_slice();
    while let Some(b) = boot_file_slice.strip_suffix(&[0]) {
        boot_file_slice = b;
    }

    let boot_file = str::from_utf8(boot_file_slice).unwrap();

    // Use pointer because of stdout() require mutable reference.
    pointer().unwrap().stdout().clear().unwrap();
    pointer().unwrap().stdout().write_str(boot_file).unwrap();

    system_table.boot_services().stall(20_000_000);

    Status::SUCCESS
}
