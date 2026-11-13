/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

// The basic layout of the tests is:
// 1. Sweep through all the SI prefixes
// 2. Sweep through each group of SI units
//    a) with both singular and plural without prefixes
//    b) with both singular and plural with one prefix
// 3. Sweep through each group of units that don't take SI prefixes
// These are broken into chunks so it is easier to see errors, when there are errors

#[test]
fn prefix_sweep() -> Result<()> {
    let expr = r#"<math>
        <mi intent=":unit">Qg</mi><mo>,</mo>
        <mi intent=":unit">Rg</mi><mo>,</mo>
        <mi intent=":unit">Yg</mi><mo>,</mo>
        <mi intent=":unit">Zg</mi><mo>,</mo>
        <mi intent=":unit">Eg</mi><mo>,</mo>
        <mi intent=":unit">Pg</mi><mo>,</mo>
        <mi intent=":unit">Tg</mi><mo>,</mo>
        <mi intent=":unit">Gg</mi><mo>,</mo>
        <mi intent=":unit">Mg</mi><mo>,</mo>
        <mi intent=":unit">kg</mi><mo>,</mo>
        <mi intent=":unit">hg</mi><mo>,</mo>
        <mi intent=":unit">dag</mi><mo>,</mo>
        <mi intent=":unit">dg</mi><mo>,</mo>
        <mi intent=":unit">cg</mi><mo>,</mo>
        <mi intent=":unit">mg</mi><mo>,</mo>
        <mi intent=":unit">µg</mi><mo>,</mo>
        <mi intent=":unit">ng</mi><mo>,</mo>
        <mi intent=":unit">pg</mi><mo>,</mo>
        <mi intent=":unit">fg</mi><mo>,</mo>
        <mi intent=":unit">ag</mi><mo>,</mo>
        <mi intent=":unit">zg</mi><mo>,</mo>
        <mi intent=":unit">yg</mi><mo>,</mo>
        <mi intent=":unit">rg</mi><mo>,</mo>
        <mi intent=":unit">qg</mi>
        </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "quetta-gram, komma, \
                ronna-gram komma, \
                yotta-gram komma, \
                zetta-gram komma, \
                exa-gram komma, \
                peta-gram komma, \
                tera-gram komma, \
                giga-gram komma, \
                mega-gram komma, \
                kilo-gram komma, \
                hekto-gram komma, \
                deka-gram komma, \
                desi-gram komma, \
                centi-gram komma, \
                milli-gram komma, \
                mikro-gram komma, \
                nano-gram komma, \
                piko-gram komma, \
                femto-gram komma, \
                atto-gram komma, \
                zepto-gram komma, \
                yokto-gram komma, \
                ronto-gram komma, \
                quecto-gram")?;
                return Ok(());

}

#[test]
fn si_base() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">A</mi><mo>,</mo><mn>2</mn><mi intent=":unit">A</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cd</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">K</mi><mo>,</mo><mn>2</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">K</mi><mo>,</mo><mn>2</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">g</mi><mo>,</mo><mn>2</mn><mi intent=":unit">g</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">m</mi><mo>,</mo><mn>2</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">mol</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mol</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">s</mi><mo>,</mo><mn>2</mn><mi intent=":unit">s</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">″</mi><mo>,</mo><mn>2</mn><mi intent=":unit">″</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">&quot;</mi><mo>,</mo><mn>2</mn><mi intent=":unit">&quot;</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">sek</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sek</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 ampere, komma, 2 ampere, komma, \
                1 candela, komma, 2 candela, komma, \
                1 kelvin, komma, 2 kelvin, komma, \
                1 kelvin, komma, 2 kelvin, komma, \
                ett gram, komma, 2 gram, komma, \
                1 meter, komma, 2 meter, komma, \
                ett mol, komma, 2 mol, komma, \
                ett sekund, komma, 2 sekunder, komma, \
                ett sekund, komma, 2 sekunder, komma, \
                ett sekund, komma, 2 sekunder, komma, \
                ett sekund, komma, 2 sekunder")?;
                return Ok(());

}

#[test]
fn si_base_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">QA</mi><mo>,</mo><mn>2</mn><mi intent=":unit">RA</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ycd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Zcd</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EK</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PK</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TK</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GK</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Mg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kg</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dam</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dmol</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cmol</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ms</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µs</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">nsek</mi><mo>,</mo><mn>2</mn><mi intent=":unit">psek</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 quetta-ampere, komma; 2 ronna-ampere, komma; \
                1 yotta-candela, komma; 2 zetta-candela, komma; \
                1 exa-kelvin, komma; 2 peta-kelvin, komma; \
                1 tera-kelvin, komma; 2 giga-kelvin, komma; \
                ett mega-gram, komma; 2 kilo-gram, komma; \
                1 hekto-meter, komma; 2 deka-meter, komma; \
                ett desi-mol, komma; 2 centi-mol, komma; \
                ett milli-sekund, komma; 2 mikro-sekunder, komma; \
                ett nano-sekund, komma; 2 piko-sekunder")?;
                return Ok(());

}


#[test]
fn si_derived_1() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Bq</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Bq</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">°C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">°C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℃</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℃</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">F</mi><mo>,</mo><mn>2</mn><mi intent=":unit">F</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Gy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">H</mi><mo>,</mo><mn>2</mn><mi intent=":unit">H</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Hz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">J</mi><mo>,</mo><mn>2</mn><mi intent=":unit">J</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kat</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kat</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">lm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">lm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">lx</mi><mo>,</mo><mn>2</mn><mi intent=":unit">lx</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 becquerel, komma, 2 becquerel, komma, \
                1 coulomb, komma, 2 coulomb, komma; \
                1 grad celsius, komma; 2 grader celsius, komma; \
                1 grad celsius, komma; 2 grader celsius, komma, \
                1 farad, komma, 2 farad, komma, \
                1 gray, komma, 2 gray, komma, \
                1 henry, komma, 2 henry, komma, \
                1 hertz, komma, 2 hertz, komma, \
                1 joule, komma, 2 joule, komma, \
                1 katal, komma, 2 katal, komma, \
                1 lumen, komma, 2 lumen, komma, \
                1 lux, komma, 2 lux")?;
                return Ok(());

}

#[test]
fn si_derived_1_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">QBq</mi><mo>,</mo><mn>2</mn><mi intent=":unit">RBq</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">YC</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZC</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EF</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PF</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TGy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GGy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MH</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kH</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">daHz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dHz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cJ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mJ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">µkat</mi><mo>,</mo><mn>2</mn><mi intent=":unit">nkat</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">plm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">flm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">alx</mi><mo>,</mo><mn>2</mn><mi intent=":unit">zlx</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">m°C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µ°C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">p℃</mi><mo>,</mo><mn>2</mn><mi intent=":unit">n℃</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 quetta-becquerel, komma; 2 ronna-becquerel, komma; \
                1 yotta-coulomb, komma; 2 zetta-coulomb, komma; \
                1 exa-farad, komma; 2 peta-farad, komma; \
                1 tera-gray, komma; 2 giga-gray, komma; \
                1 mega-henry, komma; 2 kilo-henry, komma; \
                1 deka-hertz, komma; 2 desi-hertz, komma; \
                1 centi-joule, komma; 2 milli-joule, komma; \
                1 mikro-katal, komma; 2 nano-katal, komma; \
                1 piko-lumen, komma; 2 femto-lumen, komma; \
                1 atto-lux, komma; 2 zepto-lux, komma; \
                1 milli-grad celsius, komma; 2 mikro-grader celsius; komma; \
                1 piko-grad celsius, komma; 2 nano-grader celsius")?;
                return Ok(());

}

#[test]
fn si_derived_2() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">N</mi><mo>,</mo><mn>2</mn><mi intent=":unit">N</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ω</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ω</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ω</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ω</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Pa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">S</mi><mo>,</mo><mn>2</mn><mi intent=":unit">S</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Sv</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Sv</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">T</mi><mo>,</mo><mn>2</mn><mi intent=":unit">T</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">V</mi><mo>,</mo><mn>2</mn><mi intent=":unit">V</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">W</mi><mo>,</mo><mn>2</mn><mi intent=":unit">W</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Wb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Wb</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 newton, komma, 2 newton, komma, \
                1 ohm, komma, 2 ohm, komma, \
                1 ohm, komma, 2 ohm, komma, \
                1 pascal, komma, 2 pascal, komma, \
                1 siemens, komma, 2 siemens, komma, \
                1 sievert, komma, 2 sievert, komma, \
                1 tesla, komma, 2 tesla, komma, \
                1 volt, komma, 2 volt, komma, \
                1 watt, komma, 2 watt, komma, \
                1 weber, komma, 2 weber")?;
                return Ok(());

}

#[test]
fn si_derived_2_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">qN</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rN</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">yΩ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">zΩ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">aΩ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fΩ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pPa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">nPa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">µS</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mS</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cSv</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dSv</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">daT</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hT</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GW</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TW</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">PWb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EWb</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 quecto-newton, komma; 2 ronto-newton, komma; \
                1 yokto-ohm, komma; 2 zepto-ohm, komma; \
                1 atto-ohm, komma; 2 femto-ohm, komma; \
                1 piko-pascal, komma; 2 nano-pascal, komma; \
                1 mikro-siemens, komma; 2 milli-siemens, komma; \
                1 centi-sievert, komma; 2 desi-sievert, komma; \
                1 deka-tesla, komma; 2 hekto-tesla, komma; \
                1 kilo-volt, komma; 2 mega-volt, komma; \
                1 giga-watt, komma; 2 tera-watt, komma; \
                1 peta-weber, komma; 2 exa-weber")?;
                return Ok(());

}


#[test]
fn si_accepted() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">l</mi><mo>,</mo><mn>2</mn><mi intent=":unit">l</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">L</mi><mo>,</mo><mn>2</mn><mi intent=":unit">L</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ℓ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ℓ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">t</mi><mo>,</mo><mn>2</mn><mi intent=":unit">t</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Da</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Da</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Np</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Np</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">u</mi><mo>,</mo><mn>2</mn><mi intent=":unit">u</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">eV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">eV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">rad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">sr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">a</mi><mo>,</mo><mn>2</mn><mi intent=":unit">a</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">as</mi><mo>,</mo><mn>2</mn><mi intent=":unit">as</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">b</mi><mo>,</mo><mn>2</mn><mi intent=":unit">b</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">B</mi><mo>,</mo><mn>2</mn><mi intent=":unit">B</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Bd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Bd</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 liter, komma, 2 liter, komma, 1 liter, komma, 2 liter, komma, 1 liter, komma, 2 liter, komma, ett tonn, komma, 2 tonn, komma, 1 dalton, komma, 2 dalton, komma, 1 neper, komma, 2 neper, komma; 1 atommasseenhet, komma; 2 atommasseenheter, komma; 1 elektronvolt, komma; 2 elektronvolt, komma, 1 radian, komma, 2 radianer, komma, 1 steradian, komma, 2 steradianer, komma, ett annuum, komma, 2 annua, komma, ett buesekund, komma, 2 buesekunder, komma, 1 bit, komma, 2 bit, komma, 1 byte, komma, 2 byte, komma, 1 baud, komma, 2 bauder")?;
        return Ok(());

}

#[test]
fn si_accepted_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Ql</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Rl</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">YL</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZL</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Eℓ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pℓ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Tt</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gt</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MDa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kDa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dNp</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cNp</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hu</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dau</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">meV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µeV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">nrad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">prad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fsr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">asr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ga</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ma</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">zas</mi><mo>,</mo><mn>2</mn><mi intent=":unit">yas</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Mb</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TBd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EBd</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 quetta-liter, komma; 2 ronna-liter, komma; \
                1 yotta-liter, komma; 2 zetta-liter, komma; \
                1 exa-liter, komma; 2 peta-liter, komma; \
                ett tera-tonn, komma; 2 giga-tonn, komma; \
                1 mega-dalton, komma; 2 kilo-dalton, komma; \
                1 desi-neper, komma; 2 centi-neper, komma; \
                1 hekto-atommasseenhet; komma; 2 deka-atommasseenheter; komma; \
                1 milli-elektronvolt, komma; 2 mikro-elektronvolt, komma; \
                1 nano-radian, komma; 2 piko-radianer, komma; \
                1 femto-steradian, komma; 2 atto-steradianer, komma; \
                ett giga-annuum, komma; 2 mega-annua, komma; \
                ett zepto-buesekund, komma; 2 yokto-buesekunder, komma; \
                1 kilo-bit, komma; 2 mega-bit, komma; \
                1 giga-byte, komma; 2 tera-byte, komma; \
                1 tera-baud, komma; 2 exa-bauder")?;
                return Ok(());

}

#[test]
fn without_prefix_time() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">″</mi><mo>,</mo><mn>2</mn><mi intent=":unit">″</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">&quot;</mi><mo>,</mo><mn>2</mn><mi intent=":unit">&quot;</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">′</mi><mo>,</mo><mn>2</mn><mi intent=":unit">′</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">'</mi><mo>,</mo><mn>2</mn><mi intent=":unit">'</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">min</mi><mo>,</mo><mn>2</mn><mi intent=":unit">min</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">h</mi><mo>,</mo><mn>2</mn><mi intent=":unit">h</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hr</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">Hr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">d</mi><mo>,</mo><mn>2</mn><mi intent=":unit">d</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">w</mi><mo>,</mo><mn>2</mn><mi intent=":unit">w</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">wk</mi><mo>,</mo><mn>2</mn><mi intent=":unit">wk</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">y</mi><mo>,</mo><mn>2</mn><mi intent=":unit">y</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">yr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">yr</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "ett sekund, komma, 2 sekunder, komma, \
                ett sekund, komma, 2 sekunder, komma, \
                ett minutt, komma, 2 minutter, komma, \
                ett minutt, komma, 2 minutter, komma, \
                ett minutt, komma, 2 minutter, komma, \
                1 time, komma, 2 timer, komma, \
                1 time, komma, 2 timer, komma, \
                1 time, komma, 2 timer, komma, \
                1 dag, komma, 2 dager, komma, \
                1 dag, komma, 2 dager, komma, \
                1 uke, komma, 2 uker, komma, \
                1 uke, komma, 2 uker, komma, \
                ett år, komma, 2 år, komma, \
                ett år, komma, 2 år")?;
                return Ok(());

}

#[test]
fn without_prefix_angles() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">°</mi><mo>,</mo><mn>2</mn><mi intent=":unit">°</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">deg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">deg</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">arcmin</mi><mo>,</mo><mn>2</mn><mi intent=":unit">arcmin</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">amin</mi><mo>,</mo><mn>2</mn><mi intent=":unit">amin</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">am</mi><mo>,</mo><mn>2</mn><mi intent=":unit">am</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MOA</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MOA</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">arcsec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">arcsec</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">asec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">asec</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 grad, komma, 2 grader, komma, \
                1 grad, komma, 2 grader, komma, \
                ett bueminutt, komma, 2 bueminutter, komma, \
                ett bueminutt, komma, 2 bueminutter, komma, \
                ett bueminutt, komma, 2 bueminutter, komma, \
                ett bueminutt, komma, 2 bueminutter, komma, \
                ett buesekund, komma, 2 buesekunder, komma, \
                ett buesekund, komma, 2 buesekunder")?;
                return Ok(());

}

#[test]
fn without_prefix_distance() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">au</mi><mo>,</mo><mn>2</mn><mi intent=":unit">au</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ltyr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ltyr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pc</mi><mo>,</mo><mn>2</mn><mi intent=":unit">pc</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fm</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 astronomisk enhet, komma; 2 astronomiske enheter, komma, \
                ett lysår, komma, 2 lysår, komma, \
                1 parsec, komma, 2 parsec, komma, \
                1 ångstrøm, komma, 2 ångstrøm, komma, \
                1 ångstrøm, komma, 2 ångstrøm, komma, \
                1 fermi, komma, 2 fermi")?;
                return Ok(());

}

#[test]
fn without_prefix_other() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">ha</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ha</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">atm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">atm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">amu</mi><mo>,</mo><mn>2</mn><mi intent=":unit">amu</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">bar</mi><mo>,</mo><mn>2</mn><mi intent=":unit">bar</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cal</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cal</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ci</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ci</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">grad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">grad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">M</mi><mo>,</mo><mn>2</mn><mi intent=":unit">M</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">R</mi><mo>,</mo><mn>2</mn><mi intent=":unit">R</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">rpm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rpm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℧</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℧</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dyn</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dyn</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">erg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">erg</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "ett hektar, komma, 2 hektar, komma, \
                1 desibel, komma, 2 desibel, komma, \
                1 atmosfære, komma, 2 atmosfærer, komma; \
                1 atommasseenhet, komma; 2 atommasseenheter, komma, \
                1 bar, komma, 2 bar, komma, \
                1 kalori, komma, 2 kalorier, komma, \
                1 curie, komma, 2 curier, komma, \
                1 gon, komma, 2 gon, komma, \
                1 molar, komma, 2 molar, komma, \
                1 røntgen, komma, 2 røntgen, komma; \
                1 omdreining per minutt, komma; 2 omdreininger per minutt, komma, \
                1 m-h-o, komma, 2 m-h-o, komma, \
                1 dyn, komma, 2 dyn, komma, \
                1 erg, komma, 2 erg")?;
                return Ok(());

}

#[test]
fn without_prefix_powers_of_2() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Kib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Kib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Mib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Mib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Gib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Tib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Tib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Pib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Eib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Eib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Zib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Zib</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">Yib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Yib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">KiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">KiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">PiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ZiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZiB</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">YiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">YiB</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, 
        "1 kibibit, komma, 2 kibibit, komma, \
                1 mebibit, komma, 2 mebibit, komma, \
                1 gibibit, komma, 2 gibibit, komma, \
                1 tebibit, komma, 2 tebibit, komma, \
                1 pebibit, komma, 2 pebibit, komma, \
                1 exbibit, komma, 2 exbibit, komma, \
                1 zebibit, komma, 2 zebibit, komma, \
                1 yobibit, komma, 2 yobibit, komma, \
                1 kibibyte, komma, 2 kibibyte, komma, \
                1 mebibyte, komma, 2 mebibyte, komma, \
                1 gibibyte, komma, 2 gibibyte, komma, \
                1 tebibyte, komma, 2 tebibyte, komma, \
                1 pebibyte, komma, 2 pebibyte, komma, \
                1 exbibyte, komma, 2 exbibyte, komma, \
                1 zebibyte, komma, 2 zebibyte, komma, \
                1 yobibyte, komma, 2 yobibyte")?;
                return Ok(());

}


#[test]
fn si_other_numbers() -> Result<()> {
    let expr = r#"<math><mn>1,0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2,0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2,5</mn><mi intent=":unit">&#xB5;sek</mi><mo>,</mo>
                            <mn>32,34</mn><mi intent=":unit">mol</mi></math>"#;
    test_prefs("nb", "SimpleSpeak", vec![("DecimalSeparators", ","), ("BlockSeparators", ".")], expr, 
        "1,0 liter, komma, 2,0 meter, komma; x milli-sekunder, komma; y mikro-sekunder, komma, deka-gram komma; 1235 deka-newton, komma; 2,5 mikro-sekunder, komma, 32,34 mol")?;
        return Ok(());

}


#[test]
fn test_mtext_inference() -> Result<()> {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4,5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test_prefs("nb", "SimpleSpeak", vec![("DecimalSeparators", ","), ("BlockSeparators", ".")], expr, 
        "start hakeparentes; ett tonn, komma; 2 peta-ampere, komma, 3 pascal, komma; 4,5 milli-tesla; slutt hakeparentes")?;
        return Ok(());

}


/// Tests for fractions followed by units

#[test]
fn one_neuter_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn><mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett minutt")?;
    test("nb", "SimpleSpeak", expr, "ett minutt")?;
    return Ok(());

}


#[test]
fn one_masculine_feminine_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn><mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 time")?;
    test("nb", "SimpleSpeak", expr, "1 time")?;
    return Ok(());

}


#[test]
fn half_neuter_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">sek</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett halvt sekund")?;
    test("nb", "SimpleSpeak", expr, "ett halvt sekund")?;
    
    return Ok(());

}


#[test]
fn half_masculine_feminine_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">l</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 halv liter")?;
    test("nb", "SimpleSpeak", expr, "1 halv liter")?;
    return Ok(());

}


#[test]
fn one_third_neuter_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">as</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett tredjedels buesekund")?;
    test("nb", "SimpleSpeak", expr, "ett tredjedels buesekund")?;
    return Ok(());

}


#[test]
fn one_third_masculine_feminine_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">m</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 tredjedels meter")?;
    test("nb", "SimpleSpeak", expr, "1 tredjedels meter")?;
    return Ok(());

}


#[test]
fn five_thirds_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>5</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "5 tredjedels time")?;
    test("nb", "SimpleSpeak", expr, "5 tredjedels time")?;
    return Ok(());

}


#[test]
fn four_eighths_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>4</mn><mn>8</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">g</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "4 åttedels gram")?;
    test("nb", "SimpleSpeak", expr, "4 åttedels gram")?;
    return Ok(());

}


#[test]
fn six_ninths_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>6</mn><mn>9</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "6 nidels time")?;
    test("nb", "SimpleSpeak", expr, "6 nidels time")?;
    return Ok(());

}


#[test]
fn mixed_fraction__half_neuter_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>3</mn>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "3 og ett halvt, minutt")?;
    test("nb", "SimpleSpeak", expr, "3 og ett halvt, minutt")?;
    return Ok(());

}


#[test]
fn mixed_fraction__half_masculine_feminine_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>3</mn>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "3 og 1 halv, time")?;
    test("nb", "SimpleSpeak", expr, "3 og 1 halv, time")?;
    return Ok(());

}


#[test]
fn mixed_fractions_third_neuter_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett og ett tredjedels, minutt")?;
    test("nb", "SimpleSpeak", expr, "ett og ett tredjedels, minutt")?;
    return Ok(());

}


#[test]
fn mixed_fractions_third_masculine_feminine_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 og 1 tredjedels, time")?;
    test("nb", "SimpleSpeak", expr, "1 og 1 tredjedels, time")?;
    return Ok(());

}

#[test]
fn mixed_fractions_two_and_two_thirds_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>2</mn>
    <mfrac>
        <mn>2</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "2 og 2 tredjedels, time")?;
    test("nb", "SimpleSpeak", expr, "2 og 2 tredjedels, time")?;
    return Ok(());

}

#[test]
fn mixed_fractions_four_and_five_sevenths_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>4</mn>
    <mfrac>
        <mn>5</mn><mn>7</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "4 og 5 sjudels, minutt")?;
    test("nb", "SimpleSpeak", expr, "4 og 5 sjudels, minutt")?;
    return Ok(());

}


#[test]
fn fraction_with_units_neuter_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>1</mn><mn>2</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">g</mi><mi mathvariant="normal" intent=":unit">hr</mi></mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett halvt gram per time")?;
    test("nb", "SimpleSpeak", expr, "ett halvt gram per time")?;
    return Ok(());

}


#[test]
fn fraction_with_units_masculine_feminine_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>1</mn><mn>2</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">m</mi><mi mathvariant="normal" intent=":unit">sek</mi></mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 halv meter per sekund")?;
    test("nb", "SimpleSpeak", expr, "1 halv meter per sekund")?;
    return Ok(());

}

#[test]
fn fraction_with_units_2_neuter_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mrow><mn>1</mn><mi mathvariant="normal" intent=":unit">g</mi></mrow>
        <mrow><mn>3</mn><mi mathvariant="normal" intent=":unit">hr</mi></mrow>
    </mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "brøken med teller ett gram; og nevner 3 timer")?;
    test("nb", "SimpleSpeak", expr, "brøk, ett gram, over, 3 timer, slutt brøk")?;
    return Ok(());

}


#[test]
fn fraction_with_units_2_masculine_feminine_unit() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mrow><mn>1</mn><mi mathvariant="normal" intent=":unit">m</mi></mrow>
        <mrow><mn>3</mn><mi mathvariant="normal" intent=":unit">sek</mi></mrow>
    </mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "brøken med teller 1 meter; og nevner 3 sekunder")?;
    test("nb", "SimpleSpeak", expr, "brøk, 1 meter, over, 3 sekunder, slutt brøk")?;
    return Ok(());

}


#[test]
fn fraction_not_ordinal_units_neuter() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>3</mn><mn>11</mn></mfrac><mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "3 over 11, minutter")?;
    test("nb", "SimpleSpeak", expr, "3 over 11, minutter")?;
    return Ok(());

}


#[test]
fn fraction_not_ordinal_units_masculine_feminine() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>3</mn><mn>11</mn></mfrac><mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "3 over 11, timer")?;
    test("nb", "SimpleSpeak", expr, "3 over 11, timer")?;
    return Ok(());

}

#[test]
fn fraction_not_ordinal_2_units_neuter() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>7</mn><mn>21</mn></mfrac><mi mathvariant="normal" intent=":unit">min</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "7 over 21, minutter")?;
    test("nb", "SimpleSpeak", expr, "7 over 21, minutter")?;
    return Ok(());

}


#[test]
fn fraction_not_ordinal_2_units_masculine_feminine() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>7</mn><mn>21</mn></mfrac><mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "7 over 21, timer")?;
    test("nb", "SimpleSpeak", expr, "7 over 21, timer")?;
    return Ok(());

}

/// Fractions followed by units with prefix. 

#[test]
fn one_neuter_unit_prefix() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn><mi mathvariant="normal" intent=":unit">mg</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett milli-gram")?;
    test("nb", "SimpleSpeak", expr, "ett milli-gram")?;
    return Ok(());

}


#[test]
fn one_masculine_feminine_unit_prefix() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn><mi mathvariant="normal" intent=":unit">µm</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 mikro-meter")?;
    test("nb", "SimpleSpeak", expr, "1 mikro-meter")?;
    return Ok(());

}


#[test]
fn half_neuter_unit_prefix() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>2</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">nsek</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett halvt nano-sekund")?;
    test("nb", "SimpleSpeak", expr, "ett halvt nano-sekund")?;
    return Ok(());

}


#[test]
fn third_neuter_unit_prefix() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">mt</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett tredjedels milli-tonn")?;
    test("nb", "SimpleSpeak", expr, "ett tredjedels milli-tonn")?;
    return Ok(());

}

#[test]
fn mixed_fractions_third_masculine_feminine_unit_prefix() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn>
    <mfrac>
        <mn>1</mn><mn>3</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">dN</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 og 1 tredjedels, desi-newton")?;
    test("nb", "SimpleSpeak", expr, "1 og 1 tredjedels, desi-newton")?;
    return Ok(());

}


#[test]
fn mixed_fractions_four_and_five_sevenths_unit_prefix() -> Result<()> {
    let expr = r#"<math>
    <mn>4</mn>
    <mfrac>
        <mn>5</mn><mn>7</mn>
    </mfrac>
    <mi mathvariant="normal" intent=":unit">nas</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "4 og 5 sjudels, nano-buesekund")?;
    test("nb", "SimpleSpeak", expr, "4 og 5 sjudels, nano-buesekund")?;
    return Ok(());

}


#[test]
fn fraction_not_ordinal_units_neuter_prefix() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>7</mn><mn>21</mn></mfrac><mi mathvariant="normal" intent=":unit">nsek</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "7 over 21, nano-sekunder")?;
    test("nb", "SimpleSpeak", expr, "7 over 21, nano-sekunder")?;
    return Ok(());

}


#[test]
fn fraction_not_ordinal_units_masculine_feminine_prefix() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>7</mn><mn>21</mn></mfrac><mi mathvariant="normal" intent=":unit">ml</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "7 over 21, milli-liter")?;
    test("nb", "SimpleSpeak", expr, "7 over 21, milli-liter")?;
    return Ok(());

}


///Some more complicated fractions with units

#[test]
fn acceleration_half() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>1</mn><mn>2</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">m</mi><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 halv meter per sekund i andre")?;
    test("nb", "SimpleSpeak", expr, "1 halv meter per sekund i andre")?;
    return Ok(());

}


#[test]
fn acceleration_half_one_fraction() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
    <mrow><mn>1</mn><mi mathvariant="normal" intent=":unit">m</mi></mrow>
    <mrow><mn>2</mn><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mrow>
    </mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "brøken med teller 1 meter; og nevner 2 sekunder i andre")?;
    test("nb", "SimpleSpeak", expr, "brøk, 1 meter, over, 2 sekunder i andre, slutt brøk")?;
    return Ok(());

}


#[test]
fn acceleration_four_sevenths() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>4</mn><mn>7</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">m</mi><msup><mi mathvariant="normal" intent=":unit">s</mi><mn>2</mn></msup></mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "4 sjudels meter per sekund i andre")?;
    test("nb", "SimpleSpeak", expr, "4 sjudels meter per sekund i andre")?;
    return Ok(());

}


#[test]
fn density_thirds() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>1</mn><mn>3</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">kg</mi><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>3</mn></msup></mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett tredjedels, kilo-gram per meter i tredje")?;
    test("nb", "SimpleSpeak", expr, "ett tredjedels, kilo-gram per meter i tredje")?;
    return Ok(());

}


#[test]
fn density_thirds_one_fraction() -> Result<()> {
    let expr = r#"<math>
    <mfrac>
    <mrow><mn>1</mn><mi mathvariant="normal" intent=":unit">kg</mi></mrow>
    <mrow><mn>3</mn><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>3</mn></msup></mrow>
    </mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "brøken med teller; ett kilo-gram; og nevner 3 meter i tredje")?;
    test("nb", "SimpleSpeak", expr, "brøk, ett kilo-gram, over, 3 meter i tredje, slutt brøk")?;
    return Ok(());

}


#[test]
fn density_two_fifths() -> Result<()> {
    let expr = r#"<math>
    <mfrac><mn>2</mn><mn>5</mn></mfrac>
    <mfrac><mi mathvariant="normal" intent=":unit">kg</mi><msup><mi mathvariant="normal" intent=":unit">m</mi><mn>3</mn></msup></mfrac>
</math>"#;
    test("nb", "ClearSpeak", expr, "2 femtedels, kilo-gram per meter i tredje")?;
    test("nb", "SimpleSpeak", expr, "2 femtedels, kilo-gram per meter i tredje")?;
    return Ok(());

}

#[test]
fn currency_singular() -> Result<()> {
    let expr = r#"
    <math>
    <mi>kr</mi><mn>1</mn>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 krone")?;
    test("nb", "SimpleSpeak", expr, "1 krone")?;
    return Ok(());

}

#[test]
fn currency_neuter_singular() -> Result<()> {
    let expr = r#"
    <math>
    <mi>£</mi><mn>1</mn>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett pund")?;
    test("nb", "SimpleSpeak", expr, "ett pund")?;
    return Ok(());

}
#[test]
fn currency_plural() -> Result<()> {
    let expr = r#"
    <math>
    <mi>kr</mi><mn>10</mn>
</math>"#;
    test("nb", "ClearSpeak", expr, "10 kroner")?;
    test("nb", "SimpleSpeak", expr, "10 kroner")?;
    return Ok(());

}

#[test]
fn common_fraction_mixed_unit_masculine() -> Result<()> {
    let expr = r#"
    <math>
    <mn>1</mn>
    <mfrac><mn>3</mn><mn>4</mn></mfrac>
    <mi mathvariant="normal" intent=":unit">m</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "1 og 3 fjerdedels, meter")?;
    test("nb", "SimpleSpeak", expr, "1 og 3 fjerdedels, meter")?;
    return Ok(());

}

#[test]
fn common_fraction_mixed_unit_neuter() -> Result<()> {
    let expr = r#"
    <math>
    <mn>1</mn>
    <mfrac><mn>3</mn><mn>4</mn></mfrac>
    <mi mathvariant="normal" intent=":unit">s</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "ett og 3 fjerdedels, sekund")?;
    test("nb", "SimpleSpeak", expr, "ett og 3 fjerdedels, sekund")?;
    return Ok(());

}

#[test]
fn common_fraction_mixed_unit_plural_neuter() -> Result<()> {
    let expr = r#"
    <math>
    <mn>2</mn>
    <mfrac><mn>4</mn><mn>5</mn></mfrac>
    <mi mathvariant="normal" intent=":unit">s</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "2 og 4 femtedels, sekund")?;
    test("nb", "SimpleSpeak", expr, "2 og 4 femtedels, sekund")?;
    return Ok(());

}

#[test]
fn common_fraction_mixed_unit_plural_masculine() -> Result<()> {
    let expr = r#"
    <math>
    <mn>2</mn>
    <mfrac><mn>4</mn><mn>5</mn></mfrac>
    <mi mathvariant="normal" intent=":unit">hr</mi>
</math>"#;
    test("nb", "ClearSpeak", expr, "2 og 4 femtedels, time")?;
    test("nb", "SimpleSpeak", expr, "2 og 4 femtedels, time")?;
    return Ok(());

}

/// Derived unit with :unit on the outer mrow; "/" should be "per".
#[test]
fn derived_unit_watts_per_metre_squared() -> Result<()> {
    let expr = r#"<math>
        <mrow intent=":unit">
            <mi>W</mi>
            <mo>/</mo>
            <msup>
                <mi>m</mi>
                <mn>2</mn>
            </msup>
        </mrow>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "watt per meter i andre")?;
    test("nb", "ClearSpeak", expr, "watt per meter i andre")?;
    Ok(())
}

/// Unit product with ·; multiplication is silent and only the last factor is plural.
#[test]
fn derived_unit_newton_metres() -> Result<()> {
    let expr = r#"<math>
        <mrow intent=":unit">
            <mi>N</mi>
            <mo>⋅</mo>
            <mi>m</mi>
        </mrow>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "newton meter")?;
    test("nb", "ClearSpeak", expr, "newton meter")?;
    Ok(())
}

/// Juxtaposed unit letters under :unit (invisible times inserted by canonicalize).
/// Norwegian: Should be "watt time". Need to set NumberlessUnitPlural: ["same"] to none, but then fractions followed by units become wrong.
#[test]
fn derived_unit_watt_hours_invisible_times() -> Result<()> {
    let expr = r#"<math>
        <mrow intent=":unit">
            <mi>W</mi>
            <mi>h</mi>
        </mrow>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "watt timer")?;
    test("nb", "ClearSpeak", expr, "watt timer")?;
    Ok(())
}

/// K·Wh: Wh is not in the English unit definitions, so it is spoken as text.
#[test]
fn derived_unit_kelvin_wh() -> Result<()> {
    let expr = r#"<math>
        <mrow intent=":unit">
            <mi>K</mi>
            <mo>·</mo>
            <mi>Wh</mi>
        </mrow>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "kelvin watt-time")?;
    test("nb", "ClearSpeak", expr, "kelvin watt-time")?;
    Ok(())
}

/// Mixed product and quotient: kg⋅m/s².
#[test]
fn derived_unit_kg_m_per_s_squared() -> Result<()> {
    let expr = r#"<math>
        <mrow intent=":unit">
            <mi>kg</mi>
            <mo>⋅</mo>
            <mi>m</mi>
            <mo>/</mo>
            <msup>
                <mi>s</mi>
                <mn>2</mn>
            </msup>
        </mrow>
    </math>"#;
    // "kg" is spoken via SI prefix + unit ("kilo-gram"); a short pause separates product factors.
    test("nb", "SimpleSpeak", expr, "kilo-gram, meter per sekund i andre")?;
    test("nb", "ClearSpeak", expr, "kilo-gram, meter per sekund i andre")?;
    Ok(())
}

/// Coefficient 1 with a compound unit stays singular on every factor.
#[test]
fn derived_unit_one_newton_metre() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn>
        <mrow intent=":unit">
            <mi>N</mi>
            <mo>⋅</mo>
            <mi>m</mi>
        </mrow>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "1 newton meter")?;
    test("nb", "ClearSpeak", expr, "1 newton meter")?;
    return Ok(());
}

/// A lone normal known unit token is not confirmed without quantity/product/equality context.
#[test]
fn lone_likely_unit_not_spoken_as_unit() -> Result<()> {
    let expr = r#"<math><mi mathvariant="normal">m</mi></math>"#;
    test("nb", "SimpleSpeak", expr, "m")?;
    test("nb", "ClearSpeak", expr, "m")?;
    return Ok(());
}

/// SI derived unit as base-unit product: N = kg · m · s⁻² (mathvariant=normal, no :unit).
/// Norwegian: Should be "newton er lik; kilo-gram meter, sekund opphøyd i minus 2"
#[test]
fn derived_unit_newton_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">N</mi>
        <mo>=</mo>
        <mi mathvariant="normal">kg</mi>
        <mo>&#xB7;</mo>
        <mi mathvariant="normal">m</mi>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mrow><mo>-</mo><mn>2</mn></mrow>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "newton er lik; kilo-gram meter, sekunder opphøyd i minus 2")?;
    test("nb", "ClearSpeak", expr, "newton er lik; kilo-gram meter sekunder opphøyd i minus 2")?;
    return Ok(());
}

/// J = kg · m² · s⁻².
/// Norwegian: Should be "joule er lik; kilo-gram meter i andre, sekund opphøyd i minus 2"
#[test]
fn derived_unit_joule_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">J</mi>
        <mo>=</mo>
        <mi mathvariant="normal">kg</mi>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mn>2</mn>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mrow><mo>-</mo><mn>2</mn></mrow>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "joule er lik; kilo-gram meter i andre, sekunder opphøyd i minus 2")?;
    test("nb", "ClearSpeak", expr, "joule er lik; kilo-gram meter i andre sekunder opphøyd i minus 2")?;
    return Ok(());
}

/// W = kg · m² · s⁻³.
/// Norwegian: Should be "watt er lik; kilo-gram meter i andre, sekund opphøyd i minus 3"
#[test]
fn derived_unit_watt_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">W</mi>
        <mo>=</mo>
        <mi mathvariant="normal">kg</mi>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mn>2</mn>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mrow><mo>-</mo><mn>3</mn></mrow>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "watt er lik; kilo-gram meter i andre, sekunder opphøyd i minus 3")?;
    test("nb", "ClearSpeak", expr, "watt er lik; kilo-gram meter i andre sekunder opphøyd i minus 3")?;
    return Ok(());
}

/// Pa = kg · m⁻¹ · s⁻².
/// Norwegian: Should be "pascal er lik; kilo-gram meter opphøyd i minus 1; sekund opphøyd i minus 2"
#[test]
fn derived_unit_pascal_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">Pa</mi>
        <mo>=</mo>
        <mi mathvariant="normal">kg</mi>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mrow><mo>-</mo><mn>1</mn></mrow>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mrow><mo>-</mo><mn>2</mn></mrow>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "pascal er lik; kilo-gram meter opphøyd i minus 1; sekunder opphøyd i minus 2")?;
    test("nb", "ClearSpeak", expr, "pascal er lik; kilo-gram meter opphøyd i minus 1; sekunder opphøyd i minus 2")?;
    return Ok(());
}

/// Hz = s⁻¹.
/// Norwegian: Should be "hertz er lik, sekund opphøyd i minus 1"
#[test]
fn derived_unit_hertz_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">Hz</mi>
        <mo>=</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mrow><mo>-</mo><mn>1</mn></mrow>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "hertz er lik, sekunder opphøyd i minus 1")?;
    test("nb", "ClearSpeak", expr, "hertz er lik sekunder opphøyd i minus 1")?;
    return Ok(());
}

/// C = s · A.
#[test]
fn derived_unit_coulomb_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">C</mi>
        <mo>=</mo>
        <mi mathvariant="normal">s</mi>
        <mo>&#xB7;</mo>
        <mi mathvariant="normal">A</mi>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "coulomb er lik sekund ampere")?;
    test("nb", "ClearSpeak", expr, "coulomb er lik sekund ampere")?;
    return Ok(());
}

/// V = kg · m² · s⁻³ · A⁻¹.
#[test]
fn derived_unit_volt_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">V</mi>
        <mo>=</mo>
        <mi mathvariant="normal">kg</mi>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mn>2</mn>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mrow><mo>-</mo><mn>3</mn></mrow>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">A</mi>
            <mrow><mo>-</mo><mn>1</mn></mrow>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "volt er lik; kilo-gram meter i andre sekund opphøyd i minus 3; ampere opphøyd i minus 1")?;
    test("nb", "ClearSpeak", expr, "volt er lik; kilo-gram meter i andre sekund opphøyd i minus 3; ampere opphøyd i minus 1")?;
    return Ok(());
}

/// Ω = kg · m² · s⁻³ · A⁻².
#[test]
fn derived_unit_ohm_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">&#x3A9;</mi>
        <mo>=</mo>
        <mi mathvariant="normal">kg</mi>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mn>2</mn>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mrow><mo>-</mo><mn>3</mn></mrow>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">A</mi>
            <mrow><mo>-</mo><mn>2</mn></mrow>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "ohm er lik; kilo-gram meter i andre sekund opphøyd i minus 3; ampere opphøyd i minus 2")?;
    test("nb", "ClearSpeak", expr, "ohm er lik; kilo-gram meter i andre sekund opphøyd i minus 3; ampere opphøyd i minus 2")?;
    return Ok(());
}

/// T = kg · s⁻² · A⁻¹.
#[test]
fn derived_unit_tesla_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">T</mi>
        <mo>=</mo>
        <mi mathvariant="normal">kg</mi>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mrow><mo>-</mo><mn>2</mn></mrow>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">A</mi>
            <mrow><mo>-</mo><mn>1</mn></mrow>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "tesla er lik; kilo-gram sekund opphøyd i minus 2; ampere opphøyd i minus 1")?;
    test("nb", "ClearSpeak", expr, "tesla er lik; kilo-gram sekund opphøyd i minus 2; ampere opphøyd i minus 1")?;
    return Ok(());
}

/// Currency rates: `€5/l` should pluralize like a unit numerator (`euros per litre`).
#[test]
fn currency_per_unit() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>€</mi><mn>5</mn></mrow><mi mathvariant="normal">l</mi></mfrac></math>"#;
    test("nb", "SimpleSpeak", expr, "5 euro per liter")?;
    test("nb", "ClearSpeak", expr, "5 euro per liter")?;

    let expr = r#"<math><mn>5</mn><mfrac><mi>€</mi><mi mathvariant="normal">l</mi></mfrac></math>"#;
    test("nb", "SimpleSpeak", expr, "5 euro per liter")?;
    test("nb", "ClearSpeak", expr, "5 euro per liter")?;

    let expr = r#"<math><mrow><mi>€</mi><mn>5</mn><mo>/</mo><mi mathvariant="normal">l</mi></mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "5 euro per liter")?;
    test("nb", "ClearSpeak", expr, "5 euro per liter")?;

    let expr = r#"<math><mrow><mi>€</mi><mn>1</mn><mo>/</mo><mi mathvariant="normal">l</mi></mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "1 euro per liter")?;
    test("nb", "ClearSpeak", expr, "1 euro per liter")?;

    // Postfix amount binds before the solidus: `(5 €) / l`.
    let expr = r#"<math><mrow><mn>5</mn><mi>€</mi><mo>/</mo><mi mathvariant="normal">l</mi></mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "5 euro per liter")?;
    test("nb", "ClearSpeak", expr, "5 euro per liter")?;
    return Ok(());
}

#[test]
fn currency_per_unit_2() -> Result<()> {
    let expr = r#"<math><mfrac><mrow><mi>kr</mi><mn>5</mn></mrow><mi mathvariant="normal">h</mi></mfrac></math>"#;
    test("nb", "SimpleSpeak", expr, "5 kroner per time")?;
    test("nb", "ClearSpeak", expr, "5 kroner per time")?;

    let expr = r#"<math><mn>5</mn><mfrac><mi>kr</mi><mi mathvariant="normal">h</mi></mfrac></math>"#;
    test("nb", "SimpleSpeak", expr, "5 kroner per time")?;
    test("nb", "ClearSpeak", expr, "5 kroner per time")?;

    let expr = r#"<math><mrow><mi>kr</mi><mn>5</mn><mo>/</mo><mi mathvariant="normal">h</mi></mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "5 kroner per time")?;
    test("nb", "ClearSpeak", expr, "5 kroner per time")?;

    let expr = r#"<math><mrow><mi>kr</mi><mn>1</mn><mo>/</mo><mi mathvariant="normal">h</mi></mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "1 krone per time")?;
    test("nb", "ClearSpeak", expr, "1 krone per time")?;

    // Postfix amount binds before the solidus: `(5 €) / l`.
    let expr = r#"<math><mrow><mn>5</mn><mi>kr</mi><mo>/</mo><mi mathvariant="normal">h</mi></mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "5 kroner per time")?;
    test("nb", "ClearSpeak", expr, "5 kroner per time")?;
    return Ok(());
}

/// F = kg⁻¹ · m⁻² · s⁴ · A².
#[test]
fn derived_unit_farad_base_expansion() -> Result<()> {
    let expr = r#"<math>
        <mi mathvariant="normal">F</mi>
        <mo>=</mo>
        <msup>
            <mi mathvariant="normal">kg</mi>
            <mrow><mo>-</mo><mn>1</mn></mrow>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mrow><mo>-</mo><mn>2</mn></mrow>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">s</mi>
            <mn>4</mn>
        </msup>
        <mo>&#xB7;</mo>
        <msup>
            <mi mathvariant="normal">A</mi>
            <mn>2</mn>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "farad er lik; kilo-gram opphøyd i minus 1; meter opphøyd i minus 2; sekund opphøyd i 4; ampere i andre")?;
    test("nb", "ClearSpeak", expr, "farad er lik; kilo-gram opphøyd i minus 1; meter opphøyd i minus 2; sekund opphøyd i 4; ampere i andre")?;
    return Ok(());
}

#[test]
fn plural_seconds_per_meter_squared() -> Result<()> {
    let expr = r#"<math>
    <mn>5</mn>
<mi mathvariant="normal">s</mi>
        <mo>/</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mn>2</mn>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "5, sekunder per meter i andre")?;
    test("nb", "ClearSpeak", expr, "5, sekunder per meter i andre")?;
    return Ok(());
}

#[test]
fn singular_second_per_meter_squared() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn>
<mi mathvariant="normal">s</mi>
        <mo>/</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mn>2</mn>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "ett, sekund per meter i andre")?;
    test("nb", "ClearSpeak", expr, "ett, sekund per meter i andre")?;
    return Ok(());
}

#[test]
fn singular_compund_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>1</mn>
        <mrow intent=":unit">
            <mi>W</mi>
            <mi>h</mi>
        </mrow>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "1 watt time")?;
    test("nb", "ClearSpeak", expr, "1 watt time")?;
    Ok(())
}

#[test]
fn plural_compound_unit() -> Result<()> {
    let expr = r#"<math>
    <mn>2</mn>
        <mrow intent=":unit">
            <mi>W</mi>
            <mi>h</mi>
        </mrow>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "2 watt timer")?;
    test("nb", "ClearSpeak", expr, "2 watt timer")?;
    Ok(())
}

#[test]
fn variable_unit() -> Result<()> {
    let expr = r#"<math>
    <mi>x</mi>
<mi mathvariant="normal">s</mi>
        <mo>/</mo>
        <msup>
            <mi mathvariant="normal">m</mi>
            <mn>2</mn>
        </msup>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "x, sekunder per meter i andre")?;
    test("nb", "ClearSpeak", expr, "x, sekunder per meter i andre")?;
    return Ok(());
}

#[test]
fn variable_unit_2() -> Result<()> {
    let expr = r#"<math>
    <mi>x</mi>
        <mrow intent=":unit">
            <mi>W</mi>
            <mi>h</mi>
        </mrow>
    </math>"#;
    test("nb", "SimpleSpeak", expr, "x watt timer")?;
    test("nb", "ClearSpeak", expr, "x watt timer")?;
    Ok(())
}