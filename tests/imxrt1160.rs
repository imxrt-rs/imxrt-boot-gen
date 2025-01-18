//! Tests specific to 1160 families.

#![cfg(feature = "imxrt1160")]

#[test]
fn serial_clock_frequency() {
    use imxrt_boot_gen::flexspi::SerialClockFrequency;

    assert_eq!(SerialClockFrequency::MHz30 as u32, 1);
    assert_eq!(SerialClockFrequency::MHz50 as u32, 2);
    assert_eq!(SerialClockFrequency::MHz60 as u32, 3);
    assert_eq!(SerialClockFrequency::MHz80 as u32, 4);
    assert_eq!(SerialClockFrequency::MHz100 as u32, 5);
    assert_eq!(SerialClockFrequency::MHz120 as u32, 6);
    assert_eq!(SerialClockFrequency::MHz133 as u32, 7);
    assert_eq!(SerialClockFrequency::MHz166 as u32, 8);
}

#[test]
fn ip_serial_clock_frequency() {
    use imxrt_boot_gen::serial_flash::nor::SerialClockFrequency;

    assert_eq!(SerialClockFrequency::NoChange as u32, 0);

    assert_eq!(SerialClockFrequency::MHz30 as u32, 1);
    assert_eq!(SerialClockFrequency::MHz50 as u32, 2);
    assert_eq!(SerialClockFrequency::MHz60 as u32, 3);
    assert_eq!(SerialClockFrequency::MHz80 as u32, 4);
    assert_eq!(SerialClockFrequency::MHz100 as u32, 5);
    assert_eq!(SerialClockFrequency::MHz120 as u32, 6);
    assert_eq!(SerialClockFrequency::MHz133 as u32, 7);
}
