extern crate rustc_serialize;
extern crate docopt;
extern crate linkstreams;

use docopt::Docopt;

use linkstreams::algo;
use linkstreams::data::link::Time;
use linkstreams::data::link::Node;
use linkstreams::data::link::Link;

static USAGE: &'static str = "
Usage: linkstream filter node <node>...
       linkstream filter time <start> <stop>
       linkstream filter both <start> <stop> <node>...
       linkstream calc connexity <delta> <nbNodes>
       linkstream calc connexity <delta> <nbNodes> node <node>...
       linkstream calc connexity <delta> <nbNodes> time <start> <stop>
       linkstream calc connexity <delta> <nbNodes> both <start> <stop> node <node>...
";

#[derive(RustcDecodable, Debug)]
struct Args {
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

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    if args.cmd_filter {
        if args.cmd_time {
            if let Ok(start) = Time::from_str_radix(&args.arg_start, 10) {
                if let Ok(stop) = Time::from_str_radix(&args.arg_stop, 10) {
                    linkstreams::stdin::filter(|l: Link| {
                        let cond = l.time >= start && l.time < stop;
                        if cond { println!("{}", l.to_string()); }
                        cond
                    });
                }
            }
        }
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
        else if args.cmd_both {
            if let Ok(start) = Time::from_str_radix(&args.arg_start, 10) {
                if let Ok(stop) = Time::from_str_radix(&args.arg_stop, 10) {
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
        }
    }
    if args.cmd_calc {
        if args.cmd_connexity {
            if let Ok(delta) = Time::from_str_radix(&args.arg_delta, 10) {
                if let Ok(nbNodes) = usize::from_str_radix(&args.arg_nbNodes, 10) {
                    if args.cmd_time {
                        if let Ok(start) = Time::from_str_radix(&args.arg_start, 10) {
                            if let Ok(stop) = Time::from_str_radix(&args.arg_stop, 10) {
                                algo::is_delta_connected(delta, nbNodes,
                                                         &|_| true,
                                                         &move |time: Time| {time >= start && time < stop});
                            }
                        }
                    }
                    else if args.cmd_node {
                        let nodes: Vec<Node> = args.arg_node.iter().map(|s: &String| {
                            Node::from_str_radix(s, 10).unwrap()
                        }).collect();
                        algo::is_delta_connected(delta, nbNodes,
                                                 &move |node: Node| { nodes.contains(&node) },
                                                 &|_| true);
                    }
                    else if args.cmd_both {
                        if let Ok(start) = Time::from_str_radix(&args.arg_start, 10) {
                            if let Ok(stop) = Time::from_str_radix(&args.arg_stop, 10) {
                                let nodes: Vec<Node> = args.arg_node.iter().map(|s: &String| {
                                    Node::from_str_radix(s, 10).unwrap()
                                }).collect();
                                algo::is_delta_connected(delta, nbNodes,
                                                         &move |node: Node| { nodes.contains(&node) },
                                                         &move |time: Time| {time >= start && time < stop});
                            }
                        }
                    }
                    else {
                        let res = algo::is_delta_connected(delta, nbNodes, &|_| true, &|_| true);
                    }
                }
            }
        }
    }
}
