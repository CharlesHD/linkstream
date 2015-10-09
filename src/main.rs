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
       linkstream calc connexity <delta> <nbNodes> [node <node>... | time <start> <stop> | both <start> <stop> <node>...]
       linkstream calc comps [up] <delta> <nbNodes> [node <node>... | both <start> <stop> <node>...]
       linkstream calc exist [lr | cut] <delta> <nbNodes>
       linkstream calc part [up] <delta> <nbNodes>
       linkstream rename
       linkstream gen <nbNodes> <stop> <proba>
       linkstream info (count (node | links) | degrees <nbNodes> | repart <nbNodes>)
       linkstream filter (node <node>... | time <start> <stop> | both <start> <stop> <node>...)
";

#[allow(non_snake_case)]
#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_rename: bool,
    cmd_gen: bool,
    cmd_info: bool,
    cmd_count: bool,
    cmd_degrees: bool,
    cmd_repart: bool,
    cmd_filter: bool,
    cmd_calc: bool,
    cmd_connexity: bool,
    cmd_comps: bool,
    cmd_up: bool,
    cmd_exist: bool,
    cmd_part: bool,
    cmd_lr: bool,
    cmd_cut: bool,
    cmd_node: bool,
    cmd_links: bool,
    cmd_time: bool,
    cmd_both: bool,
    arg_node: Vec<String>,
    arg_start: String,
    arg_stop: String,
    arg_nbNodes: String,
    arg_delta: String,
    arg_proba: String,
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
    let mut proba: Option<u64> = None;
    let mut stdinLinks = stdin_link_iterator::StdinLinkIter::new();
    if args.cmd_calc || args.cmd_degrees || args.cmd_repart || args.cmd_gen {
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
    if args.cmd_gen {
        stop = Some(Time::from_str_radix(&args.arg_stop, 10).unwrap());
        proba = Some(u64::from_str_radix(&args.arg_proba, 10).unwrap());
    }
    if args.cmd_calc {
        delta = Some(Time::from_str_radix(&args.arg_delta, 10).unwrap());
    }
    // RENAME
    if args.cmd_rename {
        let stream = rename_link_iterator::RenameLinkIter::new(&mut stdinLinks);
        for link in stream {
            println!("{}", link.to_string());
        }
    }
    // GEN
    if args.cmd_gen {
        let nbNodes = nbNodes.unwrap();
        let stop = stop.unwrap();
        let proba = proba.unwrap() as f64;
        let stream = uniform_link_generator::UnifLinkGenerator::new(nbNodes, stop, proba);
        for link in stream {
            println!("{}", link.to_string());
        }

    }
    // Info
    else if args.cmd_info {
        // COUNT
        if args.cmd_count {
            let (nb_nodes, nb_links) = algo::count_nodes_and_links(&mut stdinLinks);
            if args.cmd_node {println!("{}", nb_nodes);} else {println!("{}", nb_links);}
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
                let cond = l.time >= start && l.time <= stop;
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
                    && l.time <= stop
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
            let mut filter: Vec<Node> = (0..nbNodes).collect();
            // NODE
            if args.cmd_node || args.cmd_both {
                filter = nodes.unwrap().clone();
            }
            // BOTH
            if args.cmd_both {
                let start = start.unwrap();
                let stop = stop.unwrap();
                let (comps, restes) = if args.cmd_up {
                    algo::delta_components_upper(&mut stdinLinks, nbNodes, delta, &filter, &move |time: Time| {time >=start && time <= stop})
                } else {
                    algo::delta_components_lower(&mut stdinLinks, nbNodes, delta, &filter, &move |time: Time| {time >=start && time <= stop})
                };
                let mut ncomp = 0;
                let mut maxcomp = 0;
                let mut nbnodes = 0;
                let mut all = comps.clone();
                for comp in comps {
                    ncomp = ncomp + 1;
                    nbnodes = nbnodes + comp.len();
                    maxcomp = if comp.len() > maxcomp {comp.len()} else {maxcomp};
                }
                for rest in restes {
                    nbnodes = nbnodes + rest.len();
                    maxcomp = if rest.len() > maxcomp {rest.len()} else {maxcomp};
                    all.push(rest);
                }
                println!("{} {} {:?}", ncomp, maxcomp, all);
            }
            // NOT BOTH
            else {
                let (comps, restes) = if args.cmd_up {
                    algo::delta_components_upper(&mut stdinLinks, nbNodes, delta, &filter, &|_| true)
                } else {
                    algo::delta_components_lower(&mut stdinLinks, nbNodes, delta, &filter, &|_| true)
                };
                let mut ncomp = 0;
                let mut maxcomp = 0;
                let mut all = comps.clone();
                for comp in comps {
                    ncomp = ncomp + 1;
                    maxcomp = if comp.len() > maxcomp {comp.len()} else {maxcomp};
                }
                for rest in restes {
                    /*ncomp = ncomp + 1;*/
                    maxcomp = if rest.len() > maxcomp {rest.len()} else {maxcomp};
                    all.push(rest);
                }
                println!("{} {} {:?}", ncomp, maxcomp, all);
                }
        }
        // EXIST
        else if args.cmd_exist {
            let nodes: Vec<usize> = (0..nbNodes).collect();
            if args.cmd_lr {
                let (start, stop, vec) = algo::largest_boxe(&mut stdinLinks, &nodes, delta);
                println!("{} {} {} {} {:?}", start, stop, (stop-start)*(vec.len() as Time), vec.len(), vec);
            }
            // CUT
            else if args.cmd_cut {
                let nodes: Vec<usize> = (0..nbNodes).collect();
                let out: Vec<(Time, Time, Vec<Node>)> =
                    algo::existence_intervals(&mut stdinLinks, &nodes, delta);
                for (start, stop, set) in out {
                    let set_str: Vec<String> = set.iter().map(|node| format!("{}", node)).collect();
                    println!("{} {} {}", stop, start + delta+1, set_str.join(" "));
                }
            }
            else {
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
                    println!("{}", line_vec.join(" "));
                }
            }
        }
        // PART
        else if args.cmd_part {
            let nodes: Vec<usize> = (0..nbNodes).collect();
            let parts = algo::delta_partition(&mut stdinLinks, &nodes, delta, args.cmd_up);
            for part in parts {
                let (start, stop, (comps, restes)) = part;
                let mut ncomp = 0;
                let mut maxcomp = 0;
                let mut all = comps.clone();
                for comp in comps {
                    ncomp = ncomp + 1;
                    maxcomp = if comp.len() > maxcomp {comp.len()} else {maxcomp};
                }
                for rest in restes {
                    /*ncomp = ncomp + 1;*/
                    maxcomp = if rest.len() > maxcomp {rest.len()} else {maxcomp};
                    all.push(rest);
                }
                println!("{} {} {} {} {:?}", stop, start,ncomp, maxcomp, all);
            }
        }
    }
}
