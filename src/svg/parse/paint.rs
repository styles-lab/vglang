use mlang_rs::rt::serde::de::Deserialize;
use parserc::{
    ensure_char, ensure_keyword, ensure_keyword_insensitive, take_while, ControlFlow, ParseContext,
    Parser, ParserExt, Span,
};

use crate::{
    opcode::{Color, ColorKeyWord, Paint},
    svg::parse::{ParseKind, SVG_PARSE_ERROR},
};

use super::{
    iri::parse_func_iri_prv, number::parse_digits, sep::skip_ws, variant::Variant, FromSvg,
    ParseError,
};

fn parse_rgb(ctx: &mut ParseContext<'_>) -> parserc::Result<Color> {
    ensure_keyword_insensitive("rgb(").parse(ctx)?;

    skip_ws.ok().parse(ctx)?;

    let mut rgb: [Span; 3] = Default::default();

    let mut is_percent = false;

    for i in 0..3 {
        let span = parse_digits(ctx)?;

        if i == 0 {
            if let Some(_) = ensure_char('%').ok().parse(ctx)? {
                is_percent = true;
            }

            rgb[i] = span;
        } else {
            if is_percent {
                _ = ensure_char('%').fatal("expect %", span).parse(ctx)?;
            }

            rgb[i] = span;
        }

        skip_ws.ok().parse(ctx)?;

        if i != 2 {
            ensure_char(',')
                .fatal("expect whitespace and/or a comma", ctx.span())
                .parse(ctx)?;

            skip_ws.ok().parse(ctx)?;
        }
    }

    ensure_char(')')
        .parse(ctx)
        .map_err(|_| ControlFlow::Fatal)?;

    let r = u8::from_str_radix(ctx.as_str(rgb[0]), 10).map_err(|err| {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde =  rgb[0];
            "failed parsing red component: {}",err
        );
        ControlFlow::Fatal
    })?;

    let g = u8::from_str_radix(ctx.as_str(rgb[1]), 10).map_err(|err| {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde =  rgb[1];
            "failed parsing green component: {}",err
        );
        ControlFlow::Fatal
    })?;

    let b = u8::from_str_radix(ctx.as_str(rgb[2]), 10).map_err(|err| {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde =  rgb[2];
            "failed parsing blue component: {}",err
        );
        ControlFlow::Fatal
    })?;

    if is_percent {
        if r > 100 {
            log::error!(
                target: SVG_PARSE_ERROR, span:serde =  rgb[0];
                "failed parsing red component: out of range."
            );
            return Err(ControlFlow::Fatal);
        }

        if g > 100 {
            log::error!(
                target: SVG_PARSE_ERROR, span:serde =  rgb[1];
                "failed parsing green component: out of range."
            );
            return Err(ControlFlow::Fatal);
        }

        if b > 100 {
            log::error!(
                target: SVG_PARSE_ERROR, span:serde =  rgb[2];
                "failed parsing blue component: out of range."
            );
            return Err(ControlFlow::Fatal);
        }

        Ok(Color::Rgb(
            (255f32 * r as f32 / 100f32) as u8,
            (255f32 * g as f32 / 100f32) as u8,
            (255f32 * b as f32 / 100f32) as u8,
        ))
    } else {
        Ok(Color::Rgb(r, g, b))
    }
}

fn parse_hexadecimal_notation(ctx: &mut ParseContext<'_>) -> parserc::Result<Color> {
    ensure_char('#').parse(ctx)?;

    let span = take_while(|c| c.is_ascii_hexdigit()).parse(ctx)?;

    match span {
        Some(span) => match span.len() {
            3 => {
                let notation = ctx.as_str(span);

                let r = u8::from_str_radix(&notation[0..1], 16).unwrap();
                let g = u8::from_str_radix(&notation[1..2], 16).unwrap();
                let b = u8::from_str_radix(&notation[2..3], 16).unwrap();

                return Ok(Color::Rgb(r + r * 16, g + g * 16, b + b * 16));
            }
            6 => {
                let notation = ctx.as_str(span);

                let r = u8::from_str_radix(&notation[0..2], 16).unwrap();
                let g = u8::from_str_radix(&notation[2..4], 16).unwrap();
                let b = u8::from_str_radix(&notation[4..6], 16).unwrap();

                return Ok(Color::Rgb(r, g, b));
            }
            len => {
                log::error!(
                    target: SVG_PARSE_ERROR, span:serde;
                    "color hexadecimal notation length is 3 or 6, but got {}",
                    len,
                );
                return Err(ControlFlow::Fatal);
            }
        },
        _ => {
            log::error!(
                target: SVG_PARSE_ERROR, span:serde = ctx.span();
                "miss hexadecimal notation body."
            );

            return Err(ControlFlow::Fatal);
        }
    }
}

fn parse_recognized_keyword(ctx: &mut ParseContext<'_>) -> parserc::Result<Color> {
    let span = take_while(|c| c.is_ascii_alphabetic())
        .parse(ctx)?
        .ok_or(ControlFlow::Recoverable)?;

    let value = ctx.as_str(span);

    let mut deser = Variant(value);

    let keyword = ColorKeyWord::deserialize(&mut deser).map_err(|_| ControlFlow::Recoverable)?;

    Ok(Color::Keyword(keyword))
}

pub(super) fn parse_color_prv(ctx: &mut ParseContext<'_>) -> parserc::Result<Color> {
    parse_rgb
        .or(parse_hexadecimal_notation)
        .or(parse_recognized_keyword)
        .parse(ctx)
}

pub(super) fn parse_paint_prv(ctx: &mut ParseContext<'_>) -> parserc::Result<Paint> {
    ensure_keyword("none")
        .map(|_| Paint::None)
        .or(parse_color_prv.map(|color| Paint::Color(color)))
        .or(parse_func_iri_prv.map(|iri| Paint::Server(iri)))
        .parse(ctx)
}

impl FromSvg for Color {
    type Err = ParseError;
    fn from_svg(value: &str) -> Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(value.trim()).with_debug(SVG_PARSE_ERROR);

        let color =
            parse_color_prv(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Color, value))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Color, ctx.unparsed()));
        }

        Ok(color)
    }
}

impl FromSvg for Paint {
    type Err = ParseError;
    fn from_svg(value: &str) -> Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(value.trim()).with_debug(SVG_PARSE_ERROR);

        let paint =
            parse_paint_prv(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Paint, value))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Paint, ctx.unparsed()));
        }

        Ok(paint)
    }
}

#[cfg(test)]
mod tests {
    use crate::{opcode::FuncIri, svg::parse::ParseSvg};

    use super::*;

    #[test]
    fn test_color() {
        assert_eq!("navy".parse_svg(), Ok(Color::Keyword(ColorKeyWord::Navy)));
        assert_eq!(
            "brown   ".parse_svg(),
            Ok(Color::Keyword(ColorKeyWord::Brown))
        );
        assert_eq!(" #fff   ".parse_svg(), Ok(Color::Rgb(0xff, 0xff, 0xff)));
        assert_eq!(" #f0f1f2   ".parse_svg(), Ok(Color::Rgb(0xf0, 0xf1, 0xf2)));
        assert_eq!(
            "rgb(10, 20, 40  )   ".parse_svg(),
            Ok(Color::Rgb(10, 20, 40))
        );
        assert_eq!(
            "rgb(100%, 0%, 10%  )   ".parse_svg(),
            Ok(Color::Rgb(0xff, 0, 25))
        );

        assert_eq!(
            "rGb(100%, 0%, 10%  )   ".parse_svg(),
            Ok(Color::Rgb(0xff, 0, 25))
        );

        assert_eq!(
            "RGB(100%, 0%, 10%  )   ".parse_svg(),
            Ok(Color::Rgb(0xff, 0, 25))
        );

        "rgb(100%,0,10%)"
            .parse_svg::<Color>()
            .expect_err("rgb(100%,0,10%)");

        "rgb(100%,0%,10%"
            .parse_svg::<Color>()
            .expect_err("rgb(100%,0%,10%");
    }

    #[test]
    fn test_paint() {
        assert_eq!(
            "#fff".parse_svg(),
            Ok(Paint::Color(Color::Rgb(0xff, 0xff, 0xff)))
        );

        assert_eq!(" none ".parse_svg(), Ok(Paint::None));

        assert_eq!(
            " url(#hello) ".parse_svg(),
            Ok(Paint::Server(FuncIri("hello".to_string())))
        );
    }
}
