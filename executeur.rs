// ========================================================
//  VERCINTORIX V0.3 — executeur.rs
//  Execute les instructions VERCINTORIX
// ========================================================

use crate::memoire::{Memoire, Etat};

// ── CONTEXTE D'EXECUTION ────────────────────────────────
pub struct Contexte {
    pub tarvos_mode:    String,
    pub tarvos_precision: String,  // f32, f64, adn
    pub dubi_contexte:  String,    // calcul, navigation, hardware...
    pub profondeur:     usize,     // niveau d'imbrication NEMETON
    pub modules:        Vec<String>,
}

impl Contexte {
    pub fn new() -> Self {
        Contexte {
            tarvos_mode:      String::from("AUTO"),
            tarvos_precision: String::from("f32"),
            dubi_contexte:    String::from("calcul"),
            profondeur:       0,
            modules:          Vec::new(),
        }
    }

    pub fn indentation(&self) -> String {
        "  ".repeat(self.profondeur)
    }
}

// ── LABELS DUBI SELON CONTEXTE ───────────────────────────
pub fn labels_dubi(contexte: &str) -> [&'static str; 4] {
    match contexte {
        "navigation" => ["oui",       "peut-etre", "non",      "inconnu"],
        "hardware"   => ["actif",     "instable",  "inactif",  "absent"],
        "medical"    => ["present",   "attenue",   "absent",   "inconnu"],
        "humain"     => ["confirmer", "preciser",  "refuser",  "demander"],
        "texte"      => ["trouve",    "partiel",   "absent",   "erreur"],
        "science"    => ["valide",    "approx",    "aberrant", "manquant"],
        _            => ["CERTAIN",   "PROBABLE",  "BRUME",    "INCONNU"],
    }
}

// ── EXECUTEUR PRINCIPAL ──────────────────────────────────
pub fn executer_ligne(
    ligne:  &str,
    mem:    &mut Memoire,
    ctx:    &mut Contexte,
    lignes: &[&str],
    idx:    &mut usize,
) {
    let indent = ctx.indentation();

    // ── TARVOS ──────────────────────────────────────────
    if ligne.starts_with("TARVOS") {
        let reste = ligne.trim_start_matches("TARVOS").trim();
        // Precision
        if reste.contains("precision(f64)") { ctx.tarvos_precision = "f64".to_string(); }
        else if reste.contains("precision(adn)") { ctx.tarvos_precision = "adn".to_string(); }
        else { ctx.tarvos_precision = "f32".to_string(); }
        // Contexte DUBI — cherche contexte(xxx) precisement
        if let Some(start) = reste.find("contexte(") {
            let after = &reste[start+9..];
            if let Some(end) = after.find(')') {
                ctx.dubi_contexte = after[..end].to_string();
            }
        }
        ctx.tarvos_mode = reste.split_whitespace().next().unwrap_or("AUTO").to_string();
        println!("\x1b[33m[ VERCINTORIX V0.3 ]\x1b[0m");
        println!("\x1b[33mTARVOS {} | precision:{} | contexte:{}\x1b[0m",
            ctx.tarvos_mode, ctx.tarvos_precision, ctx.dubi_contexte);

    // ── RANN ────────────────────────────────────────────
    } else if ligne.starts_with("RANN") {
        let module = ligne.trim_start_matches("RANN").trim();
        ctx.modules.push(module.to_string());
        println!("{}RANN {} \x1b[95m→ charge\x1b[0m", indent, module);

    // ── NEMETON ouverture ────────────────────────────────
    } else if ligne.starts_with("NEMETON") && ligne.contains('{') {
        let nom = ligne
            .trim_start_matches("NEMETON").trim()
            .trim_end_matches('{').trim();
        // Contexte local
        if ligne.contains("contexte(") {
            if let Some(start) = ligne.find("contexte(") {
                let rest = &ligne[start+9..];
                if let Some(end) = rest.find(')') {
                    ctx.dubi_contexte = rest[..end].to_string();
                }
            }
        }
        println!("\x1b[35m{}NEMETON {} {{\x1b[0m", indent, nom);
        ctx.profondeur += 1;

    // ── Fermeture bloc ───────────────────────────────────
    } else if ligne == "}" {
        if ctx.profondeur > 0 { ctx.profondeur -= 1; }
        let indent = ctx.indentation();
        println!("\x1b[35m{}}}\x1b[0m", indent);

    // ── declarer ────────────────────────────────────────
    } else if (ligne.starts_with("d\u{e9}clarer") || ligne.starts_with("declarer")) {
        let reste = ligne.trim_start_matches("d\u{e9}clarer").trim_start_matches("declarer").trim();
        if let Some((gauche, droite)) = reste.split_once('\u{2192}') {
            let nom = gauche.split_whitespace().next().unwrap_or("").trim();
            let val = mem.eval(droite.trim());
            mem.declarer(nom);
            mem.definir(nom, &val);
        } else {
            let nom = reste.split_whitespace().next().unwrap_or("");
            mem.declarer(nom);
        }

    // ── definir ─────────────────────────────────────────
    } else if (ligne.starts_with("d\u{e9}finir") || ligne.starts_with("definir")) {
        if let Some((gauche, droite)) = ligne.split_once('\u{2192}') {
            let nom = gauche.split_whitespace().last().unwrap_or("");
            let val = mem.eval(droite.trim());
            mem.definir(nom, &val);
        }

    // ── figer ───────────────────────────────────────────
    } else if ligne.starts_with("figer") {
        if let Some((gauche, droite)) = ligne.split_once('\u{2192}') {
            let nom = gauche.split_whitespace().last().unwrap_or("");
            let val = mem.eval(droite.trim());
            mem.figer(nom, &val);
        }

    // ── effacer ─────────────────────────────────────────
    } else if ligne.starts_with("effacer") {
        let nom = ligne.trim_start_matches("effacer").trim();
        mem.effacer(nom);
        println!("{}  \x1b[31m-\x1b[0m efface : {}", indent, nom);

    // ── afficher ─────────────────────────────────────────
    } else if ligne.starts_with("afficher") {
        let reste = ligne.trim_start_matches("afficher").trim();
        let source = if let Some((src, _)) = reste.split_once('\u{2192}') {
            src.trim()
        } else {
            reste
        };
        // Concatenation avec +
        if source.contains(" + ") {
            let val = mem.eval(source);
            println!("{}\x1b[32m> {}\x1b[0m", indent, val);
        } else {
            // Essayer d'abord comme variable, puis comme texte brut
            let val = mem.eval(source);
            println!("{}\x1b[32m> {}\x1b[0m", indent, val.trim_matches('"'));
        }

    // ── si / sinon ───────────────────────────────────────
    } else if ligne.starts_with("si ") {
        let condition = ligne.trim_start_matches("si").trim().trim_end_matches('{').trim();
        let resultat = evaluer_condition(condition, mem);
        println!("{}\x1b[31msi {} → {}\x1b[0m", indent, condition, resultat);

        // Collecter le bloc si
        let mut bloc_si: Vec<String> = Vec::new();
        let mut bloc_sinon: Vec<String> = Vec::new();
        let mut profondeur = 1;
        let mut dans_sinon = false;

        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            *idx += 1;

            // Cas special : "} sinon {" sur une seule ligne
            let est_sinon = l == "sinon {"
                || l == "sinon"
                || l == "} sinon {"
                || l == "} sinon"
                || (l.starts_with('}') && l.contains("sinon"));

            if est_sinon {
                dans_sinon = true;
                continue;
            }

            // Fermeture de bloc
            if l == "}" {
                profondeur -= 1;
                if profondeur == 0 { break; }
                continue;
            }

            // Ouverture de bloc imbriqué
            if l.ends_with('{') {
                profondeur += 1;
            }

            if dans_sinon { bloc_sinon.push(l.to_string()); }
            else { bloc_si.push(l.to_string()); }
        }

        // Executer le bon bloc
        let bloc = if resultat { &bloc_si } else { &bloc_sinon };
        let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
        let mut sub_idx = 0;
        while sub_idx < refs.len() {
            executer_ligne(refs[sub_idx], mem, ctx, &refs, &mut sub_idx);
            sub_idx += 1;
        }
        return;

    // ── repeter ──────────────────────────────────────────
    } else if (ligne.starts_with("r\u{e9}p\u{e9}ter") || ligne.starts_with("repeter")) {
        let reste = ligne
            .trim_start_matches("r\u{e9}p\u{e9}ter").trim_start_matches("repeter").trim()
            .trim_end_matches('{').trim();
        let fois = mem.eval(reste).parse::<usize>().unwrap_or(0);
        println!("{}\x1b[31mrepeter {} fois\x1b[0m", indent, fois);

        // Collecter le bloc
        let mut bloc: Vec<String> = Vec::new();
        let mut profondeur = 1;
        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            *idx += 1;
            if l.ends_with('{') { profondeur += 1; }
            if l == "}" {
                profondeur -= 1;
                if profondeur == 0 {
                    // Regarder si la prochaine ligne est "sinon"
                    let prochaine = if *idx < lignes.len() { lignes[*idx].trim() } else { "" };
                    if !prochaine.starts_with("sinon") { break; }
                    // Sinon on continue pour capturer le bloc sinon
                }
            }
            bloc.push(l.to_string());
        }

        let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
        for _ in 0..fois {
            let mut sub_idx = 0;
            while sub_idx < refs.len() {
                executer_ligne(refs[sub_idx], mem, ctx, &refs, &mut sub_idx);
                sub_idx += 1;
            }
        }
        return;

    // ── tantque ──────────────────────────────────────────
    } else if ligne.starts_with("tantque") {
        let condition = ligne
            .trim_start_matches("tantque").trim()
            .trim_end_matches('{').trim();
        println!("{}\x1b[31mtantque {}\x1b[0m", indent, condition);

        let mut bloc: Vec<String> = Vec::new();
        let mut profondeur = 1;
        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            *idx += 1;
            if l.ends_with('{') { profondeur += 1; }
            if l == "}" {
                profondeur -= 1;
                if profondeur == 0 {
                    // Regarder si la prochaine ligne est "sinon"
                    let prochaine = if *idx < lignes.len() { lignes[*idx].trim() } else { "" };
                    if !prochaine.starts_with("sinon") { break; }
                    // Sinon on continue pour capturer le bloc sinon
                }
            }
            bloc.push(l.to_string());
        }

        let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
        let mut guard = 0;
        while evaluer_condition(condition, mem) && guard < 10000 {
            guard += 1;
            let mut sub_idx = 0;
            while sub_idx < refs.len() {
                executer_ligne(refs[sub_idx], mem, ctx, &refs, &mut sub_idx);
                sub_idx += 1;
            }
        }
        return;

    // ── DUBI ────────────────────────────────────────────
    } else if ligne.starts_with("DUBI") {
        let nom = ligne
            .trim_start_matches("DUBI").trim()
            .trim_end_matches('?').trim()
            .split_whitespace().next().unwrap_or("");

        let etat = if let Some(n) = mem.lire(nom) {
            n.etat
        } else {
            Etat::Brume
        };

        let labels = labels_dubi(&ctx.dubi_contexte);
        let label_choisi = match etat {
            Etat::Certain  => labels[0],
            Etat::Probable => labels[1],
            Etat::Brume    => labels[2],
            Etat::Inconnu  => labels[3],
        };

        println!("{}\x1b[31mDUBI {} → {}\x1b[0m", indent, nom, label_choisi);

        // Collecter les branches
        let mut branches: Vec<(String, Vec<String>)> = Vec::new();
        let mut branche_active: Option<String> = None;
        let mut bloc_courant: Vec<String> = Vec::new();

        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            // Fin du DUBI — ligne qui n'est pas une branche
            let est_branche = labels.iter().any(|lab| l.starts_with(lab))
                || l.starts_with("CERTAIN") || l.starts_with("PROBABLE")
                || l.starts_with("BRUME")   || l.starts_with("INCONNU");

            if !est_branche && !l.is_empty() && !l.starts_with('{') {
                // Verifier si c'est encore une branche du DUBI
                if branche_active.is_some() {
                    branches.push((
                        branche_active.take().unwrap(),
                        bloc_courant.drain(..).collect()
                    ));
                }
                break;
            }

            *idx += 1;

            if est_branche && l.contains('\u{2192}') {
                // Branche inline : CERTAIN → instruction
                if let Some(ba) = branche_active.take() {
                    branches.push((ba, bloc_courant.drain(..).collect()));
                }
                if let Some((label, action)) = l.split_once('\u{2192}') {
                    branches.push((
                        label.trim().to_string(),
                        vec![action.trim().to_string()]
                    ));
                    branche_active = None;
                }
            } else if est_branche {
                if let Some(ba) = branche_active.take() {
                    branches.push((ba, bloc_courant.drain(..).collect()));
                }
                branche_active = Some(l.to_string());
            } else if branche_active.is_some() {
                bloc_courant.push(l.to_string());
            }
        }
        if let Some(ba) = branche_active {
            branches.push((ba, bloc_courant));
        }

        // Executer la branche correspondante
        for (label, bloc) in &branches {
            let correspond = label.trim() == label_choisi
                || label.contains(label_choisi);
            if correspond {
                let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
                let mut sub_idx = 0;
                while sub_idx < refs.len() {
                    executer_ligne(refs[sub_idx], mem, ctx, &refs, &mut sub_idx);
                    sub_idx += 1;
                }
                break;
            }
        }
        return;

    // ── LUTOS ───────────────────────────────────────────
    } else if ligne.starts_with("LUTOS") {
        let reste = ligne
            .trim_start_matches("LUTOS").trim()
            .trim_end_matches('{').trim();
        let fois = mem.eval(reste).parse::<usize>().unwrap_or(0);
        println!("{}\x1b[31mLUTOS {} fois\x1b[0m", indent, fois);

        let mut bloc: Vec<String> = Vec::new();
        let mut profondeur = 1;
        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            *idx += 1;
            if l.ends_with('{') { profondeur += 1; }
            if l == "}" { profondeur -= 1; if profondeur == 0 { break; } }
            bloc.push(l.to_string());
        }
        let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
        for _ in 0..fois {
            let mut sub_idx = 0;
            while sub_idx < refs.len() {
                executer_ligne(refs[sub_idx], mem, ctx, &refs, &mut sub_idx);
                sub_idx += 1;
            }
        }
        return;

    // ── GRANN ───────────────────────────────────────────
    } else if ligne.starts_with("GRANN") {
        let nom = ligne
            .trim_start_matches("GRANN").trim()
            .trim_end_matches('{').trim();
        println!("{}\x1b[36mGRANN {}\x1b[0m", indent, nom);
        mem.declarer(nom);

    // ── HELIX ───────────────────────────────────────────
    } else if ligne.starts_with("HELIX") {
        let nom = ligne
            .trim_start_matches("HELIX").trim()
            .trim_end_matches('{').trim();
        println!("{}\x1b[96mHELIX {}\x1b[0m", indent, nom);

    // ── DRUS ────────────────────────────────────────────
    } else if ligne.starts_with("DRUS") {
        if let Some((source, cible)) = ligne
            .trim_start_matches("DRUS").trim()
            .split_once('\u{2192}')
        {
            println!("{}\x1b[33mDRUS {} → {}\x1b[0m",
                indent, source.trim(), cible.trim());
        }

    // ── AVON ────────────────────────────────────────────
    } else if ligne.starts_with("AVON") {
        if let Some((source, cible)) = ligne
            .trim_start_matches("AVON").trim()
            .split_once('\u{2192}')
        {
            let val = mem.eval(source.trim());
            mem.definir(cible.trim(), &val);
            println!("{}\x1b[34mAVON {} → {}\x1b[0m",
                indent, source.trim(), cible.trim());
        }

    // ── relier ──────────────────────────────────────────
    } else if ligne.starts_with("relier") {
        if let Some((source, cible)) = ligne
            .trim_start_matches("relier").trim()
            .split_once('\u{2192}')
        {
            let val = mem.eval(source.trim());
            mem.definir(cible.trim(), &val);
        }

    // ── Lignes internes ignorees ─────────────────────────
    } else if ligne.starts_with("ALLELE")
           || ligne.starts_with("MUTATION")
           || ligne.starts_with("SELECTION")
           || ligne.starts_with("valence")
           || ligne.starts_with("type")
           || ligne.starts_with("etat")
           || ligne.starts_with("donnees")
           || ligne == "{" {
        // ignore — partie d'un bloc HELIX ou GRANN

    // ── Ligne inconnue ───────────────────────────────────
    } else if !ligne.is_empty() {
        // Ignore silencieusement
    }
}

// ── EVALUATION DE CONDITIONS ────────────────────────────
pub fn evaluer_condition(condition: &str, mem: &Memoire) -> bool {
    let c = condition.trim();

    // Egalite : a == b
    if let Some((g, d)) = c.split_once("==") {
        let gv = mem.eval(g.trim());
        let dv = mem.eval(d.trim());
        return gv == dv;
    }
    // Inegalite : a != b
    if let Some((g, d)) = c.split_once("!=") {
        let gv = mem.eval(g.trim());
        let dv = mem.eval(d.trim());
        return gv != dv;
    }
    // Superieur ou egal : a >= b
    if let Some((g, d)) = c.split_once(">=") {
        let gv = mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0);
        let dv = mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
        return gv >= dv;
    }
    // Inferieur ou egal : a <= b
    if let Some((g, d)) = c.split_once("<=") {
        let gv = mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0);
        let dv = mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
        return gv <= dv;
    }
    // Superieur : a > b
    if let Some((g, d)) = c.split_once('>') {
        let gv = mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0);
        let dv = mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
        return gv > dv;
    }
    // Inferieur : a < b
    if let Some((g, d)) = c.split_once('<') {
        let gv = mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0);
        let dv = mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
        return gv < dv;
    }
    // Booleen direct
    let val = mem.eval(c);
    match val.as_str() {
        "vrai" | "true" | "CERTAIN" | "1" => true,
        _ => false,
    }
}