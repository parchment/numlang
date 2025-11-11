/// Parses a string containing a numeric value (integer or float) into f64.
/// Returns Err if the string is not a valid number.
pub fn from_string(s: &str) -> Result<f64, String> {
    let s_trimmed = s.trim();
    match s_trimmed.parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Not a valid number string: '{}'", s_trimmed)),
    }
}
