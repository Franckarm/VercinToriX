// ================================================================
//  VERCINTORIX V0.3 — parser.rs
//  Transforme une ligne de texte en Instruction structurée.
//
//  Principe : on ne fait PAS de récursion ici.
//  On retourne une enum Instruction que l'exécuteur traite.
//  Les blocs {} sont gérés par l'exécuteur, pas le parser.
// ================================================================

/// Une instruction VERCINTORIX, après découpe.
/// Chaque variante correspond à un mot-clé ou une primitive.
#[derive(Debug, Clone)]
pub enum Instruction {
    // ── Environnement ──────────────────────────────────────
    /// TARVOS AUTO precision(f32) contexte(calcul)
    Tarvos {
        mode:      String,
        precision: String,
        contexte:  String,
    },

    // ── Variables ──────────────────────────────────────────
    /// declarer nom
    /// declarer nom → valeur
    Declarer { nom: String, valeur: Option<String> },

    /// definir nom → valeur
    Definir { nom: String, valeur: String },

    /// figer nom → valeur
    Figer { nom: String, valeur: String },

    /// effacer nom
    Effacer { nom: String },

    // ── Entrée / Sortie ────────────────────────────────────
    /// afficher expression
    /// afficher expression → ecran
    Afficher { expression: String },

    /// lire → variable
    Lire { variable: String },

    // ── Structures de contrôle ─────────────────────────────
    /// si condition {
    Si { condition: String },

    /// sinon {
    Sinon,

    /// tantque condition {
    Tantque { condition: String },

    /// repeter N {
    Repeter { fois: String },

    /// pourChaque element dans collection {
    PourChaque { element: String, collection: String },

    /// sortir
    Sortir,

    // ── Fonctions ──────────────────────────────────────────
    /// fonction nom(param1, param2) {
    Fonction { nom: String, params: Vec<String> },

    /// appeler nom(arg1, arg2)
    /// appeler nom(arg1) → resultat
    Appeler { nom: String, args: Vec<String>, dest: Option<String> },

    /// retourner expression
    Retourner { valeur: String },

    // ── Primitives gauloises ───────────────────────────────
    /// NEMETON NomBloc {
    Nemeton { nom: String },

    /// AVON source → dest
    /// AVON RECEVOIR ext → var
    /// AVON FOURNIR val → ext
    Avon {
        mode:   AvonMode,
        source: String,
        dest:   String,
    },

    /// DUBI variable contexte(xxx) ?
    Dubi { variable: String, contexte: String },

    /// LUTOS N {
    Lutos { fois: String },

    /// RANN module
    Rann { module: String },

    /// DRUS données → disque("fichier")
    Drus { source: String, dest: String },

    /// PAR {
    /// PAR synchronise {
    Par { synchronise: bool },

    /// GRANN nom(type) → valeur
    Grann { nom: String, type_str: String, valeur: Option<String> },

    /// HELIX nom {
    Helix { nom: String },

    /// GWEL forme miroir(point)
    Gwel { source: String, miroir: String },

    /// ADBERT forme centre(point)
    Adbert { source: String, centre: String },

    // ── Fermeture de bloc ──────────────────────────────────
    /// }
    FermerBloc,

    // ── Branche DUBI ──────────────────────────────────────
    /// CERTAIN → action
    /// bon (40-55) → action
    BrancheDubi { label: String, action: Option<String> },

    // ── Sinon après DUBI ──────────────────────────────────
    SinonBloc,

    // ── Non reconnu (gardé pour debug) ────────────────────
    Inconnu { ligne: String },
}

#[derive(Debug, Clone)]
pub enum AvonMode {
    Interne,
    Recevoir,
    Fournir,
}

// ================================================================
//  Fonction principale : parser une ligne
// ================================================================

pub fn parser_ligne(ligne: &str) -> Instruction {
    let l = ligne.trim();

    // ── Bloc fermant ──────────────────────────────────────────
    if l == "}" {
        return Instruction::FermerBloc;
    }

    // ── sinon seul (avant si pour éviter collision) ───────────
    if l == "sinon" || l == "sinon {" {
        return Instruction::Sinon;
    }
    if l == "sinon {" {
        return Instruction::SinonBloc;
    }

    // ── TARVOS ────────────────────────────────────────────────
    if l.starts_with("TARVOS") {
        return parser_tarvos(l);
    }

    // ── NEMETON ───────────────────────────────────────────────
    if l.starts_with("NEMETON") {
        let reste = l.trim_start_matches("NEMETON").trim();
        let nom = reste.trim_end_matches('{').trim().to_string();
        return Instruction::Nemeton { nom };
    }

    // ── AVON ──────────────────────────────────────────────────
    if l.starts_with("AVON") {
        return parser_avon(l);
    }

    // ── DUBI ──────────────────────────────────────────────────
    if l.starts_with("DUBI") {
        return parser_dubi(l);
    }

    // ── LUTOS ─────────────────────────────────────────────────
    if l.starts_with("LUTOS") {
        let fois = l.trim_start_matches("LUTOS")
                    .trim()
                    .trim_end_matches('{')
                    .trim()
                    .to_string();
        return Instruction::Lutos { fois };
    }

    // ── RANN ──────────────────────────────────────────────────
    if l.starts_with("RANN") {
        let module = l.trim_start_matches("RANN").trim().to_string();
        return Instruction::Rann { module };
    }

    // ── DRUS ──────────────────────────────────────────────────
    if l.starts_with("DRUS") {
        let reste = l.trim_start_matches("DRUS").trim();
        if let Some((src, dst)) = split_fleche(reste) {
            return Instruction::Drus {
                source: src.to_string(),
                dest:   dst.to_string(),
            };
        }
    }

    // ── PAR ───────────────────────────────────────────────────
    if l.starts_with("PAR") {
        let reste = l.trim_start_matches("PAR").trim().trim_end_matches('{').trim();
        let sync = reste.contains("synchronis");
        return Instruction::Par { synchronise: sync };
    }

    // ── GRANN ─────────────────────────────────────────────────
    if l.starts_with("GRANN") {
        return parser_grann(l);
    }

    // ── HELIX ─────────────────────────────────────────────────
    if l.starts_with("HELIX") {
        let nom = l.trim_start_matches("HELIX")
                   .trim()
                   .trim_end_matches('{')
                   .trim()
                   .to_string();
        return Instruction::Helix { nom };
    }

    // ── GWEL ──────────────────────────────────────────────────
    if l.starts_with("GWEL") {
        return parser_gwel(l);
    }

    // ── ADBERT ────────────────────────────────────────────────
    if l.starts_with("ADBERT") {
        return parser_adbert(l);
    }

    // ── Mots-clés français ────────────────────────────────────

    // declarer
    if l.starts_with("declarer") || l.starts_with("déclarer") {
        let reste = normaliser_prefixe(l, &["déclarer", "declarer"]);
        if let Some((nom, val)) = split_fleche(reste) {
            return Instruction::Declarer {
                nom:    nom.trim().to_string(),
                valeur: Some(val.trim().to_string()),
            };
        }
        return Instruction::Declarer {
            nom:    reste.trim().to_string(),
            valeur: None,
        };
    }

    // definir
    if l.starts_with("definir") || l.starts_with("définir") {
        let reste = normaliser_prefixe(l, &["définir", "definir"]);
        if let Some((nom, val)) = split_fleche(reste) {
            return Instruction::Definir {
                nom:    nom.trim().to_string(),
                valeur: val.trim().to_string(),
            };
        }
    }

    // figer
    if l.starts_with("figer") {
        let reste = l.trim_start_matches("figer").trim();
        if let Some((nom, val)) = split_fleche(reste) {
            return Instruction::Figer {
                nom:    nom.trim().to_string(),
                valeur: val.trim().to_string(),
            };
        }
    }

    // effacer
    if l.starts_with("effacer") {
        let nom = l.trim_start_matches("effacer").trim().to_string();
        return Instruction::Effacer { nom };
    }

    // afficher
    if l.starts_with("afficher") {
        let reste = l.trim_start_matches("afficher").trim();
        // "afficher expr → ecran" — on ignore la destination (toujours écran pour l'instant)
        let expression = if let Some((expr, _dest)) = split_fleche(reste) {
            expr.trim().to_string()
        } else {
            reste.to_string()
        };
        return Instruction::Afficher { expression };
    }

    // lire
    if l.starts_with("lire") {
        let reste = l.trim_start_matches("lire").trim();
        // "lire → variable" ou "lire variable"
        let variable = if let Some((_src, var)) = split_fleche(reste) {
            var.trim().to_string()
        } else {
            reste.to_string()
        };
        return Instruction::Lire { variable };
    }

    // si
    if l.starts_with("si ") && !l.starts_with("sinon") {
        let cond = l.trim_start_matches("si ")
                    .trim_end_matches('{')
                    .trim()
                    .to_string();
        return Instruction::Si { condition: cond };
    }

    // tantque
    if l.starts_with("tantque") {
        let cond = l.trim_start_matches("tantque")
                    .trim()
                    .trim_end_matches('{')
                    .trim()
                    .to_string();
        return Instruction::Tantque { condition: cond };
    }

    // repeter
    if l.starts_with("répéter") || l.starts_with("repeter") {
        let reste = normaliser_prefixe(l, &["répéter", "repeter"]);
        let fois = reste.trim_end_matches('{').trim().to_string();
        return Instruction::Repeter { fois };
    }

    // pourChaque
    if l.starts_with("pourChaque") {
        return parser_pour_chaque(l);
    }

    // sortir
    if l == "sortir" {
        return Instruction::Sortir;
    }

    // fonction
    if l.starts_with("fonction") {
        return parser_fonction(l);
    }

    // appeler
    if l.starts_with("appeler") {
        return parser_appeler(l);
    }

    // retourner
    if l.starts_with("retourner") {
        let val = l.trim_start_matches("retourner").trim().to_string();
        return Instruction::Retourner { valeur: val };
    }

    // Branches DUBI : labels standards + labels numériques
    if est_branche_dubi(l) {
        return parser_branche_dubi(l);
    }

    // ── Inconnu ───────────────────────────────────────────────
    Instruction::Inconnu { ligne: l.to_string() }
}

// ================================================================
//  Parsers spécialisés (privés)
// ================================================================

fn parser_tarvos(l: &str) -> Instruction {
    let reste = l.trim_start_matches("TARVOS").trim();

    // mode = premier mot
    let mode = reste.split_whitespace()
                    .next()
                    .unwrap_or("AUTO")
                    .to_string();

    // precision(...)
    let precision = extraire_parametre(reste, "precision")
        .unwrap_or_else(|| "f32".to_string());

    // contexte(...)
    let contexte = extraire_parametre(reste, "contexte")
        .unwrap_or_else(|| "calcul".to_string());

    Instruction::Tarvos { mode, precision, contexte }
}

fn parser_avon(l: &str) -> Instruction {
    let reste = l.trim_start_matches("AVON").trim();

    if reste.starts_with("RECEVOIR") {
        let apres = reste.trim_start_matches("RECEVOIR").trim();
        let (src, dst) = split_fleche(apres).unwrap_or((apres, ""));
        return Instruction::Avon {
            mode:   AvonMode::Recevoir,
            source: src.trim().to_string(),
            dest:   dst.trim().to_string(),
        };
    }

    if reste.starts_with("FOURNIR") {
        let apres = reste.trim_start_matches("FOURNIR").trim();
        let (src, dst) = split_fleche(apres).unwrap_or((apres, ""));
        return Instruction::Avon {
            mode:   AvonMode::Fournir,
            source: src.trim().to_string(),
            dest:   dst.trim().to_string(),
        };
    }

    // Flux interne
    let (src, dst) = split_fleche(reste).unwrap_or((reste, ""));
    Instruction::Avon {
        mode:   AvonMode::Interne,
        source: src.trim().to_string(),
        dest:   dst.trim().to_string(),
    }
}

fn parser_dubi(l: &str) -> Instruction {
    // DUBI variable contexte(xxx) ?
    let reste = l.trim_start_matches("DUBI")
                  .trim()
                  .trim_end_matches('?')
                  .trim()
                  .to_string();

    let contexte = extraire_parametre(&reste, "contexte")
        .unwrap_or_else(|| "calcul".to_string());

    // variable = tout ce qui précède "contexte(" ou "?"
    let variable = reste
        .split("contexte(")
        .next()
        .unwrap_or(&reste)
        .trim()
        .to_string();

    Instruction::Dubi { variable, contexte }
}

fn parser_grann(l: &str) -> Instruction {
    // GRANN nom(type) → valeur
    let reste = l.trim_start_matches("GRANN").trim();
    if let Some((avant, val)) = split_fleche(reste) {
        let avant = avant.trim();
        let (nom, type_str) = if let Some(p) = avant.find('(') {
            let n = &avant[..p];
            let t = avant[p+1..].trim_end_matches(')');
            (n.trim().to_string(), t.to_string())
        } else {
            (avant.to_string(), "auto".to_string())
        };
        return Instruction::Grann {
            nom,
            type_str,
            valeur: Some(val.trim().to_string()),
        };
    }
    Instruction::Grann {
        nom:      reste.to_string(),
        type_str: "auto".to_string(),
        valeur:   None,
    }
}

fn parser_gwel(l: &str) -> Instruction {
    // GWEL forme miroir(point)
    let reste = l.trim_start_matches("GWEL").trim();
    let miroir = extraire_parametre(reste, "miroir")
        .unwrap_or_else(|| "#808080".to_string());
    let source = reste.split("miroir(")
                       .next()
                       .unwrap_or("")
                       .trim()
                       .to_string();
    Instruction::Gwel { source, miroir }
}

fn parser_adbert(l: &str) -> Instruction {
    // ADBERT forme centre(point)
    let reste = l.trim_start_matches("ADBERT").trim();
    let centre = extraire_parametre(reste, "centre")
        .unwrap_or_else(|| "#808080".to_string());
    let source = reste.split("centre(")
                       .next()
                       .unwrap_or("")
                       .trim()
                       .to_string();
    Instruction::Adbert { source, centre }
}

fn parser_pour_chaque(l: &str) -> Instruction {
    // pourChaque element dans collection {
    let reste = l.trim_start_matches("pourChaque")
                  .trim()
                  .trim_end_matches('{')
                  .trim();
    if let Some((elem, coll)) = reste.split_once(" dans ") {
        return Instruction::PourChaque {
            element:    elem.trim().to_string(),
            collection: coll.trim().to_string(),
        };
    }
    Instruction::PourChaque {
        element:    reste.to_string(),
        collection: String::new(),
    }
}

fn parser_fonction(l: &str) -> Instruction {
    // fonction nom(p1, p2) {
    let reste = l.trim_start_matches("fonction")
                  .trim()
                  .trim_end_matches('{')
                  .trim();
    if let Some(p) = reste.find('(') {
        let nom    = reste[..p].trim().to_string();
        let params_str = reste[p+1..].trim_end_matches(')');
        let params: Vec<String> = params_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        return Instruction::Fonction { nom, params };
    }
    Instruction::Fonction {
        nom:    reste.to_string(),
        params: Vec::new(),
    }
}

fn parser_appeler(l: &str) -> Instruction {
    // appeler nom(arg1, arg2) → dest
    let reste = l.trim_start_matches("appeler").trim();

    let (appel, dest) = if let Some((a, d)) = split_fleche(reste) {
        (a.trim(), Some(d.trim().to_string()))
    } else {
        (reste, None)
    };

    if let Some(p) = appel.find('(') {
        let nom = appel[..p].trim().to_string();
        let args_str = appel[p+1..].trim_end_matches(')');
        let args: Vec<String> = args_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        return Instruction::Appeler { nom, args, dest };
    }
    Instruction::Appeler {
        nom:  appel.to_string(),
        args: Vec::new(),
        dest,
    }
}

// ── Branches DUBI ──────────────────────────────────────────────

/// Détecte si une ligne est une branche de DUBI
fn est_branche_dubi(l: &str) -> bool {
    // Labels standards
    let labels = [
        "CERTAIN", "PROBABLE", "BRUME", "INCONNU",
        "oui", "peut-être", "non", "inconnu",
        "actif", "instable", "inactif", "absent",
        "présent", "atténué",
        "confirmer", "préciser", "refuser", "demander",
        "trouvé", "partiel", "erreur",
        "valide", "approximé", "aberrant", "manquant",
        "excellent", "bon", "acceptable", "faible", "critique",
        "gagner", "perdre", "égalité", "abandon",
    ];
    // Un label suivi de → ou d'une plage numérique (> 55)
    for lb in &labels {
        if l.starts_with(lb) { return true; }
    }
    false
}

fn parser_branche_dubi(l: &str) -> Instruction {
    // "CERTAIN → action"  ou  "CERTAIN" seul  ou  "bon (40-55) → action"
    if let Some((label, action)) = split_fleche(l) {
        Instruction::BrancheDubi {
            label:  label.trim().to_string(),
            action: Some(action.trim().to_string()),
        }
    } else {
        Instruction::BrancheDubi {
            label:  l.to_string(),
            action: None,
        }
    }
}

// ================================================================
//  Utilitaires
// ================================================================

/// Coupe sur la flèche → (U+2192) — retourne (gauche, droite)
pub fn split_fleche(s: &str) -> Option<(&str, &str)> {
    s.find('→').map(|i| (&s[..i], &s[i + '→'.len_utf8()..]))
}

/// Extrait "precision(f32)" → "f32"
pub fn extraire_parametre<'a>(texte: &'a str, nom: &str) -> Option<String> {
    let motif = format!("{}(", nom);
    let debut = texte.find(&motif)? + motif.len();
    let fin   = texte[debut..].find(')')?;
    Some(texte[debut..debut + fin].to_string())
}

/// Retire un préfixe parmi plusieurs variantes (avec/sans accent)
fn normaliser_prefixe<'a>(l: &'a str, prefixes: &[&str]) -> &'a str {
    for p in prefixes {
        if l.starts_with(p) {
            return l[p.len()..].trim();
        }
    }
    l
}
