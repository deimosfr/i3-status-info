pub fn define_threshold_color(warning: u8, danger: u8, critical: u8, value: f32) -> Option<String> {
    if value >= critical as f32 {
        Some("#F5737E".to_string())
    } else if value >= danger as f32 {
        Some("#FFA500".to_string())
    } else if value >= warning as f32 {
        Some("#FFFC00".to_string())
    } else {
        None
    }
}

pub fn set_text_threshold_color(
    warning: f64,
    danger: f64,
    critical: f64,
    value: f64,
    value_as_string: Option<String>,
) -> String {
    let final_value = value_as_string.unwrap_or(value.to_string());
    if value >= critical {
        format!("<span color='#F5737E'>{final_value}</span>")
    } else if value >= danger {
        format!("<span color='orange'>{final_value}</span>")
    } else if value >= warning {
        format!("<span color='yellow'>{final_value}</span>")
    } else {
        final_value
    }
}

pub fn _define_reverse_threshold_color(warning: u8, critical: u8, value: f32) -> Option<String> {
    if value <= critical as f32 {
        Some("#f5737E".to_string())
    } else if value <= warning as f32 {
        Some("#FFFC00".to_string())
    } else {
        None
    }
}
