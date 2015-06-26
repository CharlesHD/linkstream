use std::io;
use std::io::BufRead;
use data::link::*;

/// Map a function through the filtered stdin Linkstream.
pub fn filter_map<P, F, B>(mut f : F, mut pred : P) -> Vec<B> where F: FnMut(Link) -> B, P: FnMut(Link) -> bool {
    let input = io::stdin();
    let stream: Vec<B> = input.lock().lines().filter_map(|line| {
        let link = parse_line(&line.unwrap());
        if pred(link) {
            Some(f(link))
        } else {
            None
        }
    }).collect();
    stream
}

/// Convert stdin into the raw Linkstream it represent.
///
/// Implemented as :
/// ```
/// map(|x| x);
/// ```
/// or
/// ```
/// filter(|_| true);
/// ```
pub fn parse() -> LinkStream {
    map(|x| x)
}

/// Map a function through the stdin Linkstream.
///
/// Implemented as :
/// ```
/// filter_map(func, |_| true);
/// ```
pub fn map<B, F>(func : F) -> Vec<B> where F: FnMut(Link) -> B {
    filter_map(func, |_| true)
}

/// Filter the stdin Linkstream.
///
/// Implemented as :
/// ```
/// filter_map(|x| x, pred);
/// ```
pub fn filter<P>(pred : P) -> LinkStream where P: FnMut(Link) -> bool {
    filter_map(|x| x, pred)
}
