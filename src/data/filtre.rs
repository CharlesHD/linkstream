use data::link::Link;
use data::link::Node;
use data::link::Time;

pub type NodeFiltre = Box<Fn(Node) -> bool>;
pub type TimeFiltre = Box<Fn(Time) -> bool>;
pub type LinkFiltre = Box<Fn(Link) -> bool>;

pub fn combine(nfiltre: NodeFiltre, tfiltre: TimeFiltre) -> LinkFiltre {
    Box::new(move |link: Link| {
        nfiltre(link.node1) && nfiltre(link.node2) && tfiltre(link.time)
    })
}
