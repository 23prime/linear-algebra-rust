#![allow(dead_code, unused_variables, unused_imports)]

////////////////
// For tests. //
////////////////

extern crate linear_algebra;

use linear_algebra::{vector, matrix};

fn main() {
    let v0: Vec<f64> = vec![0.0, 1.0, 2.0];
    // let v1: Vec<f64> = vec![1.0, 3.0, 5.0];
    // let v03 = matrix::scal_vec_prod(&3.0, &v0);
    // let v2 = matrix::vec_add(&v0, &v1);
    // let v3 = matrix::vec_sub(&v0, &v1);
    // let ip = matrix::inner_prod(&v0, &v1);

    // let m0: Mat<f64> = vec![vec![1.0, 2.0, 4.0], vec![2.0, 3.0, 1.0], vec![5.0, 1.0, 2.0]];
    // let m1: Mat<f64> = vec![vec![3.0, 1.0, 1.0], vec![4.0, 1.0, 2.0], vec![1.0, 2.0, 3.0]];
    // let m0: Mat<f64> = vec![vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], vec![3.0, 4.0, 5.0]];
    // let m1: Mat<f64> = vec![vec![1.0, 2.0], vec![2.0, 3.0], vec![3.0, 4.0]];
    // let s: Vec<usize> = matrix::mat_size(&m0);

    // let m03: Mat<f64> = matrix::scal_mat_prod(&3.0, &m0);
    // let m0v1: Vec<f64> = matrix::mat_vec_prod(&m0, &v1);
    // let m0am1: Mat<f64> = matrix::mat_add(&m0, &m1);
    // let m0sm1: Mat<f64> = matrix::mat_sub(&m0, &m1);
    // let m0pm1: Mat<f64> = matrix::mat_prod(&m0, &m1);

    print!("v0 = ");
    linear_algebra::vector::print_vec(&v0);

    // print!("v1 = ");
    // matrix::print_vec(&v1);

    // print!("3 * v0 = ");
    // matrix::print_vec(&v03);

    // print!("v0 + v1 = ");
    // matrix::print_vec(&v2);

    // print!("v0 - v1 = ");
    // matrix::print_vec(&v3);

    // print!("v0 * v1 = ");
    // println!("{}", ip);

    // print!("m0 = ");
    // matrix::print_mat(&m0);

    // print!("m1 = ");
    // matrix::print_mat(&m1);

    // print!("Size of m0: ");
    // println!("{} >< {}", s[0], s[1]);

    // print!("3 * m0 = ");
    // matrix::print_mat(&m03);

    // print!("m0 >< v1 = ");
    // matrix::print_vec(&m0v1);

    // print!("m0 + m1 = ");
    // matrix::print_mat(&m0am1);

    // print!("m0 - m1 = ");
    // matrix::print_mat(&m0sm1);

    // print!("m0 >< m1 = ");
    // matrix::print_mat(&m0pm1);
}