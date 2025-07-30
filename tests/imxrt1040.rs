//! Tests specific to 1040 family.

use imxrt_boot_gen::{Imxrt, flexspi::SerialClockOption::*};
const CHIP: Imxrt = Imxrt::Imxrt1040;

#[test]
fn serial_clock_frequency() {
    assert_eq!(CHIP.serial_clock_frequency(MHz30).get(), 1);
    assert_eq!(CHIP.serial_clock_frequency(MHz50).get(), 2);
    assert_eq!(CHIP.serial_clock_frequency(MHz60).get(), 3);
    assert_eq!(CHIP.serial_clock_frequency(MHz75).get(), 4);
    assert_eq!(CHIP.serial_clock_frequency(MHz80).get(), 5);
    assert_eq!(CHIP.serial_clock_frequency(MHz100).get(), 6);
    assert_eq!(CHIP.serial_clock_frequency(MHz120).get(), 7);
    assert_eq!(CHIP.serial_clock_frequency(MHz133).get(), 8);
    assert_eq!(CHIP.serial_clock_frequency(MHz166).get(), 9);
}

#[test]
fn ip_serial_clock_frequency() {
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz30).get(), 1);
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz50).get(), 2);
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz60).get(), 3);
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz75).get(), 4);
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz80).get(), 5);
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz100).get(), 6);
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz120).get(), 7);
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz133).get(), 8);
    assert_eq!(CHIP.ip_serial_clock_frequency(MHz166).get(), 9);
}
