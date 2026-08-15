use std::path::PathBuf;
use std::fs;
use serde_json::Value;
use cosmic_theme::palette::{ Srgb };
use std::collections::HashMap;

#[derive(Debug)]
pub struct Colors {
    pub special: ParsedSpecial,
    pub colors: HashMap<String, Srgb<f32>>,
}

#[derive(Debug)]
pub struct ParsedSpecial {
    pub background: Srgb<f32>,
    pub foreground: Srgb<f32>,
    pub cursor: Srgb<f32>,
}

impl Colors {
    pub fn load(colors_json_path: &PathBuf) -> Result<Colors, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(colors_json_path)?;
        let root: Value = serde_json::from_str(&content)?;

        let special = ParsedSpecial {
            background: hex_to_srgb(get_hex(&root, "background")?)?,
            foreground: hex_to_srgb(get_hex(&root, "foreground")?)?,
            cursor: hex_to_srgb(get_hex(&root, "cursor")?)?,
        };

        let mut parsed_colors = HashMap::new();
        for i in 0..16 {
            let key = format!("color{}", i);
            if let Ok(hex) = get_hex(&root, &key) {
                parsed_colors.insert(key, hex_to_srgb(hex)?);
            }
        }

        Ok(Colors {
            special,
            colors: parsed_colors,
        })
    }
}

// Accepts both the pywal nested layout
//   { "special": { "background": ... }, "colors": { "color0": ... } }
// and the flat wallust layout
//   { "background": ..., "color0": ..., "special": ... }
fn get_hex<'a>(root: &'a Value, key: &str) -> Result<&'a str, Box<dyn std::error::Error>> {
    if let Some(special) = root.get("special") {
        if let Some(v) = special.get(key).and_then(|v| v.as_str()) {
            return Ok(v);
        }
    }
    if let Some(colors) = root.get("colors") {
        if let Some(v) = colors.get(key).and_then(|v| v.as_str()) {
            return Ok(v);
        }
    }
    if let Some(v) = root.get(key).and_then(|v| v.as_str()) {
        return Ok(v);
    }
    Err(format!("Color '{}' not found in colors file", key).into())
}

fn hex_to_srgb(hex: &str) -> Result<Srgb<f32>, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err("Invalid hex color format".into());
    }

    let r = (u8::from_str_radix(&hex[0..2], 16)? as f32) / 255.0;
    let g = (u8::from_str_radix(&hex[2..4], 16)? as f32) / 255.0;
    let b = (u8::from_str_radix(&hex[4..6], 16)? as f32) / 255.0;

    Ok(Srgb::new(r, g, b))
}
