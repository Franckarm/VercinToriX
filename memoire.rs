// ========================================================
//  VERCINTORIX V0.3 — memoire.rs
//  Gestion de la memoire cubique et des noeuds
// ========================================================

use std::collections::HashMap;

// ── ETATS TRINAIRES/QUADRINAIRES ────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Etat {
    Certain,
    Probable,
    Brume,
    Inconnu,
}

impl Etat {
    pub fn nom(&self) -> &str {
        match self {
            Etat::Certain  => "CERTAIN",
            Etat::Probable => "PROBABLE",
            Etat::Brume    => "BRUME",
            Etat::Inconnu  => "INCONNU",
        }
    }

    // Evalue un etat depuis une valeur texte
    pub fn depuis_valeur(val: &str) -> Self {
        if val == "CERTAIN"  { return Etat::Certain;  }
        if val == "PROBABLE" { return Etat::Probable; }
        if val == "BRUME"    { return Etat::Brume;    }
        if val == "INCONNU"  { return Etat::Inconnu;  }
        // Valeur numerique
        if let Ok(f) = val.trim_matches('"').parse::<f64>() {
            if f > 0.80 { return Etat::Certain;  }
            if f > 0.40 { return Etat::Probable; }
            if f > 0.0  { return Etat::Brume;    }
            return Etat::Brume;
        }
        // Booleen
        if val == "vrai" || val == "true"  { return Etat::Certain; }
        if val == "faux" || val == "false" { return Etat::Brume;   }
        if val == "rien" || val == "null"  { return Etat::Brume;   }
        // Texte non vide = CERTAIN
        if !val.is_empty() && val != "\"\"" {
            return Etat::Certain;
        }
        Etat::Brume
    }
}

// ── POINT 4D — position dans le cube chromatique ────────
#[derive(Debug, Clone)]
pub struct Point4D {
    pub r: f32,  // rouge    — axe X
    pub g: f32,  // vert     — axe Y
    pub b: f32,  // bleu     — axe Z
    pub l: f32,  // lambda   — axe spectral IR/UV
}

impl Point4D {
    pub fn new(r: f32, g: f32, b: f32, l: f32) -> Self {
        Point4D { r, g, b, l }
    }

    pub fn centre() -> Self {
        Point4D { r: 0.5, g: 0.5, b: 0.5, l: 0.5 }
    }

    // Distance vers le blanc (1,1,1) — CERTAIN si proche
    pub fn distance_blanc(&self) -> f32 {
        ((self.r-1.0).powi(2)
        +(self.g-1.0).powi(2)
        +(self.b-1.0).powi(2)).sqrt()
    }

    // Distance vers le noir (0,0,0) — BRUME si proche
    pub fn distance_noir(&self) -> f32 {
        (self.r.powi(2)+self.g.powi(2)+self.b.powi(2)).sqrt()
    }

    // DUBI spectral — evalue la position dans le cube
    pub fn etat_spectral(&self, seuil: f32) -> Etat {
        if self.distance_blanc() < seuil { Etat::Certain  }
        else if self.distance_noir() < seuil { Etat::Brume }
        else { Etat::Probable }
    }

    // Couleur hex pour affichage
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8)
    }
}

// ── VALEUR ──────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum Valeur {
    Texte(String),
    Entier(i64),
    Decimal(f64),
    Booleen(bool),
    Liste(Vec<Valeur>),
    Point(Point4D),
    Rien,
}

impl Valeur {
    pub fn depuis_str(s: &str) -> Self {
        let s = s.trim();
        if s == "rien" || s == "null" { return Valeur::Rien; }
        if s == "vrai" || s == "true"  { return Valeur::Booleen(true);  }
        if s == "faux" || s == "false" { return Valeur::Booleen(false); }
        if let Ok(i) = s.parse::<i64>()  { return Valeur::Entier(i);  }
        if let Ok(f) = s.parse::<f64>()  { return Valeur::Decimal(f); }
        if s.starts_with('"') && s.ends_with('"') {
            return Valeur::Texte(s.trim_matches('"').to_string());
        }
        Valeur::Texte(s.to_string())
    }

    pub fn to_affichage(&self) -> String {
        match self {
            Valeur::Texte(s)   => s.clone(),
            Valeur::Entier(i)  => i.to_string(),
            Valeur::Decimal(f) => format!("{:.4}", f),
            Valeur::Booleen(b) => if *b { "vrai".to_string() } else { "faux".to_string() },
            Valeur::Liste(v)   => format!("[{}]", v.iter().map(|x| x.to_affichage()).collect::<Vec<_>>().join(", ")),
            Valeur::Point(p)   => format!("Point4D({:.3},{:.3},{:.3},{:.3})", p.r, p.g, p.b, p.l),
            Valeur::Rien       => "rien".to_string(),
        }
    }
}

// ── NOEUD ───────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Noeud {
    pub nom:      String,
    pub etat:     Etat,
    pub valeur:   Option<Valeur>,
    pub position: Point4D,    // position dans le cube chromatique
    pub marque:   String,     // #IR, #UV, #IR·UV, #-0
    pub fige:     bool,       // immuable si vrai
    pub valence:  u8,         // nombre de connexions possibles
}

impl Noeud {
    pub fn new(nom: &str) -> Self {
        Noeud {
            nom:      nom.to_string(),
            etat:     Etat::Brume,
            valeur:   None,
            position: Point4D::centre(),
            marque:   String::new(),
            fige:     false,
            valence:  0,
        }
    }

    pub fn definir_valeur(&mut self, val: Valeur) {
        if self.fige { return; }
        self.etat   = Etat::depuis_valeur(&val.to_affichage());
        self.valeur = Some(val);
    }

    pub fn affichage(&self) -> String {
        match &self.valeur {
            Some(v) => v.to_affichage(),
            None    => "rien".to_string(),
        }
    }
}

// ── MEMOIRE CUBIQUE ──────────────────────────────────────
pub struct Memoire {
    pub vive:    HashMap<String, Noeud>,  // acces rapide O(1)
    pub cube:    Vec<Point4D>,            // points 3D/4D
    pub archive: HashMap<String, String>, // persistant
    pub holo:    bool,                    // mode holographique
}

impl Memoire {
    pub fn new() -> Self {
        Memoire {
            vive:    HashMap::new(),
            cube:    Vec::new(),
            archive: HashMap::new(),
            holo:    false,
        }
    }

    // ── Operations de base ──────────────────────────────

    pub fn declarer(&mut self, nom: &str) {
        if !self.vive.contains_key(nom) {
            self.vive.insert(nom.to_string(), Noeud::new(nom));
        }
        println!("  \x1b[32m+\x1b[0m declare  : {}", nom);
    }

    pub fn definir(&mut self, nom: &str, val_str: &str) {
        let val = Valeur::depuis_str(val_str);
        if let Some(n) = self.vive.get_mut(nom) {
            if n.fige {
                println!("  \x1b[31m!\x1b[0m {} est fige — modification ignoree", nom);
                return;
            }
            n.definir_valeur(val);
            println!("  \x1b[33m=\x1b[0m definir  : {} = {}", nom, val_str);
        } else {
            // Creation automatique si non declare
            let mut n = Noeud::new(nom);
            n.definir_valeur(val);
            self.vive.insert(nom.to_string(), n);
            println!("  \x1b[33m=\x1b[0m definir* : {} = {}", nom, val_str);
        }
    }

    pub fn figer(&mut self, nom: &str, val_str: &str) {
        self.definir(nom, val_str);
        if let Some(n) = self.vive.get_mut(nom) {
            n.fige = true;
        }
        println!("  \x1b[36m#\x1b[0m figer    : {} (immuable)", nom);
    }

    pub fn afficher(&self, nom: &str) {
        if let Some(n) = self.vive.get(nom) {
            println!("\x1b[32m> {}\x1b[0m", n.affichage());
        } else {
            // Texte brut
            let propre = nom.trim_matches('"');
            println!("\x1b[32m> {}\x1b[0m", propre);
        }
    }

    pub fn lire(&self, nom: &str) -> Option<&Noeud> {
        self.vive.get(nom)
    }

    pub fn effacer(&mut self, nom: &str) {
        self.vive.remove(nom);
    }

    // ── Evaluation d'expressions simples ────────────────

    pub fn eval(&self, expr: &str) -> String {
        let expr = expr.trim();

        // Valeur litterale entre guillemets
        if expr.starts_with('"') && expr.ends_with('"') {
            return expr.trim_matches('"').to_string();
        }

        // Variable connue
        if let Some(n) = self.vive.get(expr) {
            return n.affichage();
        }

        // Nombre direct
        if expr.parse::<f64>().is_ok() {
            return expr.to_string();
        }

        // Etats trinaires
        match expr {
            "CERTAIN"  => return "CERTAIN".to_string(),
            "PROBABLE" => return "PROBABLE".to_string(),
            "BRUME"    => return "BRUME".to_string(),
            "INCONNU"  => return "INCONNU".to_string(),
            "vrai"     => return "vrai".to_string(),
            "faux"     => return "faux".to_string(),
            "rien"     => return "rien".to_string(),
            _          => {}
        }

        // Addition simple : a + b
        if expr.contains(" + ") {
            let parts: Vec<&str> = expr.splitn(2, " + ").collect();
            if parts.len() == 2 {
                let g = self.eval(parts[0]);
                let d = self.eval(parts[1]);
                // Numerique
                if let (Ok(gf), Ok(df)) = (g.parse::<f64>(), d.parse::<f64>()) {
                    let res = gf + df;
                    if res.fract() == 0.0 { return format!("{}", res as i64); }
                    return format!("{:.4}", res);
                }
                // Concatenation
                return format!("{}{}", g, d);
            }
        }

        // Soustraction : a - b
        if expr.contains(" - ") {
            let parts: Vec<&str> = expr.splitn(2, " - ").collect();
            if parts.len() == 2 {
                let g = self.eval(parts[0]);
                let d = self.eval(parts[1]);
                if let (Ok(gf), Ok(df)) = (g.parse::<f64>(), d.parse::<f64>()) {
                    let res = gf - df;
                    if res.fract() == 0.0 { return format!("{}", res as i64); }
                    return format!("{:.4}", res);
                }
            }
        }

        // Multiplication : a * b
        if expr.contains(" * ") {
            let parts: Vec<&str> = expr.splitn(2, " * ").collect();
            if parts.len() == 2 {
                let g = self.eval(parts[0]);
                let d = self.eval(parts[1]);
                if let (Ok(gf), Ok(df)) = (g.parse::<f64>(), d.parse::<f64>()) {
                    let res = gf * df;
                    if res.fract() == 0.0 { return format!("{}", res as i64); }
                    return format!("{:.4}", res);
                }
            }
        }

        // Division : a / b
        if expr.contains(" / ") {
            let parts: Vec<&str> = expr.splitn(2, " / ").collect();
            if parts.len() == 2 {
                let g = self.eval(parts[0]);
                let d = self.eval(parts[1]);
                if let (Ok(gf), Ok(df)) = (g.parse::<f64>(), d.parse::<f64>()) {
                    if df != 0.0 {
                        return format!("{:.4}", gf / df);
                    }
                }
            }
        }

        // Retourner tel quel
        expr.to_string()
    }

    // ── Cube chromatique ────────────────────────────────

    pub fn ajouter_point(&mut self, p: Point4D) {
        self.cube.push(p);
    }

    pub fn centroide(&self) -> Option<Point4D> {
        if self.cube.is_empty() { return None; }
        let n = self.cube.len() as f32;
        let r = self.cube.iter().map(|p| p.r).sum::<f32>() / n;
        let g = self.cube.iter().map(|p| p.g).sum::<f32>() / n;
        let b = self.cube.iter().map(|p| p.b).sum::<f32>() / n;
        let l = self.cube.iter().map(|p| p.l).sum::<f32>() / n;
        Some(Point4D::new(r, g, b, l))
    }
}