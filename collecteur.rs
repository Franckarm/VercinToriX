// ========================================================
//  VERCINTORIX V0.3 — collecteur.rs
//  Collecte de blocs — convention unifiée
// ========================================================

/// Collecte les lignes d'un bloc { ... }
///
/// Convention :
///   EN ENTRÉE  — idx pointe sur la ligne APRÈS le {
///   EN SORTIE  — idx pointe sur le } fermant
///   L'appelant fait +1 pour passer après le }
///
/// Gère les blocs imbriqués correctement
pub fn collecter_bloc(lignes: &[String], idx: &mut usize) -> Vec<String> {
    let mut bloc: Vec<String> = Vec::new();
    let mut prof = 1;

    while *idx < lignes.len() {
        let l = lignes[*idx].trim().to_string();

        // Commentaires
        if l.is_empty() || l.starts_with('←') || l.starts_with("//") {
            *idx += 1;
            continue;
        }

        // } sinon { — s'arrêter SANS avancer (l'appelant gère)
        if l.starts_with('}') && l.contains("sinon") {
            break;
        }

        let ouvre = l.chars().filter(|&c| c == '{').count();
        let ferme = l.chars().filter(|&c| c == '}').count();

        // } seul — fin du bloc
        if ferme > 0 && prof <= ferme {
            break; // idx reste sur le }
        }

        prof += ouvre;
        prof -= ferme;
        bloc.push(l);
        *idx += 1;
    }

    bloc
}

// collecteur.rs — dans collecter_bloc
// JUSTE AVANT le comptage { et }

/// Compte les { dans une ligne
fn compte_ouvertures(l: &str) -> usize {
    l.chars().filter(|&c| c == '{').count()
}

/// Compte les } dans une ligne
fn compte_fermetures(l: &str) -> usize {
    l.chars().filter(|&c| c == '}').count()
}

/// Exécuteur interne de bloc — boucle sécurisée
/// Utilisé par NEMETON, LUTOS, pourChaque, si, PAR...
pub fn executer_bloc<F>(bloc: &[String], mut executeur: F)
where
    F: FnMut(&str, &[String], &mut usize),
{
    let mut sub = 0usize;
    while sub < bloc.len() {
        let l = bloc[sub].clone();
        let sub_avant = sub;
        executeur(&l, bloc, &mut sub);
        if sub == sub_avant {
            sub += 1;
        } else {
            sub += 1;
        }
    }
}
