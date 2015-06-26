use data::link::Time;
/// A simple parametrized square matrix implementation.
pub struct Matrix<V>
    where V: Copy {
        matrix: Vec<Vec<V>>,
        pub size: usize,
    }

impl<V> Matrix<V>
    where V: Copy {
        /// Create a new square Matrix filled with `elem` with size `size`
        pub fn new(elem: V, size: usize) -> Matrix<V> {
            let mut mat: Vec<Vec<V>> = Vec::with_capacity(size);
            for _ in 0..size {
                let mut line : Vec<V> = Vec::with_capacity(size+1);
                for _ in 0..size {
                    line.push(elem);
                }
                mat.push(line);
            }
            Matrix{
                matrix: mat,
                size: size,
            }
        }
        /// Modify the matrix by setting the (`x`,`y`) element with `elem`
        pub fn set(&mut self, x: usize, y: usize, elem: V) {
            assert!(y < self.size, "Vector out of range writting!");
            self.matrix[x].push(elem);
            self.matrix[x].swap_remove(y);
        }

        /// Give the (`x`,`y`) element of the matrix
        pub fn get(&self, x: usize, y: usize) -> V {
            self.matrix[x][y]
        }

        pub fn diag(&mut self, item: V) {
            for x in 0..self.size {
                self.set(x, x, item);
            }
        }

        pub fn copy(&mut self, other: &Self) {
            assert_eq!(self.size, other.size);
            for x in 0..self.size {
                for y in 00..self.size {
                    self.set(x, y, other.get(x, y));
                }
            }
        }
    }

impl Matrix<Time> {
    /// Tells if the matrix is a clique or not
    pub fn is_subset_clique(&self, nfiltre: &(Fn(usize) -> bool)) -> bool {
        for x in 0..self.size{
            if nfiltre(x) {
            for y in 0..self.size {
                if nfiltre(y) && self.get(x, y) == 0 {
                    return false;
                }
            }
            }
        }
        true
    }

    pub fn is_clique(&self) -> bool {
        for x in 0..self.size {
            for y in 0..self.size {
                if self.get(x, y) == 0 {
                    return false;
                }
            }
        }
        true
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
        let mut m: Matrix<u32> = Matrix::new(0, 10);
        assert_eq!(0, m.get(0, 0));
        assert_eq!(0, m.get(9, 9));
        m.set(0, 0, 1);
        assert_eq!(1, m.get(0, 0));
    }
}
