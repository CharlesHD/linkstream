extern crate linkstreams;
// use std::env;
use linkstreams::algo;

fn main() {
    let tfiltre = Box::new(|_| true);
    let nfiltre = Box::new(|_| true);
    let u: usize = 30;
    let b = [0; u];
    algo::is_delta_connected(0, 63, nfiltre, tfiltre);

}
