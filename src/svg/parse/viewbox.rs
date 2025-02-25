use mlang_rs::rt::opcode::Variable;
use parserc::{ParseContext, Parser, ParserExt, ensure_keyword};

use crate::{
    opcode::{MeetOrSlice, PreserveAspectRatio, ViewBox},
    svg::parse::{
        number::parse_number,
        sep::{parse_sep, skip_ws},
    },
};

use super::{FromSvg, ParseError, ParseKind};

/// parse `viewBox` attribute value.
pub(super) fn parse_viewbox(ctx: &mut ParseContext<'_>) -> parserc::Result<ViewBox, ParseError> {
    let minx = parse_number(ctx)?;
    parse_sep
        .fatal(ParseError::failed(ParseKind::ViewBox, ctx.unparsed()))
        .parse(ctx)?;
    let miny = parse_number
        .fatal(ParseError::failed(ParseKind::ViewBox, ctx.unparsed()))
        .parse(ctx)?;
    parse_sep
        .fatal(ParseError::failed(ParseKind::ViewBox, ctx.unparsed()))
        .parse(ctx)?;
    let width = parse_number
        .fatal(ParseError::failed(ParseKind::ViewBox, ctx.unparsed()))
        .parse(ctx)?;
    parse_sep
        .fatal(ParseError::failed(ParseKind::ViewBox, ctx.unparsed()))
        .parse(ctx)?;
    let height = parse_number
        .fatal(ParseError::failed(ParseKind::ViewBox, ctx.unparsed()))
        .parse(ctx)?;

    Ok(ViewBox {
        minx: Variable::Constant(minx),
        miny: Variable::Constant(miny),
        width: Variable::Constant(width),
        height: Variable::Constant(height),
        aspect: None,
    })
}

pub(super) fn parse_preserve_aspect_ratio(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<PreserveAspectRatio, ParseError> {
    let start = ctx.span();
    // ignore `defer` keyword.
    ensure_keyword("defer").ok().parse(ctx)?;
    skip_ws.ok().parse(ctx)?;

    let align = ensure_keyword("none")
        .or(ensure_keyword("xMinYMin"))
        .or(ensure_keyword("xMidYMin"))
        .or(ensure_keyword("xMaxYMin"))
        .or(ensure_keyword("xMinYMid"))
        .or(ensure_keyword("xMidYMid"))
        .or(ensure_keyword("xMaxYMid"))
        .or(ensure_keyword("xMinYMax"))
        .or(ensure_keyword("xMidYMax"))
        .or(ensure_keyword("xMaxYMax"))
        .fatal(ParseError::failed(ParseKind::ViewBox, ctx.as_str(start)))
        .parse(ctx)?;

    let align = ctx.as_str(align);
    if "none" == align {
        return Ok(PreserveAspectRatio::None);
    }

    skip_ws.ok().parse(ctx)?;

    let meet_or_slice = ensure_keyword("meet")
        .or(ensure_keyword("slice"))
        .ok()
        .parse(ctx)?
        .map(|span| ctx.as_str(span));

    let meet_or_slice = match meet_or_slice {
        Some("meet") => MeetOrSlice::Meet,
        Some("slice") => MeetOrSlice::Slice,
        _ => MeetOrSlice::Meet,
    };

    match align {
        "xMinYMin" => Ok(PreserveAspectRatio::XMinYMin(meet_or_slice)),
        "xMidYMin" => Ok(PreserveAspectRatio::XMidYMin(meet_or_slice)),
        "xMaxYMin" => Ok(PreserveAspectRatio::XMaxYMin(meet_or_slice)),
        "xMinYMid" => Ok(PreserveAspectRatio::XMinYMid(meet_or_slice)),
        "xMidYMid" => Ok(PreserveAspectRatio::XMidYMid(meet_or_slice)),
        "xMaxYMid" => Ok(PreserveAspectRatio::XMaxYMid(meet_or_slice)),
        "xMinYMax" => Ok(PreserveAspectRatio::XMinYMax(meet_or_slice)),
        "xMidYMax" => Ok(PreserveAspectRatio::XMidYMax(meet_or_slice)),
        "xMaxYMax" => Ok(PreserveAspectRatio::XMaxYMax(meet_or_slice)),
        _ => {
            panic!("not here");
        }
    }
}

impl FromSvg for ViewBox {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v = parse_viewbox(&mut ctx).map_err(|_| ParseError::failed(ParseKind::ViewBox, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::ViewBox, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for PreserveAspectRatio {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v = parse_preserve_aspect_ratio(&mut ctx)
            .map_err(|_| ParseError::failed(ParseKind::PreserveAspectRatio, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(
                ParseKind::PreserveAspectRatio,
                ctx.unparsed(),
            ));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        opcode::{MeetOrSlice, PreserveAspectRatio},
        svg::parse::ParseSvg,
    };

    #[test]
    fn test_viewbox() {
        use mlang_rs::rt::opcode::Variable;

        use crate::{opcode::ViewBox, svg::parse::ParseSvg};

        assert_eq!(
            "0 0 100, 100".parse_svg(),
            Ok(ViewBox {
                minx: Variable::Constant(0.),
                miny: Variable::Constant(0.),
                width: Variable::Constant(100.),
                height: Variable::Constant(100.),
                aspect: None,
            })
        );
    }

    #[test]
    fn test_aspect() {
        assert_eq!("none".parse_svg(), Ok(PreserveAspectRatio::None));

        assert_eq!(
            "xMidYMin".parse_svg(),
            Ok(PreserveAspectRatio::XMidYMin(MeetOrSlice::Meet))
        );

        assert_eq!(
            "xMidYMin   slice".parse_svg(),
            Ok(PreserveAspectRatio::XMidYMin(MeetOrSlice::Slice))
        );

        assert_eq!(
            "xMidYMin   ".parse_svg(),
            Ok(PreserveAspectRatio::XMidYMin(MeetOrSlice::Meet))
        );
    }
}
