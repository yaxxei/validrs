use std::borrow::Cow;

use thiserror::Error;

use crate::error::Result;

pub enum ColorFormat {
    Hex,
    Rgb,
    Rgba,
    Hsl,
    Hsla,
}

#[derive(Debug, Error)]
pub enum ColorError {
    #[error("Invalid color format")]
    InvalidFormat,

    #[error("Color values out of range")]
    OutOfRange,

    #[error("{0}")]
    Custom(String),
}

pub trait ValidateColor {
    fn validate_color(&self, format: Option<ColorFormat>, msg: Option<String>) -> Result<()> {
        let color = self.color().ok_or(
            msg.map(ColorError::Custom)
                .unwrap_or(ColorError::InvalidFormat),
        )?;

        match format {
            Some(format) => match format {
                ColorFormat::Hex => validate_hex(color),
                ColorFormat::Rgb => validate_rgb(color),
                ColorFormat::Rgba => validate_rgba(color),
                ColorFormat::Hsl => validate_hsl(color),
                ColorFormat::Hsla => validate_hsla(color),
            },
            None => try_detect_format(color),
        }
    }

    fn color(&self) -> Option<&str>;
}

fn try_detect_format(color: &str) -> Result<()> {
    if color.starts_with('#') {
        validate_hex(color)?;
    } else if color.starts_with("rgb(") {
        validate_rgb(color)?;
    } else if color.starts_with("rgba(") {
        validate_rgba(color)?;
    } else if color.starts_with("hsl(") {
        validate_hsl(color)?;
    } else if color.starts_with("hsla(") {
        validate_hsla(color)?;
    } else {
        return Err(ColorError::InvalidFormat.into());
    }
    Ok(())
}

fn validate_hex(hex: &str) -> Result<()> {
    if !hex.starts_with('#') {
        return Err(ColorError::InvalidFormat.into());
    }

    let hex = &hex[1..];

    if ![3, 4, 6, 8].contains(&hex.len()) {
        return Err(ColorError::InvalidFormat.into());
    }

    match hex.chars().all(|c| c.is_ascii_hexdigit()) {
        true => Ok(()),
        false => return Err(ColorError::InvalidFormat.into()),
    }
}

fn validate_rgb(rgb: &str) -> Result<()> {
    if !rgb.starts_with("rgb(") || !rgb.ends_with(')') {
        return Err(ColorError::InvalidFormat.into());
    }

    let inner = &rgb[4..rgb.len() - 1];
    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();

    if parts.len() != 3 {
        return Err(ColorError::InvalidFormat.into());
    }

    for part in parts {
        validate_color_component(part, 255.0)?;
    }

    Ok(())
}

fn validate_rgba(rgba: &str) -> Result<()> {
    if !rgba.starts_with("rgba(") || !rgba.ends_with(')') {
        return Err(ColorError::InvalidFormat.into());
    }

    let inner = &rgba[5..rgba.len() - 1];
    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();

    if parts.len() != 4 {
        return Err(ColorError::InvalidFormat.into());
    }

    for (i, part) in parts.iter().enumerate() {
        let max = if i == 3 { 1.0 } else { 255.0 };
        validate_color_component(part, max)?;
    }

    Ok(())
}

fn validate_color_component(s: &str, max: f64) -> Result<()> {
    if s.ends_with('%') {
        let percent = s[..s.len() - 1]
            .parse::<f64>()
            .map_err(|_| ColorError::InvalidFormat)?;
        if percent < 0.0 || percent > 100.0 {
            return Err(ColorError::OutOfRange.into());
        }
    } else {
        let val = s.parse::<f64>().map_err(|_| ColorError::InvalidFormat)?;
        if val < 0.0 || val > max {
            return Err(ColorError::OutOfRange.into());
        }
    }
    Ok(())
}

fn validate_hsl(hsl: &str) -> Result<()> {
    if !hsl.starts_with("hsl(") || !hsl.ends_with(')') {
        return Err(ColorError::InvalidFormat.into());
    }

    let inner = &hsl[4..hsl.len() - 1];
    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();

    if parts.len() != 3 {
        return Err(ColorError::InvalidFormat.into());
    }

    validate_hue(&parts[0])?;
    validate_percentage(&parts[1])?;
    validate_percentage(&parts[2])?;

    Ok(())
}

fn validate_hsla(s: &str) -> Result<()> {
    if !s.starts_with("hsla(") || !s.ends_with(')') {
        return Err(ColorError::InvalidFormat.into());
    }

    let inner = &s[5..s.len() - 1];
    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();

    if parts.len() != 4 {
        return Err(ColorError::InvalidFormat.into());
    }

    validate_hue(&parts[0])?;
    validate_percentage(&parts[1])?;
    validate_percentage(&parts[2])?;
    validate_alpha(&parts[3])?;

    Ok(())
}

fn validate_hue(s: &str) -> Result<()> {
    let val = s.parse::<f64>().map_err(|_| ColorError::InvalidFormat)?;
    if val < 0.0 || val >= 360.0 {
        return Err(ColorError::OutOfRange.into());
    }
    Ok(())
}

fn validate_percentage(s: &str) -> Result<()> {
    if !s.ends_with('%') {
        return Err(ColorError::InvalidFormat.into());
    }
    let percent = s[..s.len() - 1]
        .parse::<f64>()
        .map_err(|_| ColorError::InvalidFormat)?;
    if percent < 0.0 || percent > 100.0 {
        return Err(ColorError::OutOfRange.into());
    }
    Ok(())
}

fn validate_alpha(s: &str) -> Result<()> {
    let val = s.parse::<f64>().map_err(|_| ColorError::InvalidFormat)?;
    if val < 0.0 || val > 1.0 {
        return Err(ColorError::OutOfRange.into());
    }
    Ok(())
}

impl ValidateColor for String {
    fn color(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ValidateColor for str {
    fn color(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateColor for &str {
    fn color(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateColor for Cow<'_, str> {
    fn color(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl<T: ValidateColor> ValidateColor for Option<T> {
    fn color(&self) -> Option<&str> {
        self.as_ref().and_then(|s| s.color())
    }
}

impl<T: ValidateColor> ValidateColor for &T {
    fn color(&self) -> Option<&str> {
        (*self).color()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_validation() {
        assert!("#abc".validate_color(Some(ColorFormat::Hex), None).is_ok());
        assert!(
            "#a1b2c3"
                .validate_color(Some(ColorFormat::Hex), None)
                .is_ok()
        );
        assert!("#abcd".validate_color(Some(ColorFormat::Hex), None).is_ok());
        assert!(
            "#a1b2c3d4"
                .validate_color(Some(ColorFormat::Hex), None)
                .is_ok()
        );
    }

    #[test]
    fn test_rgb_validation() {
        assert!(
            "rgb(255, 100, 50)"
                .validate_color(Some(ColorFormat::Rgb), None)
                .is_ok()
        );
        assert!(
            "rgb(100%, 50%, 0%)"
                .validate_color(Some(ColorFormat::Rgb), None)
                .is_ok()
        );
    }

    #[test]
    fn test_hsl_validation() {
        assert!(
            "hsl(180, 50%, 50%)"
                .validate_color(Some(ColorFormat::Hsl), None)
                .is_ok()
        );
        assert!(
            "hsla(180, 50%, 50%, 0.5)"
                .validate_color(Some(ColorFormat::Hsla), None)
                .is_ok()
        );
    }

    #[test]
    fn test_auto_detection() {
        assert!("#abc".validate_color(None, None).is_ok());
        assert!("rgb(255, 255, 255)".validate_color(None, None).is_ok());
        assert!("invalid".validate_color(None, None).is_err());
    }
}
