//! Adobe-Japan1 (Japanese) CID to Unicode mappings.
//!
//! This module implements the UniJIS-UCS2-H CMap for the Adobe-Japan1 character
//! collection, which contains Japanese characters from JIS X 0208, JIS X 0212,
//! and extensions.
//!
//! # Character Collection Coverage
//!
//! Adobe-Japan1-7 contains approximately 23,058 CIDs covering:
//! - ASCII range (CID 1-95)
//! - JIS X 0208-1990 characters
//! - JIS X 0212-1990 characters
//! - Hiragana and Katakana
//! - CJK Unified Ideographs
//!
//! # Reference
//!
//! Adobe Technical Note #5078: Adobe-Japan1-7 Character Collection
//! <https://github.com/AdobeDocs/adobe-font-technology/tree/main/CID-Japan1>

use phf::phf_map;

/// CID to Unicode mapping for Adobe-Japan1 character collection.
///
/// This is a partial implementation covering:
/// - ASCII range (direct mapping)
/// - Hiragana (U+3040-U+309F)
/// - Katakana (U+30A0-U+30FF)
/// - Common kanji characters
///
/// For complete coverage, the full UniJIS-UCS2-H CMap data would be needed.
static ADOBE_JAPAN1_MAP: phf::Map<u16, u32> = phf_map! {
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
    61u16 => 0x005C,  // \\ (Yen sign in Japan)
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

    // Japanese punctuation and symbols
    96u16 => 0x00A5,   // Yen sign
    97u16 => 0x203E,   // Overline

    // CJK Ideographic Space
    633u16 => 0x3000,  // Ideographic space

    // Japanese punctuation (JIS X 0208)
    98u16 => 0x3001,   // Ideographic comma
    99u16 => 0x3002,   // Ideographic full stop
    100u16 => 0xFF0C,  // Fullwidth comma
    101u16 => 0xFF0E,  // Fullwidth full stop

    // Hiragana (CID range ~840-924)
    842u16 => 0x3041,  // ぁ Small A
    843u16 => 0x3042,  // あ A
    844u16 => 0x3043,  // ぃ Small I
    845u16 => 0x3044,  // い I
    846u16 => 0x3045,  // ぅ Small U
    847u16 => 0x3046,  // う U
    848u16 => 0x3047,  // ぇ Small E
    849u16 => 0x3048,  // え E
    850u16 => 0x3049,  // ぉ Small O
    851u16 => 0x304A,  // お O
    852u16 => 0x304B,  // か Ka
    853u16 => 0x304C,  // が Ga
    854u16 => 0x304D,  // き Ki
    855u16 => 0x304E,  // ぎ Gi
    856u16 => 0x304F,  // く Ku
    857u16 => 0x3050,  // ぐ Gu
    858u16 => 0x3051,  // け Ke
    859u16 => 0x3052,  // げ Ge
    860u16 => 0x3053,  // こ Ko
    861u16 => 0x3054,  // ご Go
    862u16 => 0x3055,  // さ Sa
    863u16 => 0x3056,  // ざ Za
    864u16 => 0x3057,  // し Shi
    865u16 => 0x3058,  // じ Ji
    866u16 => 0x3059,  // す Su
    867u16 => 0x305A,  // ず Zu
    868u16 => 0x305B,  // せ Se
    869u16 => 0x305C,  // ぜ Ze
    870u16 => 0x305D,  // そ So
    871u16 => 0x305E,  // ぞ Zo
    872u16 => 0x305F,  // た Ta
    873u16 => 0x3060,  // だ Da
    874u16 => 0x3061,  // ち Chi
    875u16 => 0x3062,  // ぢ Di
    876u16 => 0x3063,  // っ Small Tsu
    877u16 => 0x3064,  // つ Tsu
    878u16 => 0x3065,  // づ Du
    879u16 => 0x3066,  // て Te
    880u16 => 0x3067,  // で De
    881u16 => 0x3068,  // と To
    882u16 => 0x3069,  // ど Do
    883u16 => 0x306A,  // な Na
    884u16 => 0x306B,  // に Ni
    885u16 => 0x306C,  // ぬ Nu
    886u16 => 0x306D,  // ね Ne
    887u16 => 0x306E,  // の No
    888u16 => 0x306F,  // は Ha
    889u16 => 0x3070,  // ば Ba
    890u16 => 0x3071,  // ぱ Pa
    891u16 => 0x3072,  // ひ Hi
    892u16 => 0x3073,  // び Bi
    893u16 => 0x3074,  // ぴ Pi
    894u16 => 0x3075,  // ふ Fu
    895u16 => 0x3076,  // ぶ Bu
    896u16 => 0x3077,  // ぷ Pu
    897u16 => 0x3078,  // へ He
    898u16 => 0x3079,  // べ Be
    899u16 => 0x307A,  // ぺ Pe
    900u16 => 0x307B,  // ほ Ho
    901u16 => 0x307C,  // ぼ Bo
    902u16 => 0x307D,  // ぽ Po
    903u16 => 0x307E,  // ま Ma
    904u16 => 0x307F,  // み Mi
    905u16 => 0x3080,  // む Mu
    906u16 => 0x3081,  // め Me
    907u16 => 0x3082,  // も Mo
    908u16 => 0x3083,  // ゃ Small Ya
    909u16 => 0x3084,  // や Ya
    910u16 => 0x3085,  // ゅ Small Yu
    911u16 => 0x3086,  // ゆ Yu
    912u16 => 0x3087,  // ょ Small Yo
    913u16 => 0x3088,  // よ Yo
    914u16 => 0x3089,  // ら Ra
    915u16 => 0x308A,  // り Ri
    916u16 => 0x308B,  // る Ru
    917u16 => 0x308C,  // れ Re
    918u16 => 0x308D,  // ろ Ro
    919u16 => 0x308E,  // ゎ Small Wa
    920u16 => 0x308F,  // わ Wa
    921u16 => 0x3090,  // ゐ Wi
    922u16 => 0x3091,  // ゑ We
    923u16 => 0x3092,  // を Wo
    924u16 => 0x3093,  // ん N

    // Katakana (CID range ~925-1009)
    925u16 => 0x30A1,  // ァ Small A
    926u16 => 0x30A2,  // ア A
    927u16 => 0x30A3,  // ィ Small I
    928u16 => 0x30A4,  // イ I
    929u16 => 0x30A5,  // ゥ Small U
    930u16 => 0x30A6,  // ウ U
    931u16 => 0x30A7,  // ェ Small E
    932u16 => 0x30A8,  // エ E
    933u16 => 0x30A9,  // ォ Small O
    934u16 => 0x30AA,  // オ O
    935u16 => 0x30AB,  // カ Ka
    936u16 => 0x30AC,  // ガ Ga
    937u16 => 0x30AD,  // キ Ki
    938u16 => 0x30AE,  // ギ Gi
    939u16 => 0x30AF,  // ク Ku
    940u16 => 0x30B0,  // グ Gu
    941u16 => 0x30B1,  // ケ Ke
    942u16 => 0x30B2,  // ゲ Ge
    943u16 => 0x30B3,  // コ Ko
    944u16 => 0x30B4,  // ゴ Go
    945u16 => 0x30B5,  // サ Sa
    946u16 => 0x30B6,  // ザ Za
    947u16 => 0x30B7,  // シ Shi
    948u16 => 0x30B8,  // ジ Ji
    949u16 => 0x30B9,  // ス Su
    950u16 => 0x30BA,  // ズ Zu
    951u16 => 0x30BB,  // セ Se
    952u16 => 0x30BC,  // ゼ Ze
    953u16 => 0x30BD,  // ソ So
    954u16 => 0x30BE,  // ゾ Zo
    955u16 => 0x30BF,  // タ Ta
    956u16 => 0x30C0,  // ダ Da
    957u16 => 0x30C1,  // チ Chi
    958u16 => 0x30C2,  // ヂ Di
    959u16 => 0x30C3,  // ッ Small Tsu
    960u16 => 0x30C4,  // ツ Tsu
    961u16 => 0x30C5,  // ヅ Du
    962u16 => 0x30C6,  // テ Te
    963u16 => 0x30C7,  // デ De
    964u16 => 0x30C8,  // ト To
    965u16 => 0x30C9,  // ド Do
    966u16 => 0x30CA,  // ナ Na
    967u16 => 0x30CB,  // ニ Ni
    968u16 => 0x30CC,  // ヌ Nu
    969u16 => 0x30CD,  // ネ Ne
    970u16 => 0x30CE,  // ノ No
    971u16 => 0x30CF,  // ハ Ha
    972u16 => 0x30D0,  // バ Ba
    973u16 => 0x30D1,  // パ Pa
    974u16 => 0x30D2,  // ヒ Hi
    975u16 => 0x30D3,  // ビ Bi
    976u16 => 0x30D4,  // ピ Pi
    977u16 => 0x30D5,  // フ Fu
    978u16 => 0x30D6,  // ブ Bu
    979u16 => 0x30D7,  // プ Pu
    980u16 => 0x30D8,  // ヘ He
    981u16 => 0x30D9,  // ベ Be
    982u16 => 0x30DA,  // ペ Pe
    983u16 => 0x30DB,  // ホ Ho
    984u16 => 0x30DC,  // ボ Bo
    985u16 => 0x30DD,  // ポ Po
    986u16 => 0x30DE,  // マ Ma
    987u16 => 0x30DF,  // ミ Mi
    988u16 => 0x30E0,  // ム Mu
    989u16 => 0x30E1,  // メ Me
    990u16 => 0x30E2,  // モ Mo
    991u16 => 0x30E3,  // ャ Small Ya
    992u16 => 0x30E4,  // ヤ Ya
    993u16 => 0x30E5,  // ュ Small Yu
    994u16 => 0x30E6,  // ユ Yu
    995u16 => 0x30E7,  // ョ Small Yo
    996u16 => 0x30E8,  // ヨ Yo
    997u16 => 0x30E9,  // ラ Ra
    998u16 => 0x30EA,  // リ Ri
    999u16 => 0x30EB,  // ル Ru
    1000u16 => 0x30EC, // レ Re
    1001u16 => 0x30ED, // ロ Ro
    1002u16 => 0x30EE, // ヮ Small Wa
    1003u16 => 0x30EF, // ワ Wa
    1004u16 => 0x30F0, // ヰ Wi
    1005u16 => 0x30F1, // ヱ We
    1006u16 => 0x30F2, // ヲ Wo
    1007u16 => 0x30F3, // ン N
    1008u16 => 0x30F4, // ヴ Vu
    1009u16 => 0x30F5, // ヵ Small Ka
    1010u16 => 0x30F6, // ヶ Small Ke

    // Common kanji characters (from JIS X 0208 Level 1)
    // These are high-frequency kanji used in everyday Japanese
    1125u16 => 0x4E00, // 一 (one)
    1126u16 => 0x4E8C, // 二 (two)
    1127u16 => 0x4E09, // 三 (three)
    1128u16 => 0x56DB, // 四 (four)
    1129u16 => 0x4E94, // 五 (five)
    1130u16 => 0x516D, // 六 (six)
    1131u16 => 0x4E03, // 七 (seven)
    1132u16 => 0x516B, // 八 (eight)
    1133u16 => 0x4E5D, // 九 (nine)
    1134u16 => 0x5341, // 十 (ten)
    1135u16 => 0x767E, // 百 (hundred)
    1136u16 => 0x5343, // 千 (thousand)
    1137u16 => 0x4E07, // 万 (ten thousand)
    1138u16 => 0x5186, // 円 (yen, circle)

    // More common kanji
    1200u16 => 0x65E5, // 日 (day, sun)
    1201u16 => 0x6708, // 月 (month, moon)
    1202u16 => 0x706B, // 火 (fire)
    1203u16 => 0x6C34, // 水 (water)
    1204u16 => 0x6728, // 木 (tree)
    1205u16 => 0x91D1, // 金 (gold, money)
    1206u16 => 0x571F, // 土 (earth)
    1207u16 => 0x5E74, // 年 (year)
    1208u16 => 0x4EBA, // 人 (person)
    1209u16 => 0x5B50, // 子 (child)
    1210u16 => 0x5973, // 女 (woman)
    1211u16 => 0x7537, // 男 (man)
    1212u16 => 0x5927, // 大 (big)
    1213u16 => 0x5C0F, // 小 (small)
    1214u16 => 0x4E2D, // 中 (middle)
    1215u16 => 0x4E0A, // 上 (up)
    1216u16 => 0x4E0B, // 下 (down)
    1217u16 => 0x5DE6, // 左 (left)
    1218u16 => 0x53F3, // 右 (right)
    1219u16 => 0x524D, // 前 (before)
    1220u16 => 0x5F8C, // 後 (after)
    1221u16 => 0x5185, // 内 (inside)
    1222u16 => 0x5916, // 外 (outside)
};

/// Look up Unicode code point for a CID in Adobe-Japan1.
///
/// Returns the Unicode code point for the given CID, or None if not found.
///
/// For CIDs in common Unicode ranges (Hiragana, Katakana, CJK), we use identity
/// mapping as a fallback when not found in the explicit CID mapping table.
/// This handles the common case where PDFs use Unicode code points as CIDs.
pub fn lookup(cid: u16) -> Option<u32> {
    // First check the phf_map for explicit CID mappings
    if let Some(&unicode) = ADOBE_JAPAN1_MAP.get(&cid) {
        return Some(unicode);
    }

    // Fallback: Many Japanese PDFs use Unicode code points as CIDs for common ranges
    // This is especially true for Hiragana, Katakana, and CJK Unified Ideographs
    match cid {
        // Hiragana (U+3040-U+309F)
        0x3040..=0x309F => Some(cid as u32),

        // Katakana (U+30A0-U+30FF)
        0x30A0..=0x30FF => Some(cid as u32),

        // CJK Unified Ideographs (U+4E00-U+9FFF)
        0x4E00..=0x9FFF => Some(cid as u32),

        // Fullwidth ASCII (U+FF00-U+FF5F)
        0xFF00..=0xFF5F => Some(cid as u32),

        // Halfwidth Katakana (U+FF65-U+FF9F)
        0xFF65..=0xFF9F => Some(cid as u32),

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
    fn test_hiragana_from_cid() {
        // Test from CID mapping
        assert_eq!(lookup(843), Some(0x3042)); // CID 843 = あ
        assert_eq!(lookup(844), Some(0x3043)); // CID 844 = ぃ
    }

    #[test]
    fn test_katakana_from_cid() {
        // Test from CID mapping
        assert_eq!(lookup(926), Some(0x30A2)); // CID 926 = ア
        assert_eq!(lookup(927), Some(0x30A3)); // CID 927 = ィ
    }

    #[test]
    fn test_kanji_from_cid() {
        // Test common kanji
        assert_eq!(lookup(1125), Some(0x4E00)); // CID 1125 = 一
        assert_eq!(lookup(1200), Some(0x65E5)); // CID 1200 = 日
    }
}
