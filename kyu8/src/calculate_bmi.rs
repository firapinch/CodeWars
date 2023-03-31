pub fn bmi(weight: u32, height: f32) -> String {
    let bm = weight as f32 / (height * height);
    match bm {
        bm if bm <= 18.5 => "Underweight".into(),
        bm if bm <= 25.0 => "Normal".into(),
        bm if bm <= 30.0 => "Overweight".into(),
        _ => "Obese".into(),
    }
}