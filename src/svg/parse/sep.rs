use parserc::{ensure_char, take_while, ControlFlow, ParseContext, Parser, ParserExt, Span};

pub(super) fn skip_ws(ctx: &mut ParseContext<'_>) -> parserc::Result<Span> {
    take_while(|c| c.is_whitespace())
        .parse(ctx)?
        .ok_or(ControlFlow::Recoverable)
}

/// Parse `[ws] comma [ws]` seperate token.
pub(super) fn parse_sep(ctx: &mut ParseContext<'_>) -> parserc::Result<Span> {
    let start = skip_ws.ok().parse(ctx)?;

    if let Some(comma) = ensure_char(',').ok().parse(ctx)? {
        if let Some(end) = skip_ws.ok().parse(ctx)? {
            Ok(start.unwrap_or(comma).extend_to_inclusive(end))
        } else {
            Ok(start.unwrap_or(comma).extend_to_inclusive(comma))
        }
    } else {
        start.ok_or(ControlFlow::Recoverable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comma() {
        assert_eq!(
            parse_sep(&mut ParseContext::from("")),
            Err(ControlFlow::Recoverable)
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
