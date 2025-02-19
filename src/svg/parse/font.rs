use parserc::{ensure_keyword, take_till, ControlFlow, ParseContext, Parser, ParserExt};

use crate::opcode::FontFamily;

use super::{sep::parse_sep, FromSvg, ParseError, ParseKind, SVG_PARSE_ERROR};

pub(super) fn parse_font_family(ctx: &mut ParseContext<'_>) -> parserc::Result<FontFamily> {
    let keyword = ensure_keyword("serif")
        .map(|_| FontFamily::Serif)
        .or(ensure_keyword("sansSerif").map(|_| FontFamily::SansSerif))
        .or(ensure_keyword("cursive").map(|_| FontFamily::Cursive))
        .or(ensure_keyword("fantasy").map(|_| FontFamily::Fantasy))
        .or(ensure_keyword("monospace").map(|_| FontFamily::Monospace))
        .ok()
        .parse(ctx)?;

    if let Some(keyword) = keyword {
        return Ok(keyword);
    }

    let span = take_till(|c| c.is_ascii_whitespace() || c == ',')
        .parse(ctx)?
        .ok_or_else(|| ControlFlow::Recoverable)?;

    Ok(FontFamily::Generic(ctx.as_str(span).to_string()))
}

fn parse_font_family_list(ctx: &mut ParseContext<'_>) -> parserc::Result<Vec<FontFamily>> {
    let mut families = vec![];

    while let Some(family) = parse_font_family.ok().parse(ctx)? {
        families.push(family);

        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    Ok(families)
}

impl FromSvg for Vec<FontFamily> {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim()).with_debug(SVG_PARSE_ERROR);

        let v = parse_font_family_list(&mut ctx)
            .map_err(|_| ParseError::failed(ParseKind::FontFamily, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Number, ctx.unparsed()));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::{opcode::FontFamily, svg::parse::ParseSvg};

    #[test]
    fn test_font_family() {
        assert_eq!(
            "serif sansSerif ,Vect".parse_svg(),
            Ok(vec![
                FontFamily::Serif,
                FontFamily::SansSerif,
                FontFamily::Generic("Vect".to_string())
            ])
        );
        assert_eq!(
            "serif sansSerif ,Vect,".parse_svg(),
            Ok(vec![
                FontFamily::Serif,
                FontFamily::SansSerif,
                FontFamily::Generic("Vect".to_string())
            ])
        );
    }
}
