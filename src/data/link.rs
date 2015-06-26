use std::cmp::PartialEq;

/// Time is implemement as u64
pub type Time = u64;
/// Node is identified by a u32
pub type Node = usize;
/// Simple interraction between two Node at given Time
#[derive(Debug, Copy, Clone)]
pub struct Link {
    /// first interracting node
    pub node1: Node,
    /// second interracting node
    pub node2: Node,
    /// time at wich interraction occurs
    pub time: Time,
}

impl Link {
    /// String representation of a Link : "node1 node2 time"
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.node1, self.node2, self.time)
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Link) -> bool {
        self.node1 == other.node1
            && self.node2 == other.node2
            && self.time == other.time
    }

    fn ne(&self, other: &Link) -> bool {
        !self.eq(other)
    }
}


/// LinkStream is litterally a stream of Link. Implemented as a Vector.
pub type LinkStream = Vec<Link>;


/// Convert a line into a Link
///
/// # Example
/// ```
/// # use linkstreams::data::Link;
/// # use linkstreams::data::parse_line;
/// assert_eq!(parse_line("0 1 10"), Link {node1: 0, node2: 1, time: 10});
/// ```
pub fn parse_line(line: &str) -> Link {
    let data: Vec<&str> = line.split(" ").collect();
    assert!(data.len() >= 3, "Error in line, not representing a link !");
    Link {
        node1: data[0].parse::<usize>().unwrap(),
        node2: data[1].parse::<usize>().unwrap(),
        time: data[2].parse::<u64>().unwrap(),
    }
}
