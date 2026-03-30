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
}

/// Parses a string into segments of words, whitespaces, and punctuation
/// using proper Unicode Grapheme Cluster boundaries.
pub fn segment_text(input: &str) -> Vec<TextSegment> {
    let mut segments = Vec::new();
    let mut current_graphemes = Vec::new();
    let mut current_kind = SegmentKind::Text;

    let graphemes = input.graphemes(true);
    
    for g in graphemes {
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
                });
                current_graphemes = Vec::new();
            }
            // Add the special character as its own standalone segment
            segments.push(TextSegment {
                kind,
                graphemes: vec![g.to_string()],
            });
            current_kind = SegmentKind::Text; // reset next expected type
        } else {
            // New text block after a special char
            current_kind = SegmentKind::Text;
            current_graphemes.push(g.to_string());
        }
    }

    // Flush remaining graphemes
    if !current_graphemes.is_empty() {
        segments.push(TextSegment {
            kind: current_kind,
            graphemes: current_graphemes,
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
        assert_eq!(segs[0].graphemes, vec!["H", "e", "l", "l", "o"]);
        assert_eq!(segs[1].kind, SegmentKind::Space);
        assert_eq!(segs[2].kind, SegmentKind::Text);
        assert_eq!(segs[2].graphemes, vec!["👨‍👩‍👦"]); // Kept as ONE grapheme!
        assert_eq!(segs[3].kind, SegmentKind::Space);
        assert_eq!(segs[4].kind, SegmentKind::Tab);
        assert_eq!(segs[5].kind, SegmentKind::Text);
        assert_eq!(segs[6].kind, SegmentKind::LineBreak);
        assert_eq!(segs[7].kind, SegmentKind::Text);
    }
}
