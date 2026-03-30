const fs = require('fs');
const { LayoutEngine, analyze_text } = require('./pkg/pretext_wasm.js');

console.log("=== Loading Font ===");
const fontBuffer = fs.readFileSync('./Roboto-Regular.ttf');
console.log("Font loaded, byte size:", fontBuffer.length);

console.log("=== Initializing Rust Wasm Engine ===");
const engine = new LayoutEngine(new Uint8Array(fontBuffer));
console.log("Engine initialized successfully in memory!");

console.log("\n=== Test 1: Advanced Text Analysis ===");
const text = "Hello AGI 春天到了. بدأت الرحلة 👨‍👩‍👦 🚀";
const analysis = analyze_text(text);
console.log("Input Text:", text);
console.log("Grapheme Count (Accurate):", analysis.grapheme_count);
console.log("Word Count:", analysis.word_count);
console.log("Is Multi-directional (Bidi):", analysis.is_multidirectional);

console.log("\n=== Test 2: Text Layout Engine ===");
console.log("Settings: FontSize=16px, MaxWidth=300px, LineHeight=24px");

const t0 = performance.now();
// (text, fontSize, maxWidth, lineHeight)
const result = engine.layout_paragraph(text, 16.0, 300.0, 24.0);
const t1 = performance.now();

console.log("Calculation Time:", (t1 - t0).toFixed(4), "ms");
console.log("Layout Width:", result.max_width.toFixed(2), "px");
console.log("Layout Height:", result.height.toFixed(2), "px");
console.log("Wrap Lines:", result.lines);

console.log("\n=== Test 3: Break-Word Fallback ===");
const longUrl = "https://verylongurl.com/something/extremely_long_that_has_no_spaces_and_should_break_harshly_if_needed_just_like_this_test";
console.log("Input Text:", longUrl);
const result2 = engine.layout_paragraph(longUrl, 16.0, 200.0, 24.0); // Narrow container
console.log("Layout Width:", result2.max_width.toFixed(2), "px");
console.log("Layout Height:", result2.height.toFixed(2), "px");
console.log("Wrap Lines:", result2.lines);
