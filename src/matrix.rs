#![allow(dead_code)]

pub type Mat<T> = Vec<Vec<T>>;

////////////
// Vector //
////////////
pub fn print_vec(v: &Vec<f64>) {
    print!("[{}", v[0]);
    for i in 1..v.len() {
        print!(",{}", v[i]);
    }
    println!("]");
}

pub fn scal_vec_prod (s: &f64, v: &Vec<f64>) -> Vec<f64> {
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

////////////
// Matrix //
////////////
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