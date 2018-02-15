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

    for foo in psvr::iter(&hidapi).unwrap() {
    }

    let mut psvr = match psvr::get(&hidapi)? {
        Some(psvr) => psvr,
        None => return Err("no PSVR devices connected".into()),
    };

    psvr.power_on()?;

    // println!("discovered PSVR device, printing information");
    // psvr.print_information()?;

    // println!("Enabling VR tracking");
    // psvr.vr_tracking().chain_err(|| "failed to enable VR tracking")?;

    thread::sleep(time::Duration::from_millis(100));
    println!("starting to read from sensors");

    for _ in 0..200 {
        let sensor_frame = psvr.receive_sensor().expect("failed to receive from sensor");
        thread::sleep(time::Duration::from_millis(20));

        println!("{:?}", sensor_frame.instants[0]);
    }

    println!("finished reading from sensors");
    psvr.close()?;
    Ok(())
}

