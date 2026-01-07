---
name: Implementere Nynorsk språkstøtte
overview: Legger til fullstendig støtte for Nynorsk (nn) ved å opprette alle nødvendige YAML-regelfiler og testfiler basert på eksisterende norsk bokmål (nb) implementasjon, med tilpasninger for nynorsk terminologi og uttale.
todos:
  - id: create-yaml-rules
    content: Opprette alle YAML-regelfiler i Rules/Languages/nn/ basert på nb-versjonene, med tilpasning til nynorsk terminologi
    status: completed
  - id: create-shared-rules
    content: Opprette SharedRules undermappe med alle delte regelfiler (calculus, default, general, geometry, linear-algebra)
    status: completed
  - id: create-test-structure
    content: Opprette tests/Languages/nn.rs og tests/Languages/nn/ mappe med alle testmoduler
    status: completed
  - id: create-clearspeak-tests
    content: Opprette alle ClearSpeak testfiler i tests/Languages/nn/ClearSpeak/
    status: completed
  - id: create-simplespeak-tests
    content: Opprette alle SimpleSpeak testfiler i tests/Languages/nn/SimpleSpeak/
    status: completed
  - id: register-test-module
    content: Legge til mod nn; i tests/languages.rs for å registrere testmodulen
    status: completed
---

# Plan for implementering av Nynorsk språkstøtte

## Oversikt

MathCAT oppdager språk automatisk ved å skanne `Rules/Languages/` mappen. For å legge til Nynorsk (nn) må vi:

1. Opprette alle YAML-regelfiler i `Rules/Languages/nn/`
2. Opprette testfiler i `tests/Languages/nn/`
3. Registrere testmodulen i `tests/languages.rs`

## Tilnærming

Vi skal lage nynorsk (nn) basert på eksisterende bokmål (nb) oversettelser. Dette innebærer:

1. Kopiere alle filer fra `Rules/Languages/nb/` til `Rules/Languages/nn/`
2. Tilpasse terminologi fra bokmål til nynorsk der det er forskjeller
3. Kopiere og tilpasse alle testfiler fra `tests/Languages/nb/` til `tests/Languages/nn/`

## Implementeringssteg

### 1. Opprette YAML-regelfiler

Kopier alle filer fra `Rules/Languages/nb/` til `Rules/Languages/nn/` og tilpass terminologi:

- **definitions.yaml** - Språkspesifikke definisjoner (ordtall, matematiske termer)
- **ClearSpeak_Rules.yaml** - Regler for ClearSpeak-taleformat
- **SimpleSpeak_Rules.yaml** - Regler for SimpleSpeak-taleformat  
- **navigate.yaml** - Navigasjonsregler og meldinger
- **overview.yaml** - Oversiktsregler for matematiske strukturer
- **unicode.yaml** - Unicode-tegn til uttale mapping
- **unicode-full.yaml** - Fullstendig Unicode mapping
- **SharedRules/** mappe med:
- calculus.yaml
- default.yaml
- general.yaml
- geometry.yaml
- linear-algebra.yaml

**Tilpasninger:** Konverter bokmål til nynorsk der det er forskjeller:

- "telleren" → "teljaren"
- "nevneren" → "nemnaren"  
- "gjennom" → "gjennom" (samme)
- Andre terminologiske forskjeller mellom bokmål og nynorsk

### 2. Opprette testfiler

Kopier hele teststrukturen fra `tests/Languages/nb/` til `tests/Languages/nn/`:

- **nn.rs** - Hovedtestmodul (kopier direkte fra `tests/Languages/nb.rs`)
- **nn/** mappe med alle testmoduler (kopier hele mappen):
- alphabets.rs
- chemistry.rs
- mtable.rs
- shared.rs
- units.rs
- intent.rs
- ClearSpeak/ mappe med alle ClearSpeak-tester
- SimpleSpeak/ mappe med alle SimpleSpeak-tester

**Tilpasninger:** Oppdater alle forventede uttaler i testene fra bokmål til nynorsk terminologi. Mange tester kan være identiske, men alle som inneholder tekst må sjekkes og oppdateres der nødvendig.

### 3. Registrere testmodul

Legg til `mod nn;` i `tests/languages.rs` etter `mod nb;` linjen.

## Filer som må opprettes

### YAML-regelfiler (Rules/Languages/nn/)

- definitions.yaml
- ClearSpeak_Rules.yaml
- SimpleSpeak_Rules.yaml
- navigate.yaml
- overview.yaml
- unicode.yaml
- unicode-full.yaml
- SharedRules/calculus.yaml
- SharedRules/default.yaml
- SharedRules/general.yaml
- SharedRules/geometry.yaml
- SharedRules/linear-algebra.yaml

### Testfiler (tests/Languages/nn/)

- nn.rs
- alphabets.rs
- chemistry.rs
- mtable.rs
- shared.rs
- units.rs
- intent.rs
- ClearSpeak/functions.rs
- ClearSpeak/large_ops.rs
- ClearSpeak/menclose.rs
- ClearSpeak/mfrac.rs
- ClearSpeak/mroot.rs
- ClearSpeak/msup.rs
- ClearSpeak/multiline.rs
- ClearSpeak/sets.rs
- ClearSpeak/symbols_and_adornments.rs
- SimpleSpeak/functions.rs
- SimpleSpeak/geometry.rs
- SimpleSpeak/large_ops.rs
- SimpleSpeak/linear_algebra.rs