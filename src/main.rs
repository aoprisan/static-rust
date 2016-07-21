extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate nix;

extern crate rand;
extern crate libc;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::prelude::*;

fn main() {
    let mut router = Router::new();
    Iron::new(router).http("localhost:3000").unwrap();
}
