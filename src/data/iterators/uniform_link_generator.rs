use data::link::{Link, Node, Time};
// use data::iterators::link_iterator::*;
use data::rand::thread_rng;
use data::rand::Rng;


pub struct UnifLinkGenerator {
    nb_nodes: usize,
    proba: f64,
    time_max: Time,
    time: Time,
    node1: Node,
    node2: Node,
}

impl UnifLinkGenerator {
    pub fn new(nb_nodes: usize, time_max: Time, proba: f64) -> UnifLinkGenerator {
        UnifLinkGenerator {
            nb_nodes: nb_nodes,
            proba: 1. / proba,
            time_max: time_max,
            time: time_max,
            node1: 0,
            node2: 1,
        }
    }
    fn current_link(&self) -> Link {
        Link {
            node1: self.node1,
            node2: self.node2,
            time: self.time,
        }
    }
}


impl Iterator for UnifLinkGenerator {
    type Item = Link;
    fn next(&mut self) -> Option<Link> {
        while self.time <= self.time_max {
            while self.node1 < self.nb_nodes {
                while self.node2 < self.nb_nodes {
                    let link = self.current_link();
                    self.node2 = self.node2 + 1;
                    let rnf : f64 = thread_rng().gen();
                    if rnf <= self.proba {
                        return Some(link);
                    }
                }
                self.node1 = self.node1 + 1;
                self.node2 = self.node1 + 1;
            }
            self.time = self.time - 1;
            self.node1 = 0;
            self.node2 = 1;
        }
        None
    }
}
