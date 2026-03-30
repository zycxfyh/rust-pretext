use rustybuzz::{Face, UnicodeBuffer};
use crate::segmentation::{segment_text, SegmentKind};
use wasm_bindgen::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

#[wasm_bindgen]
pub struct LayoutResult {
    pub max_width: f32,
    pub height: f32,
    pub lines: usize,
}

#[wasm_bindgen]
pub struct LayoutEngine {
    // We store the font file bytes internally to satisfy the lifetime of `rustybuzz::Face`
    font_bytes: Vec<u8>,
}

#[wasm_bindgen]
impl LayoutEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(font_data: Vec<u8>) -> Self {
        Self {
            font_bytes: font_data,
        }
    }

    /// Measures the total height of the text wrapped to `max_width`.
    /// `font_size` specifies the pixel size of the font (e.g., 16.0).
    /// `line_height` is the vertical advance per line (e.g., 20.0).
    pub fn layout_paragraph(&self, text: &str, font_size: f32, max_width: f32, line_height: f32) -> Result<LayoutResult, JsValue> {
        let face = Face::from_slice(&self.font_bytes, 0).ok_or_else(|| JsValue::from_str("Invalid font data"))?;
        let units_per_em = face.units_per_em() as f32;
        let scale = font_size / units_per_em;

        // 1. Resolve Bidi runs to maintain authentic Arabic/Hebrew shaping context
        let runs = crate::bidi::resolve_bidi(text);
        
        // 2. Map every byte to a precise pixel advance
        let mut byte_advances = vec![0.0_f32; text.len()];
        
        for run in runs {
            let run_str = &text[run.start..run.limit];
            let mut buffer = UnicodeBuffer::new();
            buffer.push_str(run_str);
            buffer.set_direction(if run.is_rtl { rustybuzz::Direction::RightToLeft } else { rustybuzz::Direction::LeftToRight });
            
            let glyph_buffer = rustybuzz::shape(&face, &[], buffer);
            let positions = glyph_buffer.glyph_positions();
            let infos = glyph_buffer.glyph_infos();
            
            for (info, pos) in infos.iter().zip(positions.iter()) {
                // Info cluster is the offset relative to `run_str` slice
                let actual_cluster = run.start + (info.cluster as usize);
                let pixel_width = pos.x_advance as f32 * scale;
                if actual_cluster < byte_advances.len() {
                    byte_advances[actual_cluster] += pixel_width;
                }
            }
        }

        // 3. Line breaking and wrapping (with Break-Word fallback)
        let segments = segment_text(text);
        
        let mut current_x = 0.0;
        let mut lines = 1;
        let mut true_max_width: f32 = 0.0;

        for seg in segments {
            // Aggregate the width for this intact segment
            let seg_width: f32 = byte_advances[seg.start_byte..seg.end_byte].iter().sum();

            match seg.kind {
                SegmentKind::LineBreak => {
                    lines += 1;
                    if current_x > true_max_width { true_max_width = current_x; }
                    current_x = 0.0;
                },
                SegmentKind::Space => {
                    // Space generally wraps to next line if it triggers overflow
                    if current_x + seg_width > max_width && current_x > 0.0 {
                        lines += 1;
                        if current_x > true_max_width { true_max_width = current_x; }
                        current_x = 0.0; // Space disappears at start of new line usually
                    } else {
                        current_x += seg_width;
                    }
                },
                _ => {
                    if current_x + seg_width > max_width {
                        if current_x > 0.0 {
                            // First, wrap down to a new clean line
                            lines += 1;
                            if current_x > true_max_width { true_max_width = current_x; }
                            current_x = 0.0;
                        }

                        // Overflow-wrap fallback: If the segment ALONE is bigger than max_width
                        if seg_width > max_width {
                            // Fallback to grapheme-by-grapheme breaking (break-word)
                            let grapheme_indices = text[seg.start_byte..seg.end_byte].grapheme_indices(true);
                            for (g_start, g_str) in grapheme_indices {
                                let g_actual_start = seg.start_byte + g_start;
                                let single_g_width: f32 = byte_advances[g_actual_start..(g_actual_start + g_str.len())].iter().sum();

                                if current_x + single_g_width > max_width && current_x > 0.0 {
                                    lines += 1;
                                    if current_x > true_max_width { true_max_width = current_x; }
                                    current_x = single_g_width;
                                } else {
                                    current_x += single_g_width;
                                }
                            }
                        } else {
                            current_x += seg_width;
                        }
                    } else {
                        current_x += seg_width;
                    }
                }
            }
        }
        
        if current_x > true_max_width {
            true_max_width = current_x;
        }

        Ok(LayoutResult {
            max_width: true_max_width,
            height: (lines as f32) * line_height,
            lines,
        })
    }
}
