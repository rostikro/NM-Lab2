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

    println!("A: {a:?}");
    println!("D: {d:?}");
    println!("S: {s:?}");

    (s, d)
}

pub fn square_root_method() {
    let a = vec![
        vec![1, 2, 0],
        vec![2, 2, 3],
        vec![0, 3, 2],
    ];
    let a = a.iter().map(|inner| inner.iter().map(|&x| x as f64).collect()).collect();

    let b = vec![8., 22., 17.];

    // let a = vec![
    //     vec![1, 2, 3],
    //     vec![2, 5, 5],
    //     vec![3, 5, 6],
    // ];
    // let a = a.iter().map(|inner| inner.iter().map(|&x| x as f64).collect()).collect();
    //
    // let b = vec![1., 2., 3.];
    //
    // let a = vec![
    //     vec![1, -1, 1, -1],
    //     vec![-1, 5, -3, 3],
    //     vec![1, -3, -7, 1],
    //     vec![-1, 3, 1, 10],
    // ];
    // let a = a.iter().map(|inner| inner.iter().map(|&x| x as f64).collect()).collect();
    //
    // let b = vec![2., -4., -18., -5.];

    let (s, d) = find_matrix_s_and_d(&a);

    let mut s_t = transpose(s.clone());
    println!("Transponse S: {:?}", s_t);
    for i in 0..s_t.len() {
        s_t[i][i] *= d[i][i];
    }
    println!("Transponse S dot D: {:?}", s_t);

    solve_equation(&s, &s_t, &b);
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

    println!("Result: {:?}, y: {:?}", result_x, result_y);
}