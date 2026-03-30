use std::fs;
use pretext_wasm::layout::LayoutEngine;

#[test]
fn test_integration_hello_world() {
    let font_bytes = fs::read("Roboto-Regular.ttf").expect("Failed to read font file");
    let engine = LayoutEngine::new(font_bytes);

    let result = engine.layout_paragraph("Hello World", 16.0, 300.0, 20.0).unwrap();
    
    // A standard "Hello World" in 16px Roboto is usually short and fits inside 300px width.
    // So it should be 1 line.
    assert_eq!(result.lines, 1);
    assert_eq!(result.height, 20.0);
    assert!(result.max_width > 0.0 && result.max_width < 100.0); // usually ~70-80px
}

#[test]
fn test_integration_arabic_wrapping() {
    let font_bytes = fs::read("Roboto-Regular.ttf").expect("Failed to read font file");
    let engine = LayoutEngine::new(font_bytes);

    // This long Arabic phrase contains no English letters but should wrap normally since there are spaces.
    let text = "مرحبا العالم ".repeat(50);
    let result = engine.layout_paragraph(&text, 16.0, 100.0, 20.0).unwrap();
    
    // It should definitely wrap into multiple lines since 100px is very narrow.
    assert!(result.lines > 5);
}

#[test]
fn test_integration_word_break_fallback() {
    let font_bytes = fs::read("Roboto-Regular.ttf").expect("Failed to read font file");
    let engine = LayoutEngine::new(font_bytes);

    // Single 100-character word (no spaces)
    let text = "A".repeat(100);
    // Even though it's one word, 10px width forces a wrap on almost every character.
    let narrow_result = engine.layout_paragraph(&text, 16.0, 25.0, 20.0).unwrap();
    
    // It shouldn't overflow the text box; it should break forcibly, generating multiple lines!
    assert!(narrow_result.lines > 20);
}
