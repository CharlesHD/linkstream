use data::link::Link;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

/// Standard input based LinkIterator
///
/// Read the standard input line by line and parse it into a Link structure.
pub struct StdinLinkIter {
    stdin: io::Stdin
}

impl StdinLinkIter {
    /// Create a standard input based LinkIterator
    pub fn new() -> StdinLinkIter {
        StdinLinkIter { stdin: io::stdin()}
    }
}

impl Iterator for StdinLinkIter {
    type Item = Link;
    fn next(&mut self) -> Option<Link> {
        let line = self.stdin.lock().lines().next();
        match line {
            None => None,
            Some(res_line) => {
                let line = res_line.ok().expect("Failed to read line");
                let link = Link::from_str(&line)
                    .ok()
                    .expect(&format!("Failed to parse line : {}", line));
                Some(link)
            }
        }
    }
}
