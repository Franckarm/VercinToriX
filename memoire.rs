// ========================================================
//  VERCINTORIX V0.3 — memoire.rs
//  Gestion de la memoire cubique et des noeuds
// ========================================================

use std::collections::HashMap;
use crate::adn::Codon;   // ← AJOUTER

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

    pub fn depuis_valeur(val: &str) -> Self {
        if val == "CERTAIN"  { return Etat::Certain;  }
        if val == "PROBABLE" { return Etat::Probable; }
        if val == "BRUME"    { return Etat::Brume;    }
        if val == "INCONNU"  { return Etat::Inconnu;  }
        if let Ok(f) = val.trim_matches('"').parse::<f64>() {
            if f > 0.80 { return Etat::Certain;  }
            if f > 0.40 { return Etat::Probable; }
            return Etat::Brume;
        }
        if val == "vrai" || val == "true"  { return Etat::Certain; }
        if val == "faux" || val == "false" { return Etat::Brume;   }
        if val == "rien" || val == "null"  { return Etat::Brume;   }
        if !val.is_empty() && val != "\"\"" { return Etat::Certain; }

        // AJOUTER : valeur hex chromatique = CERTAIN
        if val.starts_with('#') && val.len() == 7 {
            return Etat::Certain;
    }
    
        // Texte non vide = CERTAIN
        if !val.is_empty() && val != "\"\"" {
            return Etat::Certain;
    }
        Etat::Brume
    }
}

// ── POINT 4D ────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Point4D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub l: f32,
}

impl Point4D {
    pub fn new(r: f32, g: f32, b: f32, l: f32) -> Self {
        Point4D { r, g, b, l }
    }

    pub fn centre() -> Self {
        Point4D { r: 0.5, g: 0.5, b: 0.5, l: 0.5 }
    }

    pub fn distance_blanc(&self) -> f32 {
        ((self.r-1.0).powi(2)
        +(self.g-1.0).powi(2)
        +(self.b-1.0).powi(2)).sqrt()
    }

    pub fn distance_noir(&self) -> f32 {
        (self.r.powi(2)+self.g.powi(2)+self.b.powi(2)).sqrt()
    }

    pub fn etat_spectral(&self, seuil: f32) -> Etat {
        if self.distance_blanc() < seuil { Etat::Certain  }
        else if self.distance_noir() < seuil { Etat::Brume }
        else { Etat::Probable }
    }

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
    Liste(Vec<String>),   // ← String pour simplicité d'itération
    Point(Point4D),
    Adn(Vec<Codon>),
    Rien,
}

impl Valeur {
    pub fn depuis_str(s: &str) -> Self {
        let s = s.trim();
        if s == "rien" || s == "null"  { return Valeur::Rien; }
        if s == "vrai" || s == "true"  { return Valeur::Booleen(true);  }
        if s == "faux" || s == "false" { return Valeur::Booleen(false); }

        // Liste : ["a", "b", "c"]
        if s.starts_with('[') && s.ends_with(']') {
            let interieur = &s[1..s.len()-1];
            let elements: Vec<String> = interieur
                .split(',')
                .map(|e| e.trim().trim_matches('"').to_string())
                .filter(|e| !e.is_empty())
                .collect();
            return Valeur::Liste(elements);
        }

        if let Ok(i) = s.parse::<i64>() { return Valeur::Entier(i); }
        if let Ok(f) = s.parse::<f64>() { return Valeur::Decimal(f); }

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
            Valeur::Booleen(b) => if *b { "vrai".into() } else { "faux".into() },
            Valeur::Liste(v)   => format!("[{}]", v.join(", ")),
            Valeur::Point(p)   => format!(
                "Point4D({:.3},{:.3},{:.3},{:.3})", p.r, p.g, p.b, p.l
            ),
            Valeur::Adn(c)     => crate::adn::vers_adn_str(c),
            Valeur::Rien => "rien".to_string(),
        }
    }

    // ← NOUVEAU — extraire comme liste de strings
    pub fn comme_liste(&self) -> Option<Vec<String>> {
        match self {
            Valeur::Liste(v) => Some(v.clone()),
            Valeur::Texte(s) if s.starts_with('[') => {
                // Tentative de parsing tardif
                let interieur = &s[1..s.len().saturating_sub(1)];
                Some(
                    interieur
                        .split(',')
                        .map(|e| e.trim().trim_matches('"').to_string())
                        .filter(|e| !e.is_empty())
                        .collect()
                )
            }
            _ => None,
        }
    }
}

// ── NOEUD ───────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Noeud {
    pub nom:      String,
    pub etat:     Etat,
    pub valeur:   Option<Valeur>,
    pub position: Point4D,
    pub marque:   String,
    pub fige:     bool,
    pub valence:  u8,
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

    // ← NOUVEAU — accès direct à la liste si c'en est une
    pub fn comme_liste(&self) -> Option<Vec<String>> {
        self.valeur.as_ref()?.comme_liste()
    }
}

// ── MEMOIRE CUBIQUE ──────────────────────────────────────
pub struct Memoire {
    pub vive:    HashMap<String, Noeud>,
    pub cube:    Vec<Point4D>,
    pub archive: HashMap<String, String>,
    pub holo:    bool,
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

    // ── Opérations de base ──────────────────────────────

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
            println!("\x1b[32m> {}\x1b[0m", nom.trim_matches('"'));
        }
    }

    pub fn lire(&self, nom: &str) -> Option<&Noeud> {
        self.vive.get(nom)
    }

    pub fn effacer(&mut self, nom: &str) {
        self.vive.remove(nom);
    }

    // ← NOUVEAU — lire une liste depuis un noeud
    pub fn lire_liste(&self, nom: &str) -> Option<Vec<String>> {
        self.vive.get(nom)?.comme_liste()
    }

    // ← NOUVEAU — définir une variable de boucle temporaire
    pub fn definir_var_boucle(&mut self, nom: &str, valeur: &str) {
        let val = Valeur::Texte(valeur.to_string());
        let mut n = Noeud::new(nom);
        n.definir_valeur(val);
        // Forcer CERTAIN — c'est une valeur connue
        n.etat = Etat::Certain;
        self.vive.insert(nom.to_string(), n);
    }

    // ── Évaluation d'expressions ─────────────────────────

    pub fn eval(&self, expr: &str) -> String {
        let expr = expr.trim();

        // Littéral entre guillemets
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

        // Mots-clés
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

        // Opérations binaires — ordre : + - * /
        for op in &[" + ", " - ", " * ", " / "] {
            if let Some(pos) = expr.find(op) {
                let g = self.eval(&expr[..pos]);
                let d = self.eval(&expr[pos + op.len()..]);
                if let (Ok(gf), Ok(df)) = (g.parse::<f64>(), d.parse::<f64>()) {
                    let res = match *op {
                        " + " => gf + df,
                        " - " => gf - df,
                        " * " => gf * df,
                        " / " => {
                            if df == 0.0 { return "ERREUR:div0".to_string(); }
                            gf / df
                        }
                        _ => unreachable!(),
                    };
                    if res.fract() == 0.0 { return format!("{}", res as i64); }
                    return format!("{:.4}", res);
                }
                // Concaténation pour +
                if *op == " + " {
                    return format!("{}{}", g, d);
                }
                return expr.to_string();
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
                        let res = gf / df;
                        if res.fract() == 0.0 { return format!("{}", res as i64); }
                        return format!("{:.4}", res);
                    }
                    return "ERREUR_DIV0".to_string();
                }
            }
        }


        expr.to_string()
        
    }

    // ── Cube chromatique ─────────────────────────────────

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

    // ── Points cube pour pourChaque ──────────────────────
    pub fn points_cube_hex(&self) -> Vec<String> {
        self.cube.iter().map(|p| p.to_hex()).collect()
    }

    // ── Stockage ADN natif ───────────────────────────────
    pub fn definir_adn(&mut self, nom: &str, codons: Vec<Codon>) {
        let mut n = Noeud::new(nom);
        n.valeur = Some(Valeur::Adn(codons));
        n.etat   = Etat::Certain;
        self.vive.insert(nom.to_string(), n);
        println!("  \x1b[35m🧬\x1b[0m adn      : {} stocké en codons natifs", nom);
    }

    pub fn lire_adn(&self, nom: &str) -> Option<&Vec<Codon>> {
        match self.vive.get(nom)?.valeur.as_ref()? {
            Valeur::Adn(c) => Some(c),
            _ => None,
        }
    }

}
