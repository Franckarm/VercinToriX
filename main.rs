#![allow(dead_code, unused_variables, unused_imports)]
// ========================================================
//  VERCINTORIX V0.3 — main.rs
//  Point d'entree — connecte memoire.rs + executeur.rs
//
//  Usage : cargo run -- programme.vtx
//  Build : cargo build --release
// ========================================================

mod memoire;
mod executeur;

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
            eprintln!("\x1b[31m[VTX-001] Fichier introuvable : {}\x1b[0m", chemin);
            eprintln!("\x1b[31m          Erreur : {}\x1b[0m", e);
            eprintln!("\x1b[33mAstuce : cargo run -- ..\\mon_programme.vtx\x1b[0m");
            return;
        }
    };

    // Filtrer lignes vides et commentaires
    let lignes: Vec<&str> = contenu
        .lines()
        .map(|l| l.trim())
        .filter(|l| {
            !l.is_empty()
            && !l.starts_with('\u{2190}')
        })
        .collect();

    if lignes.is_empty() {
        println!("\x1b[33m[ Fichier vide ]\x1b[0m");
        return;
    }

    let mut mem = Memoire::new();
    let mut ctx = Contexte::new();
    let mut idx = 0;

    while idx < lignes.len() {
        let ligne = lignes[idx];
        idx += 1;
        executer_ligne(ligne, &mut mem, &mut ctx, &lignes, &mut idx);
    }

    println!("\n\x1b[33m[ \u{2713} termine ]\x1b[0m");
}

fn afficher_aide() {
    println!("\x1b[33m[ VERCINTORIX V0.3 ]\x1b[0m");
    println!("Usage : cargo run -- programme.vtx");
    println!("Build : cargo build --release");
    println!();
    println!("Primitives : TARVOS NEMETON GRANN CORIO HELIX AVON DUBI LUTOS DRUS RANN PAR GWEL ADBERT");
    println!("Francais   : declarer definir figer afficher si sinon tantque repeter fonction");
}
