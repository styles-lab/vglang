use parserc::{ParseContext, Parser, ParserExt};

use crate::{
    opcode::Point,
    svg::parse::{ParseError, ParseKind, sep::parse_sep},
};

use super::{FromSvg, number::parse_number};

/// Parse a point value: `x,y`.
pub(super) fn parse_point(ctx: &mut ParseContext<'_>) -> parserc::Result<Point, ParseError> {
    let x = parse_number(ctx)?;

    let _ = parse_sep
        .fatal(ParseError::failed(ParseKind::Point, ctx.unparsed()))
        .parse(ctx)?;

    let y = parse_number
        .fatal(ParseError::failed(ParseKind::Point, ctx.unparsed()))
        .parse(ctx)?;

    Ok(Point(x, y))
}

pub(super) fn parse_point_list(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<Vec<Point>, ParseError> {
    let mut values = vec![];

    while let Some(v) = parse_point.ok().parse(ctx)? {
        values.push(v);
        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    Ok(values)
}

impl FromSvg for Point {
    type Err = ParseError;
    fn from_svg(value: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(value.trim());

        let v = parse_point(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Point, value))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Point, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for Vec<Point> {
    type Err = ParseError;
    fn from_svg(value: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(value.trim());

        let v =
            parse_point_list(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Points, value))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Points, ctx.unparsed()));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::{opcode::Point, svg::parse::ParseSvg};

    #[test]
    fn test_point() {
        assert_eq!("12.5,-10".parse_svg(), Ok(Point(12.5, -10.)));
        assert_eq!(
            "13.5508  ,-13.1621".parse_svg(),
            Ok(Point(13.5508, -13.1621))
        );
        assert_eq!("13.5508 -13.1621".parse_svg(), Ok(Point(13.5508, -13.1621)));

        assert_eq!(
            "13.5508,\n-13.1621".parse_svg(),
            Ok(Point(13.5508, -13.1621))
        );

        "13.5508 ,".parse_svg::<Point>().expect_err("13.5508 ,");
    }

    #[test]
    fn test_points() {
        assert_eq!(
            "12.1,-8e10 10 2e-10".parse_svg(),
            Ok(vec![Point(12.1, -8e10), Point(10., 2e-10)])
        );

        "12.1,-8e10 10"
            .parse_svg::<Vec<Point>>()
            .expect_err("12.1,-8e10 10");
    }
}
