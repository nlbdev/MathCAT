---
name: Forbedre nynorsk oversetting
overview: Gjennomgår og forbedrer nynorsk-oversettingen ved å fikse kjente problemer med brøker og ordtall, konvertere bokmål-ord til nynorsk, og verifisere konsistens mellom alle regelfiler.
todos:
  - id: fix-fraction-ordinals
    content: Legg til manglende ordtall-definisjoner i definitions.yaml (NumbersOrdinalFractionalTens, etc.)
    status: pending
  - id: fix-plural-ones
    content: Oppdater NumbersOrdinalPluralOnes til korrekt nynorsk format ("", "endelar", "todelar"...)
    status: pending
  - id: remove-en-from-large-numbers
    content: Fjern 'én' fra store ordtall i NumbersOrdinalLarge og NumbersLarge hvis det finnes
    status: pending
  - id: fix-deler-to-delar
    content: Endre alle "-deler" til "-delar" i brøk-terminologi (substantiv flertall)
    status: pending
  - id: fix-ikke-to-ikkje
    content: Endre "ikke" til "ikkje" i alle YAML-filer (~27 forekomster)
    status: pending
  - id: fix-aapen-to-open
    content: Endre "åpen/åpne/lukket/lukkede/halvåpen" til "open/opne/lukka/halvopen" (~67 forekomster)
    status: pending
  - id: fix-verb-endings
    content: Endre verb fra bokmål til nynorsk (inneholder→inneheld, tilhører→tilhøyrer, følger→følgjer, etc.)
    status: pending
  - id: fix-participle-endings
    content: Endre presens partisipp fra "-ende" til "-ande" (foregående→føregåande, påfølgende→påfølgjande)
    status: pending
  - id: review-terminology
    content: Gjennomgå alle YAML-filer for å sikre konsistent nynorsk terminologi
    status: pending
    dependencies:
      - fix-deler-to-delar
      - fix-ikke-to-ikkje
      - fix-aapen-to-open
      - fix-verb-endings
      - fix-participle-endings
  - id: update-tests
    content: Oppdatere testfiler med korrekte nynorske forventede uttaler
    status: pending
    dependencies:
      - fix-fraction-ordinals
      - fix-plural-ones
      - review-terminology
  - id: run-tests
    content: Kjøre alle nynorsk-tester og fikse eventuelle feil
    status: pending
    dependencies:
      - update-tests
---

# Plan for forbedring av nynorsk oversetting

## Bakgrunn

Nynorsk-implementasjonen har et fundament, men inneholder mange bokmål-ord som må konverteres til nynorsk. Planen fokuserer på systematisk gjennomgang og korrigering.

## Identifiserte problemer

### 1. Brøker og ordtall (definitions.yaml)

- Mangler `NumbersOrdinalFractionalTens` (trengs for 1/20 = "ein tjuedel")
- `NumbersOrdinalPluralOnes` er feil (bør vere "", "endelar", "todelar"...)
- Mangler `NumbersOrdinalFractionalHundreds` (trengs for 1/100)
- Mangler `NumbersOrdinalFractionalLargeNumbers` (trengs for 1/1000)
- "én" bør fjernast frå store ordtal

### 2. Bokmål-ord som må endrast til nynorsk

| Bokmål | Nynorsk | Tal forekomstar | Filer ||--------|---------|-----------------|-------|| `-deler` (subst.) | `-delar` | ~36 | definitions.yaml, unicode*.yaml || `ikke` | `ikkje` | ~27 | unicode*.yaml || `åpen/åpne` | `open/opne` | ~43 | unicode*.yaml, general.yaml, ClearSpeak || `lukket/lukkede` | `lukka` | ~20 | unicode*.yaml || `halvåpen` | `halvopen` | 4 | general.yaml, ClearSpeak || `foregående` | `føregåande` | 3 | navigate.yaml || `påfølgende` | `påfølgjande` | 3 | navigate.yaml || `inneholder` | `inneheld` | ~17 | unicode-full.yaml || `tilhører` | `tilhøyrer` | ~12 | unicode*.yaml || `følger` (verb) | `følgjer` | ~38 | unicode-full.yaml || `beviser` | `provar` | 3 | unicode-full.yaml || `tvinger` | `tvingar` | 2 | unicode-full.yaml || `nærmer` | `nærmar` | 1 | unicode-full.yaml || `eksisterer` | `eksisterer/finst` | 4 | unicode*.yaml |

### 3. Terminologi som allereie er korrekt nynorsk

- `teljar` / `nemnar` (i definitions.yaml, navigate.yaml, ClearSpeak)
- `mengd` / `mengden`
- `grenseverdi`

## Implementeringssteg

### Steg 1: Fikse brøker og ordtall i definitions.yaml

**Fil:** `Rules/Languages/nn/definitions.yaml`

1. Legg til `NumbersOrdinalFractionalTens`:
```yaml
- NumbersOrdinalFractionalTens: [
    "", "tidel", "tjuedel", "trettidel", "førtidel", "femtidel", 
    "sekstidel", "syttidel", "åttidel", "nittidel"
]
```




2. Legg til `NumbersOrdinalFractionalPluralTens`:
```yaml
- NumbersOrdinalFractionalPluralTens: [
    "", "tidelar", "tjuedelar", "trettidelar", "førtidelar", "femtidelar",
    "sekstidelar", "syttidelar", "åttidelar", "nittidelar"
]
```




3. Legg til `NumbersOrdinalFractionalHundreds`:
```yaml
- NumbersOrdinalFractionalHundreds: [
    "", "hundredel", "to hundredel", "tre hundredel", "fire hundredel", 
    "fem hundredel", "seks hundredel", "sju hundredel", "åtte hundredel", "ni hundredel"
]
```




4. Legg til `NumbersOrdinalFractionalPluralHundreds`:
```yaml
- NumbersOrdinalFractionalPluralHundreds: [
    "", "hundredelar", "to hundredelar", "tre hundredelar", "fire hundredelar",
    "fem hundredelar", "seks hundredelar", "sju hundredelar", "åtte hundredelar", "ni hundredelar"
]
```




5. Legg til `NumbersOrdinalFractionalLargeNumbers`:
```yaml
- NumbersOrdinalFractionalLargeNumbers: [
    "", "tusendel", "milliondel", "milliarddel", "billiondel", "billiarddel",
    "trilliondel", "trilliarddel", "kvadrilliondel", "kvadrilliarddel", "kvintilliondel",
    "kvintilliarddel", "sekstilliondel", "sekstilliarddel", "septilliondel", "septilliarddel",
    "oktilliondel", "oktilliarddel", "nonilliondel", "nonilliarddel", "desilliondel"
]
```




6. Legg til `NumbersOrdinalFractionalPluralLargeNumbers`:
```yaml
- NumbersOrdinalFractionalPluralLargeNumbers: [
    "", "tusendelar", "milliondelar", "milliarddelar", "billiondelar", "billiarddelar",
    "trilliondelar", "trilliarddelar", "kvadrilliondelar", "kvadrilliarddelar", "kvintilliondelar",
    "kvintilliarddelar", "sekstilliondelar", "sekstilliarddelar", "septilliondelar", "septilliarddelar",
    "oktilliondelar", "oktilliarddelar", "nonilliondelar", "nonilliarddelar", "desilliondelar"
]
```




7. Oppdater `NumbersOrdinalPluralOnes`:
```yaml
- NumbersOrdinalPluralOnes: [
    "", "endelar", "todelar", "tredelar", "firedelar", "femedelar", 
    "sjettedelar", "sjudelar", "åttedelar", "nidelar",
    "tidelar", "ellevedelar", "tolvdelar", "trettendelar", "fjortendelar", 
    "femtendelar", "sekstendelar", "syttendelar", "attendelar", "nittendelar"
]
```




8. Oppdater `NumbersOrdinalFractionalPluralOnes` med "-delar":
```yaml
- NumbersOrdinalFractionalPluralOnes: [
    "nulldelar", "heile", "halve", "tredjedelar", "fjerdedelar", "femtedelar", "sjettedelar", "sjudelar", "åttedelar", "nidelar",
    "tidelar", "ellevedelar", "tolvdelar", "trettendelar", "fjortendelar", "femtendelar", "sekstendelar",
    "syttendelar", "attendelar", "nittendelar"
]
```




### Steg 2: Endre "ikkje" i staden for "ikke"

**Filer:** `Rules/Languages/nn/unicode.yaml`, `Rules/Languages/nn/unicode-full.yaml`Søk og erstatt (med kontekst for å unngå feil):

- `"ikke"` → `"ikkje"` (som T: verdi)
- `"ikke ` → `"ikkje ` (start av frase)
-  `ikke"` →  `ikkje"` (slutt av frase)

### Steg 3: Endre "open/opne" i staden for "åpen/åpne"

**Filer:** `unicode-full.yaml`, `general.yaml`, `ClearSpeak_Rules.yaml`Søk og erstatt:

- `"åpen"` → `"open"`
- `"åpne"` → `"opne"`
- `"det åpne"` → `"det opne"`
- `"halvåpen"` → `"halvopen"`
- `"halvåpne"` → `"halvopne"`

### Steg 4: Endre "lukka" i staden for "lukket/lukkede"

**Filer:** `unicode-full.yaml`, `general.yaml`, `ClearSpeak_Rules.yaml`Søk og erstatt:

- `"lukket"` → `"lukka"`
- `"lukkede"` → `"lukka"`
- `"det lukkede"` → `"det lukka"`

### Steg 5: Endre verb til nynorsk

**Fil:** `Rules/Languages/nn/unicode-full.yaml` (hovudfil)| Søk | Erstatt ||-----|---------|| `"inneholder"` | `"inneheld"` || `"tilhører"` | `"tilhøyrer"` || `"følger"` (verb) | `"følgjer"` || `"beviser"` | `"provar"` || `"tvinger"` | `"tvingar"` || `"nærmer"` | `"nærmar"` || `"eksisterer"` | `"eksisterer"` (behald, eller bruk `"finst"`) |

### Steg 6: Endre presens partisipp til nynorsk

**Fil:** `Rules/Languages/nn/navigate.yaml`Søk og erstatt:

- `"foregående"` → `"føregåande"`
- `"påfølgende"` → `"påfølgjande"`
- `"tilhørende"` → `"tilhøyrande"`
- `"omsluttende"` → `"omsluttande"`

### Steg 7: Endre "-deler" til "-delar" (substantiv)

**Filer:** Alle YAML-filer og testfilerSøk og erstatt (berre substantiv, ikkje verb):

- `"tredjedeler"` → `"tredjedelar"`
- `"fjerdedeler"` → `"fjerdedelar"`
- `"femtedeler"` → `"femtedelar"`
- `"tideler"` → `"tidelar"`
- `"hundredeler"` → `"hundredelar"`
- `"tusendeler"` → `"tusendelar"`
- osv.

**Merk:** `"deler"` som verb (å dele / divisjon) skal IKKJE endrast.

### Steg 8: Oppdatere testfiler

**Filer:** `tests/Languages/nn/**/*.rs`Oppdater alle forventede uttalar til å bruke nynorsk:

- Endre `-deler` til `-delar` i forventede strengar
- Endre `ikke` til `ikkje`
- Endre `åpen/lukket` til `open/lukka`
- Endre verb til nynorsk form

### Steg 9: Køyre testar og verifisere

1. Køyre alle nynorsk-testar: `cargo test Languages::nn`
2. Fikse eventuelle feil som oppstår
3. Verifisere at brøker og ordtal no fungerer korrekt

## Forventede endringar (oppsummert)

| Kategori | Tal endringar ||----------|---------------|| Brøk/ordtall-definisjonar | ~8 nye arrays || `-deler` → `-delar` | ~36 || `ikke` → `ikkje` | ~27 || `åpen/lukket` → `open/lukka` | ~67 || Verb (inneheld, tilhøyrer, etc.) | ~70 || Presens partisipp (-ande) | ~10 || **Totalt** | **~220 endringar** |

## Forventa resultat

- Alle kjende problem med brøker og ordtal er fiksa
- Konsistent nynorsk terminologi gjennomgåande