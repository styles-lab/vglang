use parserc::{
    ensure_char, ensure_keyword, take_till, take_while, ControlFlow, ParseContext, Parser,
    ParserExt, Span,
};

use crate::{
    opcode::{FuncIri, Iri},
    svg::parse::ParseKind,
};

use super::{sep::skip_ws, FromSvg, ParseError, SVG_PARSE_ERROR};

pub(super) fn parse_iri_prv(ctx: &mut ParseContext<'_>) -> parserc::Result<(Span, bool)> {
    let local = if let Some(_) = ensure_char('#').ok().parse(ctx)? {
        true
    } else {
        false
    };

    let body = take_till(|c| c.is_whitespace())
        .parse(ctx)?
        .ok_or(ControlFlow::Fatal)?;

    Ok((body, local))
}

pub(super) fn parse_func_iri_prv(ctx: &mut ParseContext<'_>) -> parserc::Result<FuncIri> {
    ensure_keyword("url").parse(ctx)?;
    skip_ws.ok().parse(ctx)?;
    ensure_char('(').parse(ctx)?;

    skip_ws.ok().parse(ctx)?;

    ensure_char('#').parse(ctx)?;

    let span = take_while(|c| c != ')' && !c.is_whitespace())
        .parse(ctx)?
        .ok_or(ControlFlow::Fatal)?;

    skip_ws.ok().parse(ctx)?;
    ensure_char(')').parse(ctx)?;

    Ok(FuncIri(ctx.as_str(span).to_string()))
}

impl FromSvg for Iri {
    type Err = ParseError;
    fn from_svg(s: &str) -> Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim()).with_debug(SVG_PARSE_ERROR);

        let (v, local) =
            parse_iri_prv(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Iri, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Iri, ctx.unparsed()));
        }

        if local {
            Ok(Iri::Local(ctx.as_str(v).to_string()))
        } else {
            Ok(Iri::Path(ctx.as_str(v).to_string()))
        }
    }
}

impl FromSvg for FuncIri {
    type Err = ParseError;
    fn from_svg(s: &str) -> Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim()).with_debug(SVG_PARSE_ERROR);

        let v =
            parse_func_iri_prv(&mut ctx).map_err(|_| ParseError::failed(ParseKind::FuncIri, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::FuncIri, ctx.unparsed()));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        opcode::{FuncIri, Iri},
        svg::parse::ParseSvg,
    };

    #[test]
    fn test_iri() {
        assert_eq!("#hello".parse_svg(), Ok(Iri::Local("hello".to_string())));
        assert_eq!("hello".parse_svg(), Ok(Iri::Path("hello".to_string())));

        assert_eq!(" hello ".parse_svg(), Ok(Iri::Path("hello".to_string())));

        " hello world".parse_svg::<Iri>().expect_err(" hello world");
    }

    #[test]
    fn test_funciri() {
        assert_eq!("url(#hello)".parse_svg(), Ok(FuncIri("hello".to_string())));
        assert_eq!(
            " url ( #world )".parse_svg(),
            Ok(FuncIri("world".to_string()))
        );
    }
}
