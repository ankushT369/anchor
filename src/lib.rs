pub fn round_to_three_decimal(decimal_number: f64) -> f64 {
    (decimal_number * 1000.0).round() / 1000.0
}

// Covariance function expects 2 array of type f64 and returns a f64
pub fn covariance(x: &[f64], y: &[f64]) -> f64 {
    let array_len = x.len() as f64;

    // x_mean and y_means are the variable which caculates the its repective means of their arrays
    let mut x_mean: f64 = 0.0; 
    let mut y_mean: f64 = 0.0;

    // accumulate the arrays
    for i in 0..x.len() {
        x_mean = x_mean + x[i];
        y_mean = y_mean + y[i];
    }
    
    // Find the means
    x_mean = x_mean / array_len;
    y_mean = y_mean / array_len;

    // Find the covariance
    let covariance: f64 = x.iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - x_mean) * (yi - y_mean))
        .sum::<f64>()
        / array_len;

    covariance
}

pub fn variance() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = [1.0, 3.0, 5.0];
        let y = [3.7, 6.0, 1.8];

        let mut cov = covariance(&x, &y);
        cov = round_to_three_decimal(cov);
        assert_eq!(cov, -1.267);
    }
}
