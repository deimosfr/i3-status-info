pub fn define_threshold_color(warning: u8, critical: u8, value: f32) -> Option<String> {
    if value >= critical as f32 {
        Some("#FF0000".to_string())
    } else if value >= warning as f32 {
        Some("#FFFC00".to_string())
    } else {
        None
    }
}

pub fn set_text_threshold_color(
    warning: f64,
    critical: f64,
    value: f64,
    value_as_string: Option<String>,
) -> String {
    let final_value = value_as_string.unwrap_or(value.to_string());
    if value >= critical as f64 {
        format!("<span color='red'>{final_value}</span>")
    } else if value >= warning as f64 {
        format!("<span color='yellow'>{final_value}</span>")
    } else {
        final_value
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
