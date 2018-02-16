extern crate psvr;
extern crate hidapi;
extern crate nalgebra as na;

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

    let mut psvr = match psvr::get(&hidapi)? {
        Some(psvr) => psvr,
        None => return Err("no PSVR devices connected".into()),
    };

    psvr.power_on()?;

    for _ in 0..200 {
        let _ = psvr.receive_sensor().expect("failed to receive from sensor");

        println!("orientation: {:?}", psvr.orientation());
        thread::sleep(time::Duration::from_millis(20));
    }

    println!("finished reading from sensors");
    psvr.close()?;
    Ok(())
}

