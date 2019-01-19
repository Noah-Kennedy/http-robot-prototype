#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::State;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;

#[get("/")]
fn index(sum: State<AtomicUsize>) -> String {
    sum.fetch_add(1, Ordering::SeqCst).to_string()
}

#[get("/echo/<val>")]
fn echo(val: String) -> String {
    val
}

fn main() {
    rocket::ignite().manage(AtomicUsize::new(0)).mount("/", routes![index, echo]).launch();
}