pub mod Matrix {
    use rand::Rng;
    use serde::{Deserialize, Serialize};

    // The key to this program is the Matrix

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Matrix {
        data: Vec<Vec<f64>>,
    }

    impl Matrix {
        pub fn new(data: Vec<f64>, rows: usize, cols: usize) -> Matrix {
            // Asset that the vector is the correct size
            assert!(data.len() == rows * cols);

            let mut result = vec![vec![0.0; cols]; rows];
            for i in 0..rows {
                for j in 0..cols {
                    result[i][j] = data[i * cols + j];
                }
            }

            Self { data: result }
        }

        pub fn get_result(&self) -> Vec<Vec<f64>> {
            //TODO: Convert the Matrix back to An Array
            self.data.clone()
        }

        pub fn map(&self, f: impl Fn(f64, usize, usize) -> f64) -> Matrix {
            let rows = self.data.len();
            let cols = self.data[0].len();

            let mut result = vec![0.0; self.data.len() * self.data[0].len()];
            for i in 0..rows {
                for j in 0..cols {
                    result[i * cols + j] = f(self.data[i][j], i, j);
                }
            }
            Matrix::new(result, rows, cols)
        }

        pub fn mut_map(&mut self, f: impl Fn(f64, usize, usize) -> f64) {
            let rows = self.data.len();
            let cols = self.data[0].len();
            for i in 0..rows {
                for j in 0..cols {
                    self.data[i][j] = f(self.data[i][j], i, j)
                }
            }
        }

        pub fn random(rows: usize, cols: usize) -> Matrix {
            let mut result = vec![0.0; rows * cols];
            let mut rng = rand::thread_rng();
            for i in 0..rows {
                for j in 0..cols {
                    result[i * cols + j] = rng.gen::<f64>();
                }
            }

            Matrix::new(result, rows, cols)
        }

        // Add a Matrix to a Matrix
        pub fn add(&self, other: &Matrix) -> Matrix {
            let rows = self.data.len();
            let cols = self.data[0].len();
            let other_rows = other.data.len();
            let other_cols = other.data[0].len();
            println!(
                "rows: {:?} cols: {:?} other_rows: {:?} other_cols: {:?}",
                rows, cols, other_rows, other_cols
            );
            assert!(
                rows == other_rows,
                "ROWS: Self: {:?} Other: {:?}",
                self,
                other
            );
            assert!(
                cols == other_cols,
                "COLS: Self: {:?} Other: {:?}",
                self,
                other
            );

            self.map(|x: f64, i: usize, j: usize| x + other.data[i][j])
        }

        pub fn mut_add_scalar(&mut self, scalar: f64) {
            self.data = self.add_scalar(scalar).data;
        }

        pub fn add_scalar(&self, scalar: f64) -> Matrix {
            let rows = self.data.len();
            let cols = self.data[0].len();
            self.map(|x: f64, _i: usize, _j: usize| x + scalar)
        }

        pub fn mut_add(&mut self, other: &Matrix) {
            let rows = self.data.len();
            let cols = self.data[0].len();
            let other_rows = other.data.len();
            let other_cols = other.data[0].len();
            println!(
                "rows: {:?} cols: {:?} other_rows: {:?} other_cols: {:?}",
                rows, cols, other_rows, other_cols
            );
            assert!(
                rows == other_rows,
                "ROWS: Self: {:?} Other: {:?}",
                self,
                other
            );
            assert!(
                cols == other_cols,
                "COLS: Self: {:?} Other: {:?}",
                self,
                other
            );

            let result = self.map(|x: f64, i: usize, j: usize| x + other.data[i][j]);

            self.data = result.data;
        }

        pub fn subtract(&self, other: &Matrix) -> Matrix {
            let rows = self.data.len();
            let cols = self.data[0].len();
            let other_rows = other.data.len();
            let other_cols = other.data[0].len();
            assert!(rows == other_rows);
            assert!(cols == other_cols);

            self.map(|x: f64, i: usize, j: usize| x - other.data[i][j])
        }

        pub fn multiply(&self, other: &Matrix) -> Matrix {
            let rows = self.data.len();
            let cols = self.data[0].len();
            let other_rows = other.data.len();
            let other_cols = other.data[0].len();
            // Columns of Self must equal rows of other

            println!(
                "rows: {:?} cols: {:?} other_rows: {:?} other_cols: {:?}",
                rows, cols, other_rows, other_cols
            );
            assert!(cols == other_rows, "Self: {:?} Other: {:?}", self, other);

            let mut result: Vec<Vec<f64>> = vec![vec![0.0; other_cols]; rows];

            // for i in 0..cols {

            // }


            for i in 0..rows {
                // println!("i: {:?}", i);
                for j in 0..other_cols {
                    // println!("j: {:?}", j);
                    for k in 0..cols {
                        // println!("k: {:?}", k);
                        result[i][j] += self.data[i][k] * other.data[k][j];
                    }
                }
            }

            Matrix { data: result }
        }

        pub fn mut_multiply(&mut self, other: &Matrix) {
            let result = self.multiply(other);

            self.data = result.data;
        }

        pub fn mut_multiply_simple(&mut self, other: &Matrix) {
            let rows = self.data.len();
            let cols = self.data[0].len();
            let other_rows = other.data.len();
            let other_cols = other.data[0].len();
            assert_eq!(cols, other_cols);
            assert_eq!(rows, other_rows);

            for i in 0..rows {
                for j in 0..cols {
                    self.data[i][j] *= other.data[i][j];
                }
            }

        }

        pub fn mut_multiply_scalar(&mut self, scalar: f64) {
            let rows = self.data.len();
            let cols = self.data[0].len();
            for i in 0..rows {
                for j in 0..cols {
                    self.data[i][j] *= scalar;
                }
            }
        }

        pub fn transpose(&self) -> Matrix {
            let rows = self.data.len();
            let cols = self.data[0].len();

            let result: Vec<Vec<f64>> = (0..cols)
                .map(|i| (0..rows).map(|j| self.data[j][i]).collect())
                .collect();

            Matrix::new(result.into_iter().flatten().collect(), cols, rows)
        }

        // Multiply a Matrix by a scalar
    }
}

mod tests {
    use super::Matrix::Matrix;

    #[test]
    fn test_matric_transpose_col_eq_row() {
        let m1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        assert_eq!(
            m1.transpose().get_result(),
            vec![vec![1.0, 3.0], vec![2.0, 4.0]]
        );
    }
    #[test]
    fn test_matric_transpose_obtuse() {
        let m1 = Matrix::new(vec![3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0], 4, 2);
        assert_eq!(
            m1.transpose().get_result(),
            vec![vec![3.0, 5.0, 7.0, 9.0], vec![4.0, 6.0, 8.0, 1.0]]
        );
    }
    #[test]
    fn test_matric_transpose_obtuse2() {
        let m1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
        assert_eq!(
            m1.transpose().get_result(),
            vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]
        );
    }

    #[test]
    fn test_add() {
        let mut matrix2 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let matrix3 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);

        let result = matrix2.add(&matrix3);
        assert_eq!(result.get_result(), vec![vec![2.0, 4.0], vec![6.0, 8.0]]);

        matrix2.mut_add(&matrix3);
        assert_eq!(matrix2.get_result(), vec![vec![2.0, 4.0], vec![6.0, 8.0]]);

        matrix2.mut_add_scalar(3.0);
        assert_eq!(matrix2.get_result(), vec![vec![5.0, 7.0], vec![9.0, 11.0]]);
    }

    #[test]
    fn test_subtract() {
        let matrix2 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let matrix3 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let result = matrix3.subtract(&matrix2);
        println!("{:?}", result);
        assert_eq!(result.get_result(), vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
    }

    #[test]
    fn test_multiply_square() {
        let matrix4 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0], 2, 2);
        let matrix5 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0], 2, 2);
        let result = matrix4.multiply(&matrix5);
        println!("{:?}", result);
        assert_eq!(result.get_result(), vec![vec![3.0, 3.0], vec![6.0, 6.0]]);
    }
    #[test]
    fn test_multiply_rectangle() {
        let matrix4 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0], 3, 2);
        let matrix5 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0], 2, 2);
        let result = matrix4.multiply(&matrix5);
        println!("{:?}", result);
        assert_eq!(
            result.get_result(),
            vec![vec![3.0, 3.0], vec![6.0, 6.0], vec![9.0, 9.0]]
        );
    }

    #[test]
    fn test_mut_multiply_square() {
        let mut matrix4 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0], 2, 2);
        let matrix5 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0], 2, 2);
        matrix4.mut_multiply(&matrix5);

        assert_eq!(matrix4.get_result(), vec![vec![3.0, 3.0], vec![6.0, 6.0]]);
    }
    #[test]
    fn test_multiply_2() {


        let m1 = Matrix::new(vec![-0.9459075367710688, -0.9253376531713337], 1, 2);
        let m2 = Matrix::new(vec![0.05116646865075796, 0.06908788079470228], 2, 1);

        let r = m1.multiply(&m2);

        assert_eq!(
            r.get_result(),
            vec![
                vec![-0.1123283658038632389]
            ]
        );
    }

    #[test]
    fn test_mut_multiply_rectangle() {
        let mut matrix4 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0], 3, 2);
        let matrix5 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0], 2, 2);
        matrix4.mut_multiply(&matrix5);

        assert_eq!(
            matrix4.get_result(),
            vec![vec![3.0, 3.0], vec![6.0, 6.0], vec![9.0, 9.0]]
        );
    }
    #[test]
    fn test_multiply_scalar() {
        let mut matrix4 = Matrix::new(vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0], 3, 2);
        matrix4.mut_multiply_scalar(5.0);

        assert_eq!(
            matrix4.get_result(),
            vec![vec![5.0, 5.0], vec![10.0, 10.0], vec![15.0, 15.0]]
        );

        matrix4.mut_add_scalar(3.0);

        assert_eq!(
            matrix4.get_result(),
            vec![vec![8.0, 8.0], vec![13.0, 13.0], vec![18.0, 18.0]]
        );
    }
}
