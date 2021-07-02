fn main() {
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let abs_difference = (desired - result).abs();
    println!("{}", abs_difference);
    println!("{}", f32::EPSILON);
    assert!(abs_difference <= f32::EPSILON);
}