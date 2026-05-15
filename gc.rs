// ========================================================
//  VERCINTORIX V0.3 — gc.rs
//  Garbage Collector trinaire
//  CERTAIN  → conservé intact
//  PROBABLE → compressé (centroïde du cluster)
//  BRUME    → libéré après timeout
// ========================================================

use std::time::{Instant, Duration};
use crate::adn::AdresseADN;

#[derive(Debug, Clone)]
pub struct NoeudGC {
    pub nom:       String,
    pub adresse:   AdresseADN,
    pub valeur:    String,
    pub naissance: Instant,
    pub acces:     Instant,
    pub timeout:   Duration,
}

impl NoeudGC {
    pub fn nouveau(nom: &str, adresse: AdresseADN,
                   valeur: &str, timeout_secs: u64) -> Self {
        let now = Instant::now();
        NoeudGC {
            nom:       nom.to_string(),
            adresse,
            valeur:    valeur.to_string(),
            naissance: now,
            acces:     now,
            timeout:   Duration::from_secs(timeout_secs),
        }
    }

    pub fn toucher(&mut self) {
        self.acces = Instant::now();
    }

    pub fn age(&self) -> Duration {
        self.acces.elapsed()
    }

    pub fn etat(&self) -> &str {
        self.adresse.etat()
    }

    pub fn est_expire(&self) -> bool {
        self.age() > self.timeout
    }
}

// ── GC principal ────────────────────────────────────────
pub struct GCTrinaire {
    pub noeuds:      Vec<NoeudGC>,
    pub timeout:     Duration,
    pub liberes:     usize,
    pub comprimes:   usize,
}

impl GCTrinaire {
    pub fn new(timeout_secs: u64) -> Self {
        GCTrinaire {
            noeuds:    Vec::new(),
            timeout:   Duration::from_secs(timeout_secs),
            liberes:   0,
            comprimes: 0,
        }
    }

    pub fn inserer(&mut self, nom: &str,
                   adresse: AdresseADN, valeur: &str) {
        // Remplacer si existe
        if let Some(n) = self.noeuds.iter_mut()
                             .find(|n| n.nom == nom) {
            n.valeur  = valeur.to_string();
            n.adresse = adresse;
            n.toucher();
            return;
        }
        self.noeuds.push(
            NoeudGC::nouveau(nom, adresse, valeur,
                             self.timeout.as_secs())
        );
    }

    pub fn lire(&mut self, nom: &str) -> Option<String> {
        if let Some(n) = self.noeuds.iter_mut()
                             .find(|n| n.nom == nom) {
            n.toucher();
            Some(n.valeur.clone())
        } else {
            None
        }
    }

    // Cycle GC — appelé périodiquement
    pub fn cycle(&mut self) -> RapportGC {
        let mut liberes    = Vec::new();
        let mut comprimes  = Vec::new();
        let mut conserves  = Vec::new();

        for noeud in &self.noeuds {
            match noeud.etat() {
                "CERTAIN" => {
                    conserves.push(noeud.nom.clone());
                }
                "PROBABLE" => {
                    if noeud.est_expire() {
                        comprimes.push(noeud.nom.clone());
                    } else {
                        conserves.push(noeud.nom.clone());
                    }
                }
                _ => { // BRUME
                    if noeud.est_expire() {
                        liberes.push(noeud.nom.clone());
                    } else {
                        conserves.push(noeud.nom.clone());
                    }
                }
            }
        }

        // Appliquer — supprimer libérés
        self.noeuds.retain(|n| {
            !liberes.contains(&n.nom)
        });

        // Comprimer PROBABLE expirés
        // (ici : on marque, future version → centroïde)
        for nom in &comprimes {
            if let Some(n) = self.noeuds.iter_mut()
                                 .find(|n| &n.nom == nom) {
                n.valeur = format!("~{}", n.valeur);
            }
        }

        self.liberes   += liberes.len();
        self.comprimes += comprimes.len();

        RapportGC {
            liberes,
            comprimes,
            conserves,
            total_liberes:   self.liberes,
            total_comprimes: self.comprimes,
        }
    }

    pub fn afficher_etat(&self) {
        println!("\x1b[33m[ GC Trinaire — {} noeuds ]\x1b[0m",
                 self.noeuds.len());
        for n in &self.noeuds {
            let age = n.age().as_secs();
            println!("  {} | {} | {} | {}s | {}",
                n.nom,
                n.adresse.vers_hex(),
                n.etat(),
                age,
                n.valeur
            );
        }
    }
}

pub struct RapportGC {
    pub liberes:         Vec<String>,
    pub comprimes:       Vec<String>,
    pub conserves:       Vec<String>,
    pub total_liberes:   usize,
    pub total_comprimes: usize,
}
