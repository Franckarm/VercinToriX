// ========================================================
//  VERCINTORIX V0.3 — executeur.rs
// ========================================================

use crate::memoire::{Memoire, Etat};
use crate::collecteur::{collecter_bloc, executer_bloc};
use std::collections::HashMap;

// ── CONTEXTE ────────────────────────────────────────────

pub struct Contexte {
    pub tarvos_mode:      String,
    pub tarvos_precision: String,
    pub dubi_contexte:    String,
    pub profondeur:       usize,
    pub modules:          Vec<String>,
    pub fonctions:        HashMap<String, (Vec<String>, Vec<String>)>,
}

impl Contexte {
    pub fn new() -> Self {
        Contexte {
            tarvos_mode:      String::from("AUTO"),
            tarvos_precision: String::from("f32"),
            dubi_contexte:    String::from("calcul"),
            profondeur:       0,
            modules:          Vec::new(),
            fonctions:        HashMap::new(),
        }
    }
    pub fn indentation(&self) -> String {
        "  ".repeat(self.profondeur)
    }
}

// ── LABELS DUBI PAR CONTEXTE ────────────────────────────

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

// ── POINT D'ENTRÉE PRINCIPAL ────────────────────────────

pub fn executer_ligne(
    ligne:  &str,
    mem:    &mut Memoire,
    ctx:    &mut Contexte,
    lignes: &[String],
    idx:    &mut usize,
) {
    let ligne = ligne.trim();
    if ligne.is_empty() || ligne.starts_with('←') || ligne.starts_with("//") {
        return;
    }

    // Ignorer les } isolés — déjà consommés par collecter_bloc
    if ligne == "}" {
        return;
    }

    let indent = ctx.indentation();

    // ── TARVOS ──────────────────────────────────────────
    if ligne.starts_with("TARVOS") {
        let reste = ligne.trim_start_matches("TARVOS").trim();

        if reste.contains("precision(f64)")      { ctx.tarvos_precision = "f64".to_string(); }
        else if reste.contains("precision(adn)") { ctx.tarvos_precision = "adn".to_string(); }
        else                                      { ctx.tarvos_precision = "f32".to_string(); }

        if let Some(s) = reste.find("contexte(") {
            let after = &reste[s+9..];
            if let Some(e) = after.find(')') {
                ctx.dubi_contexte = after[..e].to_string();
            }
        }

        ctx.tarvos_mode = reste.split_whitespace().next()
            .unwrap_or("AUTO").to_string();

        println!("\x1b[33m[ VERCINTORIX V0.3 ]\x1b[0m");
        println!("\x1b[33mTARVOS {} | precision:{} | contexte:{}\x1b[0m",
            ctx.tarvos_mode, ctx.tarvos_precision, ctx.dubi_contexte);

    // ── RANN ────────────────────────────────────────────
    } else if ligne.starts_with("RANN") {
        let m = ligne.trim_start_matches("RANN").trim();
        ctx.modules.push(m.to_string());
        println!("{}RANN {} \x1b[95m→ charge\x1b[0m", indent, m);

    // ── NEMETON ─────────────────────────────────────────
    } else if ligne.starts_with("NEMETON") {
        let nom = ligne.trim_start_matches("NEMETON").trim()
            .trim_end_matches('{').trim().to_string();
        
        ctx.profondeur += 1;
        let indent = ctx.indentation();
        println!("{}NEMETON {} {{", indent, nom);

        // Collecter le bloc UNE SEULE FOIS
        *idx += 1;
        let bloc = collecter_bloc(lignes, idx);

        // Enregistrer les fonctions internes d'abord
        let mut i = 0;
        while i < bloc.len() {
            let l = bloc[i].trim().to_string();
            if l.starts_with("fonction") {
                let header = l.trim_start_matches("fonction").trim();
                if let Some(po) = header.find('(') {
                    let nom_fn = header[..po].trim().to_string();
                    let apres = &header[po+1..];
                    let params: Vec<String> = if let Some(pc) = apres.find(')') {
                        apres[..pc].split(',')
                            .map(|p| p.trim().to_string())
                            .filter(|p| !p.is_empty())
                            .collect()
                    } else { vec![] };
                    i += 1;
                    let mut corps: Vec<String> = Vec::new();
                    let mut prof2: i32 = 1;
                    while i < bloc.len() && prof2 > 0 {
                        let ll = bloc[i].trim().to_string();
                        let o = ll.matches('{').count() as i32;
                        let f = ll.matches('}').count() as i32;
                        prof2 += o - f;
                        if prof2 <= 0 { i += 1; break; }
                        corps.push(ll);
                        i += 1;
                    }
                    ctx.fonctions.insert(nom_fn, (params, corps));
                }
            }
            i += 1;
        }

        // Exécuter le contenu
        let mut sub = 0usize;
        while sub < bloc.len() {
            let l = bloc[sub].trim().to_string();
            if l.starts_with("fonction") {
                // Sauter les définitions déjà enregistrées
                let mut prof3: i32 = 1;
                sub += 1;
                while sub < bloc.len() && prof3 > 0 {
                    let ll = &bloc[sub];
                    prof3 += ll.matches('{').count() as i32;
                    prof3 -= ll.matches('}').count() as i32;
                    sub += 1;
                }
                continue;
            }
            let avant = sub;
            executer_ligne(&l, mem, ctx, &bloc, &mut sub);
            if sub == avant { sub += 1; }
        }

        // UNE SEULE FOIS
        if ctx.profondeur > 0 { ctx.profondeur -= 1; }
        let indent = ctx.indentation();
        println!("{}}} ← NEMETON", indent);
        return;

    // ── GRANN ───────────────────────────────────────────
    } else if ligne.starts_with("GRANN") {
        let nom = ligne.trim_start_matches("GRANN").trim()
            .trim_end_matches('{').trim();
        println!("{}\x1b[36mGRANN {}\x1b[0m", indent, nom);
        mem.declarer(nom);

    // ── CORIO ───────────────────────────────────────────
    } else if ligne.starts_with("CORIO") {
        let nom = ligne.trim_start_matches("CORIO").trim()
            .trim_end_matches('{').trim();
        println!("{}\x1b[36mCORIO {}\x1b[0m", indent, nom);

        *idx += 1;
        let bloc = collecter_bloc(lignes, idx);

        let mut sub = 0usize;
        while sub < bloc.len() {
            let l = bloc[sub].clone();
            let avant = sub;
            executer_ligne(&l, mem, ctx, &bloc, &mut sub);
            if sub == avant { sub += 1; } else { sub += 1; }
        }

    // ── HELIX ───────────────────────────────────────────
    } else if ligne.starts_with("HELIX") {
        let rest = ligne.trim_start_matches("HELIX").trim()
            .trim_end_matches('{').trim();
        
        // Extraire nom + position hex optionnelle
        // HELIX décision #FF8800
        let (nom, pos_hex) = extraire_nom_hex(rest);
        
        println!("{}\x1b[96mHELIX {} {}\x1b[0m", 
            indent, nom, pos_hex.as_deref().unwrap_or(""));

        *idx += 1;
        let bloc = collecter_bloc(lignes, idx);

        let mut alleles: Vec<(String, String, [f32;3])> = Vec::new();
        let mut mutation_taux = 0.0f32;
        let mut selection_mode = "premier"; // ou "cube"

        for l in &bloc {
            let l = l.trim();
            if l.starts_with("ALLELE") {
                let reste = l.trim_start_matches("ALLELE").trim();
                if let Some((label, action)) = reste.split_once('→') {
                    let label = label.trim();
                    // Parser la couleur hex du label si présente
                    let couleur = hex_vers_rgb(label)
                        .unwrap_or([0.5, 0.5, 0.5]);
                    alleles.push((
                        label.to_string(),
                        action.trim().to_string(),
                        couleur
                    ));
                }
            } else if l.starts_with("MUTATION") {
                mutation_taux = l.trim_start_matches("MUTATION")
                    .trim().parse::<f32>().unwrap_or(0.0);
            } else if l.starts_with("SELECTION") {
                selection_mode = if l.contains("cube") { 
                    "cube" 
                } else { 
                    "premier" 
                };
            }
        }

        if !alleles.is_empty() {
            // Position courante — depuis hex ou centre du cube
            let pos_courante = pos_hex
                .as_deref()
                .and_then(hex_vers_rgb)
                .unwrap_or([0.5, 0.5, 0.5]);

            let choix = match selection_mode {
                "cube" => {
                    // Trouver l'allele le plus proche dans le cube
                    alleles.iter().enumerate()
                        .min_by(|(_, a), (_, b)| {
                            distance_cube(pos_courante, a.2)
                                .partial_cmp(
                                &distance_cube(pos_courante, b.2))
                                .unwrap()
                        })
                        .map(|(i, _)| i)
                        .unwrap_or(0)
                }
                _ => 0
            };

            let (label, action, rgb) = &alleles[choix];
            println!("{}  \x1b[96m→ ALLELE {} {:?}\x1b[0m", 
                indent, label, rgb);
            executer_ligne(action, mem, ctx, lignes, idx);
        }


    // ── PAR ─────────────────────────────────────────────
    } else if ligne.starts_with("PAR") {
        let mode = if ligne.contains("synchronise") { "synchronisé" } else { "libre" };
        println!("{}\x1b[96mPAR [{}] → séquentiel V0.3\x1b[0m", indent, mode);

        *idx += 1;
        let bloc = collecter_bloc(lignes, idx);

        // V0.3 — séquentiel avec annotation
        // V0.4 — Rust threads + Arc<Mutex<Memoire>>
        let mut sub = 0usize;
        while sub < bloc.len() {
            let l = bloc[sub].clone();
            let avant = sub;
            executer_ligne(&l, mem, ctx, &bloc, &mut sub);
            if sub == avant { sub += 1; } else { sub += 1; }
        }

    // ── AVON ────────────────────────────────────────────
    } else if ligne.starts_with("AVON") {
        let nom = ligne.trim_start_matches("AVON").trim()
            .trim_end_matches('{').trim();
        println!("{}\x1b[96mAVON {}\x1b[0m", indent, nom);

        *idx += 1;
        let bloc = collecter_bloc(lignes, idx);

        let mut sub = 0usize;
        while sub < bloc.len() {
            let l = bloc[sub].clone();
            if l.trim().starts_with("RECEVOIR") {
                println!("{}  RECEVOIR → flux entrant", indent);
            } else if l.trim().starts_with("FOURNIR") {
                println!("{}  FOURNIR → flux sortant", indent);
            } else {
                let avant = sub;
                executer_ligne(&l, mem, ctx, &bloc, &mut sub);
                if sub == avant { sub += 1; continue; }
            }
            sub += 1;
        }

    // ── DUBI ────────────────────────────────────────────
        } else if ligne.starts_with("DUBI") {
            let reste = ligne.trim_start_matches("DUBI").trim()
                .trim_end_matches('?').trim();

            // Évaluer la variable
            let valeur = mem.eval(reste);
            let valeur_propre = valeur.trim().trim_matches('"').to_string(); // ← NOUVEAU
            let labels = labels_dubi(&ctx.dubi_contexte);

            // Déterminer l'état
            let etat = if valeur == "CERTAIN" || valeur == labels[0] {
                labels[0]
            } else if valeur == "PROBABLE" || valeur == labels[1] {
                labels[1]
            } else if valeur == "BRUME" || valeur == labels[2] {
                labels[2]
            } else {
                if let Ok(f) = valeur.parse::<f64>() {
                    if f >= 0.8      { labels[0] }
                    else if f >= 0.4 { labels[1] }
                    else if f > 0.0  { labels[2] }
                    else             { labels[3] }
                } else if valeur.starts_with('#') {
                    labels[0]
                } else {
                    labels[3]
                }
            };

            println!("{}DUBI {} → {}", indent, reste, etat);

            // Collecter les branches
            *idx += 1;
            let bloc = collecter_dubi(lignes, idx, &labels);

            // Exécuter la branche correspondante
            for (label, actions) in &bloc {
                let label_propre = label.trim().trim_matches('"').to_string(); // ← NOUVEAU

                // ── Comparer valeur brute EN PREMIER ────────────
                let correspond = label_propre == valeur_propre  // ← NOUVEAU
                    || label_propre == etat
                    || label.contains(etat);

                if correspond { // ← NOUVEAU
                    let mut sub = 0usize;
                    while sub < actions.len() {
                        let l = actions[sub].clone();
                        let avant = sub;
                        executer_ligne(&l, mem, ctx, actions, &mut sub);
                        if sub == avant { sub += 1; } else { sub += 1; }
                    }
                    break;
                }
            }


    // ── LUTOS ───────────────────────────────────────────
    } else if ligne.starts_with("LUTOS") {
        let reste = ligne.trim_start_matches("LUTOS").trim()
            .trim_end_matches('{').trim();
        let fois = mem.eval(reste).parse::<usize>().unwrap_or(0);

        *idx += 1;
        let bloc = collecter_bloc(lignes, idx);

        for i in 0..fois {
            mem.definir("_lutos_i", &i.to_string());
            let mut sub = 0usize;
            while sub < bloc.len() {
                let l = bloc[sub].clone();
                let avant = sub;
                executer_ligne(&l, mem, ctx, &bloc, &mut sub);
                if sub == avant { sub += 1; } else { sub += 1; }
            }
        }

    // ── GWEL ────────────────────────────────────────────
    } else if ligne.starts_with("GWEL") {
        let reste = ligne.trim_start_matches("GWEL").trim();
        println!("{}\x1b[96mGWEL {}\x1b[0m", indent, reste);
        // V0.4 : miroir chromatique dans le cube

    // ── DRUS ────────────────────────────────────────────
    } else if ligne.starts_with("DRUS") {
        if let Some((src, cib)) = ligne.trim_start_matches("DRUS")
            .trim().split_once('→') {
            println!("{}\x1b[33mDRUS {} → {}\x1b[0m",
                indent, src.trim(), cib.trim());
        }

    // ── fonction ────────────────────────────────────────
    } else if ligne.starts_with("fonction") {
        // Collecter la définition
        let header = ligne.trim_start_matches("fonction").trim();
        if let Some(po) = header.find('(') {
            let nom = header[..po].trim().to_string();
            let apres = &header[po+1..];
            let params: Vec<String> = if let Some(pc) = apres.find(')') {
                apres[..pc].split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect()
            } else { vec![] };

            *idx += 1;
            let corps = collecter_bloc(lignes, idx);

            println!("{}fonction {} ({} params) → définie",
                indent, nom, params.len());
            ctx.fonctions.insert(nom, (params, corps));
        }

    // ── appeler ─────────────────────────────────────────
    } else if ligne.starts_with("appeler") {
        let reste = ligne.trim_start_matches("appeler").trim();
        if let Some(po) = reste.find('(') {
            let nom = reste[..po].trim().to_string();
            let apres = &reste[po+1..];
            let args_str: Vec<String> = if let Some(pc) = apres.find(')') {
                apres[..pc].split(',')
                    .map(|a| a.trim().to_string())
                    .filter(|a| !a.is_empty())
                    .collect()
            } else { vec![] };

            let args_vals: Vec<String> = args_str.iter()
                .map(|a| mem.eval(a))
                .collect();

            if let Some((params, corps)) = ctx.fonctions.get(&nom).cloned() {
                println!("{}appeler {}({})", indent, nom,
                    args_vals.join(", "));

                // Lier les paramètres
                for (p, v) in params.iter().zip(args_vals.iter()) {
                    mem.definir(p, v);
                }

                // Exécuter le corps
                let mut sub = 0usize;
                while sub < corps.len() {
                    let l = corps[sub].clone();
                    // Détecter retourner
                    if l.trim().starts_with("retourner") {
                        let expr = l.trim()
                            .trim_start_matches("retourner").trim();
                        let val = mem.eval(expr);
                        mem.definir("_resultat", &val);
                        break;
                    }
                    let avant = sub;
                    executer_ligne(&l, mem, ctx, &corps, &mut sub);
                    if sub == avant { sub += 1; } else { sub += 1; }
                }
            } else {
                println!("{}\x1b[90m[VTX-003] Fonction inconnue : {}\x1b[0m",
                    indent, nom);
            }
        }

    // ── si / sinon ──────────────────────────────────────
    } else if ligne.starts_with("si ") {
        let condition = ligne.trim_start_matches("si").trim()
            .trim_end_matches('{').trim();
        let resultat = evaluer_condition(condition, mem);

        *idx += 1;
        let bloc_si = collecter_bloc(lignes, idx);

        // Chercher sinon
        let mut bloc_sinon: Vec<String> = Vec::new();
        if *idx < lignes.len() && lignes[*idx].trim() == "sinon {" {
            *idx += 1;
            bloc_sinon = collecter_bloc(lignes, idx);
            // idx reste sur le } de sinon — main fera +1
        } else {
            // Pas de sinon — reculer pour que main fasse +1 correctement
            if *idx > 0 { *idx -= 1; }
        }

        let bloc_actif = if resultat { &bloc_si } else { &bloc_sinon };
        let mut sub = 0usize;
        while sub < bloc_actif.len() {
            let l = bloc_actif[sub].clone();
            let avant = sub;
            executer_ligne(&l, mem, ctx, bloc_actif, &mut sub);
            if sub == avant { sub += 1; } else { sub += 1; }
        }

    // ── tantque ─────────────────────────────────────────
    } else if ligne.starts_with("tantque") {
        let condition = ligne.trim_start_matches("tantque").trim()
            .trim_end_matches('{').trim().to_string();

        *idx += 1;
        let bloc = collecter_bloc(lignes, idx);

        let mut garde = 0usize;
        while evaluer_condition(&condition, mem) {
            garde += 1;
            if garde > 10_000 {
                println!("{}\x1b[31m[VTX-004] tantque — limite 10000 iterations\x1b[0m", indent);
                break;
            }
            let mut sub = 0usize;
            while sub < bloc.len() {
                let l = bloc[sub].clone();
                if l.trim() == "sortir" { garde = 99_999; break; }
                let avant = sub;
                executer_ligne(&l, mem, ctx, &bloc, &mut sub);
                if sub == avant { sub += 1; } else { sub += 1; }
            }
        }

    // ── répéter ─────────────────────────────────────────
    } else if ligne.starts_with("r\u{e9}peter") || ligne.starts_with("repeter") {
        let reste = ligne
            .trim_start_matches("répéter")
            .trim_start_matches("repeter")
            .trim()
            .trim_end_matches('{').trim();
        let fois = mem.eval(reste).parse::<usize>().unwrap_or(0);

        *idx += 1;
        let bloc = collecter_bloc(lignes, idx);

        for i in 0..fois {
            mem.definir("_repeter_i", &i.to_string());
            let mut sub = 0usize;
            while sub < bloc.len() {
                let l = bloc[sub].clone();
                let avant = sub;
                executer_ligne(&l, mem, ctx, &bloc, &mut sub);
                if sub == avant { sub += 1; } else { sub += 1; }
            }
        }

    // ── pourChaque ──────────────────────────────────────
    } else if ligne.starts_with("pourCh\u{e0}que") || ligne.starts_with("pourChaque") {
        // pourChaque x dans liste {
        // pourChaque x, i dans liste {
        let reste = ligne
            .trim_start_matches("pourChaque")
            .trim_start_matches("pourChàque")
            .trim()
            .trim_end_matches('{').trim();

        // Parser "x dans liste" ou "x, i dans liste"
        if let Some(dans_pos) = reste.find(" dans ") {
            let vars_part = &reste[..dans_pos].trim();
            let liste_nom = reste[dans_pos+6..].trim();

            // Variables : "x" ou "x, i"
            let vars: Vec<&str> = vars_part.split(',')
                .map(|v| v.trim())
                .collect();
            let var_val = vars[0];
            let var_idx = if vars.len() > 1 { Some(vars[1]) } else { None };

            // Récupérer la liste
            let liste_val = mem.eval(liste_nom);
            let elements = parser_liste(&liste_val);

            *idx += 1;
            let bloc = collecter_bloc(lignes, idx);

            for (i, elem) in elements.iter().enumerate() {
                mem.definir(var_val, elem);
                if let Some(vi) = var_idx {
                    mem.definir(vi, &i.to_string());
                }
                let mut sub = 0usize;
                while sub < bloc.len() {
                    let l = bloc[sub].clone();
                    let avant = sub;
                    executer_ligne(&l, mem, ctx, &bloc, &mut sub);
                    if sub == avant { sub += 1; } else { sub += 1; }
                }
            }
        }

    // ── ESSAYER / ATTRAPER ──────────────────────────────
    } else if ligne.starts_with("essayer") {
        println!("{}\x1b[35messayer\x1b[0m", indent);

        let mut bloc_essayer:  Vec<String> = Vec::new();
        let mut bloc_attraper: Vec<String> = Vec::new();
        let mut var_err = String::from("err");

        // Sauter le "{" initial si présent
        if *idx < lignes.len() && (lignes[*idx].trim() == "{" || lignes[*idx].trim() == "essayer {") {
            *idx += 1;
        }

        // Collecter jusqu'à "} attraper"
        while *idx < lignes.len() {
            let l = lignes[*idx].trim().to_string();
            *idx += 1;
            if l.starts_with("} attraper") {
                let parts: Vec<&str> = l.split_whitespace().collect();
                if parts.len() >= 3 {
                    var_err = parts[2].trim_end_matches('{').trim().to_string();
                }
                break;
            }
            if l == "{" { continue; }  // ← ignorer accolades seules
            bloc_essayer.push(l);
        }

        // Collecter bloc attraper
        while *idx < lignes.len() {
            let l = lignes[*idx].trim().to_string();
            *idx += 1;
            if l == "}" { break; }
            if l == "{" { continue; }
            bloc_attraper.push(l);
        }

        // Exécuter essayer
        let mut erreur: Option<String> = None;
        let mut sub = 0usize;
        while sub < bloc_essayer.len() {
            let l = bloc_essayer[sub].clone();
            if l.starts_with("signaler") {
                let msg = l.trim_start_matches("signaler")
                        .trim()
                        .trim_matches('"')
                        .to_string();
                erreur = Some(msg);
                break;
            }
            executer_ligne(&l, mem, ctx, &bloc_essayer, &mut sub);
            sub += 1;
        }

        // Si erreur → attraper
        if let Some(msg) = erreur {
            mem.definir(&var_err, &msg);
            let mut sub = 0usize;
            while sub < bloc_attraper.len() {
                let l = bloc_attraper[sub].clone();
                executer_ligne(&l, mem, ctx, &bloc_attraper, &mut sub);
                sub += 1;
            }
        }
        return;

    // ── déclarer ────────────────────────────────────────
    } else if ligne.starts_with("d\u{e9}clarer") || ligne.starts_with("declarer") {
        if let Some((g, d)) = ligne.split_once('→') {
            let nom = g.split_whitespace().last().unwrap_or("");
            let val = mem.eval(d.trim());
            mem.declarer(nom);
            mem.definir(nom, &val);
            println!("{}déclarer {} → {}", indent, nom, val);
        } else {
            let nom = ligne
                .trim_start_matches("déclarer")
                .trim_start_matches("declarer")
                .trim();
            mem.declarer(nom);
        }

    // ── définir ─────────────────────────────────────────
    } else if ligne.starts_with("d\u{e9}finir") || ligne.starts_with("definir") {
        if let Some((g, d)) = ligne.split_once('→') {
            let nom = g.split_whitespace().last().unwrap_or("");
            let val = mem.eval(d.trim());
            mem.definir(nom, &val);
        }

    // ── figer ───────────────────────────────────────────
    } else if ligne.starts_with("figer") {
        if let Some((g, d)) = ligne.split_once('→') {
            let nom = g.split_whitespace().last().unwrap_or("");
            let val = mem.eval(d.trim());
            mem.figer(nom, &val);
        }

    // ── effacer ─────────────────────────────────────────
    } else if ligne.starts_with("effacer") {
        let nom = ligne.trim_start_matches("effacer").trim();
        mem.effacer(nom);

    // ── afficher ────────────────────────────────────────
    } else if ligne.starts_with("afficher") {
        let reste = ligne.trim_start_matches("afficher").trim();
        // Gérer "expr → écran" ou juste "expr"
        let source = if let Some((src, _)) = reste.split_once('→') {
            src.trim()
        } else {
            reste
        };
        let val = mem.eval(source);
        println!("{}\x1b[32m> {}\x1b[0m", indent, val.trim_matches('"'));

    // ── lire ────────────────────────────────────────────
    } else if ligne.starts_with("lire") {
        let nom = ligne.trim_start_matches("lire").trim();
        let mut input = String::new();
        print!("{}\x1b[36m? {}: \x1b[0m", indent, nom);
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut input);
        mem.definir(nom, input.trim());

    // ── relier ──────────────────────────────────────────
    } else if ligne.contains('→') && !ligne.starts_with('←') {
        // Affectation directe : source → cible
        if let Some((src, cib)) = ligne.split_once('→') {
            let src = src.trim();
            let cib = cib.trim();
            // Ignorer les destinations matérielles
            if cib.starts_with("écran") || cib.starts_with("ecran")
            || cib.starts_with("disque") || cib.starts_with("réseau")
            || cib.starts_with("reseau") {
                let val = mem.eval(src);
                println!("{}\x1b[32m> {}\x1b[0m", indent, val.trim_matches('"'));
            } else {
                let val = mem.eval(src);
                mem.definir(cib, &val);
            }
        }

    // ── Ligne ignorée proprement ─────────────────────────
    } else if ligne.starts_with("retourner")
           || ligne.starts_with("signaler")
           || ligne.starts_with("ALLELE")
           || ligne.starts_with("MUTATION")
           || ligne.starts_with("SELECTION")
           || ligne.starts_with("RECEVOIR")
           || ligne.starts_with("FOURNIR")
           || ligne.starts_with("fin")
           || ligne == "{"
    {
        // Géré par le contexte parent

    // ── Non reconnu ─────────────────────────────────────
    } else {
        println!("{}\x1b[90m[VTX-002] : {}\x1b[0m", indent, ligne);
    }
}

// ── DUBI — collecte des branches ────────────────────────

fn collecter_dubi(
    lignes: &[String],
    idx:    &mut usize,
    labels: &[&str; 4],
) -> Vec<(String, Vec<String>)> {
    let mut branches: Vec<(String, Vec<String>)> = Vec::new();
    let mut branche_actuelle: Option<String> = None;
    let mut actions: Vec<String> = Vec::new();

    while *idx < lignes.len() {
        let l = lignes[*idx].trim().to_string();

        // Fin de bloc DUBI — ligne vide ou début d'autre primitive
        if l.is_empty() || l == "}" {
            break;
        }

        // Est-ce un label DUBI ?
        let est_label = labels.iter().any(|lb| l.starts_with(lb))
            || ["CERTAIN","PROBABLE","BRUME","INCONNU"].iter().any(|lb| l.starts_with(lb));

        // Début d'une autre primitive — fin du DUBI
        let est_primitive = l.starts_with("TARVOS") || l.starts_with("NEMETON")
            || l.starts_with("si ") || l.starts_with("afficher")
            || l.starts_with("d\u{e9}finir") || l.starts_with("definir")
            || l.starts_with("d\u{e9}clarer") || l.starts_with("declarer")
            || l.starts_with("LUTOS") || l.starts_with("pourChaque");

        if est_primitive && !est_label {
            if let Some(lb) = branche_actuelle.take() {
                branches.push((lb, actions.drain(..).collect()));
            }
            break;
        }

        if est_label {
            if let Some(lb) = branche_actuelle.take() {
                branches.push((lb, actions.drain(..).collect()));
            }
            // Action inline : "CERTAIN → afficher x"
            if l.contains('→') {
                if let Some((lb, act)) = l.split_once('→') {
                    branches.push((lb.trim().to_string(), vec![act.trim().to_string()]));
                }
            } else {
                branche_actuelle = Some(l.clone());
            }
        } else if branche_actuelle.is_some() {
            actions.push(l.clone());
        }

        *idx += 1;
    }

    if let Some(lb) = branche_actuelle {
        branches.push((lb, actions));
    }

    branches
}

// ── UTILITAIRES ─────────────────────────────────────────

/// Parse une liste "[a, b, c]" → vec!["a", "b", "c"]
fn parser_liste(s: &str) -> Vec<String> {
    let s = s.trim().trim_start_matches('[').trim_end_matches(']');
    s.split(',')
        .map(|e| e.trim().trim_matches('"').to_string())
        .filter(|e| !e.is_empty())
        .collect()
}

pub fn evaluer_condition(condition: &str, mem: &Memoire) -> bool {
    let c = condition.trim();
    if let Some((g, d)) = c.split_once("==") {
        return mem.eval(g.trim()) == mem.eval(d.trim());
    }
    if let Some((g, d)) = c.split_once("!=") {
        return mem.eval(g.trim()) != mem.eval(d.trim());
    }
    if let Some((g, d)) = c.split_once(">=") {
        return mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0)
            >= mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
    }
    if let Some((g, d)) = c.split_once("<=") {
        return mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0)
            <= mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
    }
    if let Some((g, d)) = c.split_once('>') {
        return mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0)
            >  mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
    }
    if let Some((g, d)) = c.split_once('<') {
        return mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0)
            <  mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
    }
    matches!(mem.eval(c).as_str(), "vrai" | "true" | "CERTAIN" | "1")
}
