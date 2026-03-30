use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SegmentKind {
    Text,
    Space,
    Tab,
    ZeroWidthBreak,
    SoftHyphen,
    LineBreak,
}

#[derive(Debug, Clone)]
pub struct TextSegment {
    pub kind: SegmentKind,
    pub graphemes: Vec<String>,
    // Store the exact byte boundaries from the original text string
    pub start_byte: usize,
    pub end_byte: usize,
}

/// Parses a string into segments of words, whitespaces, and punctuation
/// using proper Unicode Grapheme Cluster boundaries.
pub fn segment_text(input: &str) -> Vec<TextSegment> {
    let mut segments = Vec::new();
    let mut current_graphemes = Vec::new();
    let mut current_kind = SegmentKind::Text;

    let mut current_start = 0;
    
    let graphemes = input.grapheme_indices(true);
    
    for (i, g) in graphemes {
        let kind = match g {
            " " => SegmentKind::Space,
            "\t" => SegmentKind::Tab,
            "\u{200B}" => SegmentKind::ZeroWidthBreak,
            "\u{00AD}" => SegmentKind::SoftHyphen,
            "\n" | "\r\n" => SegmentKind::LineBreak,
            _ => SegmentKind::Text,
        };

        if kind == current_kind && kind == SegmentKind::Text {
            // Continuation of text block
            current_graphemes.push(g.to_string());
        } else if kind != SegmentKind::Text {
            // Finish current text block if any
            if !current_graphemes.is_empty() {
                segments.push(TextSegment {
                    kind: current_kind,
                    graphemes: current_graphemes,
                    start_byte: current_start,
                    end_byte: i,
                });
                current_graphemes = Vec::new();
            }
            // Add the special character as its own standalone segment
            let next_i = i + g.len();
            segments.push(TextSegment {
                kind,
                graphemes: vec![g.to_string()],
                start_byte: i,
                end_byte: next_i,
            });
            current_kind = SegmentKind::Text; // reset next expected type
            current_start = next_i;
        } else {
            // New text block after a special char (e.g. after a Space)
            current_kind = SegmentKind::Text;
            current_graphemes.push(g.to_string());
            current_start = i;
        }
    }

    // Flush remaining graphemes
    if !current_graphemes.is_empty() {
        segments.push(TextSegment {
            kind: current_kind,
            graphemes: current_graphemes,
            start_byte: current_start,
            end_byte: input.len(),
        });
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_text() {
        let segs = segment_text("Hello 👨‍👩‍👦 \tworld\n!");
        assert_eq!(segs.len(), 8);
        assert_eq!(segs[0].kind, SegmentKind::Text);
        assert_eq!(segs[0].start_byte, 0);
        assert_eq!(segs[0].end_byte, 5); // "Hello".len() = 5
        
        assert_eq!(segs[6].kind, SegmentKind::LineBreak);
    }
}
