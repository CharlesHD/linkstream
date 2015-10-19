use data::link::*;
use data::matrix::*;
use data::filtre::NodeFilter;
use data::filtre::TimeFilter;
use data::filtre;
use data::iterators::link_iterator::LinkIterator;
use std::cmp::min;
use std::collections::HashMap;


// ////////////////////////////
//          CONNECTIVITY
// ////////////////////////////

/// Return a vector where each element (t, bool) tells us if the linkstream
/// reduce to [t, t + delta] is connected. The linkstream is delta-connected if
/// each element of the vector is (t, true) for t <= tmax - delta.
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

// Update distance matrix with a new link.
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

/// the delta-reachability-graph is the delta-reachability relation graph :
/// if u can delta-reach v then the index (u,v) equals 1, else it equals 0.
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
    let mut tmax: Time = max_val;
    for link in links {
        if filtre::combine(link, nfilter, tfilter){
            maj_distance(link, &mut dist, &mut p_dist, &mut curr);
            if first { pcurr = curr; first = false; tmax = link.time}
            if curr != pcurr {
                pcurr = curr;
                if curr < tmax - delta{
                    maj_reach_graph(&mut reach, &dist, link.time, delta);
                }
            }
        }
    }
    reach
}

// Update the delta-reachability graph using the distance matrix
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

///
pub fn delta_components_lower(links: &mut LinkIterator, size: usize,
                        delta: Time, filter: &Vec<Node>, tfilter: &TimeFilter) -> (Vec<Vec<Node>>, Vec<Vec<Node>>) {
    let order: Vec<Node> = (0..size).collect();
    let mut components: Vec<Vec<Node>> = Vec::new();
    let reste: Vec<Vec<Node>> = Vec::new();
    let mut stack: Vec<Vec<Node>> = Vec::new();
    let reach_graph: Matrix<Time> = delta_reachability_graph(links, delta, size, &|_| true, tfilter);
    stack.push(filter.clone());
    while let Some(filter) = stack.pop() {
        let filter_clone = filter.clone();
        if reach_graph.is_subset_clique(&move |node| filter_clone.contains(&node)) {
            if filter.len() > 1 {components.push(filter);};
        }
        else {
            let cuts: Vec<Vec<Node>> = connected_component(&reach_graph, &order, &filtre::node_filter(&filter, size));
            for comp in cuts {
                if comp.len() == filter.len() {
                    let mut comp_clone = comp.clone();
                    let comp_filtre = comp.clone();
                    let mnode: Node = reach_graph.get_max_deg_node(&move |node| comp_filtre.contains(&node)) as Node;
                    comp_clone.sort();
                    if let Ok(n)= comp_clone.binary_search(&mnode) {
                        comp_clone.remove(n);
                        stack.push(comp_clone);
                        stack.push(vec![mnode]);
                    }
                }
                else { stack.push(comp); }
            }
        }
    }
    (components, reste)
}

///
pub fn delta_components_upper(links: &mut LinkIterator, size: usize,
                        delta: Time, filter: &Vec<Node>, tfilter: &TimeFilter) -> (Vec<Vec<Node>>, Vec<Vec<Node>>) {
    let order: Vec<Node> = (0..size).collect();
    let mut components: Vec<Vec<Node>> = Vec::new();
    let mut reste: Vec<Vec<Node>> = Vec::new();
    let mut stack: Vec<Vec<Node>> = Vec::new();
    let reach_graph: Matrix<Time> = delta_reachability_graph(links, delta, size, &|_| true, tfilter);
    stack.push(filter.clone());
    while let Some(filter) = stack.pop() {
        let filter_clone = filter.clone();
        if reach_graph.is_subset_clique(&move |node| filter_clone.contains(&node)) {
            if filter.len() > 1 { components.push(filter); }
        }
        else {
            let cuts: Vec<Vec<Node>> = connected_component(&reach_graph, &order, &filtre::node_filter(&filter, size));
            for comp in cuts {
                if comp.len() == filter.len() {
                    reste.push(comp);
                    }
                else { stack.push(comp); }
            }
        }
    }
    (components, reste)
}


pub fn delta_partition(links: &mut LinkIterator,nodes: &Vec<Node>, delta: Time, upper: bool) -> Vec<(Time, Time, (Vec<Vec<Node>>, Vec<Vec<Node>>))> {
    let links: Vec<Link> = links.collect();
    let mut iter = links.clone().into_iter();
    let mut res: Vec<(Time, Time, (Vec<Vec<Node>>, Vec<Vec<Node>>))> = Vec::new();
    let intervals = existence_intervals(&mut iter, nodes, delta);
    for interv in intervals {
        let (stop, start, vec) = interv;
        let comps: (Vec<Vec<Node>>, Vec<Vec<Node>>);
        comps = if upper {
            delta_components_upper(&mut links.clone().into_iter(), nodes.len(), delta, &vec, &move |time: Time| {time >=start && time <= stop + delta + 1})
                } else {
            delta_components_lower(&mut links.clone().into_iter(), nodes.len(), delta, &vec, &move |time: Time| {time >=start && time <= stop + delta + 1})
                };
        res.push((start.clone(), stop.clone(), comps.clone()));
    }
    res
}

// ////////////////////////////
//        EXISTENCE
// ////////////////////////////
pub fn delta_existence(links: &mut LinkIterator,
                       nodes: &Vec<Node>, delta: Time) -> Vec<(Time, Vec<bool>)> {
    let mut results: Vec<(Time, Vec<bool>)> = Vec::new();
    let mut map: HashMap<Node, Node> = HashMap::new();
    for i in 0..nodes.len() { map.insert(nodes[i], i); }
    let map = map;
    let mut record: Vec<Time> = Vec::with_capacity(nodes.len());
    let mval = Time::max_value();
    let mut t_curr = mval;
    let mut t_max: Time = 0;
    for _ in 0..nodes.len() { record.push(mval); }
    let is_existing = |t_curr: Time, record: &Vec<Time>| {
        let res: Vec<bool> = record.iter().map(|time| time - t_curr < delta).collect();
        res
    };
    for link in links {
        let (n1, n2, t) = (link.node1, link.node2, link.time);
        if t_curr == mval {
            t_curr = t;
            t_max = t;
        }
        else if t_curr != t {
            if t < t_max - delta {
                results.push((t, is_existing(t_curr, &record)));
            }
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

pub fn new_delta_existence(links: &mut LinkIterator,
                           nodes: &Vec<Node>, delta: Time) -> Vec<(Time, Vec<bool>)> {
    let exist_mat = delta_existence(links, nodes, 1);
    let exist_mat_c = exist_mat.clone();
    let mut truevec: Vec<bool> = Vec::with_capacity(nodes.len());
    for _ in 0..nodes.len() { truevec.push(false); }
    let size = exist_mat.len();
    let mut res: Vec<(Time, Vec<bool>)> = Vec::with_capacity(size);
    for ex in exist_mat {
        let (t, _) = ex;
        res.push((t, fold_existence_at(t, exist_mat_c.clone(), delta, truevec.clone())));
    }
    res
}

/// # Examples
/// ```
/// # use linkstreams::algo::fold_existence_at;
/// let v = vec![(5, vec![false, false, false]),
///              (4, vec![true, false, true]),
///              (3, vec![true, false, true]),
///              (2, vec![true, false, true]),
///              (1, vec![true, false, true])];
/// let t = vec![false, false, false];
/// assert_eq!(fold_existence_at(3, v, 2, t), vec![true, false, true]);
/// ```
pub fn fold_existence_at(t: Time, v: Vec<(Time, Vec<bool>)>, delta: Time, init: Vec<bool>) -> Vec<bool> {
    let vectors = v.into_iter()
        .filter(|mat| {let (tv, _) = *mat; if delta > t {tv <= t + delta} else {tv >= t - delta && tv <= t + delta}})
        .map(|(_, b)| b).collect();
    or_v(&vectors)
}

/// The classic and boolean operator for boolean vector
///
/// # Examples
/// ```
/// # use linkstreams::algo::and;
/// let v1 = vec![false, true];
/// let v2 = vec![true, false];
/// assert_eq!(and(&v1, &v2), vec![false, false]);
/// ```
/// ```
/// # use linkstreams::algo::and;
/// let v1 = vec![true, true];
/// let v2 = vec![true, false];
/// assert_eq!(and(&v1, &v2), vec![true, false]);
/// ```
/// ```
/// # use linkstreams::algo::and;
/// let v1 = vec![false, true];
/// let v2 = vec![false, true];
/// assert_eq!(and(&v1, &v2), vec![false, true]);
/// ```
/// ```
/// # use linkstreams::algo::and;
/// let v1 = vec![false, true];
/// let v2 = vec![false, true, false];
/// assert_eq!(and(&v1, &v2), vec![false, true]);
/// ```
pub fn and(v1: &Vec<bool>, v2: &Vec<bool>) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    for (b1, b2) in v1.iter().zip(v2.iter()) {
        res.push(*b1 && *b2);
    }
    res
}
pub fn or(v1: &Vec<bool>, v2: &Vec<bool>) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    for (b1, b2) in v1.iter().zip(v2.iter()) {
        res.push(*b1 || *b2);
    }
    res
}
pub fn or_v(v : &Vec<Vec<bool>>) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    for i in 0..(v.get(0).unwrap().len() - 1) {
        let mut or = false;
        for b in v { or = or || *b.get(i).unwrap(); }
        res.push(or);
    }
    res
}

/// ```
/// # use linkstreams::algo::diff;
/// let v1 = vec![false, true];
/// let v2 = vec![false, true];
/// let v3 = vec![false, false];
/// assert!(!diff(&v1, &v2));
/// ```
/// ```
/// # use linkstreams::algo::diff;
/// let v1 = vec![false, true];
/// let v2 = vec![false, true];
/// let v3 = vec![false, false];
/// assert!(diff(&v1, &v3));
/// ```
pub fn diff(v1: &Vec<bool>, v2: &Vec<bool>) -> bool {
    if v1.len() != v2.len() { return true }
    for (b1, b2) in v1.iter().zip(v2.iter()) {
        if *b1 != *b2 { return true }
    }
    false
}

pub fn existence_intervals(links: &mut LinkIterator,
                           nodes: &Vec<Node>, delta: Time)
                           -> Vec<(Time, Time, Vec<Node>)>{
    let trace: Vec<(Time, Vec<bool>)> = new_delta_existence(links, nodes, delta);
    let mut intervals: Vec<(Time, Time, Vec<Node>)> = Vec::new();
    let mut curr_vec: Vec<bool>;
    let mut start: Time;
    let mut prev: Time;
    {
        let (borrow_start, ref borrow_vec) = trace[0];
        curr_vec = borrow_vec.clone();
        start = borrow_start.clone();
        prev = start.clone();
    }
    for (tcurr, mask) in trace {
        if diff(&curr_vec, &mask) {
            intervals.push((start, prev, boolvec_to_set(&curr_vec)));
            start = tcurr;
            curr_vec = mask.clone();
        }
        prev = tcurr;
    }
    intervals.push((start, prev, boolvec_to_set(&curr_vec)));
    intervals
}

pub fn largest_boxe(links: &mut LinkIterator, nodes: &Vec<Node>, delta: Time)
                         -> (Time, Time, Vec<Node>) {
    let trace: Vec<(Time, Vec<bool>)> = delta_existence(links, nodes, delta);
    let mut stack: Vec<(Time, Time, Vec<bool>)> = Vec::new();
    let mut max_score: Time = 0;
    let mut max: (Time, Time, Vec<Node>) = (0, 0, Vec::new());
    for (tcurr, vec) in trace {
        let size = stack.len();
        for i in 0..size {
            let (_, tstop, ref vec2) = stack[i].clone();
            stack[i] = (tcurr, tstop, and(&vec,&vec2));
        }
        stack.push((tcurr-1, tcurr, vec));
        for i in 0..size {
            let (tstart, tstop, ref vec) = stack[i];
            let v2 = boolvec_to_set(&vec);
            let score = (tstop - tstart)*(v2.len() as Time);
            if score > max_score {
                max_score = score;
                max = (tstart.clone(), tstop.clone(), v2.clone());
            }
        }
    }
    max
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

// ////////////////////////////
//      LARGEST RECTANGLE
// ////////////////////////////
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    fn from_coords(_x: usize, _y: usize) -> Point { Point { x: _x, y: _y }}
    fn new() -> Point { Point {x: 0, y: 0} }
}
impl ToString for Point {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}
pub fn area(ll: Point, ur: Point, tab: &Vec<(Time, Vec<bool>)>) -> usize {
    if ll.x > ur.x || ll.y > ur.y { 0 }
    else {
        let (t_ur, _) = tab[ur.x];
        let (t_ll, _) = tab[ll.x];
        ((t_ur - t_ll ) as usize) * (ur.y - ll.y + 1)
    }
}

fn update_cache(tab: &Vec<(Time, Vec<bool>)>, cache: &mut Vec<usize>, x: usize, height: usize) {
    let (_, ref col) = tab[x];
    for y in 0..height {
        if col[y] { cache[y] = cache[y] + 1; }
        else { cache[y] = 0; }
    }
}

fn grow_ones(tab: &Vec<(Time, Vec<bool>)>, cache: &Vec<usize>, width: usize, height: usize, ll: Point) -> Point {
    let mut ur = Point::from_coords(ll.x, ll.y);
    let mut y = ll.y;
    let mut x_max = width;
    let (_, ref col) = tab[ll.x];
    while y < height && col[y] == true {
        y = y + 1;
        let x = min(ll.x + cache[y] - 1, x_max);
        x_max = x;
        let candidat = Point::from_coords(x, y);
        if area(ll, candidat, tab) > area(ll, ur, tab){
            ur = candidat;
        }
    }
    ur
}

pub fn largest_rectangle(tab: &Vec<(Time, Vec<bool>)>, width: usize, height: usize) -> (Point, Point) {
    let mut cache: Vec<usize> = Vec::with_capacity(height);
    for _ in 0..height { cache.push(0); }
    let mut best_ll = Point::new();
    let mut best_ur = Point::new();
    for x1 in 0..width {
        update_cache(tab, &mut cache, x1, height);
        for y1 in 0..height {
            let ll = Point::from_coords(x1, y1);
            let ur = grow_ones(tab, &cache, width - 1, height - 1, ll);
            if area(ll, ur, tab) > area(best_ll, best_ur, tab) {
                best_ll = ll;
                best_ur = ur;
            }
        }
    }
    (best_ll, best_ur)
}

// pub fn largest_unordered_rectangle(tab: &Vec<(Time, Vec<bool>)>, width: usize, height: usize) -> (Vec<bool>, Time, Time) {
//     let mut best_set: Vec<bool> = Vec::with_capacity(height);
//     let mut best_start: Time = 0;
//     let mut best_stop: Time = 0;
//     let cache: Vec<(Vec<bool>, Time, Time)> = Vec::new();
// }
/// ```
/// # use linkstreams::algo::boolvec_to_set;
/// let v1 = vec![true, false, true, false];
/// let v2 = vec![0,2];
/// assert_eq!(boolvec_to_set(&v1), v2);
/// ```
pub fn boolvec_to_set(v1: &Vec<bool>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let mut count: usize = 0;
    for b in v1 {
        if *b { res.push(count); }
        count = count + 1;
    }
    res
}
