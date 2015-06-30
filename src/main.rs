extern crate linkstreams;
// use std::env;
use linkstreams::algo;

fn main() {
    let tfilter = |_| true;
    let nfilter = |node| node > 0;
    algo::is_delta_connected(5000, 63, &nfilter, &tfilter);

}
