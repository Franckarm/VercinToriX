// ========================================================
//  VERCINTORIX V0.3 — executeur.rs
// ========================================================

use crate::memoire::{Memoire, Etat};
use std::collections::HashMap;

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

pub fn labels_dubi(contexte: &str) -> [&'static str; 4] {
    match contexte {
        "navigation" => ["oui",       "peut-etre", "non",     "inconnu"],
        "hardware"   => ["actif",     "instable",  "inactif", "absent"],
        "medical"    => ["present",   "attenue",   "absent",  "inconnu"],
        "humain"     => ["confirmer", "preciser",  "refuser", "demander"],
        "texte"      => ["trouve",    "partiel",   "absent",  "erreur"],
        "science"    => ["valide",    "approx",    "aberrant","manquant"],
        _            => ["CERTAIN",   "PROBABLE",  "BRUME",   "INCONNU"],
    }
}

pub fn executer_ligne(
    ligne:  &str,
    mem:    &mut Memoire,
    ctx:    &mut Contexte,
    lignes: &[&str],
    idx:    &mut usize,
) {
    let indent = ctx.indentation();

    // TARVOS
    if ligne.starts_with("TARVOS") {
        let reste = ligne.trim_start_matches("TARVOS").trim();
        if reste.contains("precision(f64)")  { ctx.tarvos_precision = "f64".to_string(); }
        else if reste.contains("precision(adn)") { ctx.tarvos_precision = "adn".to_string(); }
        else { ctx.tarvos_precision = "f32".to_string(); }
        if let Some(s) = reste.find("contexte(") {
            let after = &reste[s+9..];
            if let Some(e) = after.find(')') {
                ctx.dubi_contexte = after[..e].to_string();
            }
        }
        ctx.tarvos_mode = reste.split_whitespace().next().unwrap_or("AUTO").to_string();
        println!("\x1b[33m[ VERCINTORIX V0.3 ]\x1b[0m");
        println!("\x1b[33mTARVOS {} | precision:{} | contexte:{}\x1b[0m",
            ctx.tarvos_mode, ctx.tarvos_precision, ctx.dubi_contexte);

    // RANN
    } else if ligne.starts_with("RANN") {
        let m = ligne.trim_start_matches("RANN").trim();
        ctx.modules.push(m.to_string());
        println!("{}RANN {} \x1b[95m→ charge\x1b[0m", indent, m);

    // NEMETON
    } else if ligne.starts_with("NEMETON") {
        let nom = ligne.trim_start_matches("NEMETON").trim().trim_end_matches('{').trim();
        if let Some(s) = ligne.find("contexte(") {
            let after = &ligne[s+9..];
            if let Some(e) = after.find(')') {
                ctx.dubi_contexte = after[..e].to_string();
            }
        }
        println!("\x1b[35m{}NEMETON {} {{\x1b[0m", indent, nom);
        ctx.profondeur += 1;

    // Fermeture
    } else if ligne == "}" {
        if ctx.profondeur > 0 { ctx.profondeur -= 1; }
        println!("\x1b[35m{}}}\x1b[0m", ctx.indentation());

    // declarer
    } else if ligne.starts_with("d\u{e9}clarer") || ligne.starts_with("declarer") {
        let reste = ligne
            .trim_start_matches("d\u{e9}clarer")
            .trim_start_matches("declarer")
            .trim();
        let nom = reste.split_whitespace().next().unwrap_or("");
        if nom.is_empty() { return; }
        if ligne.contains('\u{2192}') {
            if let Some((_, droite)) = ligne.split_once('\u{2192}') {
                let val = mem.eval(droite.trim());
                mem.declarer(nom);
                mem.definir(nom, &val);
            }
        } else {
            mem.declarer(nom);
        }

    // definir
    } else if ligne.starts_with("d\u{e9}finir") || ligne.starts_with("definir") {
        if let Some((g, d)) = ligne.split_once('\u{2192}') {
            let nom = g.split_whitespace().last().unwrap_or("");
            let val = mem.eval(d.trim());
            mem.definir(nom, &val);
        }

    // figer
    } else if ligne.starts_with("figer") {
        if let Some((g, d)) = ligne.split_once('\u{2192}') {
            let nom = g.split_whitespace().last().unwrap_or("");
            let val = mem.eval(d.trim());
            mem.figer(nom, &val);
        }

    // effacer
    } else if ligne.starts_with("effacer") {
        let nom = ligne.trim_start_matches("effacer").trim();
        mem.effacer(nom);

    // afficher
    } else if ligne.starts_with("afficher") {
        let reste = ligne.trim_start_matches("afficher").trim();
        let source = if let Some((src, _)) = reste.split_once('\u{2192}') {
            src.trim()
        } else { reste };
        let val = mem.eval(source);
        println!("{}\x1b[32m> {}\x1b[0m", indent, val.trim_matches('"'));

    // fonction
    } else if ligne.starts_with("fonction") {
        let reste = ligne.trim_start_matches("fonction").trim();
        if let Some(po) = reste.find('(') {
            let nom = reste[..po].trim().to_string();
            let apres = &reste[po+1..];
            let params: Vec<String> = if let Some(pc) = apres.find(')') {
                apres[..pc].split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect()
            } else { vec![] };

            let mut bloc: Vec<String> = Vec::new();
            let mut prof = 1;
            while *idx < lignes.len() {
                let l = lignes[*idx].trim();
                *idx += 1;
                if l.ends_with('{') && !l.starts_with('}') { prof += 1; }
                if l == "}" {
                    prof -= 1;
                    if prof == 0 { break; }
                }
                bloc.push(l.to_string());
            }
            println!("{}  \x1b[96mfonction\x1b[0m {} ({} params)", indent, nom, params.len());
            ctx.fonctions.insert(nom, (params, bloc));
        }
        return;

    // appeler
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

            if let Some((params, bloc)) = ctx.fonctions.get(&nom).cloned() {
                for (i, param) in params.iter().enumerate() {
                    let val = args_vals.get(i).cloned().unwrap_or_default();
                    mem.definir(param, &val);
                }
                let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
                let mut sub = 0;
                let mut retour = String::new();
                while sub < refs.len() {
                    let l = refs[sub];
                    if l.starts_with("retourner") {
                        let expr = l.trim_start_matches("retourner").trim();
                        retour = mem.eval(expr);
                        break;
                    }
                    executer_ligne(l, mem, ctx, &refs, &mut sub);
                    sub += 1;
                }
                if !retour.is_empty() {
                    mem.definir("_resultat", &retour);
                    println!("{}  \x1b[96m→\x1b[0m {} = {}", indent, nom, retour);
                } else {
                    println!("{}  \x1b[96m→\x1b[0m {} appelé", indent, nom);
                }
            } else {
                println!("{}  \x1b[31m!\x1b[0m fonction '{}' non définie", indent, nom);
            }
        }
        return;

    // si / sinon
    } else if ligne.starts_with("si ") {
        let cond = ligne.trim_start_matches("si").trim().trim_end_matches('{').trim();
        let res = evaluer_condition(cond, mem);
        println!("{}\x1b[31msi {} → {}\x1b[0m", indent, cond, res);

        let mut bloc_si:    Vec<String> = Vec::new();
        let mut bloc_sinon: Vec<String> = Vec::new();
        let mut prof = 1;
        let mut dans_sinon = false;

        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            *idx += 1;
            let est_sinon = l == "sinon" || l == "sinon {"
                || l == "} sinon {" || l == "} sinon"
                || (l.starts_with('}') && l.contains("sinon"));
            if est_sinon { dans_sinon = true; continue; }
            if l == "}" {
                prof -= 1;
                if prof == 0 { break; }
                continue;
            }
            if l.ends_with('{') { prof += 1; }
            if dans_sinon { bloc_sinon.push(l.to_string()); }
            else          { bloc_si.push(l.to_string()); }
        }

        let bloc = if res { &bloc_si } else { &bloc_sinon };
        let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
        let mut sub = 0;
        while sub < refs.len() {
            executer_ligne(refs[sub], mem, ctx, &refs, &mut sub);
            sub += 1;
        }
        return;

    // repeter
    } else if ligne.starts_with("r\u{e9}p\u{e9}ter") || ligne.starts_with("repeter") {
        let reste = ligne
            .trim_start_matches("r\u{e9}p\u{e9}ter")
            .trim_start_matches("repeter")
            .trim().trim_end_matches('{').trim();
        let fois = mem.eval(reste).parse::<usize>().unwrap_or(0);
        println!("{}\x1b[31mrepeter {} fois\x1b[0m", indent, fois);

        let mut bloc: Vec<String> = Vec::new();
        let mut prof = 1;
        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            *idx += 1;
            if l.ends_with('{') && !l.starts_with('}') { prof += 1; }
            if l == "}" { prof -= 1; if prof == 0 { break; } }
            bloc.push(l.to_string());
        }
        let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
        for _ in 0..fois {
            let mut sub = 0;
            while sub < refs.len() {
                executer_ligne(refs[sub], mem, ctx, &refs, &mut sub);
                sub += 1;
            }
        }
        return;

    // tantque
    } else if ligne.starts_with("tantque") {
        let cond = ligne.trim_start_matches("tantque").trim().trim_end_matches('{').trim();
        println!("{}\x1b[31mtantque {}\x1b[0m", indent, cond);

        let mut bloc: Vec<String> = Vec::new();
        let mut prof = 1;
        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            *idx += 1;
            if l.ends_with('{') && !l.starts_with('}') { prof += 1; }
            if l == "}" { prof -= 1; if prof == 0 { break; } }
            bloc.push(l.to_string());
        }
        let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
        let mut guard = 0;
        while evaluer_condition(cond, mem) && guard < 10000 {
            guard += 1;
            let mut sub = 0;
            while sub < refs.len() {
                executer_ligne(refs[sub], mem, ctx, &refs, &mut sub);
                sub += 1;
            }
        }
        return;

    // DUBI
    } else if ligne.starts_with("DUBI") {
        let nom = ligne.trim_start_matches("DUBI").trim()
            .trim_end_matches('?').trim()
            .split_whitespace().next().unwrap_or("");
        let etat = mem.lire(nom).map(|n| n.etat).unwrap_or(Etat::Brume);
        let labels = labels_dubi(&ctx.dubi_contexte);
        let label = match etat {
            Etat::Certain  => labels[0],
            Etat::Probable => labels[1],
            Etat::Brume    => labels[2],
            Etat::Inconnu  => labels[3],
        };
        println!("{}\x1b[31mDUBI {} → {}\x1b[0m", indent, nom, label);

        let mut branches: Vec<(String, Vec<String>)> = Vec::new();
        let mut branche: Option<String> = None;
        let mut bloc: Vec<String> = Vec::new();

        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            let est_br = labels.iter().any(|lb| l.starts_with(lb))
                || ["CERTAIN","PROBABLE","BRUME","INCONNU"].iter().any(|lb| l.starts_with(lb));

            if !est_br && !l.is_empty() && l != "{" {
                if let Some(b) = branche.take() { branches.push((b, bloc.drain(..).collect())); }
                break;
            }
            *idx += 1;

            if est_br && l.contains('\u{2192}') {
                if let Some(b) = branche.take() { branches.push((b, bloc.drain(..).collect())); }
                if let Some((lb, act)) = l.split_once('\u{2192}') {
                    branches.push((lb.trim().to_string(), vec![act.trim().to_string()]));
                }
            } else if est_br {
                if let Some(b) = branche.take() { branches.push((b, bloc.drain(..).collect())); }
                branche = Some(l.to_string());
            } else if branche.is_some() {
                bloc.push(l.to_string());
            }
        }
        if let Some(b) = branche { branches.push((b, bloc)); }

        for (lb, b) in &branches {
            if lb.trim() == label || lb.contains(label) {
                let refs: Vec<&str> = b.iter().map(|s| s.as_str()).collect();
                let mut sub = 0;
                while sub < refs.len() {
                    executer_ligne(refs[sub], mem, ctx, &refs, &mut sub);
                    sub += 1;
                }
                break;
            }
        }
        return;

    // LUTOS
    } else if ligne.starts_with("LUTOS") {
        let reste = ligne.trim_start_matches("LUTOS").trim().trim_end_matches('{').trim();
        let fois = mem.eval(reste).parse::<usize>().unwrap_or(0);
        let mut bloc: Vec<String> = Vec::new();
        let mut prof = 1;
        while *idx < lignes.len() {
            let l = lignes[*idx].trim();
            *idx += 1;
            if l.ends_with('{') && !l.starts_with('}') { prof += 1; }
            if l == "}" { prof -= 1; if prof == 0 { break; } }
            bloc.push(l.to_string());
        }
        let refs: Vec<&str> = bloc.iter().map(|s| s.as_str()).collect();
        for _ in 0..fois {
            let mut sub = 0;
            while sub < refs.len() {
                executer_ligne(refs[sub], mem, ctx, &refs, &mut sub);
                sub += 1;
            }
        }
        return;

    // GRANN
    } else if ligne.starts_with("GRANN") {
        let nom = ligne.trim_start_matches("GRANN").trim().trim_end_matches('{').trim();
        println!("{}\x1b[36mGRANN {}\x1b[0m", indent, nom);
        mem.declarer(nom);

    // HELIX
    } else if ligne.starts_with("HELIX") {
        let nom = ligne.trim_start_matches("HELIX").trim().trim_end_matches('{').trim();
        println!("{}\x1b[96mHELIX {}\x1b[0m", indent, nom);

    // DRUS
    } else if ligne.starts_with("DRUS") {
        if let Some((src, cib)) = ligne.trim_start_matches("DRUS").trim().split_once('\u{2192}') {
            println!("{}\x1b[33mDRUS {} → {}\x1b[0m", indent, src.trim(), cib.trim());
        }

    // AVON
    } else if ligne.starts_with("AVON") {
        if let Some((src, cib)) = ligne.trim_start_matches("AVON").trim().split_once('\u{2192}') {
            let val = mem.eval(src.trim());
            let nom = cib.trim().split_whitespace().next().unwrap_or("");
            mem.definir(nom, &val);
            println!("{}\x1b[34mAVON {} → {}\x1b[0m", indent, src.trim(), cib.trim());
        }

    // relier
    } else if ligne.starts_with("relier") {
        if let Some((src, cib)) = ligne.trim_start_matches("relier").trim().split_once('\u{2192}') {
            let val = mem.eval(src.trim());
            mem.definir(cib.trim(), &val);
        }

    // Lignes internes ignorées
    } else if ligne.starts_with("ALLELE") || ligne.starts_with("MUTATION")
           || ligne.starts_with("SELECTION") || ligne.starts_with("valence")
           || ligne.starts_with("donnees") || ligne == "{"
           || ligne.starts_with("CERTAIN") || ligne.starts_with("PROBABLE")
           || ligne.starts_with("BRUME")   || ligne.starts_with("INCONNU") {
        // ignoré
    }
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
            > mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
    }
    if let Some((g, d)) = c.split_once('<') {
        return mem.eval(g.trim()).parse::<f64>().unwrap_or(0.0)
            < mem.eval(d.trim()).parse::<f64>().unwrap_or(0.0);
    }
    matches!(mem.eval(c).as_str(), "vrai" | "true" | "CERTAIN" | "1")
}
