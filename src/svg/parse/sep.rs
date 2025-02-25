use parserc::{ControlFlow, ParseContext, Parser, ParserExt, Span, ensure_char, take_while};

use super::ParseError;

pub(super) fn skip_ws(ctx: &mut ParseContext<'_>) -> parserc::Result<Span, ParseError> {
    take_while(|c| c.is_whitespace())
        .parse(ctx)?
        .ok_or(ControlFlow::Recoverable(None))
}

/// Parse `[ws] comma [ws]` seperate token.
pub(super) fn parse_sep(ctx: &mut ParseContext<'_>) -> parserc::Result<Span, ParseError> {
    let start = skip_ws.ok().parse(ctx)?;

    if let Some(comma) = ensure_char(',').ok().parse(ctx)? {
        if let Some(end) = skip_ws.ok().parse(ctx)? {
            Ok(start.unwrap_or(comma).extend_to_inclusive(end))
        } else {
            Ok(start.unwrap_or(comma).extend_to_inclusive(comma))
        }
    } else {
        start.ok_or(ControlFlow::Recoverable(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comma() {
        assert_eq!(
            parse_sep(&mut ParseContext::from("")),
            Err(ControlFlow::Recoverable(None))
        );

        assert_eq!(
            parse_sep(&mut ParseContext::from("\t,\n")),
            Ok(Span::new(0, 3, 1, 1))
        );

        assert_eq!(
            parse_sep(&mut ParseContext::from(",  ")),
            Ok(Span::new(0, 3, 1, 1))
        );

        assert_eq!(
            parse_sep(&mut ParseContext::from("  ,")),
            Ok(Span::new(0, 3, 1, 1))
        );
    }
}
