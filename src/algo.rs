use data::link::*;
use data::matrix::*;
use data::filtre::NodeFilter;
use data::filtre::TimeFilter;
use data::filtre;
use stdin;
use std::cmp::min;

pub fn is_delta_connected(delta: Time,
                          size: usize,
                          nfilter: &NodeFilter,
                          tfilter: &TimeFilter) -> Vec<(Time, bool)> {
    let max_val: Time = Time::max_value();
    let mut resultat: Vec<(Time, bool)> = Vec::new();
    let mut dist: Matrix<Time> = Matrix::new(max_val, size, size);
    let mut p_dist: Matrix<Time> = Matrix::new(max_val, size, size);
    let mut curr: Time = max_val;
    let mut pcurr: Time = curr;
    stdin::map(
        |link: Link| {
            if filtre::combine(link, nfilter, tfilter){
                maj_distance(link, &mut dist, &mut p_dist, &mut curr);
                if curr != pcurr {
                    pcurr = curr;
                    let res = dist.is_subset_delta_clique(pcurr, delta, nfilter);
                    resultat.push((curr, res));
                    println!("{} {}", curr, res);
                }
            }
    });
    resultat
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
