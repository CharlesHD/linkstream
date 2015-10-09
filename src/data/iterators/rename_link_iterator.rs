use data::link::{Link, Node};
use std::collections::HashMap;
use data::iterators::link_iterator::*;

/// Decorate a LinkIterator, renaming nodes.
///
/// Links from this Iterator get Nodes starting from 0 to the cardinality of nodes in the stream.
/// Renaming cost an HashMap for linking old and new nodes name.
pub struct RenameLinkIter<'a> {
    iter: &'a mut LinkIterator,
    pub seens: HashMap<Node, Node>,
    pub count: usize,
}

impl<'a> RenameLinkIter<'a> {
    /// Decorate a LinkIterator for renaming.
    pub fn new(iterator: &'a mut LinkIterator) -> RenameLinkIter<'a> {
        RenameLinkIter {
            iter: iterator,
            seens: HashMap::new(),
            count: 0,
        }
    }
}

impl<'a> Iterator for RenameLinkIter<'a> {
    type Item = Link;
    fn next(&mut self) -> Option<Link> {
        match self.iter.next() {
            None => None,
            Some(link) => {
                if ! self.seens.contains_key(&link.node1) {
                    self.seens.insert(link.node1, self.count);
                    self.count = self.count + 1;
                }
                if ! self.seens.contains_key(&link.node2) {
                    self.seens.insert(link.node2, self.count);
                    self.count = self.count + 1;
                }
                let index1 = *self.seens.get(&link.node1).unwrap();
                let index2 = *self.seens.get(&link.node2).unwrap();
                Some(Link {
                    node1: index1,
                    node2: index2,
                    time: link.time
                })
            }
        }
    }
}
