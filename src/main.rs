extern crate rustc_serialize;
extern crate docopt;
extern crate linkstreams;

use docopt::Docopt;

use linkstreams::algo;
use linkstreams::data::link::Time;
use linkstreams::data::link::Node;
use linkstreams::data::link::Link;

static USAGE: &'static str = "
Usage:
       linkstream rename
       linkstream info (count | degrees <nbNodes> | repart <nbNodes>)
       linkstream filter (node <node>... | time <start> <stop> | both <start> <stop> <node>...)
       linkstream calc connexity <delta> <nbNodes> [node <node>... | time <start> <stop> | both <start> <stop> <node>...]
";

#[allow(non_snake_case)]
#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_rename: bool,
    cmd_info: bool,
    cmd_count: bool,
    cmd_degrees: bool,
    cmd_repart: bool,
    cmd_filter: bool,
    cmd_calc: bool,
    cmd_connexity: bool,
    cmd_node: bool,
    cmd_time: bool,
    cmd_both: bool,
    arg_node: Vec<String>,
    arg_start: String,
    arg_stop: String,
    arg_nbNodes: String,
    arg_delta: String,
}

#[allow(non_snake_case)]
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    // RENAME
    if args.cmd_rename {
        let stream = linkstreams::stdin::rename();
        for link in stream {
            println!("{}", link.to_string());
        }
    }
    // INFO
    else if args.cmd_info {
        // COUNT
        if args.cmd_count {
            let (nb_nodes, nb_links) = linkstreams::stdin::count_nodes_and_links();
            println!("number of nodes: {}\nnumber of links: {}", nb_nodes, nb_links);
        }
        // DEGREES
        else if args.cmd_degrees {
            let nbNodes = usize::from_str_radix(&args.arg_nbNodes, 10).unwrap();
            let degrees = algo::count_degrees(nbNodes);
            for i in 0..degrees.len() {
                println!("{}: {}", i, degrees[i]);
            }
        }
        // REPART
        else if args.cmd_repart {
            let nbNodes = usize::from_str_radix(&args.arg_nbNodes, 10).unwrap();
            let repart = algo::count_first_and_last_apparition(nbNodes);
            for i in 0..repart.len() {
                let (first, last) = repart[i];
                println!("{}: {} {}", i, first, last);
            }
        }
    }
    // FILTER
    else if args.cmd_filter {
        // TIME
        if args.cmd_time {
            let start = Time::from_str_radix(&args.arg_start, 10).unwrap();
            let stop = Time::from_str_radix(&args.arg_stop, 10).unwrap();
            linkstreams::stdin::filter(|l: Link| {
                let cond = l.time >= start && l.time < stop;
                if cond { println!("{}", l.to_string()); }
                cond
            });
        }
        // NODE
        else if args.cmd_node {
            let nodes: Vec<Node> = args.arg_node.iter().map(|s: &String| {
                Node::from_str_radix(s, 10).unwrap()
            }).collect();
            linkstreams::stdin::filter(|l: Link| {
                let cond = nodes.contains(&l.node1) && nodes.contains(&l.node2);
                if cond { println!("{}", l.to_string()); }
                cond
            });
        }
        // BOTH
        else if args.cmd_both {
            let start = Time::from_str_radix(&args.arg_start, 10).unwrap();
            let stop = Time::from_str_radix(&args.arg_stop, 10).unwrap();
            let nodes: Vec<Node> = args.arg_node.iter().map(|s: &String| {
                Node::from_str_radix(s, 10).unwrap()
            }).collect();
            linkstreams::stdin::filter(|l: Link| {
                let cond = l.time >= start
                    && l.time < stop
                    && nodes.contains(&l.node1)
                    && nodes.contains(&l.node2);
                if cond { println!("{}", l.to_string()); }
                cond
            });
        }
    }
    // CALC
    if args.cmd_calc {
        // CONNEXITY
        if args.cmd_connexity {
            let delta = Time::from_str_radix(&args.arg_delta, 10).unwrap();
            let nbNodes = usize::from_str_radix(&args.arg_nbNodes, 10).unwrap();
            // TIME
            if args.cmd_time {
                let start = Time::from_str_radix(&args.arg_start, 10).unwrap();
                let stop = Time::from_str_radix(&args.arg_stop, 10).unwrap();
                algo::is_delta_connected(delta, nbNodes,
                                         &|_| true,
                                         &move |time: Time| {time >= start && time < stop});
            }
            // NODE
            else if args.cmd_node {
                let nodes: Vec<Node> = args.arg_node.iter().map(|s: &String| {
                    Node::from_str_radix(s, 10).unwrap()
                }).collect();
                algo::is_delta_connected(delta, nbNodes,
                                         &move |node: Node| { nodes.contains(&node) },
                                         &|_| true);
            }
            // BOTH
            else if args.cmd_both {
                let start = Time::from_str_radix(&args.arg_start, 10).unwrap();
                let stop = Time::from_str_radix(&args.arg_stop, 10).unwrap();
                let nodes: Vec<Node> = args.arg_node.iter().map(|s: &String| {
                    Node::from_str_radix(s, 10).unwrap()
                }).collect();
                algo::is_delta_connected(delta, nbNodes,
                                         &move |node: Node| { nodes.contains(&node) },
                                         &move |time: Time| {time >= start && time < stop});
            }
            // N/A
            else {
                algo::is_delta_connected(delta, nbNodes, &|_| true, &|_| true);
            }
        }
    }
}
