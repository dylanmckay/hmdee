extern crate psvr;
extern crate libusb;

use psvr::ResultExt;
use std::{thread, time, process};

fn main() {
    match run() {
        Ok(..) => (),
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        },
    }
}

fn run() -> Result<(), psvr::Error> {
    let libusb = libusb::Context::new().chain_err(|| "could not initialize libusb")?;
    // libusb.set_log_level(libusb::LogLevel::Debug);

    let mut psvr = match psvr::usb::iter(&libusb).chain_err(|| "failed to discover psvr devices")?.next() {
        Some(Ok(psvr)) => psvr,
        Some(Err(e)) => return Err(e),
        None => return Err("no PSVR devices connected".into()),
    };

    println!("discovered PSVR device!");
    psvr.print_information()?;

    psvr.set_power(true).chain_err(|| "failed to set power to true")?;

    for _ in 0..600 {
        thread::sleep(time::Duration::from_millis(30));
        let sensor_frame = psvr.receive_sensor().expect("failed to receive from sensor");

        println!("{:?}", sensor_frame.status);
    }

    psvr.set_power(false).chain_err(|| "failed to set power to false")?;
    Ok(())
}

