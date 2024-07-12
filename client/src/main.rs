use std::borrow::Borrow;
use std::future::Future;
use futures::executor::block_on;
use std::io::{self, Write};
use server::rest_bodies;
use server::rest_responses::{self, MenuItem};
use std::thread;
use std::time::Duration;
use rand::Rng;
use rand::seq::SliceRandom;
use client::web_connection::DefaultWebConnection;
use client::client_function_interface::{ClientFunctionInterface, DefaultClientFunctionInterface};
use client::sim;

const HOST: &str = "http://127.0.0.1:8000";
const TABLE_COUNT: u32 = 5;
const TABLET_COUNT: u32 = 30;
const RUN_TIME_MILLIS: u64 = 60000; // 1 minute
const MIN_DELAY_MILLIS: u64 = 300;
const MAX_DELAY_MILLIS: u64 = 4000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 1..TABLET_COUNT + 1 {
        thread::spawn(move || sim::client_tablet(i));
    }

    thread::sleep(Duration::from_millis(RUN_TIME_MILLIS));
    Result::Ok(())
}