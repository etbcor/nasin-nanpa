use std::{borrow::Cow, collections::HashSet};
use itertools::Itertools;

use crate::{NasinNanpaVariation, glyph_blocks::base::BASE_ALT};

/// An encoding position (either a number, or `None` which prints `-1`)
#[derive(Default, Clone)]
pub enum EncPos {
    Pos(usize),
    #[default]
    None,
}

impl EncPos {
    fn inc(&mut self) {
        *self = match self {
            EncPos::Pos(p) => EncPos::Pos(*p + 1),
            EncPos::None => EncPos::None,
        };
    }

    fn gen(&self) -> String {
        match self {
            EncPos::Pos(p) => p.to_string(),
            EncPos::None => "-1".to_string(),
        }
    }
}

/// An encoding, consisting of a fontforge position and an encoding position
#[derive(Default, Clone)]
pub struct Encoding {
    pub ff_pos: usize,
    pub enc_pos: EncPos,
}

impl Encoding {
    pub fn new(ff_pos: usize, enc_pos: EncPos) -> Self {
        Self { ff_pos, enc_pos }
    }

    pub fn gen(&self) -> String {
        format!(
            "Encoding: {ff_pos} {enc_pos} {ff_pos}",
            ff_pos = self.ff_pos,
            enc_pos = self.enc_pos.gen(),
        )
    }

    pub fn gen_ref<'a>(&self, position: Cow<'a, str>) -> String {
        let Encoding { ff_pos, enc_pos } = self;
        format!(
            "Refer: {ff_pos} {enc_pos} {position}",
            enc_pos = enc_pos.gen(),
            position = position,
        )
    }
}

/// A glyph reference (with positional data)
#[derive(Default, Clone)]
pub struct Ref<'a> {
    ref_glyph: Encoding,
    position: Cow<'a, str>,
}

impl<'a> Ref<'a> {
    pub fn new(ref_glyph: Encoding, position: impl Into<Cow<'a, str>>) -> Self {
        Self { ref_glyph, position: position.into(), }
    }

    pub fn gen(&self) -> String {
        self.ref_glyph.gen_ref(self.position.clone())
    }
}

/// A glyph representation, consisting of a spline set and references
#[derive(Default, Clone)]
pub struct Rep<'a> {
    spline_set: Cow<'a, str>,
    references: Cow<'a, [Ref<'a>]>,
}

impl<'a> Rep<'a> {
    pub fn new(
        spline_set: impl Into<Cow<'a, str>>,
        references: impl Into<Cow<'a, [Ref<'a>]>>
    ) -> Self { Self {
            spline_set: spline_set.into(),
            references: references.into(),
    } }

    pub const fn const_new(
        spline_set: Cow<'a, str>,
        references: &'a [Ref<'a>],
    ) -> Self { Self {
            spline_set,
            references: Cow::Borrowed(references),
    } }

    pub const fn const_dflt() -> Self {
        Self {
            spline_set: Cow::Borrowed(""),
            references: Cow::Borrowed(&[]),
        }
    }

    pub fn gen(&self) -> String {
        let f = if !self.spline_set.is_empty() || !self.references.is_empty() {
            "Fore\n"
        } else {
            ""
        };

        let r = self
            .references
            .clone()
            .into_iter()
            .map(|r| r.gen())
            .join("\n");

        let nl = if !self.references.is_empty() {
            "\n"
        } else {
            ""
        };

        let s = if !self.spline_set.is_empty() {
            format!("SplineSet{s}\nEndSplineSet\n", s = self.spline_set)
        } else {
            String::new()
        };

        format!("{f}{r}{nl}{s}")
    }
}

/// An anchor class, either stack or scale
#[derive(Clone)]
pub enum AnchorClass {
    Stack,
    Scale,
    Special(&'static str),
}

/// An anchor type, either base (for lower/outer) or mark (for upper/inner)
#[derive(Clone, Copy)]
pub enum AnchorType {
    Base,
    Mark,
}

/// An anchor, consisting of a class, type, and position
#[derive(Clone)]
pub struct Anchor {
    class: AnchorClass,
    ty: AnchorType,
    pos: (isize, isize),
}

impl Anchor {
    pub const fn new_stack(ty: AnchorType) -> Self {
        Self {
            class: AnchorClass::Stack,
            ty,
            pos: (
                match ty {
                    AnchorType::Base => 500,
                    AnchorType::Mark => -500,
                },
                400,
            ),
        }
    }

    pub const fn new_scale(ty: AnchorType, pos: (isize, isize)) -> Self {
        Self {
            class: AnchorClass::Scale,
            ty,
            pos,
        }
    }

    pub const fn new_special(ty: AnchorType, pos: (isize, isize), name: &'static str) -> Self {
        Self {
            class: AnchorClass::Special(name),
            ty,
            pos,
        }
    }

    fn gen(&self) -> String {
        let class = match self.class {
            AnchorClass::Stack => "stack",
            AnchorClass::Scale => "scale",
            AnchorClass::Special(s) => s,
        };
        let x = self.pos.0;
        let y = self.pos.1;
        let ty = match self.ty {
            AnchorType::Base => "basechar",
            AnchorType::Mark => "mark",
        };
        format!("AnchorPoint: \"{class}\" {x} {y} {ty} 0\n")
    }
}


/// This is the smallest building block of a glyph, containing the name, width, representation, and optional anchor
#[derive(Clone)]
pub struct GlyphBasic<'a> {
    pub name: Cow<'a, str>,
    pub width: usize,
    pub rep: Rep<'a>,
    pub anchor: Option<Anchor>,
    pub anchor2: Option<Anchor>,
}

impl<'a> GlyphBasic<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>,
        width: usize,
        rep: Rep<'a>,
        anchor: Option<Anchor>,
        anchor2: Option<Anchor>
    ) -> Self { Self {
        name: name.into(),
        width,
        rep,
        anchor,
        anchor2,
    } }

    pub const fn new_const(
        name: &'static str,
        width: usize,
        rep: Rep<'a>,
        anchor: Option<Anchor>,
        anchor2: Option<Anchor>
    ) -> Self { Self {
        name: Cow::Borrowed(name),
        width,
        rep,
        anchor,
        anchor2,
    } }
}

/// This is a `GlyphBasic` that has been assigned an `EncPos`
pub struct GlyphEnc<'a> {
    glyph: GlyphBasic<'a>,
    enc: EncPos,
}

#[allow(unused)]
impl<'a> GlyphEnc<'a> {
    pub fn from_basic(glyph: GlyphBasic<'a>, enc: EncPos) -> Self {
        Self { glyph, enc }
    }

    pub const fn from_parts(enc: EncPos, name: &'static str, width: usize, rep: Rep<'a>) -> Self {
        Self {
            glyph: GlyphBasic::new_const(name, width, rep, None, None),
            enc,
        }
    }
}

/// 
pub enum LookupsMode {
    WordLigFromLetters,
    WordLigManual(Vec<String>),
    StartCont,
    Alt,
    ComboFirst,
    ComboLast,
    None,
}

#[derive(Clone)]
pub enum Lookups {
    WordLigFromLetters,
    WordLigManual(String),
    StartCont,
    EndCont,
    Alt,
    ComboFirst,
    ComboLast,
    None,
}

impl Lookups {
    fn from_mode(mode: &LookupsMode, idx: usize) -> Self {
        match mode {
            LookupsMode::WordLigFromLetters => Lookups::WordLigFromLetters,
            LookupsMode::WordLigManual(vec) => {
                let s = &vec[idx];
                if s.len() > 0 {
                    Lookups::WordLigManual(vec[idx].clone())
                } else {
                    Lookups::None
                }
            }
            LookupsMode::StartCont => Lookups::StartCont,
            LookupsMode::Alt => Lookups::Alt,
            LookupsMode::ComboFirst => Lookups::ComboFirst,
            LookupsMode::ComboLast => Lookups::ComboLast,
            LookupsMode::None => Lookups::None,
        }
    }

    fn gen(&self, name: String, full_name: String, variation: NasinNanpaVariation) -> String {

        let to_digits = |number: &str| -> (&str, &str) {
            match number {
                "1" | "VAR01" | "arrowW"  => ("one", "zero one"),
                "2" | "VAR02" | "arrowN"  => ("two", "zero two"),
                "3" | "VAR03" | "arrowE"  => ("three", "zero three"),
                "4" | "VAR04" | "arrowS"  => ("four", "zero four"),
                "5" | "VAR05" | "arrowNW" => ("five", "zero five"),
                "6" | "VAR06" | "arrowNE" => ("six", "zero six"),
                "7" | "VAR07" | "arrowSE" => ("seven", "zero seven"),
                "8" | "VAR08" | "arrowSW" => ("eight", "zero eight"),
                "9" | "VAR09"             => ("nine", "zero nine"),
                "VAR256"            => ("two five six", "zero two five six"),
                _ => panic!("{}", number),
            }
        };


        let latin_ligs = match &self {


            // Used in tok_block and tok_ext_block when NasinNanpaVariation == Main
            Lookups::WordLigFromLetters => {
                let lig = name.chars().join(" ");
                let ali = if full_name.eq("aleTok") {
r#"Ligature2: "'liga' WORD" a l i
"#.to_string()
                } else { String::new() };

                let blank_alts = if full_name.eq("jakiTok") {
                    (2..10).map(|n|
                        format!(
r#"Ligature2: "'liga' VAR" {full_name}_VAR0{n} VAR01
Ligature2: "'liga' VAR" {full_name}_VAR0{n} one
Ligature2: "'liga' VAR" {full_name}_VAR0{n} zero one
"#                      )
                    ).collect::<String>()

                } else if full_name.eq("koTok") {
                    (2..10).map(|n|
                        format!(
r#"Ligature2: "'liga' VAR" {full_name}_VAR0{n} VAR01
Ligature2: "'liga' VAR" {full_name}_VAR0{n} one
Ligature2: "'liga' VAR" {full_name}_VAR0{n} zero one
"#                      )
                    ).collect::<String>()

                } else {
                    let alts: HashSet<char> = BASE_ALT.iter().filter_map(|g|
                        if g.name.starts_with(&name) {
                            Some(g.name
                                .chars()
                                .last()
                                .unwrap()
                                .to_string()
                                .parse()
                                .unwrap()
                            )
                        } else { None }
                    ).collect();
                    let range = HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9']);
                    let blanks = range.difference(&alts);
                    blanks.into_iter().map(|b| {
                        let (digit, padded_digit) = to_digits(&b.to_string()); 
                        format!(
r#"Ligature2: "'liga' VAR" {full_name} VAR0{b}
Ligature2: "'liga' VAR" {full_name} {digit}
Ligature2: "'liga' VAR" {full_name} {padded_digit}
"#,                      )
                    }).collect::<String>()
                };

                format!(
r#"Ligature2: "'liga' WORD" {lig}
{ali}{blank_alts}"#
                )
            }

            // Used in ctrl_block, tok_ctrl_block, and tok_no_combo_block
            Lookups::WordLigManual(word) => {

                let mut do_it = true;
                let always = if word.contains("middleDotTok") {
                    do_it = false;
                    format!(
r#"Ligature2: "'liga' VAR" {word}
"#                  )
                } else if word.contains("CartAlt") {
                    format!(
r#"Ligature2: "'liga' VAR" {which}Tok VAR01
"#,                     which = if word.contains("start") { "startCart" } else { "endCart" }
                    )
                } else if name.eq("ZWJ") {
r#"Substitution2: "'ss02' ZWJ TO STACK" joinStackTok
Substitution2: "'ss01' ZWJ TO SCALE" joinScaleTok
"#.to_string()
                } else if name.eq("startCartComb") {
r#"Ligature2: "'liga' VAR" ZWJ startCartTok
"#.to_string()
                } else if word.eq("i t a n") {
r#"Ligature2: "'liga' VAR" ijoTok ZWJ tanTok ZWJ anpaTok ZWJ nanpaTok
"#.to_string()
                } else if word.eq("l e p e k a") {
r#"Ligature2: "'liga' VAR" meliTok ZWJ kuleTok ZWJ kuleTok
"#.to_string()
                } else { String::new() };

                let latin = if variation == NasinNanpaVariation::Main && do_it {
                    if word.eq("space space") {
                        format!(
r#"Ligature2: "'liga' SPACE" {word}
Ligature2: "'liga' SPACE" z z space
Ligature2: "'liga' SPACE" z z
Ligature2: "'liga' SPACE" bar space
Ligature2: "'liga' SPACE" bar 
"#                      )
                    } else if word.eq("arrow") {
                        let convert = |c: char| match c {
                            'W' => "less",
                            'N' => "asciicircum",
                            'E' => "greater",
                            'S' => "v",
                            _ => panic!(),
                        };

                        let dir1 = convert(name.chars().nth(5).unwrap());
                        if let Some(dir2) = name.chars().nth(6) {
                            let dir2 = convert(dir2);
                            format!(
r#"Ligature2: "'liga' WORD" {dir1} {dir2}
Ligature2: "'liga' WORD" {dir2} {dir1}
"#                          )
                        } else {
                            format!(
r#"Ligature2: "'liga' WORD" {dir1}
"#                          )
                        }
                    } else if word.eq("bar") {
                        format!(
r#"Ligature2: "'liga' WORD" bar
"#                      )
                    } else if word.contains("CartAlt") {
                            format!(
r#"Ligature2: "'liga' VAR" {which}Tok VAR01
Ligature2: "'liga' VAR" {which}Tok one
"#,                             which = if word.contains("start") { "startCart" } else { "endCart" }
                            )
                    } else {
                        format!(
r#"Ligature2: "'liga' WORD" {word}
"#
                        )
                    }
                } else {
                    String::new()
                };

                format!("{always}{latin}")
            } // end Lookups::WordLigManual


            // Used in start_cont_block
            Lookups::StartCont => {
                let (glyph, joiner) = full_name.rsplit_once("_").unwrap();
                format!(
r#"Ligature2: "'liga' START CONTAINER" {glyph} {joiner}
"#              )
            }


            // Used in start_cont_block for laTok
            Lookups::EndCont => {
                let (glyph, _) = full_name.split_once("_").unwrap();
                format!(
r#"Ligature2: "'liga' START CONTAINER" endRevContTok {glyph}
"#              )
            }


            // Used in tok_alt_block
            Lookups::Alt => {
                let parts: Vec<&str> = full_name.split("_").collect();
                let glyph = parts[0];
                let sel = parts[1];
                let (digit, padded_digit) = to_digits(sel); 

                // basic case
                let basic = format!(
r#"Ligature2: "'liga' VAR" {glyph} {sel}
"#              );

                // arrows (they will also get the ligs for digits/selectors)
                let arrow = if full_name.contains("niTok_arrow") {
                    format!(
r#"Ligature2: "'liga' VAR" {glyph} ZWJ {sel}
"#                  )
                } else { String::new() };

                // determine what digit(s) should lead to this glyph
                let mut digits = if full_name.contains("VAR0") || full_name.contains("niTok_arrow") {
                    format!(
r#"Ligature2: "'liga' VAR" {glyph} {digit}
Ligature2: "'liga' VAR" {glyph} {padded_digit}
"#                  )
                } else { String::new() };

                let rerand = if full_name.contains("jakiTok") || full_name.contains("koTok") {
                    // shaddow `sel` with just the single final digit
                    let sel = sel.chars().last().unwrap().to_string();
                    let converter = |name: &str| -> String {
                        if variation == NasinNanpaVariation::Main {
                            (2..10).map(|n|
                                format!(
r#"Ligature2: "'liga' VAR" {name}_VAR0{n} VAR0{sel}
Ligature2: "'liga' VAR" {name}_VAR0{n} {digit}
Ligature2: "'liga' VAR" {name}_VAR0{n} {padded_digit}
"#)).collect::<String>()
                        } else {
                            (2..10).map(|n| format!(
r#"Ligature2: "'liga' VAR" {name}_VAR0{n} VAR0{sel}
"#)).collect::<String>()
                        }
                    };
                    if full_name.starts_with("jakiTok") {
                        converter("jakiTok")
                    } else if full_name.starts_with("koTok") {
                        converter("koTok")
                    } else { String::new() }
                } else { String::new() }; // end `let rerand`

                // special cases
                let special = if full_name.eq("aTok_VAR02") {
r#"Ligature2: "'liga' VAR" aTok ZWJ aTok
"#              } else if full_name.eq("aTok_VAR03") {
r#"Ligature2: "'liga' VAR" aTok ZWJ aTok ZWJ aTok
"#              } else if full_name.eq("aTok_VAR04") {
r#"Ligature2: "'liga' VAR" semeTok ZWJ aTok
Ligature2: "'liga' VAR" aTok ZWJ semeTok
"#              } else if full_name.eq("aTok_VAR05")
                       && variation == NasinNanpaVariation::Main {
r#"Ligature2: "'liga' VAR" aTok exclam question
Ligature2: "'liga' VAR" aTok question exclam
"#              } else if full_name.eq("muteTok_VAR02") {
r#"Ligature2: "'liga' VAR" lukaTok ZWJ lukaTok ZWJ lukaTok ZWJ lukaTok
"#              } else { "" };

                // exclude digit-based ligs in UCSUR version of font
                if variation == NasinNanpaVariation::Ucsur {
                    digits = String::new();
                }

                format!("{basic}{special}{arrow}{digits}{rerand}")
            } // end Lookups::Alt


            // Used in tok_outer_block, tok_ext_outer_block, tok_alt_outer_block,
            // tok_lower_block, tok_ext_lower_block, and tok_alt_lower_block.
            Lookups::ComboFirst => {
                let (glyph, joiner) = full_name.rsplit_once('_').unwrap();
                format!(
r#"Ligature2: "'liga' GLYPH THEN JOINER" {glyph} {joiner}
MultipleSubs2: "'ccmp' RESPAWN JOINER" {full_name} {joiner}
"#              )
            }


            // Used in tok_inner_block, tok_ext_inner_block, tok_alt_inner_block,
            // tok_upper_block, tok_ext_upper_block, and tok_alt_upper_block.
            Lookups::ComboLast => {
                let (joiner, glyph) = full_name.split_once("_").unwrap();
                format!(
r#"Ligature2: "'liga' JOINER THEN GLYPH" {joiner} {glyph}
Ligature2: "'liga' CC CLEANUP" combCartExtHalfTok {full_name}
Ligature2: "'liga' CC CLEANUP" combContExtHalfTok {full_name}
Ligature2: "'liga' CC CLEANUP" combCartExtTok {full_name}
Ligature2: "'liga' CC CLEANUP" combContExtTok {full_name}
"#              )
            }


            Lookups::None => String::new(),
        }; // end let latin_ligs

        let rand = if full_name.eq("jakiTok") {
            format!(
r#"AlternateSubs2: "'rand' RAND VARIATIONS"{vars}
"#,             vars = (2..10).map(|n| format!(" jakiTok_VAR0{n}")).collect::<String>(),
            )

        } else if full_name.eq("koTok") {
            format!(
r#"AlternateSubs2: "'rand' RAND VARIATIONS"{vars}
"#,             vars = (2..10).map(|n| format!(" koTok_VAR0{n}")).collect::<String>(),
            )
        } else { String::new() }; // end let rand

        format!("{latin_ligs}{rand}")
    }
}

#[derive(Clone)]
pub enum Cc {
    Full,
    Half,
    Participant,
    None,
}
impl Cc {
    pub fn gen(&self, full_name: String) -> String {
        match self {
            Cc::Full => format!(
r#"MultipleSubs2: "'cc01' CART" {full_name} combCartExtTok
MultipleSubs2: "'cc02' CONT" {full_name} combContExtTok
"#),
            
            Cc::Half => if full_name.eq("comma") {
r#"MultipleSubs2: "'cc01' CART" combCartExt1TickTok
MultipleSubs2: "'cc02' CONT" combContExtHalfTok
"#.to_string()

            } else if full_name.eq("quotesingle") {
r#"MultipleSubs2: "'cc01' CART" combCartExt1TopTickTok
MultipleSubs2: "'cc02' CONT" combContExtHalfTok
"#.to_string()

            } else {

                format!(
r#"MultipleSubs2: "'cc01' CART" {full_name} combCartExtHalfTok
MultipleSubs2: "'cc02' CONT" {full_name} combContExtHalfTok
"#)
            },
            
            Cc::Participant => if full_name.contains("Tick") {
                format!(
r#"MultipleSubs2: "'cc01' CART" {full_name} combCartExtNoneTok
"#)
            } else if full_name.contains("dakuten") {
                format!(
r#"MultipleSubs2: "'cc01' CART" {full_name} combCartExtHalfTok
"#)
            } else {
                format!(
r#"MultipleSubs2: "'cc01' CART" {full_name} combCartExtNoneTok
MultipleSubs2: "'cc02' CONT" {full_name} combContExtNoneTok
"#)
            },
            
            Cc::None => String::new(),
        }
    }
}

#[derive(Clone)]
pub struct GlyphFull<'a> {
    pub glyph: GlyphBasic<'a>,
    pub encoding: Encoding,
    pub lookups: Lookups,
    pub cc_subs: Cc,
}

impl<'a> GlyphFull<'a> {
    pub fn from_basic(
        glyph: GlyphBasic<'a>,
        encoding: Encoding,
        lookups: Lookups,
        cc_subs: Cc,
    ) -> Self {
        Self {
            glyph,
            encoding,
            lookups,
            cc_subs,
        }
    }

    pub fn from_enc(glyph: GlyphEnc<'a>, ff_pos: usize, lookups: Lookups, cc_subs: Cc) -> Self {
        Self {
            glyph: glyph.glyph,
            encoding: Encoding::new(ff_pos, glyph.enc),
            lookups,
            cc_subs,
        }
    }

    pub fn from_parts(
        name: impl Into<Cow<'a, str>>,
        width: usize,
        rep: Rep<'a>,
        anchor: Option<Anchor>,
        anchor2: Option<Anchor>,
        encoding: Encoding,
        lookups: Lookups,
        cc_subs: Cc,
    ) -> Self {
        Self {
            glyph: GlyphBasic::new(name, width, rep, anchor, anchor2),
            encoding,
            lookups,
            cc_subs,
        }
    }

    pub fn gen(
        &self,
        prefix: String,
        suffix: String,
        color: String,
        variation: NasinNanpaVariation,
    ) -> String {
        let name = &self.glyph.name;
        let encoding = self.encoding.gen();
        let color = format!("Colour: {color}");
        if name.contains("empty") {
            return format!(
                "\nStartChar: {name}\n{encoding}\nWidth: 0\nLayerCount: 2\n{color}\nEndChar\n"
            );
        }
        let full_name = format!("{}{}{}", prefix, name, suffix);
        let width = self.glyph.width;
        let representation = self.glyph.rep.gen();
        let lookups = self
            .lookups
            .gen(name.to_string(), full_name.clone(), variation);
        let cc_subs = self.cc_subs.gen(full_name.clone());
        let flags = if full_name.eq("ZWSP")
            || full_name.eq("ZWNJ")
            || full_name.eq("ZWJ")
            || full_name.starts_with("VAR")
            || full_name.starts_with("arrow")
            || full_name.eq("joinStackTok")
            || full_name.eq("joinScaleTok")
            || full_name.contains("space")
            || full_name.eq("combCartExtNoneTok")
            || full_name.eq("combContExtNoneTok")
            || full_name.ends_with("Rad")
        {
            "Flags: W\n"
        } else {
            ""
        };
        let anchor = if let Some(anchor) = &self.glyph.anchor { anchor.gen() } else { String::new() };
        let anchor2 = if let Some(anchor2) = &self.glyph.anchor2 { anchor2.gen() } else { String::new() };
        format!("\nStartChar: {full_name}\n{encoding}\nWidth: {width}\n{flags}{anchor2}{anchor}LayerCount: 2\n{representation}{lookups}{cc_subs}{color}\nEndChar\n")
    }
}

pub struct GlyphDescriptor {
    pub name: &'static str,
    pub spline_set: &'static str,
    pub width: Option<usize>,
    pub anchor: Option<Anchor>,
    pub anchor2: Option<Anchor>,
}

impl GlyphDescriptor {
    pub const fn new(name: &'static str, spline_set: &'static str) -> Self {
        Self {
            name,
            spline_set,
            width: None,
            anchor: None,
            anchor2: None,
        }
    }

    pub const fn new_with_width(
        name: &'static str,
        width: usize,
        spline_set: &'static str,
    ) -> Self {
        Self {
            name,
            spline_set,
            width: Some(width),
            anchor: None,
            anchor2: None,
        }
    }

    pub const fn new_with_anchor(
        name: &'static str,
        anchor: Anchor,
        spline_set: &'static str,
    ) -> Self {
        Self {
            name,
            spline_set,
            width: None,
            anchor: Some(anchor),
            anchor2: None,
        }
    }

    pub const fn new_with_anchors(
        name: &'static str,
        anchor: Anchor,
        anchor2: Anchor,
        spline_set: &'static str,
    ) -> Self {
        Self {
            name,
            spline_set,
            width: None,
            anchor: Some(anchor),
            anchor2: Some(anchor2),
        }
    }
}

pub struct GlyphBlock<'a> {
    pub glyphs: Vec<GlyphFull<'a>>,
    pub prefix: Cow<'a, str>,
    pub suffix: Cow<'a, str>,
    pub color: Cow<'a, str>,
}

impl<'a> GlyphBlock<'a> {
    pub fn from_enc_glyphs(
        ff_pos: &mut usize,
        glyphs: Vec<GlyphEnc<'a>>,
        lookups: LookupsMode,
        cc_subs: Cc,
        prefix: impl Into<Cow<'a, str>>,
        suffix: impl Into<Cow<'a, str>>,
        color: impl Into<Cow<'a, str>>,
    ) -> Self {
        let mut glyphs: Vec<GlyphFull> = glyphs
            .into_iter()
            .enumerate()
            .map(|(idx, glyph)| {
                let g = GlyphFull::from_enc(
                    glyph,
                    *ff_pos,
                    Lookups::from_mode(&lookups, idx),
                    cc_subs.clone(),
                );
                *ff_pos += 1;
                g
            })
            .collect();

        let mut padding = Self::new_empty(ff_pos, 15 - ((glyphs.len() + 15) % 16), 0).glyphs;
        glyphs.append(&mut padding);

        Self {
            glyphs,
            prefix: prefix.into(),
            suffix: suffix.into(),
            color: color.into(),
        }
    }

    pub fn from_basic_glyphs(
        ff_pos: &mut usize,
        glyphs: Vec<GlyphBasic<'a>>,
        lookups: LookupsMode,
        cc_subs: Cc,
        prefix: impl Into<Cow<'a, str>>,
        suffix: impl Into<Cow<'a, str>>,
        color: impl Into<Cow<'a, str>>,
        mut enc_pos: EncPos,
    ) -> Self {
        let mut glyphs: Vec<GlyphFull> = glyphs
            .into_iter()
            .enumerate()
            .map(|(idx, glyph)| {
                let g = GlyphFull::from_basic(
                    glyph,
                    Encoding::new(*ff_pos, enc_pos.clone()),
                    Lookups::from_mode(&lookups, idx),
                    cc_subs.clone(),
                );
                *ff_pos += 1;
                enc_pos.inc();
                g
            })
            .collect();

        let mut padding = Self::new_empty(ff_pos, 15 - ((glyphs.len() + 15) % 16), 0).glyphs;
        glyphs.append(&mut padding);

        Self {
            glyphs,
            prefix: prefix.into(),
            suffix: suffix.into(),
            color: color.into(),
        }
    }

    pub fn from_const_descriptors(
        ff_pos: &mut usize,
        glyphs: &'static [GlyphDescriptor],
        lookups: LookupsMode,
        cc_subs: Cc,
        prefix: impl Into<Cow<'a, str>>,
        suffix: impl Into<Cow<'a, str>>,
        color: impl Into<Cow<'a, str>>,
        enc_pos: EncPos,
        fallback_width: usize,
    ) -> Self {
        let glyphs: Vec<GlyphBasic> = glyphs
            .into_iter()
            .map(
                |GlyphDescriptor {
                     name,
                     spline_set,
                     width,
                     anchor,
                     anchor2,
                 }| {
                    GlyphBasic::new(
                        name.to_string(),
                        width.unwrap_or(fallback_width),
                        Rep::new(spline_set.to_string(), &[]),
                        anchor.clone(),
                        anchor2.clone(),
                    )
                },
            )
            .collect();

        Self::from_basic_glyphs(
            ff_pos, glyphs, lookups, cc_subs, prefix, suffix, color, enc_pos,
        )
    }

    pub fn from_const_encs(
        ff_pos: &mut usize,
        glyphs: &'a [GlyphEnc],
        lookups: LookupsMode,
        cc_subs: Cc,
        prefix: impl Into<Cow<'a, str>>,
        suffix: impl Into<Cow<'a, str>>,
        color: impl Into<Cow<'a, str>>,
    ) -> Self {
        let glyphs: Vec<GlyphEnc> = glyphs
            .into_iter()
            .map(
                |GlyphEnc { glyph, enc }| {
                    GlyphEnc {
                        glyph: GlyphBasic::new(
                            glyph.name.to_string(),
                            glyph.width,
                            Rep::new(glyph.rep.spline_set.to_string(), &[]),
                            glyph.anchor.clone(),
                            glyph.anchor2.clone(),
                        ),
                        enc: enc.clone(),
                    }
                },
            )
            .collect();

        Self::from_enc_glyphs(
            ff_pos, glyphs, lookups, cc_subs, prefix, suffix, color,
        )
    }

    /// Generates a `GlyphBlock` whose glyphs are all references this block's glyphs, all with the same `rel_pos`
    pub fn from_refs(
        &self,
        ff_pos: &mut usize,
        rel_pos: String,
        lookups: LookupsMode,
        cc_subs: Cc,
        use_full_names: bool,
        prefix: impl Into<Cow<'a, str>>,
        suffix: impl Into<Cow<'a, str>>,
        color: impl Into<Cow<'a, str>>,
        width: Option<usize>,
        anchor: Option<Anchor>,
    ) -> Self {
        let glyphs: Vec<GlyphBasic> = self
            .glyphs
            .clone()
            .into_iter()
            .map(
                |GlyphFull {
                     glyph, encoding, ..
                 }| -> GlyphBasic {
                    let refs: Vec<Ref> = vec![
                        Some(Ref::new(encoding, rel_pos.clone())),
                        None,
                    ]
                    .into_iter()
                    .flatten()
                    .collect();
                    let name: Cow<'a, str> = if use_full_names {
                        Cow::Owned(format!(
                            "{pre}{name}{post}",
                            pre = self.prefix,
                            name = glyph.name,
                            post = self.suffix
                        ))
                    } else {
                        glyph.name
                    };
                    let g = GlyphBasic::new(
                        name,
                        match width {
                            Some(width) => width,
                            None => glyph.width,
                        },
                        Rep::new(Cow::default(), refs),
                        match &anchor {
                            Some(anchor) => Some(anchor.clone()),
                            None => glyph.anchor,
                        },
                        None,
                    );
                    g
                },
            )
            .collect();

        Self::from_basic_glyphs(
            ff_pos,
            glyphs,
            lookups,
            cc_subs,
            prefix,
            suffix,
            color,
            EncPos::None,
        )
    }

    /// Generates a `GlyphBlock` with a given `count` of empty glyphs
    pub fn new_empty(ff_pos: &mut usize, count: usize, width: usize) -> Self {
        let end = *ff_pos + count;
        let mut glyphs = vec![];

        while *ff_pos < end {
            glyphs.push(GlyphFull::from_parts(
                format!("empty{i:04}", i = *ff_pos),
                width,
                Rep::default(),
                None,
                None,
                Encoding::new(*ff_pos, EncPos::None),
                Lookups::None,
                Cc::None,
            ));
            *ff_pos += 1;
        }

        Self {
            glyphs,
            prefix: Cow::Borrowed(""),
            suffix: Cow::Borrowed(""),
            color: Cow::Borrowed("dddddd"),
        }
    }

    /// Generates a `GlyphBlock`
    pub fn gen(&self, variation: NasinNanpaVariation) -> String {
        let mut s = String::new();
        for g in &self.glyphs {
            s += &g.gen(
                self.prefix.to_string(),
                self.suffix.to_string(),
                self.color.to_string(),
                variation,
            )
        }
        s
    }
}
