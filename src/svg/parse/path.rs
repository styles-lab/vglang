use std::vec;

use parserc::{ControlFlow, ParseContext, Parser, ParserExt, ensure_char};

use crate::{
    opcode::{Arc, CubicBezier, CubicBezierSmooth, PathEvent, QuadraticBezier},
    svg::parse::SVG_PARSE_ERROR,
};

use super::{
    FromSvg, ParseError, ParseKind,
    number::{parse_bool, parse_number, parse_number_list},
    point::{parse_point, parse_point_list},
    sep::{parse_sep, skip_ws},
};

/// Parse `M (x y)+` or `m (x y)+`.
fn parse_move_to(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();
    let relative = ensure_char('M')
        .map(|_| false)
        .or(ensure_char('m').map(|_| true))
        .parse(ctx)?;

    skip_ws.ok().parse(ctx)?;

    let points = parse_point_list(ctx)?;

    if points.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `moveTo` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::MoveTo { points, relative })
}

/// Parse `Z` or `z`.
fn parse_close(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    _ = ensure_char('Z').or(ensure_char('z')).parse(ctx)?;

    Ok(PathEvent::Close)
}

/// Parse `L (x y)+` or `l (x y)+`
fn parse_line_to(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();
    let relative = ensure_char('L')
        .map(|_| false)
        .or(ensure_char('l').map(|_| true))
        .parse(ctx)?;

    skip_ws.ok().parse(ctx)?;
    let points = parse_point_list(ctx)?;

    if points.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `lineTo` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::LineTo { points, relative })
}

/// Parse `H (x y)+` or `h (x y)+`
fn parse_horizontal(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();

    let relative = ensure_char('H')
        .map(|_| false)
        .or(ensure_char('h').map(|_| true))
        .parse(ctx)?;

    skip_ws.ok().parse(ctx)?;
    let points = parse_number_list(ctx)?;

    if points.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `horizontal` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::Horizontal(*points.last().unwrap(), relative))
}

/// Parse `V (x y)+` or `v (x y)+`
fn parse_vertical(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();
    let relative = ensure_char('V')
        .map(|_| false)
        .or(ensure_char('v').map(|_| true))
        .parse(ctx)?;

    skip_ws.ok().parse(ctx)?;
    let points = parse_number_list(ctx)?;

    if points.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `Vertical` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::Vertical(*points.last().unwrap(), relative))
}

fn parse_cubic_param(ctx: &mut ParseContext<'_>) -> parserc::Result<CubicBezier, ParseError> {
    parse_sep.ok().parse(ctx)?;
    let ctrl1 = parse_point(ctx)?;
    parse_sep(ctx)?;
    let ctrl2 = parse_point(ctx)?;
    parse_sep(ctx)?;
    let to = parse_point(ctx)?;

    Ok(CubicBezier { ctrl1, ctrl2, to })
}

fn parse_cubic_smooth_param(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<CubicBezierSmooth, ParseError> {
    parse_sep.ok().parse(ctx)?;
    let ctrl2 = parse_point(ctx)?;
    parse_sep(ctx)?;
    let to = parse_point(ctx)?;

    Ok(CubicBezierSmooth { ctrl2, to })
}

/// Parse `C (x1 y1 x2 y2 x y)+`,`c (x1 y1 x2 y2 x y)+`,`S (x2 y2 x y)+` or `s (x2 y2 x y)+`
fn parse_cubic(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();
    let relative = ensure_char('C')
        .map(|_| false)
        .or(ensure_char('c').map(|_| true))
        .parse(ctx)?;

    let mut params = vec![];

    while let Some(param) = parse_cubic_param.ok().parse(ctx)? {
        params.push(param);

        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    if params.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `curveto` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::CubicBezier(params, relative))
}

fn parse_cubic_smooth(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();
    let relative = ensure_char('S')
        .map(|_| false)
        .or(ensure_char('s').map(|_| true))
        .parse(ctx)?;

    let mut params = vec![];

    while let Some(param) = parse_cubic_smooth_param.ok().parse(ctx)? {
        params.push(param);

        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    if params.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `smooth curveto` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::CubicBezierSmooth(params, relative))
}

fn parse_quadratic_param(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<QuadraticBezier, ParseError> {
    parse_sep.ok().parse(ctx)?;
    let ctrl = parse_point(ctx)?;
    parse_sep(ctx)?;
    let to = parse_point(ctx)?;

    Ok(QuadraticBezier { ctrl, to })
}

fn parse_quadratic(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();
    let relative = ensure_char('Q')
        .map(|_| false)
        .or(ensure_char('q').map(|_| true))
        .parse(ctx)?;

    let mut params = vec![];

    while let Some(param) = parse_quadratic_param.ok().parse(ctx)? {
        params.push(param);

        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    if params.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `quadratic curveto` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::QuadraticBezier(params, relative))
}

fn parse_quadratic_smooth(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();
    let relative = ensure_char('T')
        .map(|_| false)
        .or(ensure_char('t').map(|_| true))
        .parse(ctx)?;

    parse_sep.ok().parse(ctx)?;

    let params = parse_point_list(ctx)?;

    if params.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `smooth quadratic curveto` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::QuadraticBezierSmooth(params, relative))
}

fn parse_arc_param(ctx: &mut ParseContext<'_>) -> parserc::Result<Arc, ParseError> {
    parse_sep.ok().parse(ctx)?;
    let rx = parse_number(ctx)?;
    parse_sep(ctx)?;
    let ry = parse_number(ctx)?;
    parse_sep(ctx)?;
    let x_rotation = parse_number(ctx)?;
    parse_sep(ctx)?;
    let large_arc = parse_bool(ctx)?;
    parse_sep(ctx)?;
    let sweep = parse_bool(ctx)?;
    parse_sep(ctx)?;
    let to = parse_point(ctx)?;

    Ok(Arc {
        rx,
        ry,
        x_rotation,
        large_arc,
        sweep,
        to,
    })
}

fn parse_arc(ctx: &mut ParseContext<'_>) -> parserc::Result<PathEvent, ParseError> {
    let start = ctx.span();
    let relative = ensure_char('A')
        .map(|_| false)
        .or(ensure_char('a').map(|_| true))
        .parse(ctx)?;

    parse_sep.ok().parse(ctx)?;

    let mut params = vec![];

    while let Some(param) = parse_arc_param.ok().parse(ctx).map_err(|_| {
        ControlFlow::Fatal(ParseError::failed(ParseKind::PathEvent, ctx.as_str(start)))
    })? {
        params.push(param);

        if parse_sep.ok().parse(ctx)?.is_none() {
            break;
        }
    }

    if params.is_empty() {
        log::error!(
            target: SVG_PARSE_ERROR, span:serde = ctx.span();
            "failed parsing `quadratic curveto` directive: parameter list is none."
        );
        return Err(ControlFlow::Fatal(ParseError::failed(
            ParseKind::PathEvent,
            ctx.as_str(start),
        )));
    }

    Ok(PathEvent::Arc(params, relative))
}

pub(super) fn parse_path_event(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<PathEvent, ParseError> {
    parse_close
        .or(parse_move_to)
        .or(parse_line_to)
        .or(parse_horizontal)
        .or(parse_vertical)
        .or(parse_cubic)
        .or(parse_cubic_smooth)
        .or(parse_quadratic)
        .or(parse_quadratic_smooth)
        .or(parse_arc)
        .parse(ctx)
}

pub(super) fn parse_path_event_list(
    ctx: &mut ParseContext<'_>,
) -> parserc::Result<Vec<PathEvent>, ParseError> {
    let mut events = vec![];

    while let Some(event) = parse_path_event.ok().parse(ctx)? {
        events.push(event);

        parse_sep.ok().parse(ctx)?;
    }

    Ok(events)
}

impl FromSvg for PathEvent {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v =
            parse_path_event(&mut ctx).map_err(|_| ParseError::failed(ParseKind::PathEvent, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::PathEvent, ctx.unparsed()));
        }

        Ok(v)
    }
}

impl FromSvg for Vec<PathEvent> {
    type Err = ParseError;

    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ctx = ParseContext::from(s.trim());

        let v = parse_path_event_list(&mut ctx)
            .map_err(|_| ParseError::failed(ParseKind::PathEvents, s))?;

        if ctx.remaining() > 0 {
            return Err(ParseError::unparsed(ParseKind::PathEvents, ctx.unparsed()));
        }

        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        opcode::{Arc, CubicBezier, CubicBezierSmooth, PathEvent, Point, QuadraticBezier},
        svg::parse::ParseSvg,
    };

    #[test]
    fn test_path_events() {
        let events = "m 396.07785,839.18047 c -23.8186,-12.9174 -26.4383,-31.0581 -32.2366,-55.5399 l 11.6939,3.9936 c 
        -9.3787,-16.7952 -12.8922,-17.1495 -18.1588,-38.7428 6.4042,2.8287 7.9631,8.053 10.9683,14.0222 
        -0.5954,-12.0976 -3.1413,-24.1272 -7.8652,-34.8925 l -4.2394,-3.3621 c 0,0 4.0577,-23.1163 4.8882,
        -27.2765 l -5.2692,-8.2223 -9.9274,-1.5693 c -19.6828,-19.9045 -57.4766,-38.3234 -97.2124,-38.055 
        l -14.632,-0.7507 c -9.5862,1.2486 -19.0124,3.1372 -29.2301,1.8601 l 5.95,5.398 c -21.1529,6.1603 
        -31.3925,11.1 -46.8877,24.8154 7.2681,-4.1562 12.6378,-4.5749 17.5134,-0.9103 -20.5093,8.7666 
        -29.8089,20.1738 -39.32,33.5876 7.7268,-6.6235 15.182,-10.6415 22.1728,-7.9297 -20.4433,14.8632 
        -19.8418,29.7112 -23.0972,53.4456 l -20.3018,98.0694 15.7206,25.6952 46.1256,10.0254 112.094,52.9788 
        14.8549,-5.7086 6.0345,-27.6851 c 2.4975,-11.5226 13.1841,-16.3799 26.098,-13.0043 l 19.2706,4.2252 
        c 17.2844,-18.3596 26.8435,-39.9902 34.993,-64.4674 z".parse_svg::<Vec<PathEvent>>().expect("lyon d1");

        assert_eq!(
            events[events.len() - 2],
            PathEvent::CubicBezier(
                vec![CubicBezier {
                    ctrl1: Point(17.2844, -18.3596),
                    ctrl2: Point(26.8435, -39.9902),
                    to: Point(34.993, -64.4674),
                }],
                true
            )
        );

        let events = "m 250.52965,580.36007 c -2.1414,0 -4.2826,0.943 -5.4433,2.8281 
        l -11.5215,18.6191 -5.4278,0.5293 -14.9238,-16.0097 c -3.0246,-3.2365 -9.1331,-2.0377 -10.6836,
        2.1308 l -7.6465,20.4532 -5.2734,1.5976 -17.7246,-12.7558 c -3.6008,-2.6012 -9.3586,-0.2039 
        -10.0703,4.1679 l -3.5156,21.5664 -4.8418,2.5977 -19.8848,-9.0664 c -4.0414,-1.847 -9.2262,1.6236 
        -9.0567,6.0547 l 0.7618,21.8847 -4.2032,3.4551 -21.312496,-5.0234 c -4.329396,-1.0082 -8.725596,3.3815 
        -7.708896,7.7109 l 5.0195,21.3164 -3.4473,4.1934 -21.8847,-0.7637 c -4.4142,-0.127 -7.8932,5.0126 
        -6.0547,9.0625 l 9.0664,19.8887 -2.5977,4.8418 -21.5605,3.5078 c -4.3803,0.7116 -6.754,6.4695 -4.1699,10.0703 
        l 12.7558,17.7148 -1.6016,5.2793 -20.4531,7.6465 c -4.1515,1.5505 -5.366,7.6544 -2.1211,10.6875 l 16,14.9199 
        -0.5293,5.4219 -18.6152,11.5235 c -3.7702,2.3299 -3.7702,8.5691 0,10.8906 l 18.6152,11.5234 0.5293,5.4219 
        -16,14.9238 c -3.2449,3.0247 -2.0304,9.1247 2.1211,10.6836 l 20.4531,7.6465 1.6016,5.2793 -12.7558,17.7187 
        c -2.5926,3.6093 -0.2065,9.3632 4.1738,10.0664 l 21.5566,3.5079 2.5977,4.8496 -9.0664,19.8808 c -1.847,4.0414 
        1.6405,9.2385 6.0547,9.0606 l 21.875,-0.7656 3.457,4.2011 -5.0195,21.3223 c -1.0167,4.3209 3.3795,8.7081 7.708896,
        7.6914 l 21.312496,-5.0195 4.2032,3.4492 -0.7618,21.8887 c -0.1695,4.4311 5.0153,7.8958 9.0567,6.0488 l 19.8848,
        -9.0625 4.8418,2.5976 3.5156,21.55473 c 0.7117,4.3888 6.4695,6.7599 10.0703,4.1758 l 17.7148,-12.76373 5.2793,
        1.6055 7.6465,20.44533 c 1.5505,4.1515 7.659,5.3745 10.6836,2.1211 l 14.9238,-16 5.4258,0.543 11.5235,18.6133 c 
        2.3214,3.7533 8.5607,3.7617 10.8906,0 l 11.5234,-18.6133 5.4219,-0.543 14.9199,16 c 3.0247,3.2534 9.1332,2.0304 
        10.6836,-2.1211 l 7.6465,-20.44533 5.2793,-1.6055 17.7187,12.76373 c 3.6008,2.5841 9.3548,0.2045 10.0664,-4.1758 
        l 3.5157,-21.55473 4.8418,-2.5976 19.8847,9.0625 c 4.0414,1.847 9.2177,-1.6092 9.0567,-6.0488 l -0.7617,-21.8887 
        4.1972,-3.4492 21.3125,5.0195 c 4.3294,1.0167 8.7238,-3.3705 7.7071,-7.6914 l -5.0215,-21.3223 3.4492,-4.2011 
        21.8848,0.7656 c 4.4142,0.1779 7.9043,-5.0192 6.0488,-9.0606 l -9.0606,-19.8808 2.5918,-4.8496 21.5625,-3.5079 
        c 4.3887,-0.703 6.769,-6.4571 4.168,-10.0664 l -12.7559,-17.7187 1.6016,-5.2793 20.4492,-7.6465 c 4.1599,-1.5589 
        5.3745,-7.6589 2.1211,-10.6836 l -15.9941,-14.9238 0.5293,-5.4219 18.6133,-11.5234 c 3.7703,-2.3215 3.7745,-8.5607 0,
        -10.8906 l -18.6094,-11.5235 -0.5293,-5.4219 15.9961,-14.9199 c 3.2534,-3.0331 2.0369,-9.137 -2.1231,-10.6875 l 
        -20.4472,-7.6465 -1.6016,-5.2793 12.7539,-17.7148 c 2.6011,-3.6008 0.2123,-9.3502 -4.168,-10.0703 l -21.5625,-3.5078 
        -2.5918,-4.8418 9.0606,-19.8887 c 1.8555,-4.0499 -1.5924,-9.2319 -6.0488,-9.0625 l -21.8848,0.7637 -3.457,-4.1934 5.0293,
        -21.3164 c 1.0167,-4.3294 -3.3776,-8.7276 -7.7071,-7.7109 l -21.3105,5.0234 -4.2031,-3.4551 0.7675,-21.8847 c 0.1695,
        -4.4311 -5.0172,-7.8932 -9.0586,-6.0547 l -19.8847,9.0664 -4.8418,-2.5977 -3.5156,-21.5664 c -0.7117,-4.3718 -6.4637,
        -6.7606 -10.0645,-4.1679 l -17.7305,12.7558 -5.2695,-1.5976 -7.6465,-20.4532 c -1.5504,-4.1685 -7.6589,-5.3673 -10.6836,
        -2.1308 l -14.9199,16.0097 -5.4219,-0.5293 -11.5234,-18.6191 c -1.165,-1.8851 -3.3059,-2.8281 -5.4473,-2.8281 z m -0.3183,
        39.0273 c 7.1191,0.2082 12.7821,6.0399 12.7812,13.1621 -0.036,7.246 -5.9199,13.1016 -13.166,13.1016 -7.2461,0 -13.1303,
        -5.8556 -13.166,-13.1016 -9e-4,-7.4228 6.1312,-13.379 13.5508,-13.1621 z m 30.1718,21.6797 c 46.8033,8.7673 87.3844,37.6698 
        110.9707,79.0352 l -15.5293,35.0761 c -2.6815,6.0705 0.056,13.1714 6.1094,15.8614 l 29.8985,13.2793 c 0.9302,9.3532 1.0453,
        18.7694 0.3437,28.1425 -2.3121,32.5279 -16.6082,61.9623 -31.5527,84.8067 -4.4477,5.9131 -9.2949,11.5147 -14.5078,16.7656 
        l -27.8457,-5.9863 c -6.4857,-1.3894 -12.8646,2.736 -14.254,9.2343 l -6.6093,30.8399 c -43.005,19.4799 -92.3656,19.2432 
        -135.1817,-0.6484 l -6.6035,-30.8399 c -1.3895,-6.4899 -7.7705,-10.6242 -14.2519,-9.2305 l -27.2246,5.8457 c -18.8829,-19.6057 
        -32.3517,-46.9508 -40.052792,-70.5293 -5.413,-17.9722 -6.8869,-39.6839 -4.7461,-59.7089 l 28.361392,-12.6016 c 6.0578,-2.69 
        8.7954,-9.7908 6.1054,-15.8613 l -15.41847,-28.23491 c 22.9501,-43.2756 68.82067,-75.75469 112.05947,-85.20649 l 20.2734,21.2695 
        c 4.5836,4.7997 12.1753,4.9784 16.9707,0.3906 z m -65.5412,26.5901 c 10.5534,5.9789 5.3273,9.0727 1.9065,11.1833 l 
        -34.1133,19.3829 c -1.8868,10.0873 -15.2847,12.9704 -18.834,20.5625 -5.5733,13.9749 -7.5388,23.812 -17.5195,34.58 -2.5663,
        -3.3693 -2.8809,-8.3297 -2.586,-12.7597 l -2.9668,18.8261 -6.7851,7.9082 c -13.3233,25.0651 -13.2349,34.9456 -14.5155,62.0915 
        l 4.1888,-4.6238 c -3.8819,20.2985 -4.8107,27.1826 -4.9644,49.3764 6.0391,10.5788 13.002,20.1265 20.6544,28.697 7.6954,-1.5681 
        14.8565,-3.3824 20.2754,-4.6348 6.5535,-1.5146 11.7318,-0.5414 16.3105,2.4219 4.4696,2.8927 6.9498,7.8762 8.5215,14.8281 
        1.5578,6.8904 3.1658,15.1494 5.334,23.6191 31.0093,14.814 77.5528,15.106 105.5215,6.6426 l -8.153,-12.4009 -9.7181,3.2329 
        -1.7279,-24.9943 -8.713,14.1181 6.156,-37.9433 -13.6025,21.6241 11.636,-41.5101 -7.6776,3.8931 -2.3835,-22.1172 -21.6075,
        13.2168 -8.2285,14.7949 -1.5918,-6.9375 -11.4297,11.5683 -6.7617,-11.7187 -3.9629,1.7129 -3.2929,-14.1699 c -21.7188,7.045 
        -24.588,5.3234 -45.959,0.7343 l -1.5821,8.3242 -13.1191,-6.3828 -3.4351,-11.7999 -0.8903,-3.0581 -1.3348,-4.5853 c 
        4.8531,-6.9332 11.5348,-13.6695 18.2307,-18.729 l 8.9787,-0.015 c 16.9206,14.3068 22.7999,16.201 43.1285,20.795 l 
        -2.3203,9.6425 5.3574,-6.0683 6.9629,-2.3242 5.8457,-17.4688 c -4.1256,3.2853 -8.255,7.1879 -13.8183,6.9727 -22.5537,
        -2.5343 -26.7858,-4.7127 -44.4063,-16.5567 -4.2425,-13.259 -2.9455,-10.774 4.7012,-22.2304 l 12.8574,-3.9297 6.25,-6.0703 
        -12.7265,-8.1583 -14.6133,3.7149 -2.9004,6.5058 -2.0234,-6.8437 -8.5196,-5.1289 24.4258,-26.0723 c -3.8904,-4.447 -7.4299,
        -1.1034 -12.3027,0.8028 l 6.9101,-8.0371 3.7442,-9.4747 1.8086,1.334 0.875,5.6075 4.1074,2.6757 0.1797,2.8575 8.3554,
        -7.6407 7.8575,-13.3925 -8.2129,3.455 -12.4707,-4.705 1.2207,-3.5743 c 1.7725,-4.5334 10.8131,-4.6077 14.5371,-14.9394 
        l 26.4082,-12.3653 c 8.0166,-4.1158 10.4266,3.5167 6.1836,11.211 l -13.1055,21.3379 22.084,-21.8809 c 8.5665,-9.885 2.0885,
        -31.7889 -16.6389,-35.0097 z m 136.1545,19.5427 c -3.2412,-0.067 -7.179,0.4855 -11.8867,1.7363 35.2691,-0.5633 15.9833,
        40.0662 -2.5391,57.5547 2.7108,8.1296 6.6751,11.1062 13.4864,15.4864 l -2.3946,-10.7735 c 20.7201,-27.8521 27.6543,
        -62.6598 3.334,-64.0039 z m -24.6211,18.4551 c -7.4897,7.1855 -14.537,14.0869 -17.0976,23.6953 l 4.0195,0 3.5977,
        15.6582 2.75,-3.1738 7.1191,0.3203 -3.5996,-7.6094 c 4.3309,-16.5471 8.4807,-14.1966 3.2109,-28.8906 z m 
        -87.0742,32.5176 -5.5352,13.3183 -7.6035,11.4258 c 10.7182,-6.2141 18.2156,-5.4537 30.1836,-5.3223 l 5.0996,6.1016 
        0.1426,-9.7422 9.7676,-4.1777 -9.0195,-13.2031 c -3.5598,-4.3115 -21.1851,-2.5003 -23.0352,1.5996 z m -146.156192,0.4238 
        c 7.118292,0.2082 12.780992,6.0388 12.781192,13.1602 -0.036,7.2461 -5.9199,13.1015 -13.165992,13.1015 -7.2462,0 
        -13.1303,-5.8554 -13.166,-13.1015 2e-4,-7.422 6.1319,-13.377 13.5508,-13.1602 z m 157.874992,0.057 c -2.9435,1.8505 
        -4.5411,5.6736 -3.9864,9.5391 0.3712,2.5335 1.6309,4.7854 3.4766,6.2148 -5.0612,0.18 -9.0937,-2.3398 -9.6328,-6.0195 
        -0.6222,-4.2599 3.6134,-8.5733 9.4629,-9.6367 0.2261,-0.038 0.4527,-0.07 0.6797,-0.098 z m 156.2539,0.5567 
        c 7.1184,0.2082 12.781,6.0387 12.7812,13.1601 -0.036,7.2461 -5.9199,13.1016 -13.166,13.1016 -7.2461,0 -13.1303,
        -5.8555 -13.166,-13.1016 2e-4,-7.422 6.1319,-13.377 13.5508,-13.1601 z m -131.4903,53.998 -7.3105,25.7227 
        0.4804,19.7851 9.0157,-17.8906 z m -132.0385,1.2128 4.894,1.1227 c 9.5988,11.9136 9.7535,8.7816 7.5801,22.7168 
        -5.7663,2.4849 -7.9484,2.2921 -13.1387,-0.3926 -3.3584,-8.2662 -1.2505,-15.1608 0.6646,-23.4469 z m 8.9233,129.7091 
        c 7.1184,0.2082 12.781,6.0388 12.7812,13.1602 -0.036,7.2461 -5.9198,13.1015 -13.166,13.1015 -7.2461,0 -13.1303,-5.8554 
        -13.166,-13.1015 2e-4,-7.422 6.132,-13.377 13.5508,-13.1602 z m 195.082,0.6133 c 7.1192,0.2082 12.7821,6.0399 12.7813,
        13.1621 -0.036,7.246 -5.9199,13.1016 -13.166,13.1016 -7.2462,0 -13.1303,-5.8556 -13.1661,-13.1016 -8e-4,-7.4228 6.1312,
        -13.379 13.5508,-13.1621 z".parse_svg::<Vec<PathEvent>>().expect("lyon d2");

        assert_eq!(
            events[events.len() - 2],
            PathEvent::CubicBezier(
                vec![
                    CubicBezier {
                        ctrl1: Point(7.1192, 0.2082),
                        ctrl2: Point(12.7821, 6.0399),
                        to: Point(12.7813, 13.1621),
                    },
                    CubicBezier {
                        ctrl1: Point(-0.036, 7.246),
                        ctrl2: Point(-5.9199, 13.1016),
                        to: Point(-13.166, 13.1016),
                    },
                    CubicBezier {
                        ctrl1: Point(-7.2462, 0.),
                        ctrl2: Point(-13.1303, -5.8556),
                        to: Point(-13.1661, -13.1016),
                    },
                    CubicBezier {
                        ctrl1: Point(-8e-4, -7.4228),
                        ctrl2: Point(6.1312, -13.379),
                        to: Point(13.5508, -13.1621),
                    }
                ],
                true
            )
        );

        assert_eq!(
            "M100,200 C100,100 250,100,\n\t250,200 S400,300 400,200".parse_svg(),
            Ok(vec![
                PathEvent::MoveTo {
                    points: vec![Point(100., 200.)],
                    relative: false
                },
                PathEvent::CubicBezier(
                    vec![CubicBezier {
                        ctrl1: Point(100., 100.),
                        ctrl2: Point(250., 100.),
                        to: Point(250., 200.)
                    }],
                    false
                ),
                PathEvent::CubicBezierSmooth(
                    vec![CubicBezierSmooth {
                        ctrl2: Point(400., 300.),
                        to: Point(400., 200.)
                    }],
                    false
                ),
            ]),
        );

        assert_eq!(
            "M200,300 Q400,50 600,300 T1000,300".parse_svg(),
            Ok(vec![
                PathEvent::MoveTo {
                    points: vec![Point(200., 300.)],
                    relative: false
                },
                PathEvent::QuadraticBezier(
                    vec![QuadraticBezier {
                        ctrl: Point(400., 50.),
                        to: Point(600., 300.)
                    }],
                    false
                ),
                PathEvent::QuadraticBezierSmooth(vec![Point(1000., 300.)], false),
            ]),
        );

        assert_eq!(
            "M300,200 h-150 a150,150 0 1,0 150,-150 z".parse_svg(),
            Ok(vec![
                PathEvent::MoveTo {
                    points: vec![Point(300., 200.)],
                    relative: false
                },
                PathEvent::Horizontal(-150., true,),
                PathEvent::Arc(
                    vec![Arc {
                        rx: 150.,
                        ry: 150.,
                        x_rotation: 0.,
                        large_arc: true,
                        sweep: false,
                        to: Point(150., -150.)
                    }],
                    true
                ),
                PathEvent::Close
            ]),
        );

        assert_eq!(
            "M600,350 l 50,-25 
           a25,25 -30 0,1 50,-25 l 50,-25 
           a25,50 -30 0,1 50,-25 l 50,-25 
           a25,75 -30 0,1 50,-25 l 50,-25 
           a25,100 -30 0,1 50,-25 l 50,-25"
                .parse_svg(),
            Ok(vec![
                PathEvent::MoveTo {
                    points: vec![Point(600., 350.)],
                    relative: false,
                },
                PathEvent::LineTo {
                    points: vec![Point(50., -25.)],
                    relative: true
                },
                PathEvent::Arc(
                    vec![Arc {
                        rx: 25.,
                        ry: 25.,
                        x_rotation: -30.,
                        large_arc: false,
                        sweep: true,
                        to: Point(50., -25.)
                    }],
                    true
                ),
                PathEvent::LineTo {
                    points: vec![Point(50., -25.)],
                    relative: true
                },
                PathEvent::Arc(
                    vec![Arc {
                        rx: 25.,
                        ry: 50.,
                        x_rotation: -30.,
                        large_arc: false,
                        sweep: true,
                        to: Point(50., -25.)
                    }],
                    true
                ),
                PathEvent::LineTo {
                    points: vec![Point(50., -25.)],
                    relative: true
                },
                PathEvent::Arc(
                    vec![Arc {
                        rx: 25.,
                        ry: 75.,
                        x_rotation: -30.,
                        large_arc: false,
                        sweep: true,
                        to: Point(50., -25.)
                    }],
                    true
                ),
                PathEvent::LineTo {
                    points: vec![Point(50., -25.)],
                    relative: true
                },
                PathEvent::Arc(
                    vec![Arc {
                        rx: 25.,
                        ry: 100.,
                        x_rotation: -30.,
                        large_arc: false,
                        sweep: true,
                        to: Point(50., -25.)
                    }],
                    true
                ),
                PathEvent::LineTo {
                    points: vec![Point(50., -25.)],
                    relative: true
                },
            ])
        );

        assert_eq!(
            "L 10,20 l20,1e10".parse_svg(),
            Ok(vec![
                PathEvent::LineTo {
                    points: vec![Point(10., 20.)],
                    relative: false
                },
                PathEvent::LineTo {
                    points: vec![Point(20., 1e10)],
                    relative: true
                }
            ])
        );

        assert_eq!(
            "H 10,20 v20,1e10".parse_svg(),
            Ok(vec![
                PathEvent::Horizontal(20., false),
                PathEvent::Vertical(1e10, true)
            ])
        );
    }

    #[test]
    fn test_path_events2() {
        assert_eq!("z".parse_svg(), Ok(PathEvent::Close));
        assert_eq!("Z".parse_svg(), Ok(PathEvent::Close));

        assert_eq!(
            "M10.1,20e-10".parse_svg(),
            Ok(PathEvent::MoveTo {
                points: vec![Point(10.1, 20e-10)],
                relative: false
            })
        );
        assert_eq!(
            "m10.1,20e-10".parse_svg(),
            Ok(PathEvent::MoveTo {
                points: vec![Point(10.1, 20e-10)],
                relative: true
            })
        );

        assert_eq!(
            "L10.1,20e-10".parse_svg(),
            Ok(PathEvent::LineTo {
                points: vec![Point(10.1, 20e-10)],
                relative: false
            })
        );
        assert_eq!(
            "l 10.1,20e-10".parse_svg(),
            Ok(PathEvent::LineTo {
                points: vec![Point(10.1, 20e-10)],
                relative: true
            })
        );

        assert_eq!(
            "M100,200 c100,100 250,100,\n\t250,200 s400,300 400,200".parse_svg(),
            Ok(vec![
                PathEvent::MoveTo {
                    points: vec![Point(100., 200.)],
                    relative: false
                },
                PathEvent::CubicBezier(
                    vec![CubicBezier {
                        ctrl1: Point(100., 100.),
                        ctrl2: Point(250., 100.),
                        to: Point(250., 200.)
                    }],
                    true
                ),
                PathEvent::CubicBezierSmooth(
                    vec![CubicBezierSmooth {
                        ctrl2: Point(400., 300.),
                        to: Point(400., 200.)
                    }],
                    true
                ),
            ]),
        );

        assert_eq!(
            "M200,300 q400,50 600,300 t1000,300".parse_svg(),
            Ok(vec![
                PathEvent::MoveTo {
                    points: vec![Point(200., 300.)],
                    relative: false
                },
                PathEvent::QuadraticBezier(
                    vec![QuadraticBezier {
                        ctrl: Point(400., 50.),
                        to: Point(600., 300.)
                    }],
                    true
                ),
                PathEvent::QuadraticBezierSmooth(vec![Point(1000., 300.)], true),
            ]),
        );
    }

    #[test]
    fn test_quad_01() {
        "M 200 300 Q 400 50 600 300 T 1000 300"
            .parse_svg::<Vec<PathEvent>>()
            .unwrap();
    }
}
