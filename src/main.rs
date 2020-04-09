use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
// placeholder for now
pub struct Row {
    e: [f32; 3],
}

impl Index<usize, > for Row {
    type Output = f32;
    fn index<'a>(&'a self, i: usize) -> &'a f32 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f32 {
        &mut self.e[i]
    }
}


fn print_matrix<'a, T: std::fmt::Display + std::fmt::Debug>(m: &Matrix<'a, T>) {
    for row in 0..m.n_rows {
        for col in 0..m.n_cols {
            print!("{}\t", &m.matrix[row][col]);
        }
        println!("");
    }
}

#[derive(Debug)]
struct Matrix<'a, T: std::fmt::Display>{
    matrix: &'a [&'a [T]],
    n_rows: usize,
    n_cols: usize,
}


fn main() {

    let m: [&[f64]; 2] = [
        &[1.1,  2.2],
        &[1.3,  3.4]
    ];
    let matrix = Matrix {matrix: &m, n_rows: 2, n_cols: 2};

    let m2: [&[f64]; 3] = [
        &[1.1,  2.2,    1.0,    2.5],
        &[-1.3, 3.4,    -0.0,   1.2],
        &[1.0, -3.4,    -0.0,   0.0]
    ];
    let matrix2 = Matrix {matrix: &m2, n_rows: 3, n_cols: 4};

    print_matrix(&matrix);
    print_matrix(&matrix2);
}