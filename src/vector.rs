//! Define Vector calculations.

pub fn print_vec(v: &Vec<f64>) {
    print!("[{}", v[0]);
    for i in 1..v.len() {
        print!(",{}", v[i]);
    }
    println!("]");
}

pub fn scal_vec_prod(s: &f64, v: &Vec<f64>) -> Vec<f64> {
    return v.iter().map(|&x| s * x).collect::<Vec<f64>>();
}

pub fn vec_add(v0: &Vec<f64>, v1: &Vec<f64>) -> Vec<f64> {
    let len0 = v0.len();
    let len1 = v1.len();
    let mut v = vec![];
    if len0 != len1 {
        panic!("Size mismatched.");
    }
    for i in 0..len0 {
        v.push(v0[i] + v1[i]);
    }
    return v;
}

pub fn vec_sub(v0: &Vec<f64>, v1: &Vec<f64>) -> Vec<f64> {
    let v1 = scal_vec_prod(&-1.0, &v1);
    return vec_add(v0, &v1);
}

pub fn inner_prod(v0: &Vec<f64>, v1: &Vec<f64>) -> f64 {
    let len0 = v0.len();
    let len1 = v1.len();
    if len0 != len1 {
        panic!("Size mismatched.");
    }
    let mut s = 0.0;
    for i in 0..len0 {
        s += v0[i] * v1[i];
    }
    return s;
}