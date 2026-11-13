/// Tests for rules shared between various speech styles:
/// *  this has tests focused on the various alphabets
use crate::common::*;
use anyhow::Result;


#[test]
fn silent_intent() -> Result<()> {
    let expr = "<math> <mrow intent='testing:silent($arg1, $arg2)'><mn arg='arg1'>2</mn> <mi arg='arg2'>x</mi></mrow> </math>";
    test("nb", "SimpleSpeak", expr, "2 x")?;
    test("nb", "LiteralSpeak", expr, "2 x")?;
    return Ok(());

}

#[test]
fn prefix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='testing:prefix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("nb", "SimpleSpeak", expr, "testing x")?;
    return Ok(());

}

#[test]
fn postfix_intent() -> Result<()> {
    let expr = r#"<math><msup intent='testing:postfix($x)'> <mi arg='x'>x</mi> <mi>T</mi> </msup> </math>"#;
    test("nb", "SimpleSpeak", expr, "x testing")?;
    return Ok(());

}

#[test]
fn infix_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:infix($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("nb", "SimpleSpeak", expr, "x testing y testing z testing 2")?;
    return Ok(());

}

#[test]
fn infix_intent_no_args() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:infix()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("nb", "SimpleSpeak", expr, "x")?;
    return Ok(());

}

#[test]
fn infix_intent_one_arg() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:infix($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    // Note: we say the intent name because there are infix plus/minus with a single arg due to continued rows or combined columns
    test("nb", "SimpleSpeak", expr, "testing x")?;
    return Ok(());

}

#[test]
fn function_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:function($x, $y, $z, 2)'>
        <mi arg='x'>x</mi>
        <mi arg='y'>y</mi>
        <mi arg='z'>z</mi>
    </mrow> </math>"#;
    test("nb", "SimpleSpeak", expr, "testing av x komma; y komma; z komma; 2")?;
    return Ok(());

}

#[test]
fn function_no_args_intent() -> Result<()> {
    // this is illegal intent, so it is just an mrow with one child
    let expr = r#"<math><mrow intent='testing:function()'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("nb", "SimpleSpeak", expr, "x")?;
    return Ok(());

}

#[test]
fn function_one_arg_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='testing:function($x)'>
        <mi arg='x'>x</mi>
    </mrow> </math>"#;
    test("nb", "SimpleSpeak", expr, "testing av x")?;
    return Ok(());

}

#[test]
fn silent_intent_mi() -> Result<()> {
    let expr = "<math> <mn>2</mn> <mi intent=':silent'>x</mi></math>";
    test("nb", "SimpleSpeak", expr, "2")?;
    test("nb", "ClearSpeak", expr, "2")?;
    return Ok(());

}

#[test]
fn silent_intent_msup() -> Result<()> {
    let expr = "<math>
        <msup intent='index:silent($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("nb", "SimpleSpeak", expr, "stor h 2")?;
    test("nb", "ClearSpeak", expr, "stor h 2")?;
    return Ok(());

}

#[test]
fn silent_intent_underscore() -> Result<()> {
    let expr = "<math>
        <msup intent='_($H,$n)'>
            <mi arg='H' mathvariant='normal'>H</mi>
            <mn arg='n'>2</mn>
        </msup></math>";
    test("nb", "SimpleSpeak", expr, "stor h 2")?;
    test("nb", "ClearSpeak", expr, "stor h 2")?;
    return Ok(());

}

#[test]
fn intent_prob_x() -> Result<()> {
    let expr = "<math>
    <msup intent='$op($arg)'>
        <mi arg='arg'>x</mi>
        <mi arg='op' intent='sannsynligheten' mathvariant='normal'>P</mi>
    </msup></math>";
    test("nb", "ClearSpeak", expr, "sannsynligheten av x")?;
    return Ok(());

}

#[test]
fn image_function_intent() -> Result<()> {
    let expr = "<math>
    <mrow intent='image:function($x)'>
        <mi arg='x'>f</mi>
    </mrow></math>";
    test_prefs("nb", "ClearSpeak", vec![("Verbosity", "Verbose")], expr, "bildet til f")?;
    return Ok(());

}

#[test]
fn domain_function_intent() -> Result<()> {
    let expr = r#"<math><msub intent='domain:function($f)'>
        <mi>D</mi>
        <mi arg='f'>f</mi>
    </msub></math>"#;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "definisjonsmengden til f")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "definisjonsmengden til f")?;
    return Ok(());
}

#[test]
fn codomain_function_intent() -> Result<()> {
    let expr = r#"<math><msub intent='codomain:function($f)'>
        <mi>V</mi>
        <mi arg='f'>f</mi>
    </msub></math>"#;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "verdimengden til f")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "verdimengden til f")?;
    return Ok(());
}

#[test]
fn gcd_function() -> Result<()> {
    let expr = r#"<math><mrow intent='greatest-common-divisor:function($x, $y)'>
        <mi>gcd</mi><mo>(</mo><mn arg='x'>8</mn><mo>,</mo><mn arg='y'>4</mn><mo>)</mo>
    </mrow></math>"#;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "største felles divisor til 8 og, 4")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "største felles divisor til 8 og, 4")?;
    return Ok(());
}

#[test]
fn lcm_function() -> Result<()> {
    let expr = r#"<math><mrow intent='least-common-multiple:function($x, $y)'>
        <mi>lcm</mi><mo>(</mo><mn arg='x'>2</mn><mo>,</mo><mn arg='y'>6</mn><mo>)</mo>
    </mrow></math>"#;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "minste felles multiplum til 2 og, 6")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "minste felles multiplum til 2 og, 6")?;
    return Ok(());
}


#[test]
fn set_difference_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='set-difference:function($x, $y)'>
        <mi arg='x'>A</mi>
        <mo>\</mo>
        <mn arg='y'>B</mn>
    </mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "mengdedifferanse av stor a og, stor b")?;
    test("nb", "ClearSpeak", expr, "mengdedifferanse av stor a og, stor b")?;
    return Ok(());
}

#[test]
fn cardinality_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='cardinality:function($x)'>
    <mo>|</mo>
        <mi arg='x'>A</mi>
        <mo>|</mo>
    </mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "kardinaliteten til stor a")?;
    test("nb", "ClearSpeak", expr, "kardinaliteten til stor a")?;
    return Ok(());
}

#[test]
fn sum_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='sum:function($x, $y, $z)'>
    <munderover>
    <mo>∑</mo>
        <mn arg='x'>1</mn>
        <mn arg='y'>10</mn>
        </munderover><mi arg='z'>x</mi>
    </mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "sum fra 1 til 10 av x")?;
    test("nb", "ClearSpeak", expr, "sum fra 1 til 10 av x")?;
    return Ok(());
}

#[test]
fn sin_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='sine:function($x)'>
    <mi>sin</mi><mi arg='x'>x</mi>
    </mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "sinus av x")?;
    test("nb", "ClearSpeak", expr, "sinus av x")?;
    return Ok(());
}

#[test]
fn conditional_probability_intent() -> Result<()> {
    let expr = r#"<math><mrow intent='conditional-probability:function($x, $y)'>
    <mi>P</mi><mo>(</mo>
        <mn arg='x'>A</mn>
        <mo>|</mo>
        <mn arg='y'>B</mn><mo>)</mo>
    </mrow></math>"#;
    test("nb", "SimpleSpeak", expr, "sannsynligheten av stor a gitt, stor b")?;
    test("nb", "ClearSpeak", expr, "sannsynligheten av stor a gitt, stor b")?;
    return Ok(());
}

#[test]
fn braced_group_function() -> Result<()> {
    let expr = r#"<math><mrow intent='braced-group:function($x, $y)'>
        <mo>(</mo><mi arg='x'>x</mi><mo>,</mo><mi arg='y'>y</mi><mo>)</mo>
    </mrow></math>"#;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")], expr, "gruppering av x komma; y")?;
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr, "gruppering av x komma; y")?;
    return Ok(());
}
