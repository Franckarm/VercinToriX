// ========================================================
//  VERCINTORIX V0.3 — cube.rs
//  Fondation géométrique — le cube chromatique RGB
//
//  Le cube est IMMUABLE :
//    8 sommets fixes
//    coordonnées 0.0 → 1.0 sur chaque axe
//    tout point de l'univers VTX y vit
// ========================================================

use std::fmt;

// ── POINT 3D dans le cube ───────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Point3D {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Point3D {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }

    pub fn origine() -> Self { Point3D::new(0.0, 0.0, 0.0) }
    pub fn centre()  -> Self { Point3D::new(0.5, 0.5, 0.5) }

    // Distance euclidienne entre deux points
    pub fn distance(&self, autre: &Point3D) -> f32 {
        let dr = self.r - autre.r;
        let dg = self.g - autre.g;
        let db = self.b - autre.b;
        (dr*dr + dg*dg + db*db).sqrt()
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.r, self.g, self.b)
    }
}

// ── SOMMETS NOMMÉS — les 8 coins du cube ────────────────
pub const NOIR:    Point3D = Point3D { r: 0.0, g: 0.0, b: 0.0 };
pub const ROUGE:   Point3D = Point3D { r: 1.0, g: 0.0, b: 0.0 };
pub const VERT:    Point3D = Point3D { r: 0.0, g: 1.0, b: 0.0 };
pub const BLEU:    Point3D = Point3D { r: 0.0, g: 0.0, b: 1.0 };
pub const JAUNE:   Point3D = Point3D { r: 1.0, g: 1.0, b: 0.0 };
pub const MAGENTA: Point3D = Point3D { r: 1.0, g: 0.0, b: 1.0 };
pub const CYAN:    Point3D = Point3D { r: 0.0, g: 1.0, b: 1.0 };
pub const BLANC:   Point3D = Point3D { r: 1.0, g: 1.0, b: 1.0 };

pub fn sommets() -> [(&'static str, Point3D); 8] {
    [
        ("noir",    NOIR),
        ("rouge",   ROUGE),
        ("vert",    VERT),
        ("bleu",    BLEU),
        ("jaune",   JAUNE),
        ("magenta", MAGENTA),
        ("cyan",    CYAN),
        ("blanc",   BLANC),
    ]
}

// ── ZONES du cube ───────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub enum Zone {
    Sommet(&'static str),  // collé à un coin
    Arete,                 // sur une arête
    Face,                  // sur une face
    Interieur,             // dans le volume
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Zone::Sommet(n)  => write!(f, "Sommet({})", n),
            Zone::Arete      => write!(f, "Arête"),
            Zone::Face       => write!(f, "Face"),
            Zone::Interieur  => write!(f, "Intérieur"),
        }
    }
}

// ── HEX ↔ POINT3D ───────────────────────────────────────
pub fn hex_vers_point(hex: &str) -> Option<Point3D> {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 { return None; }
    let r = u8::from_str_radix(&h[0..2], 16).ok()?;
    let g = u8::from_str_radix(&h[2..4], 16).ok()?;
    let b = u8::from_str_radix(&h[4..6], 16).ok()?;
    Some(Point3D::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    ))
}

pub fn point_vers_hex(p: &Point3D) -> String {
    let r = (p.r * 255.0).round() as u8;
    let g = (p.g * 255.0).round() as u8;
    let b = (p.b * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

// ── ANALYSE GÉOMÉTRIQUE ─────────────────────────────────

/// Trouve le sommet le plus proche d'un point
pub fn sommet_plus_proche(p: &Point3D) -> (&'static str, Point3D, f32) {
    let mut meilleur = ("noir", NOIR, p.distance(&NOIR));
    for (nom, s) in sommets().iter() {
        let d = p.distance(s);
        if d < meilleur.2 {
            meilleur = (nom, *s, d);
        }
    }
    meilleur
}

/// Détermine la zone d'un point
pub fn zone_du_point(p: &Point3D) -> Zone {
    let seuil_sommet = 0.15;
    let seuil_bord   = 0.05;

    // Sommet ?
    let (nom, _, d) = sommet_plus_proche(p);
    if d < seuil_sommet { return Zone::Sommet(nom); }

    // Combien de coordonnées sont sur un bord (0 ou 1) ?
    let bords = [p.r, p.g, p.b].iter()
        .filter(|&&v| v < seuil_bord || v > 1.0 - seuil_bord)
        .count();

    match bords {
        3 => Zone::Sommet("inconnu"),
        2 => Zone::Arete,
        1 => Zone::Face,
        _ => Zone::Interieur,
    }
}

/// Mapping linéaire d'une valeur sur un axe du cube
/// fps=45, plage=0..60, axe="R" → point (0.75, 0, 0)
pub fn valeur_vers_point(valeur: f32, min: f32, max: f32, axe: char) -> Point3D {
    let t = ((valeur - min) / (max - min)).clamp(0.0, 1.0);
    match axe {
        'R' | 'r' => Point3D::new(t, 0.0, 0.0),
        'G' | 'g' => Point3D::new(0.0, t, 0.0),
        'B' | 'b' => Point3D::new(0.0, 0.0, t),
        _         => Point3D::new(t, t, t),  // diagonale
    }
}

/// Parmi une liste de candidats hex, trouve le plus proche
pub fn plus_proche_parmi(
    cible:     &Point3D,
    candidats: &[(String, Point3D)],
) -> Option<(String, f32)> {
    candidats.iter()
        .map(|(nom, p)| (nom.clone(), cible.distance(p)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
}
