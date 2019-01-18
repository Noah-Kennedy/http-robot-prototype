#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use core::borrow::Borrow;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::RwLock;

static mut NUM_SENDER: Option<Arc<Sender<u64>>> = Option::None;
static mut NUM_RECEIVER: Option<Arc<Receiver<u64>>> = Option::None;
static mut SUM: AtomicUsize = AtomicUsize::new(0);

#[get("/")]
fn index() -> String {
    unsafe {
        SUM.load(Ordering::SeqCst).to_string()
    }
}

fn main() {
    set_channels();
    rocket::ignite().mount("/", routes![index]).launch();
}

fn set_channels() {
    unsafe {
        let (sn, rn) = channel();
        NUM_SENDER = Some(Arc::new(sn));
        NUM_RECEIVER = Some(Arc::new(rn));
    }
}

fn run_sum_thread() {
    loop {
        let num = read_next_num();
    }
}

fn read_next_num() -> u64 {
    unsafe {
        match match NUM_RECEIVER {
            Some(x) => x,
            None => unimplemented!(),
        }.recv() {
            Ok(x) => x,
            Err(_) => unimplemented!(),
        }
    }
}

fn add_to_sum(n: usize) {
    unsafe {
        SUM.fetch_add(n, Ordering::SeqCst);
    }
}