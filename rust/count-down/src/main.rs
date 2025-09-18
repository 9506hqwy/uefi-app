#![no_main]
#![no_std]

use core::fmt::Write;
use core::format_args;
use uefi::boot;
use uefi::prelude::*;
use uefi::runtime::{self, ResetType};
use uefi::system;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    system::with_stdout(|stdout| stdout.clear()).unwrap();

    let sec = 5usize;

    system::with_stdout(|stdout| {
        stdout.write_fmt(format_args!("shutdown after {sec} seconds.\n\n"))
    })
    .unwrap();

    let (x, y) = system::with_stdout(|stdout| stdout.cursor_position());

    for i in (0..=sec).rev() {
        system::with_stdout(|stdout| stdout.write_fmt(format_args!("{i} second\n"))).unwrap();

        boot::stall(1_000_000);
        system::with_stdout(|stdout| stdout.set_cursor_position(x, y)).unwrap();
    }

    // https://uefi.org/specs/UEFI/2.10/08_Services_Runtime_Services.html#reset-system
    runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, None)
}
