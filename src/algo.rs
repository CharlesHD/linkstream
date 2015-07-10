use data::link::*;
use data::matrix::*;
use data::filtre::NodeFilter;
use data::filtre::TimeFilter;
use data::filtre;
use data::iterators::link_iterator::LinkIterator;
use std::cmp::min;
use std::collections::HashMap;



// ////////////////////////////
//          CONNEXITE
// ////////////////////////////

pub fn is_delta_connected(links: &mut LinkIterator,
                          delta: Time,
                          size: usize,
                          nfilter: &NodeFilter,
                          tfilter: &TimeFilter) -> Vec<(Time, bool)> {
    let max_val: Time = Time::max_value();
    let mut resultat: Vec<(Time, bool)> = Vec::new();
    let mut dist: Matrix<Time> = Matrix::new(max_val, size, size);
    let mut p_dist: Matrix<Time> = Matrix::new(max_val, size, size);
    let mut curr: Time = max_val;
    let mut pcurr: Time = curr;
    let mut first: bool = true;
    for link in links {
        if filtre::combine(link, nfilter, tfilter){
            maj_distance(link, &mut dist, &mut p_dist, &mut curr);
            if first { pcurr = curr; first = false; }
            if curr != pcurr {
                pcurr = curr;
                let res = dist.is_subset_delta_clique(pcurr, delta, nfilter);
                resultat.push((curr, res));
                println!("{} {}", curr, res);
            }
        }
    }
    resultat
}

pub fn delta_reachability_graph(links: &mut LinkIterator,
                                delta: Time,
                                size: usize,
                                nfilter: &NodeFilter,
                                tfilter: &TimeFilter) -> Matrix<Time> {
    let max_val: Time = Time::max_value();
    let mut dist: Matrix<Time> = Matrix::new(max_val, size, size);
    let mut p_dist: Matrix<Time> = Matrix::new(max_val, size, size);
    let mut reach: Matrix<Time> = Matrix::new(1, size, size);
    let mut curr: Time = max_val;
    let mut pcurr: Time = curr;
    let mut first: bool = true;
    for link in links {
        if filtre::combine(link, nfilter, tfilter){
            maj_distance(link, &mut dist, &mut p_dist, &mut curr);
            if first { pcurr = curr; first = false; }
            if curr != pcurr {
                pcurr = curr;
                maj_reach_graph(&mut reach, &dist, link.time, delta);
            }
        }
    }
    reach
}

fn maj_reach_graph(reach: &mut Matrix<Time>,
                   dist: &Matrix<Time>, time: Time, delta: Time) {
    for i in 0..dist.width {
        for j in 0..dist.height {
            if reach.get(i, j) == 1 {
                if dist.get(i, j) - time > delta {
                    reach.set(i, j, 0);
                }
            }
        }
    }
}

fn maj_distance(link: Link,
                dist: &mut Matrix<Time>,
                p_dist: &mut Matrix<Time>,
                curr: &mut Time) {
    let (u, v, t) = (link.node1, link.node2, link.time);
    if *curr == Time::max_value() {
        *curr = t;
        dist.diag(t);
    }
    if *curr != t {
        p_dist.copy(dist);
        dist.diag(t);
        *curr = t;
    }
    for i in 0..dist.width {
        if u == i {
            dist.set(v, i, t);
        }
        else if v == i {
            dist.set(u, i, t);
        }
        else {
            if p_dist.get(u, i) > t {
                let min_res = min(dist.get(v, i), p_dist.get(u, i));
                dist.set(v, i, min_res);
            }
            if p_dist.get(v, i) > t {
                let min_res = min(dist.get(u, i), p_dist.get(v, i));
                dist.set(u, i, min_res);
            }
        }
    }
}


// ////////////////////////////
//          PARTITION
// ////////////////////////////

/// Perform the deep-first search algorithm on the matrix induced by `mat` and the `nodefilter`. The algorithm use an `order` for selecting nodes.
///
/// In this implementation the traversal checks node successors in decreasing order (node 2 before node 1 for example)
/// # Examples
/// ```
/// # use linkstreams::data::matrix::*;
/// # use linkstreams::data::link::*;
/// # use linkstreams::algo::dfs;
/// let mat: Matrix<Time> = Matrix::parse(vec![
///     vec![1, 0, 1, 0],
///     vec![1, 1, 1, 0],
///     vec![0, 1, 0, 1],
///     vec![0, 1, 1, 0]
///         ]);
/// let order_1 = vec![0, 1, 2, 3];
/// let nodefilter_1 = vec![true, true, true, true];
/// assert_eq!(vec![0, 2, 3, 1], dfs(&mat, &order_1, &nodefilter_1));
///
/// let nodefilter_2 = vec![true, true, false, true];
/// assert_eq!(vec![0, 1, 3], dfs(&mat, &order_1, &nodefilter_2));
///
/// let order_2 = vec![2, 1, 3, 0];
/// assert_eq!(vec![1, 0, 3], dfs(&mat, &order_2, &nodefilter_2));
/// ```
pub fn dfs(mat: &Matrix<Time>, order: &Vec<Node>, nodefilter: &Vec<bool>) -> Vec<Node> {
    assert!(nodefilter.len() == mat.width);
    let mut result: Vec<Node> = Vec::with_capacity(mat.width);
    let mut marks: Vec<bool> = Vec::with_capacity(mat.width);
    for _ in 0..mat.width { marks.push(false); }
    let mut stack: Vec<Node> = Vec::new();
    for refnode in order {
        let &node = refnode;
        if nodefilter[node] && marks[node] == false {
            stack.push(node);
            while let Some(v) = stack.pop() {
                if nodefilter[v] && marks[v] == false {
                    marks[v] = true;
                    result.push(v);
                    for succ in mat.successors(v){
                        stack.push(succ);
                    }
                }
            }
        }
    }
    result
}


/// Returns the strongly connected components of the matrix induced by `mat` and `nodefilter`
///
/// # Examples
/// ```
/// # use linkstreams::data::matrix::*;
/// # use linkstreams::data::link::*;
/// # use linkstreams::algo::connected_component;
/// let mat: Matrix<Time> = Matrix::parse(vec![
///     vec![1, 0, 0, 0],
///     vec![0, 1, 0, 0],
///     vec![0, 0, 1, 0],
///     vec![0, 0, 0, 1]
///         ]);
/// let order = vec![0, 1, 2, 3];
/// let nodefilter = vec![true, true, true, true];
/// let result = vec![vec![0], vec![1], vec![2], vec![3]];
/// assert_eq!(result, connected_component(&mat, &order, &nodefilter));
///
/// let mat: Matrix<Time> = Matrix::parse(vec![
///     vec![1, 1, 0, 1],
///     vec![1, 1, 0, 0],
///     vec![0, 0, 1, 1],
///     vec![0, 0, 1, 1]
///         ]);
/// let result = vec![vec![0, 1], vec![3, 2]];
/// assert_eq!(result, connected_component(&mat, &order, &nodefilter));
///
/// let mat: Matrix<Time> = Matrix::parse(vec![
///     vec![1, 1, 0, 1],
///     vec![1, 1, 0, 0],
///     vec![0, 1, 1, 1],
///     vec![0, 0, 1, 1]
///         ]);
/// let result = vec![vec![0, 1, 2, 3]];
/// assert_eq!(result, connected_component(&mat, &order, &nodefilter));
/// ```
pub fn connected_component(mat: &Matrix<Time>, order: &Vec<Node>, nodefilter: &Vec<bool>) -> Vec<Vec<Node>> {
    let dfsorder = dfs(mat, order, nodefilter);
    let mut result: Vec<Vec<Node>> = Vec::new();
    let mut marks: Vec<bool> = Vec::with_capacity(mat.width);
    for _ in 0..mat.width { marks.push(false); }
    let mut stack: Vec<Node> = Vec::new();
    for node in dfsorder {
        if nodefilter[node] && marks[node] == false {
            let mut component: Vec<Node> = Vec::new();
            stack.push(node);
            while let Some(v) = stack.pop() {
                if nodefilter[v] && marks[v] == false {
                    marks[v] = true;
                    component.push(v);
                    for pred in mat.predecessors(v) {
                        stack.push(pred);
                    }
                }
            }
            result.push(component);
        }
    }
    result
}

pub fn delta_components(links: &mut LinkIterator, size: usize,
                        delta: Time) -> (Vec<Vec<Node>>, Vec<Vec<Node>>){
    let order: Vec<Node> = (0..size).collect();
    let mut components: Vec<Vec<Node>> = Vec::new();
    let mut reste: Vec<Vec<Node>> = Vec::new();
    let mut stack: Vec<Vec<Node>> = Vec::new();
    let reach_graph: Matrix<Time> = delta_reachability_graph(links, delta, size, &|_| true, &|_| true);
    stack.push((0..size).collect());
    while let Some(filter) = stack.pop() {
        let filter_clone = filter.clone();
        if reach_graph.is_subset_clique(&move |node| filter_clone.contains(&node)) {
            components.push(filter);
        }
        else {
            let cuts: Vec<Vec<Node>> = connected_component(&reach_graph, &order, &filtre::node_filter(&filter, size));
            for comp in cuts {
                if comp.len() == filter.len() { reste.push(comp); }
                else { stack.push(comp); }
            }
        }
    }
    (components, reste)
}

// ////////////////////////////
//        EXISTENCE
// ////////////////////////////
pub fn delta_existence(links: &mut LinkIterator,
                       nodes: &Vec<Node>, delta: Time) -> Vec<(Time, Vec<bool>)> {
    let mut results: Vec<(Time, Vec<bool>)> = Vec::new();
    let mut map: HashMap<Node, Node> = HashMap::new();
    for i in 0..nodes.len() { map.insert(nodes[i], i); }
    let map = map; // YOU SHALL NOT TOUCH THIS ANYMORE FOOL
    let mut record: Vec<Time> = Vec::with_capacity(nodes.len());
    let mval = Time::max_value();
    let mut t_curr = mval;
    for _ in 0..nodes.len() { record.push(mval); }
    let is_existing = |t_curr: Time, record: &Vec<Time>| {

        let res: Vec<bool> = record.iter().map(|time| time - t_curr < delta).collect();
        res
    };
    for link in links {
        let (n1, n2, t) = (link.node1, link.node2, link.time);
        if t_curr == mval { t_curr = t; }
        else if t_curr != t {
            results.push((t, is_existing(t_curr, &record)));
            t_curr = t;
        }
        if map.contains_key(&n1) {
            let &bij = map.get(&n1).unwrap();
            record[bij] = t; }
        if map.contains_key(&n2) {
            let &bij = map.get(&n2).unwrap();
            record[bij] = t; }
    }
    results
}

// ////////////////////////////
//         SMALL ALGOS
// ////////////////////////////
/// Count degrees of each node in the stdin linkstream.
pub fn count_degrees(links: &mut LinkIterator, size: usize) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::with_capacity(size);
    for _ in 0..size { result.push(0); }
    let mut mat: Matrix<bool> = Matrix::new(false, size, size);
    for link in links {
        if ! mat.get(link.node1, link.node2) {
            mat.set(link.node1, link.node2, true);
            mat.set(link.node2, link.node1, true);
            result[link.node1] = result[link.node1] + 1;
            result[link.node2] = result[link.node2] + 1;
        }
    }
    result
}

/// Return the list of first and last time apparition for each node in the stdin linkstream.
pub fn count_first_and_last_apparition(links: &mut LinkIterator, size: usize) -> Vec<(Time, Time)> {
    let mut result: Vec<(Time, Time)>= Vec::with_capacity(size);
    let mut seens: Vec<bool> = Vec::with_capacity(size);
    for _ in 0..size { result.push((0, 0)); seens.push(false); }
    for link in links {
        let n1 = link.node1;
        let n2 = link.node2;
        if ! seens[n1] {
            result[n1] = (link.time, link.time);
            seens[n1] = true;
        }
        if ! seens[n2] {
            result[n2] = (link.time, link.time);
            seens[n2] = true;
        }
        let (_, last) = result[n1];
        result[n1] = (link.time, last);
        let (_, last) = result[n2];
        result[n2] = (link.time, last);
    }
    result
}

/// Count the number of different nodes the stdin linkstream has and the number of links
pub fn count_nodes_and_links(links: &mut LinkIterator) -> (usize, usize) {
    let mut seens: Vec<Node> = Vec::new();
    let mut count = 0;
    for link in links {
        count = count + 1;
        if ! seens.contains(&link.node1) { seens.push(link.node1); }
        if ! seens.contains(&link.node2) { seens.push(link.node2); }
    }
    (seens.len(), count)
}
