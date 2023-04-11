pub fn define_threshold_color(warning: u8, critical: u8, value: f32) -> Option<String> {
    if value >= critical as f32 {
        Some("#FF0000".to_string())
    } else if value >= warning as f32 {
        Some("#FFFC00".to_string())
    } else {
        None
    }
}

pub fn _define_reverse_threshold_color(warning: u8, critical: u8, value: f32) -> Option<String> {
    if value <= critical as f32 {
        Some("#FF0000".to_string())
    } else if value <= warning as f32 {
        Some("#FFFC00".to_string())
    } else {
        None
    }
}
