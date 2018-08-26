#![allow(dead_code, type_alias_bounds)]

//! Define Matrix calculations.

extern crate std;
extern crate num;

use vector::scal_vec_prod;

pub type Mat<T: num::traits::Num + std::fmt::Display + Copy> = Vec<Vec<T>>;

fn mk_mat<T>(vs: Vec<Vec<T>>) -> Option<Mat<T>> {
    let l0 = vs[0].len();
    let mut c = 0;
    for v in &vs {
        if v.len() != l0 {
            c += 1;
            break;
        }
    }
    match c {
        0 => return Some(vs),
        _ => return None
    }
}

pub fn print_mat(m: &Mat<f64>) {
    println!("[");
    for i in 0..m.len() {
        print!("  {}", m[i][0]);
        for j in 1..m[i].len() {
            print!(",{}", m[i][j]);
        }
        println!("");
    }
    println!("]");
}

pub fn mat_rows(m: &Mat<f64>) -> usize {
    let r = m.len();
    return r;
}

pub fn mat_cols(m: &Mat<f64>) -> usize {
    let c = m[0].len();
    return c;
}

pub fn mat_size(m: &Mat<f64>) -> Vec<usize> {
    return vec![mat_rows(&m), mat_cols(&m)];
}

pub fn scal_mat_prod(s: &f64, m: &Mat<f64>) -> Mat<f64> {
    return m.iter().map(|v| scal_vec_prod(&s, &v)).collect::<Mat<f64>>();
}

pub fn mat_vec_prod(m: &Mat<f64>, v: &Vec<f64>) -> Vec<f64> {
    let m_row   = mat_rows(&m);
    let m_col   = mat_cols(&m);
    let v_size  = v.len();
    let mut rlt = vec![];
    
    if m_row != v_size {
        panic!("Size mismatched.");
    } else {
        for i in 0..m_row {
            let mut r = 0.0;
            for j in 0..m_col {
                r += m[i][j] * v[j];
            }
            rlt.push(r);
        }
        return rlt;
    }
}

pub fn mat_add(m0: &Mat<f64>, m1: &Mat<f64>) -> Mat<f64> {
    let size0 = mat_size(m0);
    let size1 = mat_size(m1);
    let row0  = size0[0];
    let col0  = size0[1];
    let mut rlt   = vec![];
    
    if size0 != size1 {
        panic!("Size mismatched.");
    } else {
        for i in 0..row0 {
            let mut r = vec![];
            for j in 0..col0 {
                r.push(m0[i][j] + m1[i][j]);
            }
            rlt.push(r);
        }
    }
    return rlt;
}

pub fn mat_sub(m0: &Mat<f64>, m1: &Mat<f64>) -> Mat<f64> {
    let m1 = scal_mat_prod(&-1.0, m1);
    return mat_add(m0, &m1);
}

pub fn mat_prod(m0: &Mat<f64>, m1: &Mat<f64>) -> Mat<f64> {
    let row0 = mat_rows(m0);
    let col0 = mat_cols(m0);
    let row1 = mat_rows(m1);
    let col1 = mat_cols(m1);
    let mut rlt = vec![];

    if col0 != row1 {
        panic!("Size mismatched.");
    } else {
        for i in 0..row0 {
            let mut r = vec![];
            for j in 0..col1 {
                let mut s = 0.0;
                for k in 0..col0 {
                    s += m0[i][k] * m1[k][j];
                }
                r.push(s);
            }
            rlt.push(r);
        }
        return rlt;
    }
}