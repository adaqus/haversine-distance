const RADIANS_PER_DEGREE: f64 = 0.017_453_292_f32 as f64;
const EARTH_RADIUS: f64 = 6372.8;

pub fn calculate(pair: &[f64; 4]) -> f64 {
    // Preserve the reference function's mapping: y is latitude, x is longitude.
    let d_lat = RADIANS_PER_DEGREE * (pair[3] - pair[1]);
    let d_lon = RADIANS_PER_DEGREE * (pair[2] - pair[0]);
    let lat1 = RADIANS_PER_DEGREE * pair[1];
    let lat2 = RADIANS_PER_DEGREE * pair[3];
    let sin_lat = (d_lat / 2.0).sin();
    let sin_lon = (d_lon / 2.0).sin();
    let a = sin_lat * sin_lat + lat1.cos() * lat2.cos() * (sin_lon * sin_lon);
    // Floating-point roundoff at antipodal points can otherwise produce NaN.
    let c = 2.0 * a.clamp(0.0, 1.0).sqrt().asin();
    EARTH_RADIUS * c
}
