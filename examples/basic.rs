extern crate psvr;
extern crate hidapi;
extern crate nalgebra as na;
extern crate delta;

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
    let mut timer = delta::Timer::new();

    let mut psvr = match psvr::get(&hidapi)? {
        Some(psvr) => psvr,
        None => return Err("no PSVR devices connected".into()),
    };

    psvr.power_on()?;

    for _ in 0..200 {
        let sensor = psvr.receive_sensor().expect("failed to receive from sensor");
        let delta = timer.mark();

        println!("elapsed: {}, orientation: {:?}", delta, psvr.orientation());
    }

    println!("finished reading from sensors");
    // psvr.close()?;
    Ok(())
}

