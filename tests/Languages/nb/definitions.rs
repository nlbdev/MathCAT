/// Tests for rules in definitions:
/// *  modified var
use crate::common::*;
use anyhow::Result;

#[test]
fn tuple_basic() -> Result<()> {
    // function 
    let expr = r#"
      <math>
        <mrow intent="tuple($x,$y)">
          <mi arg="x">x</mi>
          <mi arg="y">y</mi>
        </mrow>
      </math>
    "#;

    test("nb", "ClearSpeak", expr, "tuppel av x komma; y")?;
    test("nb", "SimpleSpeak", expr, "tuppel av x komma; y")?;

    return Ok(());
}

#[test]
fn my_set_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="set($x,$y)">
          <mi arg="x">x</mi>
          <mi arg="y">y</mi>
        </mrow>
      </math>
    "#;

    test("nb", "ClearSpeak", expr, "mengden av x komma; y")?;

    Ok(())
}

#[test]
fn fixed_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="set-of-reals">R
        </mi>
      </math>
    "#;

    test("nb", "ClearSpeak", expr, "mengden av alle reelle tall")?;
    Ok(())
}

#[test]
fn i_test() -> Result<()> {
    let expr = r#"
      <math>
        <mi intent="imaginary-i">i
        </mi>
      </math>
    "#;

    test("nb", "ClearSpeak", expr, "i")?;
    Ok(())
}


#[test]
fn floor_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="floor($x)">
          <mi arg="x">x</mi>
        </mrow>
      </math>
    "#;

    test("nb", "ClearSpeak", expr, "gulvfunksjon av x")?;

    Ok(())
}

#[test]
fn ratio_function_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="ratio($x,$y)">
          <mi arg="x">6</mi>
          <mi arg="y">3</mi>
        </mrow>
      </math>
    "#;

    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Terse")], expr, "forholdet 6 til, 3")?;
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Medium")], expr, "forholdet 6 til, 3")?;
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "forholdet 6 til, 3")?;
    test("nb", "ClearSpeak", expr, "forholdet 6 til, 3")?;
    Ok(())
}

#[test]
fn set_difference_basic() -> Result<()> {
    let expr = r#"
      <math>
        <mrow intent="set-difference($A,$B)">
          <mi arg="A">A</mi>
          <mo>&#x2216;</mo>
          <mi arg="B">B</mi>
        </mrow>
      </math>
    "#;

    test( "nb", "ClearSpeak", expr, "mengdedifferanse av stor a og, stor b")?;

    Ok(())
}

#[test]
fn postfix_test() -> Result<()> {
    let tests = [
        (
            "transpose",
            r#"
            <msup intent="transpose($x)">
                <mi arg="x">x</mi>
                <mo>T</mo>
            </msup>
            "#,
            "x transponert",
        ),
        (
            "highlight",
            r#"
            <menclose intent="highlight($x)">
                <mi arg="x">x</mi>
            </menclose>
            "#,
            "x uthevet",
        ),
    ];

    // Loop through all test cases, name _, body, and expected result
    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn prefix_test() -> Result<()> {
    // Prefix test limit, unit-vector, line-segment
    // Directed-line-segment, line, ray, arc
    let tests = [
        (
            "limit",
            r#"
            <mrow intent="limit($x)">
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "grenseverdien når x",
        ),
        (
            "unit-vector",
            r#"
            <mover intent="unit-vector($x)">
                <mi arg="x">x</mi>
                <mo>^</mo>
            </mover>
            "#,
            "enhetsvektor x",
        ),
        (
            "line-segment",
            r#"
            <mover intent="line-segment($x)">
                <mi arg="x">x</mi>
                <mo>¯</mo>
            </mover>
            "#,
            "linjestykke x",
        ),
    ];

    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn functions_and_inverses_tests() -> Result<()> {
    let tests = vec![
        //("closed-interval", "closed interval between x and y"),
        //("closed-open-interval", "interval between x included and y"),
        //("open-closed-interval", "interval between x and y included"),
        //("open-interval", "open interval between x and y"),

        ("inverse", "invers av x"),
        ("domain", "definisjonsmengden til x"),
        ("codomain", "verdimengden til x"),
        ("image", "bildet til x"),

        //("fraction", "fraction x over y end fraction"),
        ("mixed-fraction", "x og y"),
        ("quotient", "heltallsdel av x delt på, y"),
        ("evaluated-at", "x evaluert i y"),
        ("remainder", "resten av x delt på, y"),

        ("max", "maks av x komma; y komma; z"),
        ("min", "min av x komma; y komma; z"),

        ("power", "x opphøyd i y"),

        // ("root", ...) skipped: ClearSpeak `tag: root` requires an index
        // (same as the Polish suite).

        ("greatest-common-divisor", "største felles divisor til x komma; y komma; z"),
        ("least-common-multiple", "minste felles multiplum til x komma; y komma; z"),

        ("absolute-value", "absoluttverdien av x"),

        ("complex-conjugate", "kompleks-konjugert av x"),
        ("complex-arg", "argumentet til x"),
        ("real-part", "den reelle delen av x"),
        ("imaginary-part", "den komplekse delen av x"),

        ("polar-coordinate", "polarkoordinat x komma; y"),
        ("spherical-coordinate", "sfærisk koordinat x komma; y komma; z"),
        ("cartesian-coordinate", "kartesisk koordinat x komma; y komma; z"),
        ("coordinate", "koordinat x komma; y komma; z"),

        ("floor", "gulvfunksjon av x"),
        ("ceiling", "takfunksjon av x"),
        ("round", "avrundet verdi av x"),
        ("fractional-part", "desimaldelen av x"),

        
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            
            "max"
            | "min"
            | "greatest-common-divisor"
            | "least-common-multiple"
            | "spherical-coordinate"
            | "cartesian-coordinate"
            | "coordinate" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b,$c)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                            <mi arg='c'>z</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            "power"
            | "mixed-fraction"
            | "quotient"
            | "evaluated-at"
            | "remainder"
            | "closed-interval"
            | "closed-open-interval"
            | "open-closed-interval"
            | "open-interval" 
            | "polar-coordinate" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn calculus_tests() -> Result<()> {
    let tests = vec![
        //("derivative", "the derivative of x with respect to x"),
        //"definite-integral", "integral over x from x to x"),

        // prefix
        ("limit", "grenseverdien når x"),

        // infix
        ("tends-to", "x går mot y"),
        ("tends-to-from-above", "x går mot ovenfra y"),
        ("tends-to-from-below", "x går mot nedenfra y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "limit" => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            _ => {
                // infix cases
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn set_tests() -> Result<()> {
    let tests = vec![
        ("set", "mengden av x"),
        // ("set-difference", "set difference of x and y"),
        ("complement", "komplementet til x"),
        //("empty-set", "empty set"),
        ("cardinality", "kardinaliteten til x"),
        ("list", "liste av x"),
        ("tuple", "tuppel av x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}


#[test]
fn sequence_and_series_intents() -> Result<()> {
    let tests = [
        (
            "sum-1",
            r#"
            <mrow intent="sum($x)">
                <mo>&#x2211;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "sum av x",
        ),
        (
            "sum-2",
            r#"
            <mrow intent="sum($i,$x)">
                <mo>&#x2211;</mo>
                <mi arg="i">i</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "sum over i av x",
        ),
        (
            "sum-3",
            r#"
            <mrow intent="sum($i,$n,$x)">
                <mo>&#x2211;</mo>
                <mi arg="i">i</mi>
                <mi arg="n">n</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "sum fra i til n av x",
        ),
        (
            "product-1",
            r#"
            <mrow intent="product($x)">
                <mo>&#x220F;</mo>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "produkt av x",
        ),
        (
            "product-2",
            r#"
            <mrow intent="product($i,$x)">
                <mo>&#x220F;</mo>
                <mi arg="i">i</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "produkt over i av x",
        ),
        (
            "product-3",
            r#"
            <mrow intent="product($i,$n,$x)">
                <mo>&#x220F;</mo>
                <mi arg="i">i</mi>
                <mi arg="n">n</mi>
                <mi arg="x">x</mi>
            </mrow>
            "#,
            "produkt fra i til n av x",
        ),
    ];

    for (_, body, expected) in tests {
        let expr = format!(
            r#"
            <math>
                {}
            </math>
            "#,
            body
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn elementary_classical_tests() -> Result<()> {
    let tests = vec![
        // Trig
        ("sine", "sinus av x"),
        ("cosine", "cosinus av x"),
        ("tangent", "tangens av x"),
        ("secant", "sekans av x"),
        ("cosecant", "cosekans av x"),
        ("cotangent", "cotangens av x"),

        // Inverse trig
        ("arcsine", "arcus sinus av x"),
        ("arccosine", "arcus cosinus av x"),
        ("arctangent", "arcus tangens av x"),
        ("arcsecant", "arcus sekans av x"),
        ("arccosecant", "arcus cosekans av x"),
        ("arccotangent", "arcus cotangens av x"),

        // Hyperbolic trig
        ("hyperbolic-sine", "sinus hyperbolicus av x"),
        ("hyperbolic-cosine", "cosinus hyperbolicus av x"),
        ("hyperbolic-tangent", "tangens hyperbolicus av x"),
        ("hyperbolic-secant", "sekans hyperbolicus av x"),
        ("hyperbolic-cosecant", "cosekans hyperbolicus av x"),
        ("hyperbolic-cotangent", "cotangens hyperbolicus av x"),

        // Inverse hyperbolic trig
        ("arc-hyperbolic-sine", "arcus sinus hyperbolicus av x"),
        ("arc-hyperbolic-cosine", "arcus cosinus hyperbolicus av x"),
        ("arc-hyperbolic-tangent", "arcus tangens hyperbolicus av x"),
        ("arc-hyperbolic-secant", "arcus sekans hyperbolicus av x"),
        ("arc-hyperbolic-cosecant", "arcus cosekans hyperbolicus av x"),
        ("arc-hyperbolic-cotangent", "arcus cotangens hyperbolicus av x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn statistics_and_probability_tests() -> Result<()> {
    let tests = vec![
        ("mean", "gjennomsnitt av x"),
        ("standard-deviation", "standardavviket til x"),
        ("variance", "variansen til x"),
        ("median", "medianen av x"),
        ("mode", "typetallet av x"),

        // conditional probability typically two arguments
        // ("conditional-probability", "probability of x given y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "conditional-probability" => format!(
                "<math>
                    <mrow intent='{}($A,$B)'>
                        <mi>P</mi>
                        <mo>(</mo>
                        <mrow>
                          <mi arg='x'>x</mi>
                          <mo>|</mo>
                          <mi arg='y'>y</mi>
                        </mrow>
                        <mo>)</mo>
                    </mrow>
                </math>",
                intent
            ),
            _ => format!(
                "<math>
                    <mrow intent='{}($x)'>
                        <mi arg='x'>x</mi>
                    </mrow>
                </math>",
                intent
            ),
        };

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn linear_algebra_tests() -> Result<()> {
    let tests = vec![
        ("vector", "vektor av x"),
        // ("matrix", "matrix of x"),
        ("determinant", "determinanten til x"),
        ("adjugate", "adjungert av x"),
        ("magnitude", "størrelsen av x"),
        ("norm", "normen til x"),
        ("span", "spennet av x"),

        // transpose supports both postfix and function; we test function explicitly
        ("transpose", "transponert av x"),

        // dimensional product is infix
        ("dimensional-product", "x ganger y"),

        // unit-vector is prefix
        ("unit-vector", "enhetsvektor x")
    ];

    for (intent, expected) in tests {
        let expr: String = match intent {
          "dimensional-product" => {
              "<math>
                  <mrow intent='dimensional-product($x,$y)'>
                      <mi arg='x'>x</mi>
                      <mi arg='y'>y</mi>
                  </mrow>
              </math>"
              .to_string()
          }
          "transpose" => "<math>
                  <mrow intent='transpose:function($x)'>
                      <mi arg='x'>x</mi>
                  </mrow>
              </math>".to_string(),
          _ => format!(
              "<math>
                  <mrow intent='{intent}($x)'>
                      <mi arg='x'>x</mi>
                  </mrow>
              </math>"
          ),
      };

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn nofix_set_tests() -> Result<()> {
    let tests = vec![
        ("set-of-integers", "ℤ", "mengden av alle heltall"),
        ("set-of-reals", "ℝ", "mengden av alle reelle tall"),
        ("set-of-rationals", "ℚ", "mengden av alle rasjonale tall"),
        ("set-of-natural-numbers", "ℕ", "mengden av alle naturlige tall"),
        ("set-of-complex-numbers", "ℂ", "mengden av alle komplekse tall"),
        ("set-of-primes", "ℙ", "mengden av alle primtall"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn geometry_prefix_multi_value_tests() -> Result<()> {
    let tests = vec![
        ("line-segment", "linjestykke x y"),
        ("directed-line-segment", "rettet linjestykke x y"),
        ("line", "linje x y"),
        ("ray", "stråle x y"),
        ("arc", "bue x y"),
        ("point", "punkt x y z"),
    ];

    for (intent, expected) in tests {
        let expr = match intent{
            "point" => {
                format!(
                    "<math>
                        <mrow intent='{intent}($x,$y,$z)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                            <mi arg='z'>z</mi>
                        </mrow>
                    </math>"
                )
            }

            _ => { 
                format!(
                    "<math>
                        <mrow intent='{intent}($x,$y)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                        </mrow>
                    </math>"
                )
            }
        };

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn geometry_prefix_tests() -> Result<()> {
    let tests = vec![
        ("length", "lengden av x"),
        ("area", "arealet av x"),
        ("volume", "volumet av x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{intent}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>"
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn separator_tests() -> Result<()> {
  let expr = "<math>
            <mrow intent='time-separator($x,$y)'>
                <mi arg='x'>x</mi>
                <mi arg='y'>y</mi>
            </mrow>
        </math>".to_string();

    test("nb", "ClearSpeak", &expr, "x y")?;
    Ok(())
}

#[test]
fn general_concepts_tests() -> Result<()> {
    let tests = vec![
        // Unary structural
        ("fenced-group", "gruppert uttrykk av x"),
        ("highlight", "x uthevet"),
        ("least-common-denominator", "minste felles nevner til x komma; y komma; z"), // add x, y , z ...
        ("pochhammer", "permutasjon av x"),
        ("permutation-cycle", "syklisk permutasjon av x"),

        // Binary structural / infix-style
        // ("ordered-pair", "the pair of x and y"),
        ("rate", "x per y"),
        
        ("binomial-coefficient", "x over y"),
        ("embellished-name", "x med merknad y"),
        ("indexed-by", "x, senket y"),
        // ("translation", "translation by of x comma, y"), // "of" after "by" is awkward; needs dedicated rule or of-suppression
        ("constraint", "x med begrensning y"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "ordered-pair"
            | "rate"
            | "constraint"
            | "binomial-coefficient"
            | "embellished-name"
            | "translation"
            | "indexed-by" => {
                format!(
                    "<math>
                        <mrow intent='{}($a,$b)'>
                            <mi arg='a'>x</mi>
                            <mi arg='b'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
            "least-common-denominator" => {
                format!(
                    "<math>
                        <mrow intent='{}($x,$y,$z)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                            <mi arg='z'>z</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }

            // unary cases
            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
        };

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn grouping_tests() -> Result<()> {
    let tests = vec![
        ("annotation", "x som er y"),
        // ("braced-group", "grouped x end-grouped"),
        // ("repeating-decimal", "repeating decimal of x"),
    ];

    for (intent, expected) in tests {
        let expr = match intent {
            "annotation" => {
                format!(
                    "<math>
                        <mrow intent='{}($x,$y)'>
                            <mi arg='x'>x</mi>
                            <mi arg='y'>y</mi>
                        </mrow>
                    </math>",
                    intent
                )
            }
            _ => {
                format!(
                    "<math>
                        <mrow intent='{}($x)'>
                            <mi arg='x'>x</mi>
                        </mrow>
                    </math>",
                    intent
                )
            } 
        };

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn function_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("curl", "curl x"),
        ("divergence", "div x"),
        ("gradient", "gradienten til x"),
        ("laplacian", "laplas av x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn prefix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("angle", "vinkel x"),
        ("angle-measure", "vinkelmål x"),
        ("change", "endring i x"),
        ("for-all", "for alle x"),
        ("measured-angle", "målt vinkel x"),
        ("not", "ikke x"),
        ("number-of", "antall av x"),
        ("partial-derivative", "partiell derivert x"),
        ("right-angle", "rett vinkel x"),
        ("there-does-not-exist", "det eksisterer ikke x"),
        ("there-exists", "det eksisterer x"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn infix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("and", "x og y"),
        ("applied-to", "x anvendt på y"),
        ("approximately", "x omtrent y"),
        ("congruent", "x kongruent med y"),
        ("cartesian-product", "x kartesisk produkt y"),
        ("composed-with", "x sammensatt med y"),
        ("cross-product", "x kryssprodukt y"),
        ("defined-as", "x definert som y"),
        ("divided-by", "x delt på y"),
        ("divides", "x deler y"),
        ("does-not-belong-to", "x tilhører ikke y"),
        ("does-not-divide", "x deler ikke y"),
        ("dot-product", "x skalarprodukt y"),
        ("downwards-diagonal-ellipsis", "x nedadgående diagonal ellipse y"),
        ("direct-product", "x direkte produkt y"),
        ("element-of", "x element av y"),
        ("ellipsis", "x ellipse y"),
        ("equals", "x lik y"),
        ("equivalent-to", "x ekvivalent med y"),
        ("evaluates-to", "x evalueres til y"),
        ("given", "x gitt y"),
        ("greater-than", "x større enn y"),
        ("greater-than-or-equal-to", "x større enn eller lik y"),
        ("identically-equals", "x identisk lik y"),
        ("if-and-only-if", "x hvis og bare hvis y"),
        ("implies", "x impliserer y"),
        ("inner-product", "x indreprodukt y"),
        ("intersection", "x snitt y"),
        ("less-than", "x mindre enn y"),
        ("less-than-or-equal-to", "x mindre enn eller lik y"),
        ("list-separator", "x komma y"),
        ("maps-to", "x avbildes til y"),
        ("member-of", "x medlem av y"),
        ("minus", "x minus y"),
        ("minus-or-plus", "x minus eller pluss y"),
        ("not-subset", "x ikke delmengde av y"),
        ("not-superset", "x ikke overmengde av y"),
        ("not-equal-to", "x ikke lik y"),
        ("not-member-of", "x ikke medlem av y"),
        ("not-parallel-to", "x ikke parallell med y"),
        ("obtained-from", "x utledet fra y"),
        ("or", "x eller y"),
        ("outer-product", "x ytreprodukt y"),
        ("parallel-to", "x parallell med y"),
        ("perpendicular", "x vinkelrett på y"),
        ("plus", "x pluss y"),
        ("plus-or-minus", "x pluss eller minus y"),
        ("precedes", "x foregår y"),
        ("proportional", "x proporsjonal med y"),
        ("range-separator", "x til y"),
        ("ratio", "forholdet x til, y"),
        ("similar", "x tilnærmet lik y"),
        ("subset", "x delmengde av y"),
        ("subset-or-equal", "x delmengde av eller lik y"),
        ("succeeds", "x følger y"),
        ("such-that", "x slik at y"),
        ("superset", "x overmengde av y"),
        ("superset-or-equal", "x overmengde av eller lik y"),
        ("tilde", "x tilde y"),
        ("times", "x ganger y"),
        ("union", "x union y"),
        ("upwards-diagonal-ellipsis", "x oppadgående vertikal ellipse y"),
        ("vertical-ellipsis", "x vertikal ellipse y"),
        ("xor", "x ekskluderende eller y"),
    ];
    
    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($a,$b)'>
                    <mi arg='a'>x</mi>
                    <mi arg='b'>y</mi>
                </mrow>
            </math>",
            intent
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn postfix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("factorial", "x fakultet"),
        ("percent", "x prosent"),
    ];

    for (intent, expected) in tests {
        let expr = format!(
            "<math>
                <mrow intent='{}($x)'>
                    <mi arg='x'>x</mi>
                </mrow>
            </math>",
            intent
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}

#[test]
fn nofix_default_fixity_tests() -> Result<()> {
    let tests = vec![
        ("diameter", "d", "diameter"),
        ("distance", "D", "avstand"),
        ("probability", "P", "sannsynlighet"),
        ("radius", "r", "radius"),
        ("volume", "V", "volum"),
        ("exponential-e", "e", "e"),
        ("imaginary-i", "i", "i"),
        ("differential-d", "d", "d"),
        ("golden-ratio", "φ", "det gylne snitt"),
    ];

    for (intent, symbol, expected) in tests {
        let expr = format!(
            "<math>
                <mi intent='{}'>{}</mi>
            </math>",
            intent,
            symbol
        );

        test("nb", "ClearSpeak", &expr, expected)?;
    }

    Ok(())
}