use parserc::{ControlFlow, ParseContext, Parser, ParserExt, Span, ensure_char, take_while};

use crate::{
    opcode::NumberOptNumber,
    svg::parse::{ParseError, SVG_PARSE_ERROR, sep::parse_sep},
};

use super::{FromSvg, ParseKind};

/// parse decimal digits: `10121..`
pub(super) fn parse_digits(ctx: &mut ParseContext<'_>) -> parserc::Result<Span, ParseError> {
    take_while(|c| c.is_ascii_digit())
        .parse(ctx)?
        .ok_or(ControlFlow::Recoverable(ParseError::failed(
            ParseKind::Number,
            ctx.unparsed(),
        )))
}

/// parse numeric signature char: `+` or `-`.
pub(super) fn parse_sign(ctx: &mut ParseContext<'_>) -> parserc::Result<Span, ParseError> {
    ensure_char('+').or(ensure_char('-')).parse(ctx)
}

/// parse integer: `12,-1,...`.
fn parse_integer_prv(ctx: &mut ParseContext<'_>) -> parserc::Result<Span, ParseError> {
    let sign = parse_sign.ok().parse(ctx)?;

    if let Some(sign) = sign {
        let digits = parse_digits
            .fatal(ParseError::failed(ParseKind::Number, ctx.unparsed()))
            .parse(ctx)?;

        Ok(sign.extend_to_inclusive(digits))
    } else {
        parse_digits(ctx)
    }
}

pub(super) fn parse_integer(ctx: &mut ParseContext<'_>) -> parserc::Result<i32, ParseError> {
    let span = parse_integer_prv(ctx)?;

    i32::from_str_radix(ctx.as_str(span), 10)
        .map_err(|_| ControlFlow::Fatal(ParseError::failed(ParseKind::Integer, ctx.as_str(span))))
}

/// parse exponent: `E-10` or `e100`
pub(super) fn parse_exponent(ctx: &mut ParseContext<'_>) -> parserc::Result<Span, ParseError> {
    let start = ensure_char('E').or(ensure_char('e')).parse(ctx)?;

    let integer = parse_integer_prv.parse(ctx)?;

    Ok(start.extend_to_inclusive(integer))
}

fn parse_number_prv(ctx: &mut ParseContext<'_>) -> parserc::Result<Span, ParseError> {
    let sign = parse_sign.ok().parse(ctx)?;
    let trunc = parse_integer_prv.ok().parse(ctx)?;

    if let Some(comma) = ensure_char('.').ok().parse(ctx)? {
        let fract = parse_digits
            .ok()
            .fatal(ParseError::failed(ParseKind::Number, ctx.unparsed()))
            .parse(ctx)?;

        let exponent = parse_exponent
            .ok()
            .fatal(ParseError::failed(ParseKind::Number, ctx.unparsed()))
            .parse(ctx)?;

        Ok(sign
            .unwrap_or(trunc.unwrap_or(comma))
            .extend_to_inclusive(exponent.unwrap_or(fract.unwrap_or(comma))))
    } else {
        let exponent = parse_exponent
            .ok()
            .fatal(ParseError::failed(ParseKind::Number, ctx.unparsed()))
            .parse(ctx)?;

        let trunc = trunc.ok_or_else(|| {
            if exponent.is_some() {
                log::error!(
                    target: SVG_PARSE_ERROR,span:serde = ctx.span();
                    "expect number trunc part."
                );
                ControlFlow::Fatal(ParseError::failed(ParseKind::Number, ctx.unparsed()))
            } else {
                ControlFlow::Recoverable(ParseError::failed(ParseKind::Number, ctx.unparsed()))
            }
        })?;

        if let Some(sign) = sign {
            Ok(sign.extend_to_inclusive(exponent.unwrap_or(trunc)))
        } else {
            if let Some(exponent) = exponent {
                Ok(trunc.extend_to_inclusive(exponent))
            } else {
                Ok(trunc)
            }
        }
    }
}

pub(super) fn parse_number(ctx: &mut ParseContext<'_>) -> parserc::Result<f32, ParseError> {
    let span = parse_number_prv(ctx)?;

    ctx.as_str(span)
        .parse()
        .map_err(|_| ControlFlow::Fatal(ParseError::failed(ParseKind::Number, ctx.unparsed())))
}

/// Parse svg value: `0` or `1` as bool value.
pub(super) fn parse_bool(ctx: &mut ParseContext<'_>) -> parserc::Result<bool, ParseError> {
    ensure_char('0')
        .map(|_| false)
        .or(ensure_char('1').map(|_| true))
        .parse(ctx)
}

pub(super) fn parse_number_opt_number(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<NumberOptNumber, ParseError> {
    let x = parse_number_prv(ctx)?;

    let x = ctx.as_str(x).parse::<f32>().map_err(|err| {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = x;
            "failed parsing the first argument of number_opt_number: {}",err
        );
        ControlFlow::Fatal(ParseError::failed(
            ParseKind::NumberOptNumber,
            ctx.as_str(x),
        ))
    })?;

    if let Some(_) = parse_sep.ok().parse(ctx)? {
        let y = parse_number_prv(ctx)?;

        let y = ctx.as_str(y).parse::<f32>().map_err(|err| {
            log::error!(
                target: SVG_PARSE_ERROR, span:serde = y;
                "failed parsing the first argument of number_opt_number: {}",err
            );
            ControlFlow::Fatal(ParseError::failed(
                ParseKind::NumberOptNumber,
                ctx.as_str(y),
            ))
        })?;

        Ok(NumberOptNumber(x, Some(y)))
    } else {
        Ok(NumberOptNumber(x, None))
    }
}

pub(super) fn parse_number_list(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<Vec<f32>, ParseError> {
    let mut values = vec![];

    while let Some(v) = parse_number.ok().parse(ctx)? {
        values.push(v);
        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    Ok(values)
}

impl FromSvg for bool {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v =
            parse_bool(&mut ctx).map_err(|_| ParseError::failed(ParseKind::NumberOptNumber, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Number, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for f32 {
    type Err = ParseError;
    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v = parse_number(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Number, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Number, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for Vec<f32> {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v =
            parse_number_list(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Numbers, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Numbers, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for i32 {
    type Err = ParseError;
    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v = parse_integer(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Integer, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Integer, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for NumberOptNumber {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v = parse_number_opt_number(&mut ctx)
            .map_err(|_| ParseError::failed(ParseKind::NumberOptNumber, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Number, ctx.unparsed()));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::svg::parse::ParseSvg;

    use super::*;

    #[test]
    fn test_number() {
        assert_eq!("12.5".parse_svg(), Ok(12.5));
        assert_eq!("\t12.5\n".parse_svg(), Ok(12.5));

        assert_eq!(".5".parse_svg(), Ok(0.5));
        assert_eq!("-.5".parse_svg(), Ok(-0.5));
        assert_eq!("-.5e10".parse_svg(), Ok(-0.5e10));
        assert_eq!("-.5E-10".parse_svg(), Ok(-0.5e-10));
        assert_eq!("-13.1621".parse_svg(), Ok(-13.1621));
        assert_eq!("-8e4".parse_svg(), Ok(-8e4));

        assert_eq!("-10".parse_svg(), Ok(-10f32));

        "-.".parse_svg::<f32>().expect_err("-.");
        "+".parse_svg::<f32>().expect_err("+");
        ".".parse_svg::<f32>().expect_err(".");
    }

    #[test]
    fn test_bool() {
        assert_eq!(parse_bool(&mut ParseContext::from("0")), Ok(false));
        assert_eq!(parse_bool(&mut ParseContext::from("1")), Ok(true));
    }

    #[test]
    fn test_number_list() {
        assert_eq!("12.0 -8e04,+10.2".parse_svg(), Ok(vec![12.0, -8e4, 10.2]));
    }

    #[test]
    fn test_number_opt_number() {
        assert_eq!("1.0, -10".parse_svg(), Ok(NumberOptNumber(1.0, Some(-10.))));

        "1.0,".parse_svg::<NumberOptNumber>().expect_err("1.0,");

        assert_eq!(" 1.0 ".parse_svg(), Ok(NumberOptNumber(1.0, None)));
    }
}
