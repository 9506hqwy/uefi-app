#![no_main]
#![no_std]

use core::fmt::Write;
use core::format_args;
use uefi::prelude::*;
use uefi::table::runtime::ResetType;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    system_table.stdout().clear().unwrap();

    let sec = 5usize;

    system_table
        .stdout()
        .write_fmt(format_args!("shutdown after {} seconds.\n\n", sec))
        .unwrap();

    let (x, y) = system_table.stdout().cursor_position();

    for i in (0..=sec).rev() {
        system_table
            .stdout()
            .write_fmt(format_args!("{} second\n", i))
            .unwrap();

        system_table.boot_services().stall(1_000_000);
        system_table.stdout().set_cursor_position(x, y).unwrap();
    }

    // https://uefi.org/specs/UEFI/2.10/08_Services_Runtime_Services.html#reset-system
    system_table
        .runtime_services()
        .reset(ResetType::SHUTDOWN, Status::SUCCESS, None)
}
