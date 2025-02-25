use parserc::{ParseContext, Parser, ParserExt, ensure_keyword};

use crate::opcode::Length;

use super::{FromSvg, ParseError, ParseKind, number::parse_number, sep::parse_sep};

pub(super) fn parse_length(ctx: &mut ParseContext<'_>) -> parserc::Result<Length, ParseError> {
    let num = parse_number(ctx)?;
    if let Some(length) = ensure_keyword("px")
        .map(|_| Length::Px(num))
        .or(ensure_keyword("em").map(|_| Length::Em(num)))
        .or(ensure_keyword("ex").map(|_| Length::Ex(num)))
        .or(ensure_keyword("in").map(|_| Length::Inch(num)))
        .or(ensure_keyword("cm").map(|_| Length::Cm(num)))
        .or(ensure_keyword("mm").map(|_| Length::Mm(num)))
        .or(ensure_keyword("pt").map(|_| Length::Pt(num)))
        .or(ensure_keyword("pc").map(|_| Length::Pc(num)))
        .or(ensure_keyword("%").map(|_| Length::Percent(num)))
        .ok()
        .parse(ctx)?
    {
        return Ok(length);
    }

    Ok(Length::Px(num))
}

pub(super) fn parse_length_list(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<Vec<Length>, ParseError> {
    let mut values = vec![];

    while let Some(angle) = parse_length.ok().parse(ctx)? {
        values.push(angle);

        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    Ok(values)
}

impl FromSvg for Length {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v = parse_length(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Length, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Length, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for Vec<Length> {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v =
            parse_length_list(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Lengths, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Lengths, ctx.unparsed()));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::{opcode::Length, svg::parse::ParseSvg};

    #[test]
    fn test_length() {
        assert_eq!("1.".parse_svg(), Ok(vec![Length::Px(1.)]));
        assert_eq!(
            "1. 10%".parse_svg(),
            Ok(vec![Length::Px(1.), Length::Percent(10.)])
        );
        assert_eq!(
            "1. 10%,-8e10".parse_svg(),
            Ok(vec![
                Length::Px(1.),
                Length::Percent(10.),
                Length::Px(-8e10)
            ])
        );

        assert_eq!("1em".parse_svg(), Ok(vec![Length::Em(1.)]));
        assert_eq!("1.2em".parse_svg(), Ok(vec![Length::Em(1.2)]));
    }
}
