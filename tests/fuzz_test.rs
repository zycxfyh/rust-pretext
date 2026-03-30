use std::fs;
use pretext_wasm::layout::LayoutEngine;
use rand::Rng;

#[test]
fn test_engine_init_memory_loop() {
    // 5.1: Memory Bounds / Loop Initialization
    let font_bytes = fs::read("Roboto-Regular.ttf").expect("Failed to read font file");
    
    // We instantiate the engine 10,000 times. If there were native leaks in Harfbuzz FFI, this would blow up.
    // Notice LayoutEngine takes ownership, so we clone the Vec.
    for i in 0..10_000 {
        let engine = LayoutEngine::new(font_bytes.clone());
        if i == 0 {
            assert!(engine.layout_paragraph("Test", 16.0, 100.0, 20.0).is_ok());
        }
    }
}

#[test]
fn test_hardcore_fuzzing_unicode() {
    let font_bytes = fs::read("Roboto-Regular.ttf").expect("Failed to read font file");
    let engine = LayoutEngine::new(font_bytes);

    let mut rng = rand::rngs::OsRng;
    
    // Combine various horrific sequences
    let mut strings = vec![
        String::from(""),
        String::from("A"),
        // Huge single word
        "A".repeat(10000),
        // Massive spaces
        " ".repeat(5000),
        // ZWJ Storm
        "\u{200D}".repeat(2000),
        // Emojis with zero width joiners
        "👨‍👩‍👦".repeat(500),
        // Arabic LTR / RTL mixed hell
        "مرحبا العالم ".repeat(50) + "Hello World " + &"مرحبا ".repeat(50),
        // Formatting overrides (LRE, RLE, PDF)
        "\u{202A} LeftToRight \u{202B} RightToLeft \u{202C}".to_string(),
        // Soft Hyphens
        "super\u{00AD}cali\u{00AD}fragilistic\u{00AD}expialidocious".to_string(),
    ];

    // Generate random Unicode garbage 
    for _ in 0..100 {
        let mut random_str = String::new();
        // Insert 100 random valid chars
        for _ in 0..100 {
            // Generate valid unicode scalars
            loop {
                let code = rng.gen_range(0x0000..0x10FFFF);
                if let Some(c) = char::from_u32(code) {
                    random_str.push(c);
                    break;
                }
            }
        }
        strings.push(random_str);
    }

    // Run layout paragraph on all of them
    for s in strings {
        // We only care about it not panicking. It can wrap to 1,000 lines or return true/false widths.
        let result = engine.layout_paragraph(&s, 16.0, 200.0, 20.0);
        // It must return Ok (so no font error or invalid processing). We don't assert specific widths here.
        assert!(result.is_ok());
    }
}
