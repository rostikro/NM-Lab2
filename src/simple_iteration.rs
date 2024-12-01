const KF: f64 = 1. / 6.;

fn iter_stop(arr1: &[f64; 4], arr2: &[f64; 4], precision: f64) -> bool {
    let diff: Vec<f64> = arr1
        .iter()
        .zip(arr2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();
    diff.iter().cloned().fold(f64::NEG_INFINITY, f64::max) >= precision
}

pub fn simple_iteration() {
    let mut prev_result = [1.; 4];
    let mut result = [0.; 4];

    let mut i = 1;
    while iter_stop(&prev_result, &result, 0.001) {
        prev_result = result;

        result[0] = prev_result[0]
            - KF * (7. * prev_result[0] + 2. * prev_result[2] + 3. * prev_result[3] - 32.);
        result[1] = prev_result[1]
            - KF * (3. * prev_result[2] + 2. * prev_result[3] + 6. * prev_result[1] - 47.);
        result[2] = prev_result[2]
            - KF * (2. * prev_result[0] + 5. * prev_result[2] + 1. * prev_result[3] - 23.);
        result[3] = prev_result[3]
            - KF * (1. * prev_result[2] + 4. * prev_result[3] + 2. * prev_result[1] - 29.);

        println!(
            "Iteration: {}: [x1: {}, x2: {}, x3: {}, x4: {}]",
            i, result[0], result[1], result[2], result[3]
        );
        i += 1;
    }
}
