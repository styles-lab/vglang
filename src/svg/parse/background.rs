use parserc::{ControlFlow, ParseContext, Parser, ParserExt, ensure_keyword};

use crate::{
    opcode::{Background, BackgroundNew},
    svg::parse::SVG_PARSE_ERROR,
};

use super::{FromSvg, ParseError, ParseKind, number::parse_number_list, sep::skip_ws};

pub(super) fn parse_background(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<Background, ParseError> {
    if let Some(_) = ensure_keyword("accumulate").ok().parse(ctx)? {
        return Ok(Background::Accumulate);
    }

    ensure_keyword("new").parse(ctx)?;

    skip_ws.ok().parse(ctx)?;

    let start = ctx.span();
    let params = parse_number_list(ctx)?;

    match params.len() {
        0 => Ok(Background::New(None)),
        4 => Ok(Background::New(Some(BackgroundNew {
            x: params[0],
            y: params[1],
            width: params[2],
            height: params[3],
        }))),
        len => {
            log::error!(
                target: SVG_PARSE_ERROR, span:serde = start;
                "the counter of the background new params is mismatch({}), expect 0 or 4.",
                len
            );
            return Err(ControlFlow::Fatal(Some(ParseError::failed(
                ParseKind::Background,
                ctx.as_str(start),
            ))));
        }
    }
}

impl FromSvg for Background {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v =
            parse_background(&mut ctx).map_err(|_| ParseError::failed(ParseKind::Background, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::Background, ctx.unparsed()));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        opcode::{Background, BackgroundNew},
        svg::parse::ParseSvg,
    };

    #[test]
    fn test_background() {
        assert_eq!(" accumulate ".parse_svg(), Ok(Background::Accumulate));
        assert_eq!(" new ".parse_svg(), Ok(Background::New(None)));

        assert_eq!(
            " new 1 20,5 +8e10".parse_svg(),
            Ok(Background::New(Some(BackgroundNew {
                x: 1.,
                y: 20.,
                width: 5.,
                height: 8e10
            })))
        );
    }
}
