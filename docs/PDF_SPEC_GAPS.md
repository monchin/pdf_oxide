# PDF Specification Gap Analysis

**Document Version:** 0.2.6
**Last Updated:** 2026-01-09
**Reference:** ISO 32000-1:2008 (PDF 1.7)

## Executive Summary

This document analyzes the gap between PDFOxide's current implementation and the full PDF specification requirements for text extraction. Each gap is prioritized by estimated impact on real-world PDF documents.

**Current Coverage Estimate:** ~85% of PDF documents will extract correctly.

---

## Priority Legend

| Priority | Description | Target Release |
|----------|-------------|----------------|
| **P0 - Critical** | Causes data loss or mojibake for common PDFs | 0.2.6 |
| **P1 - High** | Affects significant portion of professional documents | 0.2.6 |
| **P2 - Medium** | Affects specific document types or edge cases | 0.2.6 |
| **P3 - Low** | Minor improvements or rare edge cases | 0.2.6 |

---

## Gap Analysis

### 1. ActualText Replacement (Section 14.9.4)

**Priority:** P0 - Critical
**PDF Spec Reference:** Section 14.9.4 "Replacement Text"
**Affected PDFs:** ~15-20% of all PDFs

#### Current State
- `/Alt` (alternate description) is parsed from structure tree
- `/ActualText` is NOT implemented

#### What's Missing
The `ActualText` entry provides exact replacement text for:
- Ligatures (fi, fl, ffi, ffl) → individual characters
- Mathematical formulas → readable text
- Decorated/illuminated characters → plain text
- Drop caps and special typography

#### Impact
```
Without ActualText:
  "The effect of ﬁnancial..." → "The effect of nancial..." (missing 'fi')
  "π ≈ 3.14159" → "[Formula]" or garbled output

With ActualText:
  "The effect of financial..." (correct)
  "pi approximately equals 3.14159" (accessible)
```

#### Implementation Estimate
- **Effort:** 2-3 days
- **Files:** `src/structure/parser.rs`, `src/structure/types.rs`, `src/extractors/text.rs`
- **Complexity:** Low - similar pattern to existing `/Alt` parsing

#### Affected Document Types
- Academic papers with formulas (~40% of academic PDFs)
- Professional typography with ligatures (~30% of designed PDFs)
- Accessibility-compliant PDFs (Section 508, WCAG)
- Legal documents with special characters

---

### 2. Predefined CMap Support for CJK (Section 9.10.2)

**Priority:** P0 - Critical
**PDF Spec Reference:** Section 9.10.2 "Mapping Character Codes to Unicode Values"
**Affected PDFs:** ~10-15% of all PDFs (higher in Asia-Pacific: ~60%)

#### Current State
- ToUnicode CMaps are fully supported
- Identity-H/Identity-V CMaps work for direct Unicode mapping
- Predefined CMaps have only partial stub implementations

#### What's Missing
Full CID-to-Unicode mapping tables for:

| CMap | Character Collection | Status |
|------|---------------------|--------|
| UniGB-UCS2-H | Adobe-GB1 (Simplified Chinese) | ~5% coverage |
| UniJIS-UCS2-H | Adobe-Japan1 (Japanese) | ~5% coverage |
| UniCNS-UCS2-H | Adobe-CNS1 (Traditional Chinese) | ~5% coverage |
| UniKS-UCS2-H | Adobe-Korea1 (Korean) | ~5% coverage |

#### Impact
```
Without Predefined CMaps:
  Chinese PDF: "我是中国人" → "" (empty or replacement characters)
  Japanese PDF: "日本語テスト" → "" (empty)

With Predefined CMaps:
  Correct CJK text extraction
```

#### Implementation Estimate
- **Effort:** 1-2 weeks
- **Files:** `src/fonts/font_dict.rs`, new `src/fonts/cmap_tables/` module
- **Complexity:** Medium - requires embedding ~20,000 CID-to-Unicode mappings per collection
- **Data Sources:** Adobe Technical Notes #5078-5093

#### Affected Document Types
- Chinese business/government documents
- Japanese technical manuals, manga (scanned text layers)
- Korean official documents
- Any CJK PDF without embedded ToUnicode

---

### 3. Type 0 Font Width Arrays (/W, /DW) (Section 9.7.4.3)

**Priority:** P1 - High
**PDF Spec Reference:** Section 9.7.4.3 "Glyph Metrics in CIDFonts"
**Affected PDFs:** ~20-25% of all PDFs

#### Current State
- Simple font width arrays (`/Widths`) are implemented
- CIDFont width arrays (`/W`, `/DW`) are NOT implemented

#### What's Missing
CIDFont glyph width parsing for proper spacing calculation:
```
/W [
  1 [ 500 600 700 ]      % CID 1-3 have widths 500, 600, 700
  100 200 300            % CID 100-200 all have width 300
]
/DW 1000                 % Default width for unmapped CIDs
```

#### Impact
```
Without /W parsing:
  "Hello World" → "HelloWorld" (words merge)
  Chinese text → characters overlap or have wrong spacing

With /W parsing:
  Correct word boundaries based on glyph metrics
```

#### Implementation Estimate
- **Effort:** 3-5 days
- **Files:** `src/fonts/font_dict.rs`
- **Complexity:** Medium - requires parsing variable-length array format

#### Affected Document Types
- CJK documents with embedded fonts
- Documents using custom Type 0 fonts
- PDFs from Adobe InDesign, QuarkXPress

---

### 4. Artifact Filtering (Section 14.8.2.2)

**Priority:** P1 - High
**PDF Spec Reference:** Section 14.8.2.2 "Artifacts"
**Affected PDFs:** ~30-40% of Tagged PDFs

#### Current State
- Structure tree parsing identifies content types
- Artifacts are NOT filtered during extraction

#### What's Missing
Artifacts are decorative elements that should be excluded from text extraction:
- Page numbers, headers, footers
- Background graphics
- Watermarks
- Ruled lines and borders

#### Impact
```
Without Artifact Filtering:
  "Page 1 of 10\nConfidential\nActual content here..." → noise in output
  Headers/footers repeat on every page

With Artifact Filtering:
  "Actual content here..." (clean output)
```

#### Implementation Estimate
- **Effort:** 2-3 days
- **Files:** `src/structure/parser.rs`, `src/extractors/text.rs`
- **Complexity:** Low - artifacts already identified, need filtering logic

#### Affected Document Types
- Government/legal documents with headers
- Corporate documents with footers
- Watermarked confidential documents
- Any Tagged PDF with pagination

---

### 5. Word Break Detection (Section 14.8.2.4.3)

**Priority:** P2 - Medium
**PDF Spec Reference:** Section 14.8.2.4.3 "Word Breaks"
**Affected PDFs:** ~10-15% of all PDFs

#### Current State
- TJ offset-based word detection (spec-compliant)
- Geometric spacing detection
- No explicit `/WB` (Word Break) structure element support

#### What's Missing
Explicit word break markers in Tagged PDFs:
- `/WB` structure elements
- Word-level MCID grouping

#### Impact
```
Without explicit word breaks:
  Complex layouts may have incorrect word grouping
  CJK text may have wrong segmentation

With explicit word breaks:
  Perfect word-level extraction for Tagged PDFs
```

#### Implementation Estimate
- **Effort:** 2-3 days
- **Files:** `src/structure/parser.rs`, `src/extractors/text.rs`
- **Complexity:** Low

---

### 6. TagSuspect Attribute (Section 14.8.5.2)

**Priority:** P2 - Medium
**PDF Spec Reference:** Section 14.8.5.2 "Layout Attributes"
**Affected PDFs:** ~5% of Tagged PDFs

#### Current State
Not implemented

#### What's Missing
The `TagSuspect` attribute indicates that structure tagging may be incorrect:
```
/TagSuspect true  % Structure tree may not reflect actual layout
```

When true, extraction should fall back to geometric analysis.

#### Implementation Estimate
- **Effort:** 1 day
- **Complexity:** Low

---

### 7. Soft Hyphen Handling (Section 9.10.2)

**Priority:** P2 - Medium
**PDF Spec Reference:** Section 9.10.2, Unicode Standard
**Affected PDFs:** ~5-10% of all PDFs

#### Current State
- Basic hyphenation reconstruction implemented
- Soft hyphen (U+00AD) not explicitly handled

#### What's Missing
Soft hyphens should be:
- Removed when reconstructing hyphenated words
- Preserved when copying text (optional)

#### Impact
```
Without soft hyphen handling:
  "busi-\nness" → "busi-ness" (hyphen preserved incorrectly)

With soft hyphen handling:
  "busi-\nness" → "business" (correct reconstruction)
```

#### Implementation Estimate
- **Effort:** 1 day
- **Files:** `src/text/word_boundary.rs`
- **Complexity:** Low

---

### 8. Object Reference Resolution (Structure Tree)

**Priority:** P2 - Medium
**PDF Spec Reference:** Section 14.7.2
**Affected PDFs:** ~10% of Tagged PDFs

#### Current State
- Direct structure element parsing works
- Indirect object references in structure tree need verification

#### What's Missing
Robust handling of:
- Deeply nested object references
- Circular reference detection
- Cross-page structure references

#### Implementation Estimate
- **Effort:** 3-5 days
- **Complexity:** Medium

---

### 9. Expansion of Abbreviations (/E) (Section 14.9.5)

**Priority:** P3 - Low
**PDF Spec Reference:** Section 14.9.5 "Expansion of Abbreviations"
**Affected PDFs:** ~2% of all PDFs

#### Current State
Not implemented

#### What's Missing
The `/E` entry provides expansion text for abbreviations:
```
/E (United States of America)  % Expansion for "USA"
```

#### Implementation Estimate
- **Effort:** 1 day
- **Complexity:** Low

---

### 10. Type 3 Font Support Improvements

**Priority:** P3 - Low
**PDF Spec Reference:** Section 9.6.5
**Affected PDFs:** ~1% of all PDFs

#### Current State
- Basic Type 3 font handling exists
- Complex glyph definitions may fail

#### What's Missing
- Complete Type 3 glyph rendering
- Proper bounding box calculation
- CharProc interpretation

#### Implementation Estimate
- **Effort:** 1-2 weeks
- **Complexity:** High

---

## Impact Summary by Region

| Region | Affected PDFs | Primary Gaps |
|--------|---------------|--------------|
| North America/Europe | ~15% | ActualText, Artifacts |
| China | ~40% | Predefined CMaps (GB1) |
| Japan | ~45% | Predefined CMaps (Japan1), /W arrays |
| Korea | ~35% | Predefined CMaps (Korea1) |
| Taiwan/HK | ~40% | Predefined CMaps (CNS1) |

---

## Implementation Roadmap: v0.2.6

All features targeting v0.2.6 release for comprehensive PDF spec compliance.

### Priority Order

1. **ActualText support** (P0) - 2-3 days
2. **Predefined CMap full tables** (P0) - 1-2 weeks
3. **Artifact filtering** (P1) - 2-3 days
4. **Type 0 /W array parsing** (P1) - 3-5 days
5. **Word break structure elements** (P2) - 2-3 days
6. **Soft hyphen handling** (P2) - 1 day
7. **TagSuspect attribute** (P2) - 1 day
8. **Object reference improvements** (P2) - 3-5 days
9. **Abbreviation expansion /E** (P3) - 1 day
10. **Type 3 font improvements** (P3) - 1-2 weeks

**Target Outcome:** Coverage increases from ~85% to ~99%

---

## Testing Strategy

### Regression Test Sources
1. **PDF Association Test Suite** - Official conformance tests
2. **CommonCrawl PDF Corpus** - Real-world document diversity
3. **CJK Font Foundry Samples** - Adobe, Morisawa, DynaFont test files
4. **Accessibility Test Files** - PAC3, NVDA compatibility tests

### Coverage Metrics
| Metric | Current | Target (v0.2.6) |
|--------|---------|-----------------|
| ToUnicode extraction | 98% | 99.5% |
| CJK extraction | 60% | 98% |
| Tagged PDF reading order | 85% | 99% |
| Ligature handling | 70% | 99% |

---

## References

1. ISO 32000-1:2008 - PDF 1.7 Specification
2. Adobe Technical Note #5014 - CMap and CID Font Files
3. Adobe Technical Note #5078 - Adobe-Japan1-4 Character Collection
4. Adobe Technical Note #5079 - Adobe-GB1-4 Character Collection
5. Adobe Technical Note #5080 - Adobe-CNS1-4 Character Collection
6. Adobe Technical Note #5093 - Adobe-Korea1-2 Character Collection
7. Adobe Technical Note #5411 - ToUnicode Mapping File Tutorial
8. PDF/UA-1 (ISO 14289-1) - Universal Accessibility
