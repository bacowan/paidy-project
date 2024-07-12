use std::thread;
use std::time::Duration;
use client::sim;

const TABLET_COUNT: u32 = 30;
const RUN_TIME_MILLIS: u64 = 60000; // 1 minute

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..TABLET_COUNT + 1 {
        thread::spawn(move || sim::client_tablet(i));
    }

    thread::sleep(Duration::from_millis(RUN_TIME_MILLIS));
    Result::Ok(())
}