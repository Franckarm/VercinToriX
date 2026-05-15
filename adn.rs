// ========================================================
//  VERCINTORIX V0.3 — adn.rs
//  Encodage/décodage ADN natif + Uracile (méta-info)
//  1 codon = 4 bases = 8 bits données + 4 bits marqueurs U
// ========================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Base {
    A = 0b00,
    T = 0b01,
    G = 0b10,
    C = 0b11,
}

impl Base {
    pub fn depuis_bits(b: u8) -> Self {
        match b & 0b11 {
            0b00 => Base::A,
            0b01 => Base::T,
            0b10 => Base::G,
            _    => Base::C,
        }
    }

    pub fn vers_bits(&self) -> u8 { *self as u8 }

    pub fn symbole(&self) -> char {
        match self {
            Base::A => 'A',
            Base::T => 'T',
            Base::G => 'G',
            Base::C => 'C',
        }
    }

    pub fn depuis_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Base::A),
            'T' | 'U' => Some(Base::T),  // U lu comme T en binaire
            'G' => Some(Base::G),
            'C' => Some(Base::C),
            _   => None,
        }
    }
}

// ── Codon — 4 bases + 4 marqueurs Uracile ──────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Codon {
    pub bases:     [Base; 4],
    pub marqueurs: u8,        // 4 bits : 1 = U, 0 = T (ou neutre)
}

impl Codon {
    pub fn depuis_octet(octet: u8) -> Self {
        Codon {
            bases: [
                Base::depuis_bits((octet >> 6) & 0b11),
                Base::depuis_bits((octet >> 4) & 0b11),
                Base::depuis_bits((octet >> 2) & 0b11),
                Base::depuis_bits( octet       & 0b11),
            ],
            marqueurs: 0,
        }
    }

    pub fn vers_octet(&self) -> u8 {
        (self.bases[0].vers_bits() << 6)
        | (self.bases[1].vers_bits() << 4)
        | (self.bases[2].vers_bits() << 2)
        |  self.bases[3].vers_bits()
    }

    /// Marque une position : si la base est T, elle devient U
    pub fn marquer(&mut self, position: usize) {
        if position < 4 && self.bases[position] == Base::T {
            self.marqueurs |= 1 << position;
        }
    }

    /// Retire le marquage Uracile à une position
    pub fn demarquer(&mut self, position: usize) {
        if position < 4 {
            self.marqueurs &= !(1 << position);
        }
    }

    /// True si position contient un U (T marqué)
    pub fn est_marquee(&self, position: usize) -> bool {
        position < 4 && (self.marqueurs & (1 << position)) != 0
    }

    /// Transcription ADN → ARN : tous les T deviennent U
    pub fn transcrire(&mut self) {
        for i in 0..4 {
            if self.bases[i] == Base::T {
                self.marqueurs |= 1 << i;
            }
        }
    }

    /// Rétro-transcription ARN → ADN : tous les U redeviennent T
    pub fn retro_transcrire(&mut self) {
        self.marqueurs = 0;
    }

    /// Compte les U dans le codon (info volatilité)
    pub fn nb_uraciles(&self) -> u32 {
        self.marqueurs.count_ones()
    }

    /// Représentation textuelle : ATGC ou AUGC selon marqueurs
    pub fn to_string(&self) -> String {
        let mut s = String::with_capacity(4);
        for i in 0..4 {
            if self.bases[i] == Base::T && self.est_marquee(i) {
                s.push('U');
            } else {
                s.push(self.bases[i].symbole());
            }
        }
        s
    }
}

// ── Encodage de valeurs en suite de codons ──────────────

/// Encode un i32 en 4 codons (4 octets little-endian)
pub fn encoder_i32(valeur: i32) -> Vec<Codon> {
    valeur.to_le_bytes()
          .iter()
          .map(|&o| Codon::depuis_octet(o))
          .collect()
}

/// Décode 4 codons en i32
pub fn decoder_i32(codons: &[Codon]) -> i32 {
    if codons.len() < 4 { return 0; }
    let octets: [u8; 4] = [
        codons[0].vers_octet(),
        codons[1].vers_octet(),
        codons[2].vers_octet(),
        codons[3].vers_octet(),
    ];
    i32::from_le_bytes(octets)
}

/// Encode un f32 en 4 codons
pub fn encoder_f32(valeur: f32) -> Vec<Codon> {
    valeur.to_le_bytes()
          .iter()
          .map(|&o| Codon::depuis_octet(o))
          .collect()
}

/// Décode 4 codons en f32
pub fn decoder_f32(codons: &[Codon]) -> f32 {
    if codons.len() < 4 { return 0.0; }
    let octets: [u8; 4] = [
        codons[0].vers_octet(),
        codons[1].vers_octet(),
        codons[2].vers_octet(),
        codons[3].vers_octet(),
    ];
    f32::from_le_bytes(octets)
}

/// Encode un texte UTF-8 en codons (1 codon = 1 octet UTF-8)
pub fn encoder_texte(texte: &str) -> Vec<Codon> {
    texte.bytes().map(Codon::depuis_octet).collect()
}

/// Décode des codons en texte UTF-8
pub fn decoder_texte(codons: &[Codon]) -> String {
    let octets: Vec<u8> = codons.iter().map(|c| c.vers_octet()).collect();
    String::from_utf8_lossy(&octets).into_owned()
}

// ── Sérialisation textuelle (pour debug / .vtxm) ───────

/// Convertit une suite de codons en "ATGC·UAAA·..."
pub fn vers_adn_str(codons: &[Codon]) -> String {
    codons.iter()
          .map(|c| c.to_string())
          .collect::<Vec<_>>()
          .join("·")
}

/// Parse "ATGC·UAAA·..." en suite de codons
pub fn depuis_adn_str(s: &str) -> Vec<Codon> {
    s.split('·')
     .filter(|p| !p.is_empty())
     .filter_map(|p| {
         let chars: Vec<char> = p.chars().take(4).collect();
         if chars.len() != 4 { return None; }
         let mut bases = [Base::A; 4];
         let mut marqueurs: u8 = 0;
         for (i, &c) in chars.iter().enumerate() {
             bases[i] = Base::depuis_char(c)?;
             if c == 'U' { marqueurs |= 1 << i; }
         }
         Some(Codon { bases, marqueurs })
     })
     .collect()
}

// ── Adresse chromatique (déjà existant — préservé) ─────
#[derive(Debug, Clone, Copy)]
pub struct AdresseADN {
    pub r: Codon,
    pub g: Codon,
    pub b: Codon,
}

impl AdresseADN {
    pub fn depuis_hex(hex: &str) -> Option<Self> {
        let h = hex.trim_start_matches('#');
        if h.len() != 6 { return None; }
        let r = u8::from_str_radix(&h[0..2], 16).ok()?;
        let g = u8::from_str_radix(&h[2..4], 16).ok()?;
        let b = u8::from_str_radix(&h[4..6], 16).ok()?;
        Some(AdresseADN {
            r: Codon::depuis_octet(r),
            g: Codon::depuis_octet(g),
            b: Codon::depuis_octet(b),
        })
    }

    pub fn vers_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}",
                self.r.vers_octet(),
                self.g.vers_octet(),
                self.b.vers_octet())
    }

    pub fn distance_centre(&self) -> f32 {
        let r = self.r.vers_octet() as f32 / 255.0;
        let g = self.g.vers_octet() as f32 / 255.0;
        let b = self.b.vers_octet() as f32 / 255.0;
        let dr = r - 0.5;
        let dg = g - 0.5;
        let db = b - 0.5;
        (dr*dr + dg*dg + db*db).sqrt()
    }

    pub fn etat(&self) -> &str {
        let d = self.distance_centre();
        if d > 0.60      { "CERTAIN"  }
        else if d < 0.29 { "PROBABLE" }
        else             { "BRUME"    }
    }
}

// ========================================================
//  TYPAGE PAR CODON START (V0.3 — Solution 1)
// ========================================================
//  ATG → i32     (START canonique — méthionine)
//  GTG → f32     (START alternatif réel)
//  TTG → texte   (START rare → chaîne)
//  CTG → f64     (START rare → haute précision)
//  TAA → STOP    (fin de séquence)
// ========================================================

    use Base::*;  // ← DOIT être au niveau module, pas dans un bloc

    pub const START_I32:   [Base; 3] = [A, T, G];
    pub const START_F32:   [Base; 3] = [G, T, G];
    pub const START_TEXTE: [Base; 3] = [T, T, G];
    pub const START_F64:   [Base; 3] = [C, T, G];
    pub const STOP_CODON:  [Base; 3] = [T, A, A];

    fn codon_start(bases: [Base; 3]) -> Codon {
        Codon {
            bases:     [bases[0], bases[1], bases[2], A],
            marqueurs: 0,   // ← C'était ça qui manquait
        }
    }

    pub fn encoder_type_i32(v: i32) -> Vec<Codon> {
        let mut out = vec![codon_start(START_I32)];
        out.extend(encoder_i32(v));
        out
    }

    pub fn encoder_type_f32(v: f32) -> Vec<Codon> {
        let mut out = vec![codon_start(START_F32)];
        out.extend(encoder_f32(v));
        out
    }

    pub fn encoder_type_texte(s: &str) -> Vec<Codon> {
        let mut out = vec![codon_start(START_TEXTE)];
        out.extend(encoder_texte(s));
        out
    }

    pub enum TypeADN {
        I32,
        F32,
        F64,
        Texte,
        Inconnu,
    }

    pub fn detecter_type(codons: &[Codon]) -> TypeADN {
        if codons.is_empty() { return TypeADN::Inconnu; }
        let b = &codons[0].bases;
        if b[0..3] == START_I32   { return TypeADN::I32;   }
        if b[0..3] == START_F32   { return TypeADN::F32;   }
        if b[0..3] == START_TEXTE { return TypeADN::Texte; }
        if b[0..3] == START_F64   { return TypeADN::F64;   }
        TypeADN::Inconnu
    }

    pub fn decoder_auto(codons: &[Codon]) -> String {
        match detecter_type(codons) {
            TypeADN::I32     => decoder_i32(&codons[1..]).to_string(),
            TypeADN::F32     => {
                let f = decoder_f32(&codons[1..]);
                // Supprime les zéros inutiles : 3.14159 pas 3.141590118...
                if f.fract() == 0.0 { format!("{:.1}", f) }
                else { format!("{}", f) }
            },
            TypeADN::Texte   => decoder_texte(&codons[1..]),
            TypeADN::F64     => "[f64 V0.4]".to_string(),
            TypeADN::Inconnu => {
                if codons.len() == 4 { decoder_i32(codons).to_string() }
                else { decoder_texte(codons) }
            }
        }
    }


// ────────────────────────────────────────────────────────
//  TESTS INTÉGRÉS — lance avec : cargo test
// ────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codon_aller_retour_octet() {
        for o in [0x00u8, 0x45, 0x80, 0xFF, 0xAB] {
            let c = Codon::depuis_octet(o);
            assert_eq!(c.vers_octet(), o, "octet {:#x}", o);
        }
    }

    #[test]
    fn test_encoder_decoder_i32() {
        for v in [0, 1, 45, -1, 12345, i32::MAX, i32::MIN] {
            let codons = encoder_i32(v);
            assert_eq!(codons.len(), 4);
            assert_eq!(decoder_i32(&codons), v, "i32 {}", v);
        }
    }

    #[test]
    fn test_encoder_decoder_f32() {
        for v in [0.0f32, 1.5, -3.14, 100.25] {
            let codons = encoder_f32(v);
            assert_eq!(decoder_f32(&codons), v, "f32 {}", v);
        }
    }

    #[test]
    fn test_encoder_decoder_texte() {
        for s in ["hello", "VercinToriX", "ATGC", "⚔️🧬"] {
            let codons = encoder_texte(s);
            assert_eq!(decoder_texte(&codons), s, "texte {}", s);
        }
    }

    #[test]
    fn test_adn_str_aller_retour() {
        let v = 45i32;
        let codons = encoder_i32(v);
        let s = vers_adn_str(&codons);
        let codons2 = depuis_adn_str(&s);
        assert_eq!(decoder_i32(&codons2), v);
        println!("45 → {}", s);
    }

    #[test]
    fn test_uracile_transcription() {
        let mut c = Codon::depuis_octet(0x45);  // contient des T (01)
        let avant = c.to_string();
        c.transcrire();
        let apres = c.to_string();
        assert_ne!(avant, apres, "transcription doit changer T en U");
        // Valeur binaire identique
        assert_eq!(c.vers_octet(), 0x45);
        // Rétro-transcription
        c.retro_transcrire();
        assert_eq!(c.to_string(), avant);
    }

    #[test]
    fn test_uracile_marquage_individuel() {
        let mut c = Codon::depuis_octet(0b01010101); // TTTT
        assert_eq!(c.to_string(), "TTTT");
        c.marquer(0);
        c.marquer(2);
        assert_eq!(c.to_string(), "UTUT");
        assert!(c.est_marquee(0));
        assert!(!c.est_marquee(1));
        assert_eq!(c.nb_uraciles(), 2);
    }

    #[test]
    fn test_uracile_serialisation() {
        let mut c = Codon::depuis_octet(0b01010101);
        c.transcrire();
        let s = vers_adn_str(&[c]);
        assert_eq!(s, "UUUU");
        let c2 = &depuis_adn_str(&s)[0];
        assert_eq!(c2.vers_octet(), c.vers_octet());
        assert_eq!(c2.nb_uraciles(), 4);
    }

    #[test]
    fn test_hex_aller_retour() {
        for hex in ["#FF0000","#00FF00","#0000FF",
                    "#FFFFFF","#000000","#808080","#C00000"] {
            let a = AdresseADN::depuis_hex(hex).unwrap();
            assert_eq!(a.vers_hex(), hex);
        }
    }

    #[test]
    fn test_pipeline_complet_45() {
        // Le test que tu voulais faire en .vtx
        let valeur = 45i32;
        let codons = encoder_i32(valeur);
        let adn = vers_adn_str(&codons);
        println!("\n  45 encodé en ADN : {}", adn);
        let codons2 = depuis_adn_str(&adn);
        let restitue = decoder_i32(&codons2);
        assert_eq!(restitue, 45);
        println!("  Restitué : {}\n", restitue);
    }
}