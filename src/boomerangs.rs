use std::collections::HashMap;

/// Return the number of boomerangs in list of points, where points[i] = (xi, yi)
/// A boomerang is a tuple of points (i, j, k) such that the distance between i and j equals the
/// distance between i and k (the order of the tuple matters).
///
/// # Arguments
///
/// * `points` - Vector of tuples where each tuple contains point coordinates (x,y)
///
pub fn count_boomerangs(points: Vec<(i32, i32)>) -> i32 {
    let mut distance: i32;
    let mut result: i32 = 0;
    let mut distance_frequency: HashMap<i32, i32> = HashMap::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            distance = (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2);
            *distance_frequency.entry(distance).or_default() += 1;
        }

        for freq in distance_frequency.values() {
            result += freq * (freq - 1);
        }

        distance_frequency.clear();
    }
    result
}
