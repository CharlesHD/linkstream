use data::link::Link;
use data::link::Node;
use data::link::Time;

pub type NodeFilter = Fn(Node) -> bool;
pub type TimeFilter = Fn(Time) -> bool;
pub type LinkFilter = Fn(Link) -> bool;

pub fn combine(link: Link, nfilter: &NodeFilter, tfilter: &TimeFilter) -> bool {
    nfilter(link.node1) && nfilter(link.node2) && tfilter(link.time)
}
