pub mod segmentation;
pub mod bidi;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PreparedTextAnalysis {
    grapheme_count: usize,
    word_count: usize,
    is_multidirectional: bool,
}

#[wasm_bindgen]
impl PreparedTextAnalysis {
    #[wasm_bindgen(getter)]
    pub fn grapheme_count(&self) -> usize {
        self.grapheme_count
    }
    
    #[wasm_bindgen(getter)]
    pub fn word_count(&self) -> usize {
        self.word_count
    }
    
    #[wasm_bindgen(getter)]
    pub fn is_multidirectional(&self) -> bool {
        self.is_multidirectional
    }
}

#[wasm_bindgen]
pub fn analyze_text(text: &str) -> PreparedTextAnalysis {
    let segments = segmentation::segment_text(text);
    let runs = bidi::resolve_bidi(text);

    let mut words = 0;
    let mut total_graphemes = 0;

    for seg in segments {
        if seg.kind == segmentation::SegmentKind::Text {
            words += 1;
        }
        total_graphemes += seg.graphemes.len();
    }

    PreparedTextAnalysis {
        grapheme_count: total_graphemes,
        word_count: words,
        is_multidirectional: runs.iter().any(|r| r.is_rtl),
    }
}
