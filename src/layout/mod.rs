//! Layout analysis algorithms for PDF documents.
//!
//! This module provides algorithms for analyzing document layout:
//! - DBSCAN clustering (characters → words → lines)
//! - Reading order determination
//! - Font clustering and normalization
//! - Bounded text extraction (v0.3.1)

use crate::geometry::Rect;

pub mod clustering;
pub mod document_analyzer;
pub mod reading_order;
pub mod text_block;

// Phase 2: Core architectural components
pub mod bold_validation;
pub mod font_normalization;

// Re-export main types
pub use document_analyzer::{AdaptiveLayoutParams, DocumentProperties};
pub use reading_order::graph_based_reading_order;
pub use text_block::{Color, FontWeight, TextBlock, TextChar, TextSpan};

// Re-export Phase 2 components
pub use bold_validation::{BoldGroup, BoldMarkerDecision, BoldMarkerValidator};
pub use font_normalization::{FontWeightNormalizer, NormalizedSpan, SpanType};

// Note: RectFilterMode, TextSpanSpatial, and TextSpanFiltering are defined below and exported directly

// Bounded text extraction (v0.3.1)

/// Filter mode for bounded text extraction.
///
/// Determines how text spans are selected based on their bounding box
/// relationship with a target region.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum RectFilterMode {
    /// Include spans that have any overlap with the target rectangle.
    #[default]
    Intersects,
    /// Include only spans that are fully contained within the target rectangle.
    FullyContained,
    /// Include spans where at least the specified fraction (0.0-1.0) overlaps
    /// with the target rectangle.
    MinOverlap(f32),
}

/// Extension trait for spatial filtering of text spans.
pub trait TextSpanSpatial {
    /// Check if this span intersects with the given rectangle.
    fn intersects_rect(&self, rect: &Rect) -> bool;

    /// Check if this span is fully contained within the given rectangle.
    fn contained_in_rect(&self, rect: &Rect) -> bool;

    /// Calculate the overlap fraction (0.0-1.0) with the given rectangle.
    ///
    /// Returns the ratio of the intersection area to this span's area.
    fn overlap_with_rect(&self, rect: &Rect) -> f32;

    /// Check if this span matches the given filter mode for a rectangle.
    fn matches_filter(&self, rect: &Rect, mode: RectFilterMode) -> bool;
}

impl TextSpanSpatial for TextSpan {
    fn intersects_rect(&self, rect: &Rect) -> bool {
        self.bbox.intersects(rect)
    }

    fn contained_in_rect(&self, rect: &Rect) -> bool {
        self.bbox.x >= rect.x
            && self.bbox.y >= rect.y
            && (self.bbox.x + self.bbox.width) <= (rect.x + rect.width)
            && (self.bbox.y + self.bbox.height) <= (rect.y + rect.height)
    }

    fn overlap_with_rect(&self, rect: &Rect) -> f32 {
        let intersection = self.bbox.intersection(rect);
        match intersection {
            Some(inter) => {
                let span_area = self.bbox.width * self.bbox.height;
                if span_area > 0.0 {
                    (inter.width * inter.height) / span_area
                } else {
                    0.0
                }
            },
            None => 0.0,
        }
    }

    fn matches_filter(&self, rect: &Rect, mode: RectFilterMode) -> bool {
        match mode {
            RectFilterMode::Intersects => self.intersects_rect(rect),
            RectFilterMode::FullyContained => self.contained_in_rect(rect),
            RectFilterMode::MinOverlap(threshold) => self.overlap_with_rect(rect) >= threshold,
        }
    }
}

/// Extension trait for filtering collections of text spans.
pub trait TextSpanFiltering {
    /// Filter spans by their spatial relationship with a rectangle.
    fn filter_by_rect(&self, rect: &Rect, mode: RectFilterMode) -> Vec<TextSpan>;

    /// Get text from spans in a rectangular region as a single string.
    fn extract_text_in_rect(&self, rect: &Rect, mode: RectFilterMode) -> String;
}

impl TextSpanFiltering for Vec<TextSpan> {
    fn filter_by_rect(&self, rect: &Rect, mode: RectFilterMode) -> Vec<TextSpan> {
        self.iter()
            .filter(|span| span.matches_filter(rect, mode))
            .cloned()
            .collect()
    }

    fn extract_text_in_rect(&self, rect: &Rect, mode: RectFilterMode) -> String {
        self.filter_by_rect(rect, mode)
            .iter()
            .map(|span| span.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl TextSpanFiltering for [TextSpan] {
    fn filter_by_rect(&self, rect: &Rect, mode: RectFilterMode) -> Vec<TextSpan> {
        self.iter()
            .filter(|span| span.matches_filter(rect, mode))
            .cloned()
            .collect()
    }

    fn extract_text_in_rect(&self, rect: &Rect, mode: RectFilterMode) -> String {
        self.filter_by_rect(rect, mode)
            .iter()
            .map(|span| span.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod bounded_extraction_tests {
    use super::*;
    use crate::layout::text_block::Color;

    fn create_test_span(text: &str, x: f32, y: f32, width: f32, height: f32) -> TextSpan {
        TextSpan {
            text: text.to_string(),
            bbox: Rect::new(x, y, width, height),
            font_name: "Helvetica".to_string(),
            font_size: 12.0,
            font_weight: FontWeight::Normal,
            is_italic: false,
            color: Color::black(),
            mcid: None,
            sequence: 0,
            split_boundary_before: false,
            offset_semantic: false,
            char_spacing: 0.0,
            word_spacing: 0.0,
            horizontal_scaling: 100.0,
            primary_detected: false,
        }
    }

    #[test]
    fn test_intersects_rect() {
        let span = create_test_span("Test", 100.0, 100.0, 50.0, 12.0);

        // Overlapping rectangle
        let rect1 = Rect::new(90.0, 90.0, 30.0, 30.0);
        assert!(span.intersects_rect(&rect1));

        // Non-overlapping rectangle
        let rect2 = Rect::new(200.0, 200.0, 50.0, 50.0);
        assert!(!span.intersects_rect(&rect2));

        // Containing rectangle
        let rect3 = Rect::new(0.0, 0.0, 500.0, 500.0);
        assert!(span.intersects_rect(&rect3));
    }

    #[test]
    fn test_contained_in_rect() {
        let span = create_test_span("Test", 100.0, 100.0, 50.0, 12.0);

        // Containing rectangle
        let rect1 = Rect::new(50.0, 50.0, 200.0, 200.0);
        assert!(span.contained_in_rect(&rect1));

        // Partial overlap (not fully contained)
        let rect2 = Rect::new(110.0, 100.0, 50.0, 50.0);
        assert!(!span.contained_in_rect(&rect2));

        // Exact match
        let rect3 = Rect::new(100.0, 100.0, 50.0, 12.0);
        assert!(span.contained_in_rect(&rect3));
    }

    #[test]
    fn test_overlap_with_rect() {
        let span = create_test_span("Test", 100.0, 100.0, 100.0, 100.0);

        // 50% overlap
        let rect1 = Rect::new(150.0, 100.0, 100.0, 100.0);
        let overlap = span.overlap_with_rect(&rect1);
        assert!((overlap - 0.5).abs() < 0.01);

        // No overlap
        let rect2 = Rect::new(300.0, 300.0, 50.0, 50.0);
        assert_eq!(span.overlap_with_rect(&rect2), 0.0);

        // Full overlap (contained)
        let rect3 = Rect::new(0.0, 0.0, 500.0, 500.0);
        assert!((span.overlap_with_rect(&rect3) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_filter_by_rect_intersects() {
        let spans = vec![
            create_test_span("Header", 100.0, 700.0, 100.0, 14.0),
            create_test_span("Body", 100.0, 500.0, 200.0, 12.0),
            create_test_span("Footer", 100.0, 50.0, 100.0, 12.0),
        ];

        // Filter for header region (top of page)
        let header_rect = Rect::new(0.0, 650.0, 612.0, 142.0);
        let filtered = spans.filter_by_rect(&header_rect, RectFilterMode::Intersects);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].text, "Header");
    }

    #[test]
    fn test_filter_by_rect_fully_contained() {
        let spans = vec![
            create_test_span("Inside", 110.0, 110.0, 80.0, 12.0),
            create_test_span("Partial", 150.0, 110.0, 100.0, 12.0), // Extends past boundary
            create_test_span("Outside", 300.0, 300.0, 50.0, 12.0),
        ];

        let boundary = Rect::new(100.0, 100.0, 100.0, 100.0);
        let filtered = spans.filter_by_rect(&boundary, RectFilterMode::FullyContained);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].text, "Inside");
    }

    #[test]
    fn test_filter_by_rect_min_overlap() {
        let spans = vec![
            create_test_span("Full", 100.0, 100.0, 50.0, 12.0), // 100% overlap
            create_test_span("Partial", 140.0, 100.0, 50.0, 12.0), // ~60% overlap
            create_test_span("Slight", 180.0, 100.0, 50.0, 12.0), // ~20% overlap
        ];

        let rect = Rect::new(100.0, 100.0, 100.0, 50.0);

        // 50% threshold
        let filtered = spans.filter_by_rect(&rect, RectFilterMode::MinOverlap(0.5));
        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().any(|s| s.text == "Full"));
        assert!(filtered.iter().any(|s| s.text == "Partial"));
    }

    #[test]
    fn test_extract_text_in_rect() {
        let spans = vec![
            create_test_span("Hello", 100.0, 100.0, 50.0, 12.0),
            create_test_span("World", 160.0, 100.0, 50.0, 12.0),
            create_test_span("Outside", 500.0, 500.0, 50.0, 12.0),
        ];

        let rect = Rect::new(50.0, 50.0, 200.0, 100.0);
        let text = spans.extract_text_in_rect(&rect, RectFilterMode::Intersects);

        assert!(text.contains("Hello"));
        assert!(text.contains("World"));
        assert!(!text.contains("Outside"));
    }

    #[test]
    fn test_empty_result() {
        let spans = vec![create_test_span("Text", 100.0, 100.0, 50.0, 12.0)];

        let rect = Rect::new(500.0, 500.0, 50.0, 50.0);
        let filtered = spans.filter_by_rect(&rect, RectFilterMode::Intersects);

        assert!(filtered.is_empty());
    }

    #[test]
    fn test_rect_filter_mode_default() {
        let mode: RectFilterMode = Default::default();
        assert_eq!(mode, RectFilterMode::Intersects);
    }
}
