extern crate psvr;
extern crate hidapi;

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
    let hidapi = hidapi::HidApi::new().unwrap();

    for foo in psvr::usb::iter(&hidapi).unwrap() {
    }

    let mut psvr = match psvr::usb::Psvr::open(&hidapi)? {
        Some(psvr) => psvr,
        None => return Err("no PSVR devices connected".into()),
    };

    println!("discovered PSVR device, printing information");
    psvr.print_information()?;

    println!("Setting power");
    psvr.set_power(true).chain_err(|| "failed to set power to true")?;
    // println!("Enabling VR tracking");
    // psvr.vr_tracking().chain_err(|| "failed to enable VR tracking")?;

    thread::sleep(time::Duration::from_millis(100));
    println!("starting to read from sensors");

    for _ in 0..600 {
        let sensor_frame = psvr.receive_sensor().expect("failed to receive from sensor");
        thread::sleep(time::Duration::from_millis(30));

        println!("{:?}", sensor_frame.status);
    }

    println!("finished reading from sensors");
    psvr.set_power(false).chain_err(|| "failed to set power to false")?;
    Ok(())
}

