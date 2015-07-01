use std::io;
use std::io::BufRead;
use std::collections::HashMap;
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


/// Rename the stdin linkstream replacing not existing nodes.
pub fn rename() -> LinkStream {
    let mut seens: HashMap<Node, Node> = HashMap::new();
    let mut count = 0;
    map(|link: Link| {
        if ! seens.contains_key(&link.node1) {
            seens.insert(link.node1, count);
            count = count + 1;
        }
        if ! seens.contains_key(&link.node2) {
            seens.insert(link.node2, count);
            count = count + 1;
        }
        let index1 = *seens.get(&link.node1).unwrap();
        let index2 = *seens.get(&link.node2).unwrap();
        Link {
            node1: index1,
            node2: index2,
            time: link.time
        }
    })
}

/// Count the number of different nodes the stdin linkstream has and the number of links
pub fn count_nodes_and_links() -> (usize, usize) {
    let mut seens: Vec<Node> = Vec::new();
    let mut count = 0;
    map(|link: Link| {
        count = count + 1;
        if ! seens.contains(&link.node1) { seens.push(link.node1); }
        if ! seens.contains(&link.node2) { seens.push(link.node2); }
    });
    (seens.len(), count)
}
