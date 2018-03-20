extern crate psvr;
extern crate failure;
extern crate hidapi;
extern crate nalgebra as na;
extern crate delta;

use std::process;

fn main() {
    match run() {
        Ok(..) => (),
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        },
    }
}

fn run() -> Result<(), failure::Error> {
    let hidapi = hidapi::HidApi::new().unwrap();
    let mut timer = delta::Timer::new();

    let mut psvr = match psvr::get(&hidapi)? {
        Some(psvr) => psvr,
        None => panic!("no PSVR devices connected"),
    };

    psvr.power_on()?;

    for _ in 0..200 {
        let sensor = psvr.receive_sensor().expect("failed to receive from sensor");
        let delta = timer.mark();

        println!("elapsed: {}, orientation: {:?}, buttons: {:?}", delta, psvr.orientation(), sensor.buttons);
    }

    println!("finished reading from sensors");
    // psvr.close()?;
    Ok(())
}

