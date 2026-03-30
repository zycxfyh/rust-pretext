use unicode_bidi::{BidiInfo, Level};

#[derive(Debug)]
pub struct BidiRun {
    pub level: u8,
    pub is_rtl: bool,
    pub start: usize,
    pub limit: usize,
}

/// Takes a single paragraph string and resolves its bidirectional levels.
/// Returns a list of directional runs mapped back to byte indices.
pub fn resolve_bidi(text: &str) -> Vec<BidiRun> {
    // If the text is pure ASCII (LTR), fast path:
    if text.is_ascii() {
        return vec![BidiRun {
            level: 0,
            is_rtl: false,
            start: 0,
            limit: text.len(),
        }];
    }

    // Default base level to LTR (Level 0), or let the algorithm infer it per paragraph
    let bidi_info = BidiInfo::new(text, Some(Level::ltr()));
    let levels = &bidi_info.levels; // One Level per byte in `text`

    let mut runs = Vec::new();
    if levels.is_empty() {
        return runs;
    }

    let mut current_level = levels[0];
    let mut current_start = 0;

    for (i, &lvl) in levels.iter().enumerate().skip(1) {
        // If we hit a character that isn't continuing the current run
        // OR it's a structural boundary (like newline), unicode-bidi might change the level
        if lvl != current_level {
            runs.push(BidiRun {
                level: current_level.number(),
                is_rtl: current_level.is_rtl(),
                start: current_start,
                limit: i,
            });
            current_level = lvl;
            current_start = i;
        }
    }

    // Push the final run
    runs.push(BidiRun {
        level: current_level.number(),
        is_rtl: current_level.is_rtl(),
        start: current_start,
        limit: levels.len(),
    });
    
    runs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_bidi() {
        let text = "english123عربى";
        let runs = resolve_bidi(text);
        assert!(runs.len() > 1); // Because it contains both LTR and RTL
        assert!(!runs[0].is_rtl); // "english123" is LTR
        assert!(runs[1].is_rtl);  // "عربى" is RTL
    }
}
