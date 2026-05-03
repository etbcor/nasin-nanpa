use ffir::*;
use glyph_blocks::{base::*, ctrl::*, inner::*, lower::*, outer::*, *};
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, fs::File, io::Write};

mod ffir;
mod glyph_blocks;

#[derive(PartialEq, Eq, Clone, Copy)]
enum NasinNanpaVariation {
    Main,
    Ucsur,
}

impl Display for NasinNanpaVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NasinNanpaVariation::Main => "main",
            NasinNanpaVariation::Ucsur => "UCSUR",
        })
    }
}

fn gen_nasin_nanpa(variation: NasinNanpaVariation) -> std::io::Result<()> {
    print!("[fft]: Generating nasitan ({} v{})... ", variation, VERSION);
    let mut ff_pos: usize = 0;

    let ctrl_temp = CTRL;
    let mut ctrl_block = GlyphBlock::from_const_encs(
        &mut ff_pos,
        &ctrl_temp,
        LookupsMode::WordLigManual(vec![
            String::new(),                                     // NUL
            String::new(),                                     // ZWSP
            String::new(),                                     // ZWNJ
            "ampersand".to_string(),                           // ZWJ
            String::new(),                               // 1 tick
            String::new(),                         // 2 ticks
            String::new(),                   // 3 ticks
            String::new(),             // 4 ticks
            String::new(),       // 5 ticks
            String::new(), // 6 ticks
            String::new(),                         // top tick
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(), // dakuten
            String::new(), // handakuten
            // "comma".to_string(),                               // 1 tick
            // "comma comma".to_string(),                         // 2 ticks
            // "comma comma comma".to_string(),                   // 3 ticks
            // "comma comma comma comma".to_string(),             // 4 ticks
            // "comma comma comma comma comma".to_string(),       // 5 ticks
            // "comma comma comma comma comma comma".to_string(), // 6 ticks
            // "quotesingle".to_string(),                         // top tick
            // "quotesingle quotesingle".to_string(),
            // "quotesingle quotesingle quotesingle".to_string(),
            // "quotesingle quotesingle quotesingle quotesingle".to_string(),
            // "quotesingle quotesingle quotesingle quotesingle quotesingle".to_string(),
            // "quotesingle quotesingle quotesingle quotesingle quotesingle quotesingle".to_string(),
            // "quotedbl".to_string(), // dakuten
            // "asterisk".to_string(), // handakuten
            String::new(),          // combCartExtHalfTok
            String::new(),          // combContExtHalfTok
            String::new(),          // combCartExtNoneTok
            String::new(),          // combContExtNoneTok
            String::new(),          // VAR01
            String::new(),          // VAR02
            String::new(),          // VAR03
            String::new(),          // VAR04
            String::new(),          // VAR05
            String::new(),          // VAR06
            String::new(),          // VAR07
            String::new(),          // VAR08
            String::new(),          // VAR09
            String::new(),          // VAR256
            "arrow".to_string(),    // arrowW
            "arrow".to_string(),    // arrowN
            "arrow".to_string(),    // arrowE
            "arrow".to_string(),    // arrowS
            "arrow".to_string(),    // arrowNW
            "arrow".to_string(),    // arrowNE
            "arrow".to_string(),    // arrowSE
            "arrow".to_string(),    // arrowSW
        ]),
        Cc::Participant,
        "",
        "",
        "fa6791",
    );
    // NUL char
    ctrl_block.glyphs[0].cc_subs = Cc::None;

    let mut tok_ctrl_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        TOK_CTRL.as_slice(),
        LookupsMode::WordLigManual(vec![
            "bracketleft".to_string(),
            "bracketright".to_string(),
            "equal".to_string(),
            String::new(),
            String::new(),
            "hyphen".to_string(),
            "plus".to_string(),
            "parenleft".to_string(),
            "parenright".to_string(),
            "underscore".to_string(),
            "braceleft".to_string(),
            "braceright".to_string(),
            "startCartAlt".to_string(),
            "endCartAlt".to_string(),
            "t e".to_string(),
            "t o".to_string(),
            "ZWJ startCartTok".to_string(),
        ]),
        Cc::None,
        "",
        "Tok",
        "aaafff",
        EncPos::Pos(0xF1990),
        0,
    );
    // join stack & join scale
    tok_ctrl_block.glyphs[5].cc_subs = Cc::Participant;
    tok_ctrl_block.glyphs[6].cc_subs = Cc::Participant;
    // alt cartouche
    tok_ctrl_block.glyphs[12].encoding.enc_pos = EncPos::None;
    tok_ctrl_block.glyphs[13].encoding.enc_pos = EncPos::None;
    // start cartouche, combining
    tok_ctrl_block.glyphs[14].cc_subs = Cc::Participant;
    tok_ctrl_block.glyphs[14].encoding.enc_pos = EncPos::None;

    let mut start_cont_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        START_CONT.as_slice(),
        LookupsMode::StartCont,
        Cc::None,
        "",
        "_startContTok",
        "aaafff",
        EncPos::None,
        1000,
    );
    // reverse extended la
    start_cont_block.glyphs[7].lookups = Lookups::EndCont;

    let latn_block = if variation == NasinNanpaVariation::Main {
        GlyphBlock::from_const_descriptors(
            &mut ff_pos,
            LATN.as_slice(),
            LookupsMode::None,
            Cc::Half,
            "",
            "",
            "fffaaa",
            EncPos::Pos(0x0020),
            500,
        )
    } else {
        GlyphBlock::new_empty(&mut ff_pos, 0, 0)
    };

    let mut no_comb_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        TOK_NO_COMB.as_slice(),
        LookupsMode::WordLigManual(vec![
            "period".to_string(),
            "colon".to_string(),
            "middleDotTok middleDotTok".to_string(),
            "middleDotTok middleDotTok middleDotTok".to_string(),
            "space space".to_string(),
            "i t a n".to_string(),
            "l i p a m a n k a".to_string(),
            "l e p e k a".to_string(),
            "S e k a".to_string(),
            "L i n k u".to_string(),
        ]),
        Cc::Full,
        "",
        "Tok",
        "cccfff",
        EncPos::None,
        1000,
    );
    // dot and colon
    no_comb_block.glyphs[0].encoding.enc_pos = EncPos::Pos(0xF199C);
    no_comb_block.glyphs[1].encoding.enc_pos = EncPos::Pos(0xF199D);
    // ideographic space
    no_comb_block.glyphs[4].encoding.enc_pos = EncPos::Pos(0x3000);

    let radicals_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        RADICALS.as_slice(),
        LookupsMode::None,
        Cc::Full,
        "",
        "Rad",
        "7777cc",
        EncPos::Pos(0xF1C80),
        1000,
    );

    let base_core_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        BASE_CORE.as_slice(),
        if variation == NasinNanpaVariation::Main {
            LookupsMode::WordLigFromLetters
        } else {
            LookupsMode::None
        },
        Cc::Full,
        "",
        "Tok",
        "bf80ff",
        EncPos::Pos(0xF1900),
        1000,
    );

    let base_ucsur_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        BASE_UCSUR.as_slice(),
        if variation == NasinNanpaVariation::Main {
            LookupsMode::WordLigFromLetters
        } else {
            LookupsMode::None
        },
        Cc::Full,
        "",
        "Tok",
        "df80ff",
        EncPos::Pos(0xF19A0),
        1000,
    );

    let base_ext_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        BASE_EXT.as_slice(),
        if variation == NasinNanpaVariation::Main {
            LookupsMode::WordLigFromLetters
        } else {
            LookupsMode::None
        },
        Cc::Full,
        "",
        "Tok",
        "8f80ff",
        EncPos::Pos(0xF19E0),
        1000,
    );

    let mut base_alt_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        BASE_ALT.as_slice(),
        LookupsMode::Alt,
        Cc::Full,
        "",
        "",
        "ff80e6",
        EncPos::None,
        1000,
    );
    // alt ni
    base_alt_block.glyphs[0].encoding.enc_pos = EncPos::Pos(0xF1989);
    base_alt_block.glyphs[1].encoding.enc_pos = EncPos::Pos(0xF198A);
    base_alt_block.glyphs[2].encoding.enc_pos = EncPos::Pos(0xF198B);
    // alt sewi
    base_alt_block.glyphs[43].encoding.enc_pos = EncPos::Pos(0xF198C);

    let outer_cor_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        OUTER_COR.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
        "",
        "Tok_joinScaleTok",
        "ffff",
        EncPos::None,
        1000,
    );

    let outer_ext_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        OUTER_EXT.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
        "",
        "Tok_joinScaleTok",
        "ffff",
        EncPos::None,
        1000,
    );

    let outer_alt_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        OUTER_ALT.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
        "",
        "_joinScaleTok",
        "ffff",
        EncPos::None,
        1000,
    );

    let inner_cor_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        INNER_COR.as_slice(),
        LookupsMode::ComboLast,
        Cc::Full,
        "joinScaleTok_",
        "Tok",
        "80ffff",
        EncPos::None,
        0,
    );

    let inner_ext_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        INNER_EXT.as_slice(),
        LookupsMode::ComboLast,
        Cc::Full,
        "joinScaleTok_",
        "Tok",
        "80ffff",
        EncPos::None,
        0,
    );

    let inner_alt_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        INNER_ALT.as_slice(),
        LookupsMode::ComboLast,
        Cc::Full,
        "joinScaleTok_",
        "",
        "80ffff",
        EncPos::None,
        0,
    );

    let lower_cor_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        LOWER_COR.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
        "",
        "Tok_joinStackTok",
        "ff00",
        EncPos::None,
        1000,
    );

    let lower_ext_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        LOWER_EXT.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
        "",
        "Tok_joinStackTok",
        "ff00",
        EncPos::None,
        1000,
    );

    let lower_alt_block = GlyphBlock::from_const_descriptors(
        &mut ff_pos,
        LOWER_ALT.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
        "",
        "_joinStackTok",
        "ff00",
        EncPos::None,
        1000,
    );

    let upper_cor_block = lower_cor_block.from_refs(
        &mut ff_pos,
        "S 1 0 0 1 -1000 500 2".to_string(),
        LookupsMode::ComboLast,
        Cc::Full,
        false,
        "joinStackTok_",
        "Tok",
        "80ff80",
        Some(0),
        Some(Anchor::new_stack(AnchorType::Mark)),
    );

    let upper_ext_block = lower_ext_block.from_refs(
        &mut ff_pos,
        "S 1 0 0 1 -1000 500 2".to_string(),
        LookupsMode::ComboLast,
        Cc::Full,
        false,
        "joinStackTok_",
        "Tok",
        "80ff80",
        Some(0),
        Some(Anchor::new_stack(AnchorType::Mark)),
    );

    let upper_alt_block = lower_alt_block.from_refs(
        &mut ff_pos,
        "S 1 0 0 1 -1000 500 2".to_string(),
        LookupsMode::ComboLast,
        Cc::Full,
        false,
        "joinStackTok_",
        "",
        "80ff80",
        Some(0),
        Some(Anchor::new_stack(AnchorType::Mark)),
    );

    let put_in_class = |orig: String| format!("Class: {} {}", orig.len(), orig);

    let zwj_calt = {
        let scale_names = vec![&outer_cor_block, &outer_ext_block, &outer_alt_block]
            .iter()
            .enumerate()
            .map(|(i, &block)| {
                block
                    .glyphs
                    .iter()
                    .filter_map(|glyph| {
                        if glyph.glyph.name.contains("empty") {
                            None
                        } else {
                            Some(format!(
                                "{}{}",
                                glyph.glyph.name,
                                if i != 2 { "Tok" } else { "" }
                            ))
                        }
                    })
                    .join(" ")
            })
            .join(" ");

        let scale_glyphs = vec![&outer_cor_block, &outer_ext_block, &outer_alt_block]
            .iter()
            .map(|block| {
                block
                    .glyphs
                    .iter()
                    .filter_map(|glyph| {
                        if glyph.glyph.name.contains("empty") {
                            None
                        } else {
                            Some(glyph.glyph.name.clone())
                        }
                    })
                    .collect_vec()
            })
            .flatten()
            .collect::<HashSet<_>>();

        let stack_names = vec![&lower_cor_block, &lower_ext_block, &lower_alt_block]
            .iter()
            .enumerate()
            .map(|(i, block)| {
                block
                    .glyphs
                    .iter()
                    .filter_map(|glyph| {
                        if glyph.glyph.name.contains("empty")
                            || glyph.glyph.name.contains("arrow")
                            || scale_glyphs.contains(&glyph.glyph.name)
                        {
                            None
                        } else {
                            Some(format!(
                                "{}{}",
                                glyph.glyph.name,
                                if i != 2 { "Tok" } else { "" }
                            ))
                        }
                    })
                    .join(" ")
            })
            .join(" ");

        let zwj = put_in_class("ZWJ".to_string());
        let scale = put_in_class(scale_names);
        let stack = put_in_class(stack_names);

        let put_in_sub = |c: &str| format!("  {c}{zwj}\n  {c}{scale}\n  {c}{stack}\n");
        let subs = format!("{}{}{}", put_in_sub(""), put_in_sub("B"), put_in_sub("F"));

        format!("ContextSub2: class \"'calt' CHANGE ZWJ\" 4 4 4 2\n{subs}")
    };

    let mut main_blocks = vec![
        latn_block,
        no_comb_block,
        radicals_block,
        base_core_block,
        base_ucsur_block,
        base_ext_block,
        base_alt_block,
        outer_cor_block,
        outer_ext_block,
        outer_alt_block,
        inner_cor_block,
        inner_ext_block,
        inner_alt_block,
        lower_cor_block,
        lower_ext_block,
        lower_alt_block,
        upper_cor_block,
        upper_ext_block,
        upper_alt_block,
    ];

    let chain_calt = {
        let put_in_class = |orig: String| format!("Class: {} {}", orig.len(), orig);

        let base = {
            let ctrl_names = ctrl_block
                .glyphs
                .iter()
                .filter_map(|glyph| {
                    if glyph.glyph.name.contains("Half") || glyph.glyph.name.contains("Tick") {
                        None
                    } else {
                        Some(format!(
                            "{}{}{}",
                            ctrl_block.prefix, glyph.glyph.name, ctrl_block.suffix
                        ))
                    }
                })
                .join(" ");

            let main_names = main_blocks
                .iter()
                .map(|block| {
                    block
                        .glyphs
                        .iter()
                        .map(|glyph| {
                            format!("{}{}{}", block.prefix, glyph.glyph.name, block.suffix)
                        })
                        .join(" ")
                })
                .join(" ");

            put_in_class(format!(
                "{} joinStackTok joinScaleTok {}",
                ctrl_names, main_names
            ))
        };

        let cart = put_in_class(format!(
            "{} {} {} {}",
            "combCartExtHalfTok combCartExtNoneTok",
            (1..=6)
                .map(|x| format!("combCartExt{}TickTok", x))
                .join(" "),
            (1..=6)
                .map(|x| format!("combCartExt{}TopTickTok", x))
                .join(" "),
            "startCartTok combCartExtTok startCartAltTok startCartCombTok"
        ));

        let cont = {
            let longs = start_cont_block
                .glyphs
                .iter()
                .filter_map(|glyph| {
                    if glyph.glyph.name.eq("laTok") {
                        None
                    } else {
                        Some(format!(
                            "{}{}{}",
                            start_cont_block.prefix, glyph.glyph.name, start_cont_block.suffix
                        ))
                    }
                })
                .join(" ");

            put_in_class(format!("combContExtNoneTok combContExtHalfTok startLongPiTok combLongPiExtTok startContTok combContExtTok startRevContTok {}", longs))
        };

        let put_in_sub = |c: &str| format!("  {c}{base}\n  {c}{cart}\n  {c}{cont}\n");
        let subs = format!("{}{}{}", put_in_sub(""), put_in_sub("B"), put_in_sub("F"));
        format!("ChainSub2: class \"'calt' CART AND CONT\" 4 4 4 2\n{subs}")
    };

    let mut meta_block = vec![ctrl_block, tok_ctrl_block, start_cont_block];
    meta_block.append(&mut main_blocks);
    let glyphs_string = format!(
        "{}",
        meta_block.iter().map(|block| block.gen(variation)).join("")
    );

    let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();

    let filename = format!(
        "nasin-nanpa-{VERSION}{}.sfd",
        if variation == NasinNanpaVariation::Ucsur {
            "-UCSUR"
        } else {
            ""
        }
    );
    let mut file = File::create(filename)?;

    // FINAL `.sfd` COMPOSITIION
    writeln!(
        &mut file,
        r#"{HEADER}Version: {VERSION}
{DETAILS1}ModificationTime: {time}{DETAILS2}{LOOKUPS}DEI: 91125
{zwj_calt}{AFTER_ZWJ_CALT}{chain_calt}{AFTER_CHAIN_CALT}{VERSION}{OTHER}BeginChars: {ff_pos} {ff_pos}
{glyphs_string}EndChars
EndSplineFont"#
    )?;
    println!("done.");
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("");
    gen_nasin_nanpa(NasinNanpaVariation::Main)?;
    gen_nasin_nanpa(NasinNanpaVariation::Ucsur)?;
    Ok(())
}
