use data::link::*;
use data::matrix::*;
use data::filtre::NodeFiltre;
use data::filtre::TimeFiltre;
use data::filtre;
use stdin;
use std::cmp::min;
use std::boxed;

pub fn is_delta_connected(delta: u32,
                          size: usize,
                          nfiltre: NodeFiltre,
                          tfiltre: TimeFiltre) -> Vec<(Time, bool)> {
    let max_val: Time = Time::max_value();
    let mut resultat: Vec<(Time, bool)> = Vec::new();
    let mut dist: Matrix<Time> = Matrix::new(max_val, size);
    let mut p_dist: Matrix<Time> = Matrix::new(max_val, size);
    let mut curr: Time = max_val;
    let mut pcurr: Time = curr;
    stdin::filter_map(
        |link: Link| {
            maj_distance(link, &mut dist, &mut p_dist, &mut curr);
            if curr != pcurr {
                pcurr = curr;
                let res = dist.is_clique();
                resultat.push((curr, res));
            }
    }, &*filtre::combine(nfiltre, tfiltre));
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
    for i in 0..dist.size {
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
