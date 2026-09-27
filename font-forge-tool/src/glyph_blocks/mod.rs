#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod ctrl;
pub mod base;
pub mod outer;
pub mod inner;
pub mod lower;

//MARK: HEADERS
pub const HEADER: &str = r#"SplineFontDB: 3.2
FontName: nasin-nanpa
FullName: nasin-nanpa
FamilyName: nasin-nanpa
Weight: Regular
Copyright: jan Itan li mama. jan mute a li pona e pali ona.
"#;

pub const VERSION: &str = "5.0.0-beta.4";

pub const DETAILS1: &str = r#"ItalicAngle: 0
UnderlinePosition: 0
UnderlineWidth: 0
Ascent: 900
Descent: 100
InvalidEm: 0
sfntRevision: 0x00010000
LayerCount: 2
Layer: 0 0 "Back" 1
Layer: 1 0 "Fore" 0
XUID: [1021 700 1229584016 12833]
StyleMap: 0x0040
FSType: 0
OS2Version: 4
OS2_WeightWidthSlopeOnly: 0
OS2_UseTypoMetrics: 0
CreationTime: 1640950552
"#;

pub const DETAILS2: &str = r#"
PfmFamily: 81
TTFWeight: 400
TTFWidth: 5
LineGap: 0
VLineGap: 0
Panose: 0 0 8 0 0 0 0 6 0 0
OS2TypoAscent: 1000
OS2TypoAOffset: 0
OS2TypoDescent: 0
OS2TypoDOffset: 0
OS2TypoLinegap: 0
OS2WinAscent: 1000
OS2WinAOffset: 0
OS2WinDescent: 386
OS2WinDOffset: 0
HheadAscent: 1000
HheadAOffset: 0
HheadDescent: -386
HheadDOffset: 0
OS2SubXSize: 650
OS2SubYSize: 699
OS2SubXOff: 0
OS2SubYOff: 140
OS2SupXSize: 650
OS2SupYSize: 699
OS2SupXOff: 0
OS2SupYOff: 479
OS2StrikeYSize: 49
OS2StrikeYPos: 258
OS2CapHeight: 1000
OS2XHeight: 500
OS2Vendor: 'XXXX'
OS2CodePages: 00000001.00000000
OS2UnicodeRanges: 0000000f.00000000.00000000.00000000
"#;

pub const LOOKUPS: &str = r#"Lookup: 4 0 0 "'liga' SPACE" { "'liga' SPACE"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'half' RESTORE SPACE" { "'half' RESTORE SPACE"  } ['half' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' WORDS" { "'liga' WORD"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 3 0 0 "'rand' RAND VARIATIONS" { "'rand' RAND VARIATIONS"  } ['rand' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' VARIATIONS" { "'liga' VAR"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' START CONTAINER" { "'liga' START CONTAINER"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 5 0 0 "'calt' CHANGE ZWJ" { "'calt' CHANGE ZWJ"  } ['calt' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 1 0 0 "'ss01' ZWJ TO SCALE" { "'ss01' ZWJ TO SCALE"  } ['ss01' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 1 0 0 "'ss02' ZWJ TO STACK" { "'ss02' ZWJ TO STACK"  } ['ss02' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' GLYPH THEN JOINER" { "'liga' GLYPH THEN JOINER"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 2 0 0 "'ccmp' RESPAWN JOINER" { "'ccmp' RESPAWN JOINER"  } ['ccmp' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' JOINER THEN GLYPH" { "'liga' JOINER THEN GLYPH"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 6 0 0 "'calt' CART AND CONT" { "'calt' CART AND CONT"  } ['calt' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 2 2 0 "'cc01' CART" { "'cc01' CART"  } ['cc01' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 2 2 0 "'cc02' CONT" { "'cc02' CONT"  } ['cc02' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' CC CLEANUP" { "'liga' CC CLEANUP"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 257 0 0 "'sqsh' SPACE SHIFT" { "'sqsh' SPACE SHIFT"  } ['sqsh' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 260 0 0 "'mark' POSITION COMBO" { "'mark' SPECIAL"  "'mark' STACK"  "'mark' SCALE"  } ['mark' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
MarkAttachClasses: 1
"#;

pub const AFTER_ZWJ_CALT: &str = r#" 2 0 0
  ClsList: 2 1
  BClsList:
  FClsList:
 1
  SeqLookup: 1 "'ss01' ZWJ TO SCALE"
 2 0 0
  ClsList: 3 1
  BClsList:
  FClsList:
 1
  SeqLookup: 1 "'ss02' ZWJ TO STACK"
  ClassNames: "other" "zwj" "scale" "stack"
  BClassNames: "other" "zwj" "scale" "stack"
  FClassNames: "other" "zwj" "scale" "stack"
EndFPST
"#;

pub const AFTER_CHAIN_CALT: &str = r#" 1 1 0
  ClsList: 1
  BClsList: 2
  FClsList:
 1
  SeqLookup: 0 "'cc01' CART"
 1 1 0
  ClsList: 1
  BClsList: 3
  FClsList:
 1
  SeqLookup: 0 "'cc02' CONT"
  ClassNames: "other" "base" "cart" "cont"
  BClassNames: "other" "base" "cart" "cont"
  FClassNames: "other" "base" "cart" "cont"
EndFPST
LangName: 1033 "" "" "" "" "" ""#;

pub const OTHER: &str = r#"" "" "+ACIA-jan Itan 2023+ACIA" "+ACIAIgAA" "+ACIA-jan Itan+ACIA" "+ACIAIgAA" "+ACIAIgAA" "+ACIA-https://etbcor.com/+ACIA" "+ACIA-MIT License+ACIA" "+ACIA-https://opensource.org/licenses/MIT+ACIA" "" "nasin-nanpa" "Regular"
Encoding: Custom
UnicodeInterp: none
NameList: AGL For New Fonts
DisplaySize: -72
AntiAlias: 1
FitToEm: 1
WinInfo: 0 8 16
BeginPrivate: 12
BlueValues 22 [-2 1 414 417 796 797]
OtherBlues 11 [-385 -384]
BlueFuzz 1 1
BlueScale 8 0.039625
BlueShift 1 7
StdHW 5 [100]
StdVW 5 [100]
StemSnapH 5 [100]
StemSnapV 5 [100]
ForceBold 5 false
LanguageGroup 1 0
ExpansionFactor 4 0.06
EndPrivate
AnchorClass2: "tokipona" "'mark' SPECIAL" "stack" "'mark' STACK" "scale" "'mark' SCALE"
"#;

