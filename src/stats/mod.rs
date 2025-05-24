// https://users.rust-lang.org/t/observed-cdf-of-a-vector/77566/4
pub fn cdf(x: &[f64]) -> Vec<(f64, f64)> {
    let ln = x.len() as f64;
    let mut x_ord = x.to_vec();
    x_ord.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if let Some(mut previous) = x_ord.get(0).map(|&f| f) {
        let mut cdf = Vec::new();
        for (i, f) in x_ord.into_iter().enumerate() {
            if f != previous {
                cdf.push((previous, i as f64 / ln));
                previous = f;
            }
        }

        cdf.push((previous, 1.0));
        cdf
    } else {
        Vec::new()
    }
}

pub fn pmf_from_cdf(cdf: &[f64]) -> Vec<f64> {
    let mut pmf = Vec::new();
    let mut k = 0;
    while k < cdf.len() {
        if k == 0 {
            pmf.push(cdf[k]);
        }
        if k > 0 {
            pmf.push(cdf[k] - cdf[k - 1]);
        }
        k += 1;
    }
    pmf
}

// fn main() {
//     let dist = Exp::new(0.05_f64).unwrap();
//     let mut rng = rand::thread_rng();
//     let input = dist.sample_iter(&mut rng).take(1000).collect::<Vec<f64>>();
//     let output = cdf(&input).iter().map(|(_, y)| *y).collect::<Vec<f64>>();
//     let pmf_sum = pmf_from_cdf(&output).iter().sum::<f64>();
//     let thresh = 0.0001;
//     assert!((pmf_sum - 1.0) < thresh);
//     println!("Test passed.");
// }
