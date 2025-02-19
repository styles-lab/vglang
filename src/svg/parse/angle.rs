use parserc::{ensure_keyword, ParseContext, Parser, ParserExt};

use crate::opcode::Angle;

use super::{
    number::parse_number, sep::parse_sep, FromSvg, ParseError, ParseKind, SVG_PARSE_ERROR,
};

pub(super) fn parse_angle(ctx: &mut ParseContext<'_>) -> parserc::Result<Angle> {
    let num = parse_number(ctx)?;
    if let Some(angle) = ensure_keyword("deg")
        .map(|_| Angle::Deg(num))
        .or(ensure_keyword("grad").map(|_| Angle::Grad(num)))
        .or(ensure_keyword("rad").map(|_| Angle::Rad(num)))
        .ok()
        .parse(ctx)?
    {
        return Ok(angle);
    }

    Ok(Angle::Deg(num))
}

pub(super) fn parse_angle_list(ctx: &mut ParseContext<'_>) -> parserc::Result<Vec<Angle>> {
    let mut values = vec![];

    while let Some(angle) = parse_angle.ok().parse(ctx)? {
        values.push(angle);

        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    Ok(values)
}

impl FromSvg for Angle {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim()).with_debug(SVG_PARSE_ERROR);

        let v = parse_angle(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Angle, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Angle, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for Vec<Angle> {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim()).with_debug(SVG_PARSE_ERROR);

        let v = parse_angle_list(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Angles, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Angles, ctx.unparsed()));
        }

        Ok(v)
    }
}
