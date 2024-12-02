use nalgebra::Matrix3;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn multiply_matrix_with_diagonal_matrix(s: Vec<Vec<f64>>, d: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let d: Vec<f64> = d
        .iter()
        .enumerate()
        .map(|(i, row)| row[i])
        .collect();

    s.iter().map(|row| {row.iter().zip(d.iter()).map(|(n, d_n)| n * d_n).collect()}).collect()
}

fn find_d(a: &Vec<Vec<f64>>, s: &Vec<Vec<f64>>, d: &Vec<Vec<f64>>, i: usize) -> f64 {
    let res = a[i][i];

    let mut sum = 0.;
    for p in 0..i {
        sum += s[p][i] * s[p][i] * d[p][p];
    }

    (res - sum).signum()
}

fn find_s(a: &Vec<Vec<f64>>, s: &Vec<Vec<f64>>, d: &Vec<Vec<f64>>, i: usize, j: usize) -> f64 {
    if i == j {
        let res = a[i][i];

        let mut sum = 0.;
        for p in 0..i {
            sum += s[p][i] * s[p][i] * d[p][p];
        }

        (res - sum).abs().sqrt()
    } else {
        let res = a[i][j];

        let mut sum = 0.;
        for p in 0..i {
            sum += s[p][i] * d[p][p] * s[p][j];
        }

        (res - sum) / (d[i][i] * s[i][i])
    }
}

fn find_matrix_s_and_d(a: &Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let size = a.len();
    let mut s = vec![vec![0.; size]; size];
    let mut d = vec![vec![0.; size]; size];

    for i in 0..size {
        d[i][i] = find_d(&a, &s, &d, i);

        for j in i..size {
            s[i][j] = find_s(&a, &s, &d, i, j);
        }
    }

    println!("Matrix S:");
    for row in &s {
        for n in row {
            print!("{:10.5}", n);
        }
        println!();
    }

    println!("Matrix D:");
    for row in &d {
        for n in row {
            print!("{:10}", n);
        }
        println!();
    }

    (s, d)
}

fn find_det(s: &Vec<Vec<f64>>, d: &Vec<Vec<f64>>) -> f64 {
    let mut result = 1.;

    for i in 0..d.len() {
        result *= d[i][i] * s[i][i] * s[i][i];
    }

    result
}

fn find_condition_number(a: &Vec<Vec<f64>>) -> f64 {
    let a = Matrix3::new(1., 2., 0., 2., 2., 3., 0., 3., 2.);
    let norm_a = a.norm();
    let a_inv = a.try_inverse().unwrap();
    let norm_a_inv = a_inv.norm();

    norm_a * norm_a_inv
}

pub fn square_root_method() {
    let a = vec![
        vec![1, 2, 0],
        vec![2, 2, 3],
        vec![0, 3, 2],
    ];
    let a = a.iter().map(|inner| inner.iter().map(|&x| x as f64).collect()).collect();
    let b = vec![8., 22., 17.];

    let (s, d) = find_matrix_s_and_d(&a);

    let s_t = transpose(s.clone());
    println!("Matrix S transposed:");
    for row in &s_t {
        for n in row {
            print!("{:10.5}", n);
        }
        println!();
    }

    let s_t_dot_d = multiply_matrix_with_diagonal_matrix(s_t, &d);
    println!("Matrix S_T dot D:");
    for row in &s_t_dot_d {
        for n in row {
            print!("{:10.5}", n);
        }
        println!();
    }

    solve_equation(&s, &s_t_dot_d, &b);

    println!("Det A: {}", find_det(&s, &d));
    println!("Condition number: {}", find_condition_number(&a));
}

fn solve_equation(s: &Vec<Vec<f64>>, s_t: &Vec<Vec<f64>>, b: &Vec<f64>) {
    let mut result_y = vec![0.; s.len()];

    result_y[0] = b[0] / s_t[0][0];
    result_y[1] = (b[1] - s_t[1][0] * result_y[0]) / s_t[1][1];
    result_y[2] = (b[2] - s_t[2][0] * result_y[0] - s_t[2][1] * result_y[1]) / s_t[2][2];

    for i in 0..result_y.len() {
        let mut sum = 0.;
        for j in 0..i {
            sum += s_t[i][j] * result_y[j];
        }
        result_y[i] = (b[i] - sum) / s_t[i][i];
    }

    let mut result_x = vec![0.; s.len()];

    result_x[2] = result_y[2] / s[2][2];
    result_x[1] = (result_y[1] - s[1][2] * result_x[2]) / s[1][1];
    result_x[0] = (result_y[0] - s[0][2] * result_x[2] - s[0][1] * result_x[1]) / s[0][0];

    for i in (0..result_x.len()).rev() {
        let mut sum = 0.;
        for j in (i+1..result_x.len()).rev() {
            sum += s[i][j] * result_x[j];
        }
        result_x[i] = (result_y[i] - sum) / s[i][i];
    }

    println!("y: {:?}", result_y);
    println!("Result: x: {:?}", result_x);
}