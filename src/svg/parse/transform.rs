use parserc::{ParseContext, Parser, ParserExt, ensure_char, ensure_keyword};

use crate::opcode::Transform;

use super::{
    FromSvg, ParseError, ParseKind,
    number::parse_number,
    point::parse_point,
    sep::{parse_sep, skip_ws},
};

fn parse_transform_matrix(ctx: &mut ParseContext<'_>) -> parserc::Result<Transform, ParseError> {
    let start = ensure_keyword("matrix(").parse(ctx)?;

    skip_ws.ok().parse(ctx)?;

    let mut values: [f32; 6] = Default::default();

    for i in 0..6 {
        values[i] = parse_number
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        if i == 5 {
            skip_ws.ok().parse(ctx)?;
        } else {
            parse_sep
                .fatal(ParseError::failed(
                    ParseKind::TransformList,
                    ctx.as_str(start),
                ))
                .parse(ctx)?;
        }
    }

    ensure_char(')')
        .fatal(ParseError::failed(
            ParseKind::TransformList,
            ctx.as_str(start),
        ))
        .parse(ctx)?;

    Ok(Transform::Matrix(values))
}

fn parse_rotate(ctx: &mut ParseContext<'_>) -> parserc::Result<Transform, ParseError> {
    let start = ensure_keyword("rotate").parse(ctx)?;

    ensure_char('(')
        .fatal(ParseError::failed(
            ParseKind::TransformList,
            ctx.as_str(start),
        ))
        .parse(ctx)?;

    skip_ws.ok().parse(ctx)?;

    let one = parse_number
        .fatal(ParseError::failed(
            ParseKind::TransformList,
            ctx.as_str(start),
        ))
        .parse(ctx)?;

    if let Some(_) = parse_sep.ok().parse(ctx)? {
        let c = parse_point(ctx)?;

        ensure_char(')')
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        Ok(Transform::Rotate {
            angle: one,
            center: Some(c),
        })
    } else {
        skip_ws.ok().parse(ctx)?;

        ensure_char(')')
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        Ok(Transform::Rotate {
            angle: one,
            center: None,
        })
    }
}
fn parse_transform_two_params(
    name: &str,
) -> impl parserc::Parser<Output = (f32, Option<f32>), Error = ParseError> + Clone + use<'_> {
    move |ctx: &mut ParseContext<'_>| {
        let start = ensure_keyword(name).parse(ctx)?;

        ensure_char('(')
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        skip_ws.ok().parse(ctx)?;

        let one = parse_number
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        let two = if let Some(_) = parse_sep.ok().parse(ctx)? {
            parse_number.ok().parse(ctx)?
        } else {
            None
        };

        skip_ws.ok().parse(ctx)?;

        ensure_char(')')
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        Ok((one, two))
    }
}

fn parse_transform_one_param(
    name: &str,
) -> impl parserc::Parser<Output = f32, Error = ParseError> + Clone + use<'_> {
    move |ctx: &mut ParseContext<'_>| {
        let start = ensure_keyword(name).parse(ctx)?;

        ensure_char('(')
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        skip_ws.ok().parse(ctx)?;

        let one = parse_number
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        skip_ws.ok().parse(ctx)?;

        ensure_char(')')
            .fatal(ParseError::failed(
                ParseKind::TransformList,
                ctx.as_str(start),
            ))
            .parse(ctx)?;

        Ok(one)
    }
}

pub(super) fn parse_transform_list(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<Vec<Transform>, ParseError> {
    let mut values = vec![];

    while let Some(transform) = parse_transform_matrix
        .or(parse_transform_two_params("translate")
            .map(|(one, two)| Transform::Translate(one, two)))
        .or(parse_transform_two_params("scale").map(|(one, two)| Transform::Scale(one, two)))
        .or(parse_rotate)
        .or(parse_transform_one_param("skewX").map(|one| Transform::SkewX(one)))
        .or(parse_transform_one_param("skewY").map(|one| Transform::SkewY(one)))
        .ok()
        .parse(ctx)?
    {
        values.push(transform);

        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    Ok(values)
}

impl FromSvg for Vec<Transform> {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v = parse_transform_list(&mut ctx)
            .map_err(|_| ParseError::failed(ParseKind::TransformList, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(
                ParseKind::TransformList,
                ctx.unparsed(),
            ));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::{opcode::Transform, svg::parse::ParseSvg};

    #[test]
    fn test_transform_list() {
        assert_eq!(
            "matrix(1.0,1.0,1.1,1.2,1.3,1.4)".parse_svg(),
            Ok(vec![Transform::Matrix([1.0, 1.0, 1.1, 1.2, 1.3, 1.4])])
        );

        assert_eq!(
            " matrix(1.0,1.0 1.1,1.2,1.3,   1.4  ) ".parse_svg(),
            Ok(vec![Transform::Matrix([1.0, 1.0, 1.1, 1.2, 1.3, 1.4])])
        );

        assert_eq!(
            "matrix(1.0,1.0,1.1,1.2,1.3,   1.4  ) translate(-10,-20) , scale(2) rotate(45) translate(5,10)".parse_svg(),
            Ok(
                vec![
                Transform::Matrix([
                    1.0, 1.0,1.1, 1.2, 1.3, 1.4
                ]),
                Transform::Translate(-10., Some(-20.)),
                Transform::Scale(2., None),
                Transform::Rotate{
                    angle: 45., center: None,
                },
                Transform::Translate(5., Some(10.)),
            ]),
        );

        assert_eq!("skewX(10)".parse_svg(), Ok(vec![Transform::SkewX(10.)]));
        assert_eq!(
            "skewY(-10.1e-10)".parse_svg(),
            Ok(vec![Transform::SkewY(-10.1E-10)])
        );
    }
}
