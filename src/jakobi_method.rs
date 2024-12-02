fn iter_stop(arr1: &[f64; 4], arr2: &[f64; 4], precision: f64) -> bool {
    let diff: Vec<f64> = arr1
        .iter()
        .zip(arr2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();
    diff.iter().cloned().fold(f64::NEG_INFINITY, f64::max) >= precision
}

pub fn jakobi_method() {
    let mut prev_result = [1.; 4];
    let mut result = [0.; 4];

    let a = vec![
        vec![4., 0., 1., 0.],
        vec![0., 3., 0., 2.],
        vec![1., 0., 5., 1.],
        vec![0., 2., 1., 4.],
    ];
    let b = vec![12., 19., 27., 30.];

    let mut k = 1;
    while iter_stop(&prev_result, &result, 0.00001) {
        prev_result = result;

        for i in 0..result.len() {
            let mut sum = b[i];
            for j in 0..result.len() {
                if i != j {
                    sum -= a[i][j] * prev_result[j];
                }
            }

            result[i] = sum / a[i][i];
        }
        println!(
            "Iteration: {}: [x1: {}, x2: {}, x3: {}, x4: {}]",
            k, result[0], result[1], result[2], result[3]
        );
        k += 1;
    }

    println!("Result: [x1: {}, x2: {}, x3: {}, x4: {}]", result[0], result[1], result[2], result[3])

}