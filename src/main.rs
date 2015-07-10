extern crate rustc_serialize;
extern crate docopt;
extern crate linkstreams;

use docopt::Docopt;

use linkstreams::*;
use linkstreams::data::link::Time;
use linkstreams::data::link::Node;
use linkstreams::data::iterators::*;

static USAGE: &'static str = "
Usage:
       linkstream rename
       linkstream info (count | degrees <nbNodes> | repart <nbNodes>)
       linkstream filter (node <node>... | time <start> <stop> | both <start> <stop> <node>...)
       linkstream calc connexity <delta> <nbNodes> [node <node>... | time <start> <stop> | both <start> <stop> <node>...]
       linkstream calc comps <delta> <nbNodes>
       linkstream calc exist <delta> <nbNodes>
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
    cmd_comps: bool,
    cmd_exist: bool,
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
    // Args extraction
    let mut nbNodes: Option<usize> = None;
    let mut start: Option<Time> = None;
    let mut stop: Option<Time> = None;
    let mut delta: Option<Time> = None;
    let mut nodes: Option<Vec<Node>> = None;
    let mut stdinLinks = stdin_link_iterator::StdinLinkIter::new();
    if args.cmd_calc || args.cmd_degrees || args.cmd_repart {
        nbNodes = Some(usize::from_str_radix(&args.arg_nbNodes, 10).unwrap());
    }
    if args.cmd_time || args.cmd_both {
        start = Some(Time::from_str_radix(&args.arg_start, 10).unwrap());
        stop = Some(Time::from_str_radix(&args.arg_stop, 10).unwrap());
    }
    if args.cmd_node || args.cmd_both {
        nodes = Some(args.arg_node.iter().map(|s: &String| {
            Node::from_str_radix(s, 10).unwrap()
        }).collect());
    }
    if args.cmd_calc {
        delta = Some(Time::from_str_radix(&args.arg_delta, 10).unwrap());
    }
    // RENAME
    if args.cmd_rename {
        // let stream = linkstreams::stdin::rename();
        // for link in stream {
        //     println!("{}", link.to_string());
        // }
    }
    // INFO
    else if args.cmd_info {
        // COUNT
        if args.cmd_count {
            let (nb_nodes, nb_links) = algo::count_nodes_and_links(&mut stdinLinks);
            println!("number of nodes: {}\nnumber of links: {}", nb_nodes, nb_links);
        }
        // DEGREES
        else if args.cmd_degrees {
            let nbNodes = nbNodes.unwrap();
            let degrees = algo::count_degrees(&mut stdinLinks, nbNodes);
            for i in 0..degrees.len() {
                println!("{}: {}", i, degrees[i]);
            }
        }
        // REPART
        else if args.cmd_repart {
            let nbNodes = nbNodes.unwrap();
            let repart = algo::count_first_and_last_apparition(&mut stdinLinks, nbNodes);
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
            let start = start.unwrap();
            let stop = stop.unwrap();
            for l in stdinLinks {
                let cond = l.time >= start && l.time < stop;
                if cond { println!("{}", l.to_string()); }
            }
        }
        // NODE
        else if args.cmd_node {
            let nodes: Vec<Node> = nodes.clone().unwrap();
             for l in stdinLinks {
                let cond = nodes.contains(&l.node1) && nodes.contains(&l.node2);
                if cond { println!("{}", l.to_string()); }
            }
        }
        // BOTH
        else if args.cmd_both {
            let start = start.unwrap();
            let stop = stop.unwrap();
            let nodes: Vec<Node> = nodes.clone().unwrap();
            for l in stdinLinks {
                let cond = l.time >= start
                    && l.time < stop
                    && nodes.contains(&l.node1)
                    && nodes.contains(&l.node2);
                if cond { println!("{}", l.to_string()); }
            }
        }
    }
    // CALC
    else if args.cmd_calc {
        let delta = delta.unwrap();
        let nbNodes = nbNodes.unwrap();
        // CONNEXITY
        if args.cmd_connexity {
            // TIME
            if args.cmd_time {
                let start = start.unwrap();
                let stop = stop.unwrap();
                algo::is_delta_connected(&mut stdinLinks, delta, nbNodes,
                                         &|_| true,
                                         &move |time: Time| {time >= start && time < stop});
            }
            // NODE
            else if args.cmd_node {
                let nodes: Vec<Node> = nodes.unwrap();
                algo::is_delta_connected(&mut stdinLinks, delta, nbNodes,
                                         &move |node: Node| { nodes.contains(&node) },
                                         &|_| true);
            }
            // BOTH
            else if args.cmd_both {
                let start = start.unwrap();
                let stop = stop.unwrap();
                let nodes: Vec<Node> = nodes.unwrap();
                algo::is_delta_connected(&mut stdinLinks, delta, nbNodes,
                                         &move |node: Node| { nodes.contains(&node) },
                                         &move |time: Time| {time >= start && time < stop});
            }
            // N/A
            else {
                algo::is_delta_connected(&mut stdinLinks, delta, nbNodes, &|_| true, &|_| true);
            }
        }
        // COMPS
        else if args.cmd_comps {
            let (comps, restes)= algo::delta_components(&mut stdinLinks, nbNodes, delta);
            println!("Composantes :");
            for comp in comps { println!("{:?} ({})", comp, comp.len()); }
            println!("Restes :");
            for reste in restes { println!("{:?} ({})", reste, reste.len()); }
        }
        // EXIST
        else if args.cmd_exist {
            let nodes: Vec<usize> = (0..nbNodes).collect();
            let mut out: Vec<Vec<bool>> = algo::delta_existence(&mut stdinLinks, &nodes, delta).iter().map(
                |&(_, ref exist): &(Time, Vec<bool>)| {
                    exist.clone()
                }).collect();
            out.reverse();
                let mat = data::matrix::Matrix{
                    matrix: out.clone(),
                    width: out.len().clone(),
                    height: out[0].len().clone(),
                };
                let mat = mat.transpose();
                for vec in mat.matrix {
                    let line_vec: Vec<&str> = vec.iter().map(|b: &bool| if *b {"1"} else {"0"}).collect();
                    println!("{}", line_vec.connect(" "));
                }
        }
    }
}
