/// Tests for:
/// *  functions including trig functions, logs, and functions to powers
/// *  implied times/functional call and explicit times/function call
/// *  parens
/// These are all intertwined, so they are in one file
use crate::common::*;

#[test]
fn trig_names() {
    let expr = "<math><mrow>
    <mi>sin</mi><mi>x</mi><mo>+</mo>
    <mi>cos</mi><mi>y</mi><mo>+</mo>
    <mi>tan</mi><mi>z</mi><mo>+</mo>
    <mi>sec</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csc</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>cot</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("nb", "SimpleSpeak", expr, "sinus av x pluss cosinus av y pluss tangens av z pluss sekans av alfa, pluss cosekans av fi, pluss cotangens av fi");
}

#[test]
fn hyperbolic_trig_names() {
    let expr = "<math><mrow>
    <mi>sinh</mi><mi>x</mi><mo>+</mo>
    <mi>cosh</mi><mi>y</mi><mo>+</mo>
    <mi>tanh</mi><mi>z</mi><mo>+</mo>
    <mi>sech</mi><mi>&#x03B1;</mi><mo>+</mo>
    <mi>csch</mi><mi>&#x03D5;</mi><mo>+</mo>
    <mi>coth</mi><mi>&#x03C6;</mi>
    </mrow></math>";
    test("nb", "SimpleSpeak", expr, "sinus hyperbolicus av x, pluss cosinus hyperbolicus av y, pluss tangens hyperbolicus av z, pluss, sekans hyperbolicus av alfa, pluss cosekans hyperbolicus av fi, pluss cotangens hyperbolicus av fi");
}


#[test]
fn inverse_trig() {
    let expr = "<math><msup><mi>sin</mi><mrow><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("nb", "SimpleSpeak", expr, "invers sinus av x");
}

#[test]
fn trig_squared() {
    let expr = "<math><msup><mi>sin</mi><mn>2</mn></msup><mi>x</mi></math>";
    test("nb", "SimpleSpeak", expr, "sinus i andre av x");
}

#[test]
fn trig_cubed() {
    let expr = "<math><msup><mi>tan</mi><mn>3</mn></msup><mi>x</mi></math>";
    test("nb", "SimpleSpeak", expr, "tangens i tredje av x");
}

#[test]
fn trig_fourth() {
    let expr = "<math><msup><mi>sec</mi><mn>4</mn></msup><mi>x</mi></math>";
    test("nb", "SimpleSpeak", expr, "sekans opphøyd i 4, av x");
}


#[test]
fn trig_power_other() {
    let expr = "<math><msup><mi>sinh</mi><mrow>><mi>n</mi><mo>-</mo><mn>1</mn></mrow></msup><mi>x</mi></math>";
    test("nb", "SimpleSpeak", expr, "sinus hyperbolicus opphøyd i n minus 1, av x");
}

#[test]
fn simple_log() {
    let expr = "<math> <mrow>  <mi>log</mi><mi>x</mi></mrow> </math>";
    test("nb", "SimpleSpeak", expr, "log av x");
}

#[test]
fn normal_log() {
    let expr = "<math><mrow><mi>log</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("nb", "SimpleSpeak", expr, "log av, startparentes; x pluss y; sluttparentes");
}

#[test]
fn simple_log_with_base() {
    let expr = "<math> <mrow>  <msub><mi>log</mi><mi>b</mi></msub><mi>x</mi></mrow> </math>";
    test("nb", "SimpleSpeak", expr, "logaritmen med grunntall b, av x");
}

#[test]
fn normal_log_with_base() {
    let expr = "<math><mrow><msub><mi>log</mi><mi>b</mi></msub><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test("nb", "SimpleSpeak", expr, "logaritmen med grunntall b, av; startparentes; x pluss y; sluttparentes");
}

#[test]
fn normal_ln() {
    let expr = "<math><mrow><mi>ln</mi><mrow><mo>(</mo><mrow><mi>x</mi><mo>+</mo><mi>y</mi></mrow><mo>)</mo></mrow></mrow></math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n av, startparentes; x pluss y; sluttparentes");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "den naturlige logaritmen av, startparentes; x pluss y; sluttparentes");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "den naturlige logaritmen av, startparentes; x pluss y; sluttparentes");
}

#[test]
fn simple_ln() {
    let expr = "<math> <mrow>  <mi>ln</mi><mi>x</mi></mrow> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "l n av x");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "l n av x");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Verbose")],
                expr, "den naturlige logaritmen av x");
}

#[test]
fn other_names() {
    let expr = "<math> <mrow><mi>Cov</mi><mi>x</mi></mrow> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "kovarians x");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "kovarians x");
    let expr = "<math> <mrow><mi>exp</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow> </math>";
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Terse")],
                expr, "exp av x");
    test_prefs("nb", "SimpleSpeak", vec![("Verbosity", "Medium")],
                expr, "eksponensialfunksjonen av x");
}

#[test]
fn explicit_function_call_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("nb", "SimpleSpeak", expr, "t av x");
}


#[test]
fn explicit_times_with_parens() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mo>(</mo><mi>x</mi><mo>)</mo></mrow></mrow></math>";
    test("nb", "SimpleSpeak", expr, "t ganger x");
}

#[test]
fn explicit_function_call() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2061;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("nb", "SimpleSpeak", expr, "t av x");
}

#[test]
fn explicit_times() {
    let expr = "<math><mrow><mi>t</mi><mo>&#x2062;</mo><mrow><mi>x</mi></mrow></mrow></math>";
    test("nb", "SimpleSpeak", expr, "t x");
}


/*
    * Tests for times
    */
#[test]
fn no_times_binomial() {
    let expr = "<math><mrow><mi>x</mi> <mo>&#x2062;</mo> <mi>y</mi></mrow></math>";
    test("nb", "SimpleSpeak", expr, "x y");
}

#[test]
fn times_following_paren() {
    let expr = "<math><mrow>
        <mn>2</mn>
        <mrow>  <mo>(</mo> <mn>3</mn>  <mo>)</mo> </mrow>
        </mrow></math>";
    test("nb", "SimpleSpeak", expr, "2 ganger 3");
}

#[test]
fn times_preceding_paren() {
    let expr = "<math><mrow>
        <mrow>  <mo>(</mo> <mn>2</mn>  <mo>)</mo> </mrow>
        <mn>3</mn>
        </mrow></math>";
    test("nb", "SimpleSpeak", expr, "2 ganger 3");
}

#[test]
fn no_times_sqrt() {
    let expr = "<math><mrow>
        <msqrt> <mi>a</mi>  </msqrt>
        <msqrt> <mi>b</mi>  </msqrt>
        <mo>=</mo>
        <msqrt> <mrow>  <mi>a</mi><mi>b</mi></mrow> </msqrt>
        </mrow></math>";
    test("nb", "SimpleSpeak", expr, 
            "kvadratroten av a; ganger kvadratroten av b; er lik, kvadratroten av a b, slutt rot");
    test_prefs("nb", "SimpleSpeak", vec![("Impairment", "LearningDisability")], expr,
            "kvadratroten av a; ganger kvadratroten av b; er lik, kvadratroten av a b, slutt rot");
}

/*
    * Tests for parens
    */
    #[test]
    fn no_parens_number() {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mn>25</mn>
        <mo>)</mo></mrow>
        <mi>x</mi>
        </mrow></math>";
        test("nb", "SimpleSpeak", expr, "25 ganger x");
    }

    #[test]
    fn no_parens_monomial() {
        let expr = "<math><mrow>
        <mi>b</mi>
        <mrow><mo>(</mo>
        <mrow><mi>x</mi><mi>y</mi></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("nb", "SimpleSpeak", expr, "b; startparentes; x y, sluttparentes");
    }

    #[test]
    fn no_parens_negative_number() {
        let expr = "<math><mrow>
        <mn>2</mn><mo>+</mo>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow>
        <mo>)</mo></mrow>
        </mrow></math>";
        test("nb", "SimpleSpeak", expr, "2 pluss minus 2");
    }


    #[test]
    fn no_parens_negative_number_with_var() {
        let expr = "<math><mrow>
        <mrow><mo>(</mo>
        <mrow><mo>&#x2212;</mo><mn>2</mn></mrow><mi>x</mi>
        <mo>)</mo></mrow>
        <mo>+</mo><mn>1</mn>
        </mrow></math>";
        test("nb", "SimpleSpeak", expr, "minus 2 x, pluss 1");
    }

    #[test]
    fn parens_superscript() {
        let expr = "<math><mrow>
        <mrow>
        <msup>
        <mrow>
            <mrow><mo>(</mo>
            <mrow> <mn>2</mn><mi>x</mi></mrow>
            <mo>)</mo></mrow></mrow>
        <mn>2</mn>
        </msup>
        </mrow>
    </mrow></math>";
        test("nb", "SimpleSpeak", expr, ", startparentes; 2 x, sluttparentes i andre");
    }

    #[test]
    fn no_parens_fraction() {
        let expr = "<math><mrow>
        <mn>2</mn>
        <mo>+</mo>
        <mrow>
            <mrow><mo>(</mo>
            <mfrac> <mn>1</mn><mn>2</mn></mfrac>
            <mo>)</mo></mrow></mrow>
    </mrow></math>";
        test("nb", "SimpleSpeak", expr, "2 pluss 1 halv");
    }


    // Tests for the four types of intervals in SimpleSpeak
    #[test]
    fn parens_interval_open_open() {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("nb", "SimpleSpeak",expr, "det åpne intervallet fra c til d");
}

#[test]
    fn parens_interval_closed_open() {
        let expr = "<math> 
        <mrow intent='closed-open-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
            <mo>)</mo></mrow>
        </math>";
    test("nb", "SimpleSpeak",expr, "det halvåpne intervallet fra og med c til d");
}


#[test]
fn parens_interval_open_closed() {
    let expr = "<math> 
    <mrow intent='open-closed-interval($start, $end)'><mo>(</mo>
        <mrow> <mo arg='open'>(</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("nb", "SimpleSpeak",expr,"det halvåpne intervallet fra c til og med d");
}


#[test]
fn parens_interval_closed_closed() {
    let expr = "<math> 
        <mrow intent='closed-interval($start, $end)'><mo>[</mo>
            <mrow> <mo arg='open'>[(]</mo><mi arg='start'>c</mi><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
            <mo>]</mo></mrow>
    </math>";
    test("nb", "SimpleSpeak",expr, "det lukkede intervallet fra c til d");
}

    #[test]
    fn parens_interval_neg_infinity_open_open() {
        let expr = "<math> 
        <mrow intent='open-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>)</mo>
        <mo>)</mo></mrow>
    </math>";
    test("nb", "SimpleSpeak",expr,
    "det åpne intervallet fra minus uendelig til d");
}

    #[test]
    fn parens_interval_neg_infinity_open_closed() {
        let expr = "<math> 
        <mrow intent='open-closed-interval($start, $end)'><mo arg='open'>(</mo>
        <mrow><mrow arg='start'><mo>-</mo> <mi>∞</mi></mrow><mo>,</mo><mi arg='end'>d</mi></mrow><mo arg='close'>]</mo>
        <mo>]</mo></mrow>
    </math>";
    test("nb", "SimpleSpeak",expr,
    "det halvåpne intervallet fra minus uendelig til og med d");
}

