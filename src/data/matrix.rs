use data::link::Time;
use data::link::Node;
use data::filtre::*;
use std::cmp::min;
/// A simple parametrized matrix implementation.
#[derive(Debug)]
pub struct Matrix<V>
    where V: Copy {
        pub matrix: Vec<Vec<V>>,
        pub width: usize,
        pub height: usize
    }

impl<V> Matrix<V>
    where V: Copy {
        /// Create a new `width`x`height` Matrix filled with `elem`
        pub fn new(elem: V, width: usize, height: usize) -> Matrix<V> {
            let mut mat: Vec<Vec<V>> = Vec::with_capacity(width);
            for _ in 0..width {
                let mut line : Vec<V> = Vec::with_capacity(height);
                for _ in 0..height {
                    line.push(elem);
                }
                mat.push(line);
            }
            Matrix{
                matrix: mat,
                width: width,
                height: height
            }
        }
        /// Modify the matrix by setting the (`x`,`y`) element with `elem`
        pub fn set(&mut self, x: usize, y: usize, elem: V) {
            assert!(y < self.height && x < self.width, "Vector out of range writting!");
            self.matrix[x][y] = elem;
        }

        /// Give the (`x`,`y`) element of the matrix
        pub fn get(&self, x: usize, y: usize) -> V {
            assert!(y < self.height && x < self.width, "Vector out of range writting!");
            self.matrix[x][y]
        }

        pub fn diag(&mut self, item: V) {
            let size: usize = min(self.width, self.height);
            for x in 0..size {
                self.set(x, x, item);
            }
        }

        pub fn copy(&mut self, other: &Self) {
            assert_eq!(self.width, other.width);
            assert_eq!(self.height, other.height);
            for x in 0..self.width {
                for y in 00..self.height {
                    self.set(x, y, other.get(x, y));
                }
            }
        }

        pub fn is_square(&self) -> bool { self.width == self.height }

        pub fn transpose(&self) -> Matrix<V> {
            let mut transpose: Matrix<V> = Matrix::new(self.get(0, 0), self.height, self.width);
            for i in 0..self.width {
                for j in 0..self.height {
                    transpose.set(j, i, self.get(i, j));
                }
            }
            transpose
        }
    }

impl Matrix<Time> {

    /// Transform a Vec<Vec<usize>> into a matrix. Usefull for tests and example.
    ///
    /// # Examples
    /// ```
    /// # use linkstreams::data::matrix::*;
    /// use linkstreams::data::link::Time;
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 2],
    ///         vec![3, 4]
    ///             ]
    ///         );
    /// assert_eq!(1 as Time, m.get(0, 0));
    /// assert_eq![2 as Time, m.get(0, 1)];
    /// assert_eq![3 as Time, m.get(1, 0)];
    /// assert_eq![4 as Time, m.get(1, 1)];
    /// ```
    pub fn parse(table: Vec<Vec<usize>>) -> Matrix<Time> {
        let mut mat: Vec<Vec<Time>> = Vec::new();
        let width = table.len();
        let mut height = 0;
        for tab_line in table {
            let mut line: Vec<Time> = Vec::new();
            if tab_line.len() > height { height = tab_line.len(); }
            for elem in tab_line { line.push(elem as Time); }
            mat.push(line);
        }
        Matrix{
            matrix: mat,
            width: width,
            height: height,
        }
    }

    /// Tells if the matrix induced by the subset is a clique or not
    ///
    /// # Example
    /// ```
    /// # use linkstreams::data::matrix::*;
    /// use linkstreams::data::link::{Time, Node};
    /// let nodefilter1 = |node: Node| { node != 1 };
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 1, 1],
    ///         vec![1, 0, 1],
    ///         vec![1, 1, 1]
    ///         ]
    ///     );
    /// assert_eq!(true, m.is_subset_clique(&nodefilter1));
    /// let nodefilter2 = |node: Node| { node != 2};
    /// assert_eq!(false, m.is_subset_clique(&nodefilter2));
    /// ```
    pub fn is_subset_clique(&self, nfilter: &NodeFilter) -> bool {
        let mval = Time::max_value();
        for x in 0..self.width{
            if nfilter(x) {
                for y in 0..self.height {
                    if nfilter(y)
                        && (self.get(x, y) == 0 || self.get(x, y) == mval){
                            return false;
                    }
                }
            }
        }
        true
    }

    /// Tells if the matrix is a clique or not
    ///
    /// # Example
    /// ```
    /// # use linkstreams::data::matrix::*;
    /// use linkstreams::data::link::Time;
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 1, 1],
    ///         vec![1, 1, 1],
    ///         vec![1, 1, 1]
    ///         ]
    ///     );
    /// assert_eq!(true, m.is_clique());
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 1, 1],
    ///         vec![1, 0, 1],
    ///         vec![1, 1, 1]
    ///         ]
    ///     );
    /// assert_eq!(false, m.is_clique());
    /// ```
    pub fn is_clique(&self) -> bool {
        self.is_subset_clique(&|_| true)
    }

    /// Tells if the induced matrix is a delta-clique or not
    ///
    /// # Example
    /// ```
    /// # use linkstreams::data::matrix::*;
    /// use linkstreams::data::link::{Time, Node};
    /// let nodefilter1 = |node: Node| { node != 1 };
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 4, 2],
    ///         vec![1, 5, 3],
    ///         vec![3, 2, 4]
    ///         ]
    ///     );
    /// let start = 0 as Time;
    /// let delta = 4 as Time;
    /// assert_eq!(true, m.is_subset_delta_clique(start, delta, &nodefilter1));
    /// let nodefilter2 = |node: Node| { node != 2};
    /// assert_eq!(false, m.is_subset_delta_clique(start, delta, &nodefilter2));
    /// ```
    pub fn is_subset_delta_clique(&self, time: Time, delta: Time, nfilter: &NodeFilter) -> bool {
        for x in 0..self.width {
            if nfilter(x) {
                for y in 0..self.height {
                    if nfilter(y) && self.get(x, y) - time > delta {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn get_max_deg_node(&self, nfilter: &NodeFilter) -> Node {
        let mval = Time::max_value();
        let mut max_node = 0;
        let mut max_deg = Node::max_value();
        for x in 0..self.width {
            if nfilter(x) {
                let mut deg = 0;
                for y in 0..self.height {
                    if nfilter(y) && self.get(x, y) > 0 && self.get(x, y) < mval {
                        deg = deg + 1;
                    }
                    if nfilter(y) && self.get(y, x) > 0 && self.get(y, x) < mval {
                        deg = deg + 1
                    }
                }
                if deg <= max_deg {
                    max_deg = deg;
                    max_node = x;
                }
            }
        }
        max_node
    }
    /// Tells if the matrix is a `delta`-clique at time `time` or not
    ///
    /// A matrix is a `delta`-clique at time `time` if each value `v`
    /// in the matrix satisfy ` v - t <= delta `
    ///
    /// # Example
    /// ```
    /// # use linkstreams::data::matrix::*;
    /// use linkstreams::data::link::Time;
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 2, 3],
    ///         vec![0, 3, 4],
    ///         vec![1, 0, 2]
    ///         ]
    ///     );
    /// let start = 0 as Time;
    /// let delta = 4 as Time;
    /// assert_eq!(true, m.is_delta_clique(start, delta));
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 2, 3],
    ///         vec![0, 5, 4],
    ///         vec![1, 0, 2]
    ///         ]
    ///     );
    /// assert_eq!(false, m.is_delta_clique(start, delta));
    /// ```
    pub fn is_delta_clique(&self, time: Time, delta: Time) -> bool {
        self.is_subset_delta_clique(time, delta, &|_| true)
    }

    /// Return the predecessors of `node` in the graph represented by this matrix
    ///
    /// # Examples
    /// ```
    /// # use linkstreams::data::matrix::*;
    /// use linkstreams::data::link::Time;
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 1, 1],
    ///         vec![1, 0, 1],
    ///         vec![0, 1, 0]
    ///         ]
    ///     );
    /// assert_eq!(vec![0, 1], m.predecessors(0));
    /// assert_eq!(vec![0, 2], m.predecessors(1));
    /// assert_eq!(vec![0, 1], m.predecessors(2));
    /// ```
    pub fn predecessors(&self, node: Node) -> Vec<Node> {
        let max_value = Time::max_value();
        let mut succs: Vec<Node> = Vec::new();
        for i in 0..self.width {
            if self.get(i, node) != 0 && self.get(i, node) < max_value {
                succs.push(i);
            }
        }
        succs
    }

    /// return the successors of `node` in the graph represented by the matrix
    ///
    /// # Examples
    /// ```
    /// # use linkstreams::data::matrix::*;
    /// use linkstreams::data::link::Time;
    /// let m: Matrix<Time> = Matrix::parse(
    ///     vec![
    ///         vec![1, 1, 1],
    ///         vec![1, 0, 1],
    ///         vec![0, 1, 0]
    ///         ]
    ///     );
    /// assert_eq!(vec![0, 1, 2], m.successors(0));
    /// assert_eq!(vec![0, 2], m.successors(1));
    /// assert_eq!(vec![1], m.successors(2));
    /// ```
    pub fn successors(&self, node: Node) -> Vec<Node> {
        let max_value = Time::max_value();
        let mut preds: Vec<Node> = Vec::new();
        for j in 0..self.height {
            if self.get(node, j) != 0 && self.get(node, j) < max_value {
                preds.push(j);
            }
        }
        preds
    }
}

// /////////////
// / TEST ZONE /
// /////////////
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn matrix_set_get_test() {
        let mut m: Matrix<u32> = Matrix::new(0, 10, 10);
        assert_eq!(0, m.get(0, 0));
        assert_eq!(0, m.get(9, 9));
        m.set(0, 0, 1);
        assert_eq!(1, m.get(0, 0));
    }
}
