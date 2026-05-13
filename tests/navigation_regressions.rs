#![allow(non_snake_case)]

mod common;

use common::*;
use anyhow::Result;

// Shared setup for navigation regression tests.
fn init_navigation_prefs(mathml: &str, language: &str, nav_mode: &str) {
    set_rules_dir(abs_rules_dir_path()).unwrap();
    set_preference("Language", language).unwrap();
    set_preference("SpeechStyle", "SimpleSpeak").unwrap();
    set_preference("NavMode", nav_mode).unwrap();
    set_preference("NavVerbosity", "Verbose").unwrap();
    set_preference("Overview", "False").unwrap();
    set_preference("AutoZoomOut", "True").unwrap();
    set_mathml(mathml).unwrap();
}

#[test]
fn nb_enhanced_down_arrow_zoom_in_does_not_panic() -> Result<()> {
    // Down arrow keyCode is 0x28. In Enhanced mode this maps to ZoomIn.
    let mathml = "<math id='id-0'><mfrac id='mfrac'><mi id='id-1'>x</mi><mi id='id-2'>y</mi></mfrac></math>";
    init_navigation_prefs(mathml, "nb", "Enhanced");

    let response = do_navigate_keypress(0x28, false, false, false, false);
    assert!(
        response.is_ok(),
        "Pil ned i nb/Enhanced feilet: {:?}",
        response.err()
    );
    Ok(())
}

#[test]
fn move_last_location_from_initial_state_does_not_panic() -> Result<()> {
    let mathml = "<math id='id-0'><mrow id='id-1'><mi id='id-2'>x</mi><mo id='id-3'>+</mo><mi id='id-4'>y</mi></mrow></math>";
    init_navigation_prefs(mathml, "nb", "Enhanced");

    // This previously exposed stack-underflow behavior.
    let response = do_navigate_command("MoveLastLocation");
    assert!(
        response.is_ok(),
        "MoveLastLocation fra initial state feilet: {:?}",
        response.err()
    );
    Ok(())
}
