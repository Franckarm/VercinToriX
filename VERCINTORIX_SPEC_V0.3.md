bash

cat > /mnt/user-data/outputs/VERCINTORIX_SPEC_V0.3.md << 'SPEC'
# VERCINTORIX — Spécification Officielle V0.3
## Corrections et nouveautés par rapport à V0.2

**Ce fichier complète V0.2 — ne remplace pas.**
**Toutes les primitives V0.2 restent valides et rétrocompatibles.**

---

## Résumé des changements V0.3

| Changement | Description |
|------------|-------------|
| HELIX redéfini | Réservé aux comportements variants et évolutifs |
| DUBI enrichi | Seuils numériques + sinon + contextes multiples |
| AVON enrichi | RECEVOIR et FOURNIR — entrées et sorties explicites |
| DRUS affiné | 3 modes : disque / matériel / système |
| PAR | Nouvelle primitive — parallélisme structurel |
| GWEL | Nouvelle primitive — miroir et réflexion |
| ADBERT | Nouvelle primitive — symétrie et centre |
| ADN en premier plan | Adressage natif du cube — hex comme raccourci |
| TARVOS enrichi | précision + gc(trinaire) + mode(2D/3D) + plan |
| Carré chromatique | Mode 2D avec plan et fond choisis |
| Contextes DUBI | Vocabulaires dynamiques par domaine |

---

## 1. TARVOS — version complète V0.3

```vercintorix
← Précision chromatique
TARVOS AUTO précision(f32)    ← 16 777 216 couleurs — défaut standard
TARVOS AUTO précision(f64)    ← 4 294 967 296 — CAO, médical, simulation
TARVOS AUTO précision(adn)    ← 656 trillions — quantique, génomique

← GC trinaire — gestion mémoire intelligente
TARVOS AUTO gc(trinaire) timeout(30s)
← BRUME depuis timeout  → libéré automatiquement
← PROBABLE              → compressé en mémoire.cube
← CERTAIN               → conservé intact en mémoire.vive

← Contexte DUBI global
TARVOS AUTO contexte(calcul)       ← défaut
TARVOS AUTO contexte(navigation)
TARVOS AUTO contexte(hardware)
TARVOS AUTO contexte(médical)
TARVOS AUTO contexte(humain)
TARVOS AUTO contexte(texte)
TARVOS AUTO contexte(science)

← Adressage du cube chromatique
TARVOS AUTO adressage(adn)    ← natif — machine
TARVOS AUTO adressage(hex)    ← raccourci humain — défaut
TARVOS AUTO adressage(coords) ← coordonnées flottantes

← Mode géométrique
TARVOS AUTO mode(3D)          ← cube chromatique complet — défaut
TARVOS AUTO mode(2D) plan(RG) fond(noir)    ← carré chromatique
TARVOS AUTO mode(2D) plan(RB) fond(rouge)
TARVOS AUTO mode(2D) plan(GB) fond(bleu)

← Combinaisons
TARVOS AUTO précision(f32) contexte(hardware) adressage(hex)
TARVOS AUTO précision(f64) contexte(médical)  adressage(adn)
TARVOS AUTO précision(f32) mode(2D) plan(RG) fond(noir)
TARVOS AUTO précision(adn) mode(3D) gc(trinaire) timeout(60s)
```

### Précision f64 — quand l'utiliser

```
f32 → défaut — jeux, 3D standard, IA légère, visualisation
f64 → CAO industrielle, simulation physique longue durée,
       imagerie médicale, astronomie, finance haute précision
adn → génomique, cryptographie, IA quantique, compression ultra-dense
```

---

## 2. Le cube chromatique — espace fondamental

### 2.1 Architecture

Le cube chromatique est l'espace de coordonnées de VERCINTORIX.
Chaque position (r, g, b) dans [0,1]³ est simultanément
une coordonnée ET une couleur ET une adresse ADN.

```
Position (r, g, b) = coordonnée spatiale
                   = couleur RGB
                   = codon ADN sur 3 axes
                   = adresse mémoire dans mémoire.cube
```

### 2.2 Les 8 sommets — adresses fondamentales

| Sommet | r | g | b | Hex | ADN natif | Paire complémentaire |
|--------|---|---|---|-----|-----------|---------------------|
| Noir | 0 | 0 | 0 | #000000 | A0A0A0A0·A0A0A0A0·A0A0A0A0 | Blanc |
| Rouge | 1 | 0 | 0 | #FF0000 | T3T3T3T3·A0A0A0A0·A0A0A0A0 | Cyan |
| Vert | 0 | 1 | 0 | #00FF00 | A0A0A0A0·T3T3T3T3·A0A0A0A0 | Magenta |
| Bleu | 0 | 0 | 1 | #0000FF | A0A0A0A0·A0A0A0A0·T3T3T3T3 | Jaune |
| Jaune | 1 | 1 | 0 | #FFFF00 | T3T3T3T3·T3T3T3T3·A0A0A0A0 | Bleu |
| Cyan | 0 | 1 | 1 | #00FFFF | A0A0A0A0·T3T3T3T3·T3T3T3T3 | Rouge |
| Magenta | 1 | 0 | 1 | #FF00FF | T3T3T3T3·A0A0A0A0·T3T3T3T3 | Vert |
| Blanc | 1 | 1 | 1 | #FFFFFF | T3T3T3T3·T3T3T3T3·T3T3T3T3 | Noir |

**T = FF = blanc = saturation totale — cohérence physique et biologique.**

### 2.3 Les paires complémentaires naturelles

```
noir    ↔ blanc      ← contraste maximum — axe diagonal principal
rouge   ↔ cyan       ← complémentaires sur axe R
vert    ↔ magenta    ← complémentaires sur axe G
bleu    ↔ jaune      ← complémentaires sur axe B
```

Ces paires sont des propriétés géométriques du cube — pas des choix arbitraires.

### 2.4 Adressage natif ADN

```vercintorix
← L'ADN est l'adressage natif — le hex est le raccourci humain

← Adresse ADN complète (machine)
figer gris_centre → ADN.rgb(
    axeR: A2,A0,A0,A0    ← 10 00 00 00 = 128/255
    axeG: A2,A0,A0,A0
    axeB: A2,A0,A0,A0
)

← Raccourci hex équivalent (humain)
figer gris_centre → #808080

← Les deux désignent le même point dans le cube
← VERCINTORIX convertit automatiquement selon adressage(...)
```

### 2.5 Conversion hex → ADN

```
#RRGGBB → chaque canal 0-255 → binaire 8 bits → 4 groupes de 2 bits → 4 bases ADN

Exemple : #FF8800
  R = FF = 255 = 11111111 → A3,A3,A3,A3  ← mais T3=FF plus court
  G = 88 = 136 = 10001000 → A2,A0,A2,A0
  B = 00 =   0 = 00000000 → A0,A0,A0,A0

Notation courte : #RGB(T3·A2A0A2A0·A0)
```

---

## 3. Le carré chromatique — mode 2D

### 3.1 Principe

En mode 2D, on travaille sur une **face du cube**.
Le cube a 6 faces — chacune est un carré chromatique défini
par deux axes et une valeur fixe sur le troisième.

### 3.2 Les 6 faces disponibles

| Plan | Axe fixe | Fond noir | Sommets | Usage naturel |
|------|----------|-----------|---------|---------------|
| RG | B=0 | noir→rouge→jaune→vert | chaud | interface, dessin |
| RG | B=1 | bleu→magenta→blanc→cyan | froid | ciel, eau |
| RB | G=0 | noir→rouge→magenta→bleu | vif | contraste |
| RB | G=1 | vert→jaune→blanc→cyan | lumineux | nature |
| GB | R=0 | noir→vert→cyan→bleu | froid | tech, sci-fi |
| GB | R=1 | rouge→jaune→blanc→magenta | chaud | feu, énergie |

### 3.3 Déclaration

```vercintorix
← Mode 2D — choisir le plan et le fond
TARVOS AUTO mode(2D) plan(RG) fond(noir)
← Carré actif : noir(0,0) → rouge(1,0) → jaune(1,1) → vert(0,1)

TARVOS AUTO mode(2D) plan(GB) fond(bleu)
← Carré actif : bleu(0,0) → cyan(0,1) → blanc(1,1) → magenta(1,0)

← Fond et premier plan sont des complémentaires naturels
TARVOS AUTO mode(2D) plan(RG) fond(noir)    ← blanc est le premier plan
TARVOS AUTO mode(2D) plan(RB) fond(rouge)   ← cyan est le premier plan
```

### 3.4 Adressage 2D

```vercintorix
← Point 2D dans le carré actif
figer point_a → #8080     ← X=128, Y=128 dans le plan actif
figer point_b → (0.5, 0.5) ← coordonnées directes

← ADN sur 2 axes
figer point_c → ADN.plan(
    axe1: A2,A0,A0,A0    ← 128 sur axe 1
    axe2: A3,G2,C1,T0    ← valeur sur axe 2
)
```

---

## 4. AVON — enrichissement V0.3

### Principe

```
AVON source → dest          ← flux interne (V0.2 — inchangé)
AVON RECEVOIR ext → var     ← entrée depuis le monde extérieur
AVON FOURNIR val → ext      ← sortie vers système actif
```

### Philosophie des flux

```
AVON          = la rivière — circulation interne
AVON RECEVOIR = la source  — entrée depuis l'extérieur
AVON FOURNIR  = l'embouchure — sortie vers système actif
afficher      = la parole  — sortie humaine
DRUS          = l'action   — effet physique sur le monde
```

### Syntaxe complète

```vercintorix
← Flux interne
AVON calcul → résultat
AVON position #IR → analyse #UV     ← transformation spectrale

← Entrées externes
AVON RECEVOIR capteur.température → temp_int
AVON RECEVOIR gps.position        → position
AVON RECEVOIR utilisateur.consigne → consigne
AVON RECEVOIR api.données          → données_réseau

← Sorties vers systèmes actifs
AVON FOURNIR qualité_vidéo → moteur.rendu
AVON FOURNIR puissance     → système(chauffage)
AVON FOURNIR score         → interface.hud
```

---

## 5. DRUS — 3 modes officiels V0.3

```
afficher  → l'humain lit        (console, écran, log)
DRUS      → une machine reçoit  (disque, matériel, système)
```

### Mode 1 — disque

```vercintorix
DRUS données       → disque("résultat.obj")
DRUS données       → disque("résultat.obj") qualité(haute)
DRUS scène         → disque("scène.vtxm")
DRUS export        → disque("données.json")
```

### Mode 2 — matériel

```vercintorix
DRUS signal        → matériel(arduino.pin(13)) état(HAUT)
DRUS données       → matériel(série("COM3"))
DRUS forme         → matériel(impression3D)
DRUS tracé         → matériel(laser) #UV
DRUS tracé         → matériel(laser) #IR·UV
```

### Mode 3 — système

```vercintorix
DRUS puissance     → système(actionneur.chauffage.pourcentage)
DRUS qualité       → système(moteur.video.qualité)
DRUS position      → système(moteur.physique.joueur)
DRUS données       → système(api.endpoint.envoyer())
```

### Règle DUBI avant DRUS

```vercintorix
DUBI données ?
    CERTAIN  → DRUS données → disque("résultat.obj")
    PROBABLE → DRUS données → disque("brouillon.obj")
    BRUME    → afficher "Export annulé" → écran
    INCONNU  → afficher "Données manquantes" → écran
```

---

## 6. HELIX — redéfinition V0.3

### Définition officielle

HELIX est l'opérateur de **comportements variants et évolutifs**.

```
Utiliser HELIX quand :
  ✓ Évolution, variabilité, expérimentation
  ✓ Changement de variante dynamique
  ✓ Plusieurs stratégies cohérentes
  ✓ Aléatoire ou adaptation souhaitée
  ✓ IA, génération procédurale, simulation

Ne pas utiliser HELIX quand :
  ✗ Choix de configuration fixe
  ✗ Pas de besoin d'évolution
  ✗ Un DUBI ou si/sinon suffit
```

### Syntaxe

```vercintorix
← Sélection automatique
HELIX nom {
    ALLELE label_1 → action_1
    ALLELE label_2 → action_2
    ALLELE label_3 → action_3
    MUTATION 0.30
    SELECTION auto
}

← Sélection via codon ADN
HELIX nom {
    ALLELE label_1 → action_1
    ALLELE label_2 → action_2
    MUTATION 0.05
    SELECTION adn(A1,G0,C2,T1)
}

← Appel direct
appeler nom.sélection(label_2)
```

---

## 7. DUBI — enrichissement V0.3

### Seuils numériques

```vercintorix
DUBI fps contexte(hardware) ?
    excellent  (> 55) → définir qualité → "Ultra"
    bon        (40-55)→ définir qualité → "Élevée"
    acceptable (25-40)→ définir qualité → "Standard"
    faible     (15-25)→ définir qualité → "Économique"
    critique   (< 15) → définir qualité → "Minimum"
```

### sinon après DUBI

```vercintorix
DUBI consigne contexte(humain) ?
    confirmer → définir mode → "manuel"
    refuser   → définir mode → "automatique"
sinon {
    définir mode → "défaut"
}
```

### Vocabulaires par contexte

| Contexte | Label 1 | Label 2 | Label 3 | Label 4 |
|----------|---------|---------|---------|---------|
| calcul (défaut) | CERTAIN | PROBABLE | BRUME | INCONNU |
| navigation | oui | peut-être | non | inconnu |
| hardware | actif | instable | inactif | absent |
| médical | présent | atténué | absent | inconnu |
| humain | confirmer | préciser | refuser | demander |
| texte | trouvé | partiel | absent | erreur |
| science | valide | approximé | aberrant | manquant |

### Contexte personnalisé

```vercintorix
NEMETON Jeu contexte(personnalisé: gagner/perdre/égalité/abandon) {
    DUBI résultat ?
        gagner  → ajouter(point_victoire)
        perdre  → ajouter(point_défaite)
        égalité → partager(points)
        abandon → annuler(partie)
}
```

---

## 8. PAR — Parallélisme structurel (nouveau V0.3)

PAR est la primitive d'**exécution parallèle**.
Les blocs à l'intérieur s'exécutent simultanément.

```vercintorix
← Parallélisme libre
PAR {
    calculer(physique)  → résultat_physique
    calculer(rendu)     → résultat_rendu
    calculer(audio)     → résultat_audio
}

← Parallélisme synchronisé — attend que tous finissent
PAR synchronisé {
    répéter 8 axe(R) pas(0.125) → couche_r
    répéter 8 axe(G) pas(0.125) → couche_g
    répéter 8 axe(B) pas(0.125) → couche_b
}
← Ici les 3 colonnes sont toutes générées
définir réseau → géométrie.croiser(couche_r, couche_g, couche_b)
```

---

## 9. GWEL — Miroir et Réflexion (nouveau V0.3)

GWEL (gallois/breton : *voir, reflet*) — miroir dans le cube chromatique.

### 5 façons de désigner le point de référence

```vercintorix
← 1. Nom sémantique
GWEL forme miroir(neutre)           ← centre #808080

← 2. Point spécial nommé
GWEL forme miroir(rouge_pur)
GWEL forme miroir(blanc)
GWEL forme miroir(noir)

← 3. Hex — raccourci humain (mode privilégié)
GWEL forme miroir(#808080)
GWEL forme miroir(#FF0000)

← 4. Coordonnées directes
GWEL forme miroir(0.5, 0.5, 0.5)

← 5. Variable Point4D
déclarer ref → géométrie.point4D(0.2, 0.8, 0.4, #IR)
GWEL forme miroir(ref)
```

### Options supplémentaires

```vercintorix
← Sur axe chromatique
GWEL forme miroir(axe: R)       ← réflexion sur plan YZ
GWEL forme miroir(axe: G)       ← réflexion sur plan XZ
GWEL forme miroir(axe: B)       ← réflexion sur plan XY

← Sur plan chromatique
GWEL forme miroir(plan: RG)
GWEL forme miroir(plan: RGB)
```

---

## 10. ADBERT — Symétrie et Centre (nouveau V0.3)

ADBERT (gaulois : *ce qui revient au centre*) — symétrie dans le cube.

### 5 façons de désigner le centre

Identiques à GWEL — même cohérence :

```vercintorix
← 1. Nom sémantique
ADBERT forme centre(neutre)

← 2. Point spécial
ADBERT forme centre(blanc)
ADBERT forme centre(noir)

← 3. Hex — raccourci humain
ADBERT forme centre(#808080)
ADBERT forme centre(#FF8800)

← 4. Coordonnées
ADBERT forme centre(0.5, 0.5, 0.5)
ADBERT forme centre(0.7, 0.3, 0.8)

← 5. Variable
déclarer ref → géométrie.point4D(0.2, 0.8, 0.4, #IR)
ADBERT forme centre(ref)
```

### Options supplémentaires

```vercintorix
ADBERT forme axe(R)             ← symétrie axiale
ADBERT forme axes(R, G)         ← double symétrie
ADBERT forme complet            ← 3 axes = 8 copies
```

### Exemple complet — modélisation symétrique

```vercintorix
RANN géométrie

NEMETON TestSymétrie {
    déclarer centre_cube → géométrie.centre()
    déclarer forme → géométrie.cube(rayon: 0.3)

    ← Par nom
    GWEL   forme miroir(neutre)
    ADBERT forme centre(neutre)

    ← Par hex
    GWEL   forme miroir(#808080)
    ADBERT forme centre(#FF8800)

    ← Par coordonnées
    GWEL   forme miroir(0.5, 0.5, 0.5)
    ADBERT forme centre(0.7, 0.3, 0.8)

    ← Par variable
    déclarer pt → géométrie.point4D(0.2, 0.8, 0.4, #IR)
    GWEL   forme miroir(pt)
    ADBERT forme centre(pt)
}

← Voiture — modélisation par symétrie
NEMETON ModèleVoiture {
    déclarer demi en mémoire.cube #IR
    définir  demi → géométrie.modeler(côté_gauche)

    ← Miroir sur axe R → carrosserie complète
    GWEL demi miroir(axe: R)

    DRUS demi → disque("voiture.obj")
}

← Château — symétrie complète
NEMETON ChâteauSymétrique {
    déclarer quart en mémoire.cube #IR
    définir  quart → géométrie.modeler(quart_nord_est)

    GWEL   quart miroir(axe: R)
    ADBERT quart axes(R, B)

    DRUS quart → disque("château.obj")
}
```

---

## 11. Système ADN quadrinaire — premier plan

### Les 4 bases avec 4 états chacune

| Base | 0 | 1 | 2 | 3 | Sens interne |
|------|---|---|---|---|-------------|
| A | 00 | 01 | 10 | 11 | CERTAIN |
| G | 00 | 01 | 10 | 11 | PROBABLE |
| C | 00 | 01 | 10 | 11 | BRUME |
| T | 00 | 01 | 10 | 11 | INCONNU / FF chromatique |

### Mode pur — calcul

```vercintorix
figer clé → ADN.pur(A1, G2, C0, T3)   ← 01 10 00 11 = 99
figer clé → #ADN(A1G2C0T3)             ← notation courte
```

### Mode chromatique — cube RGB

```vercintorix
figer couleur → ADN.rgb(
    axeR: A0,G1,C3,T1
    axeG: T1,A2,G0,C2
    axeB: G3,C1,A2,T0
)
figer couleur → #RGB(A0G1C3T1 · T1A2G0C2 · G3C1A2T0)
```

### Hex comme raccourci officiel

```vercintorix
← Ces deux lignes sont équivalentes
GWEL forme miroir(#808080)
GWEL forme miroir(ADN.rgb(A2A0A0A0 · A2A0A0A0 · A2A0A0A0))

← Le hex est le pont naturel entre humain et ADN machine
← #RRGGBB = 3 octets = 3 codons ADN de 4 bases chacun
```

### Correspondance mathématique

```
1 codon  (4 bases)  = 256 états      = 8 bits  → f32
3 codons (12 bases) = 16 777 216     = 24 bits  → f32 RGB complet
4 codons (16 bases) = 4 294 967 296  = 32 bits  → f64
Superposition IR·UV = ~656 trillions            → adn
```

### Table de codons VERCINTORIX

```
ATG → START      — initialisation
TAA → STOP       — fin de séquence
GCT → stable     — maintien
CGG → fort       — renforcement liaison triple
TGG → adaptatif  — HELIX actif
CCC → brume      — DUBI BRUME
AAA → répétition — LUTOS
TTT → saturation — blanc chromatique T=FF
GGG → neutre     — origine cube (0,0,0)
CGA → transition — AVON flux
AGC → mémoire    — stockage mémoire.cube
```

---

## 12. Primitives V0.3 — tableau complet

| Primitive | Nature | Rôle | Statut |
|-----------|--------|------|--------|
| TARVOS | Gaulois | Env + précision + gc + mode | enrichi V0.3 |
| NEMETON | Gaulois | Bloc + contexte local | enrichi V0.3 |
| GRANN | Gaulois | Brique universelle | V0.2 |
| CORIO | Gaulois | Groupe structuré | V0.2 |
| HELIX | Gaulois | Comportement variant évolutif | redéfini V0.3 |
| AVON | Gaulois | Flux + RECEVOIR + FOURNIR | enrichi V0.3 |
| DUBI | Gaulois | Décision contextuelle + seuils | enrichi V0.3 |
| LUTOS | Gaulois | Répétition | V0.2 |
| DRUS | Gaulois | Sortie machine — 3 modes | affiné V0.3 |
| RANN | Gaulois | Import module | V0.2 |
| PAR | Gaulois | Parallélisme structurel | nouveau V0.3 |
| GWEL | Gallois | Miroir / Réflexion | nouveau V0.3 |
| ADBERT | Gaulois | Symétrie / Centre | nouveau V0.3 |
| déclarer | Français | Créer variable | V0.2 |
| définir | Français | Assigner valeur | V0.2 |
| figer | Français | Valeur immuable | V0.2 |
| effacer | Français | Supprimer | V0.2 |
| relier | Français | Flux simple | V0.2 |
| afficher | Français | Sortie humaine | V0.2 |
| lire | Français | Entrée | V0.2 |
| retourner | Français | Valeur de retour | V0.2 |
| appeler | Français | Appel fonction | V0.2 |
| fonction | Français | Définir fonction | V0.2 |
| si / sinon | Français | Condition binaire | V0.2 |
| tantque | Français | Boucle conditionnelle | V0.2 |
| répéter | Français | Boucle comptée | V0.2 |
| pourChaque | Français | Itération | V0.2 |
| sortir | Français | Break | V0.2 |
| continuer | Français | Continue | V0.2 |
| essayer / attraper | Français | Gestion erreurs | V0.2 |
| signaler | Français | Lever erreur | V0.2 |

---

## 13. Feuille de route Rust V0.3

```
✅ Rust V0.2  → lecture .vtx + déclarer + définir + afficher

En cours — Rust V0.3 :
⬜ si / sinon / tantque / répéter
⬜ DUBI contextuel avec vocabulaires
⬜ HELIX avec mutation et allèles
⬜ DRUS 3 modes
⬜ AVON RECEVOIR / FOURNIR
⬜ PAR (crate rayon)
⬜ GWEL + ADBERT
⬜ ADN quadrinaire (src/adn.rs)
⬜ GRANN + cube chromatique 4D
⬜ cargo build --release → vercintorix.exe
```

---

*VERCINTORIX V0.3 — Spécification officielle*
*"Penser comme le vivant. Parler comme l'humain.*
*Voir comme la lumière. Se lier comme la matière."*
SPEC
echo "Spec: $(wc -l < /mnt/user-data/outputs/VERCINTORIX_SPEC_V0.3.md) lignes · $(ls -lh /mnt/user-data/outputs/VERCINTORIX_SPEC_V0.3.md | awk '{print $5}')"

Sortie

Spec: 671 lignes · 20K