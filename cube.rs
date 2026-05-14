pub fn hex_vers_rgb(s: &str) -> Option<[f32; 3]> {
    let s = s.trim_start_matches('#');
    if s.len() != 6 { return None; }
    let r = u8::from_str_radix(&s[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&s[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&s[4..6], 16).ok()? as f32 / 255.0;
    Some([r, g, b])
}

pub fn distance_cube(a: [f32;3], b: [f32;3]) -> f32 {
    ((a[0]-b[0]).powi(2)
   + (a[1]-b[1]).powi(2)
   + (a[2]-b[2]).powi(2)).sqrt()
}

pub fn extraire_nom_hex(s: &str) -> (String, Option<String>) {
    let parts: Vec<&str> = s.split_whitespace().collect();
    let hex = parts.iter()
        .find(|p| p.starts_with('#') && p.len() == 7)
        .map(|s| s.to_string());
    let nom = parts.iter()
        .filter(|p| !p.starts_with('#'))
        .cloned().collect::<Vec<_>>().join(" ");
    (nom, hex)
}

pub const NOIR:    [f32;3] = [0.0, 0.0, 0.0];
pub const BLANC:   [f32;3] = [1.0, 1.0, 1.0];
pub const ROUGE:   [f32;3] = [1.0, 0.0, 0.0];
pub const VERT:    [f32;3] = [0.0, 1.0, 0.0];
pub const BLEU:    [f32;3] = [0.0, 0.0, 1.0];
pub const JAUNE:   [f32;3] = [1.0, 1.0, 0.0];
pub const CYAN:    [f32;3] = [0.0, 1.0, 1.0];
pub const MAGENTA: [f32;3] = [1.0, 0.0, 1.0];