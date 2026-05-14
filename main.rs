#![allow(dead_code, unused_variables, unused_imports)]
mod memoire;
mod collecteur;
mod cube;
mod executeur;
mod adn;
mod gc;

use std::env;
use std::fs;
use memoire::Memoire;
use executeur::{Contexte, executer_ligne};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        afficher_aide();
        return;
    }

    let chemin = &args[1];

    let contenu = match fs::read_to_string(chemin) {
        Ok(c)  => c,
        Err(e) => {
            eprintln!("[VTX-001] Fichier introuvable : {}", chemin);
            eprintln!("          Erreur : {}", e);
            return;
        }
    };

    let lignes: Vec<String> = contenu
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('\u{2190}'))
            .collect();

        let mut mem = Memoire::new();
        let mut ctx = Contexte::new();
        let mut idx = 0;

        while idx < lignes.len() {
            let ligne = lignes[idx].clone();
            let avant = idx;
            executer_ligne(&ligne, &mut mem, &mut ctx, &lignes, &mut idx);
            // Si executer_ligne n'a pas avancé idx, on avance nous-mêmes
            if idx == avant {
                idx += 1;
            }
            // Sécurité contre débordement
            if idx > lignes.len() {
                break;
            }
        }

        println!("\n[ termine ]");
}

        

fn afficher_aide() {
    println!("VERCINTORIX V0.3");
    println!("Usage : cargo run -- programme.vtx");
}