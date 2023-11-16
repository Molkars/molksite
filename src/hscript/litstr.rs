use dparse::parse::{Parse, ParseError, ParseStream, Span};

#[derive(Debug)]
pub struct StrLit {
    pub content: String,
    pub span: Span,
}

impl<'a> Parse<'a> for StrLit {
    fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        input.take_while(|c| c.is_whitespace());
        let span = input.spanner();

        if !input.take_char('"') {
            return Err(input.mismatch());
        }

        let mut content = String::new();
        while let Some(c) = input.peek_char() {
            if c == '\r' || c == '\n' || c == '"' {
                break;
            }

            if c != '\\' {
                content.push(c);
                input.advance();
                continue;
            }

            input.advance();
            let Some(c) = input.advance() else {
                break;
            };

            match c {
                'n' => content.push('\n'),
                'r' => content.push('\r'),
                't' => content.push('\t'),
                '\\' => content.push('\\'),
                '"' => content.push('"'),
                'u' => {
                    if !input.take_char('{') {
                        return Err(input.error("invalid escape sequence"));
                    }
                    let span = input.spanner();
                    let _ = std::iter::from_fn(|| input.advance())
                        .take(4)
                        .filter(char::is_ascii_hexdigit)
                        .count();
                    let span = input.span(span);
                    if !input.take_char('}') {
                        return Err(input.error("invalid escape sequence -- missing closing `}`"));
                    }
                    let substring = input.source_for_span(span);
                    let Ok(codepoint) = u32::from_str_radix(substring, 16) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    let Some(codepoint) = char::from_u32(codepoint) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    content.push(codepoint);
                }
                'U' => {
                    if !input.take_char('{') {
                        return Err(input.error("invalid escape sequence"));
                    }

                    let span = input.spanner();
                    let _ = std::iter::from_fn(|| input.advance())
                        .take(8)
                        .filter(char::is_ascii_hexdigit)
                        .count();
                    let span = input.span(span);

                    if !input.take_char('}') {
                        return Err(input.error("invalid escape sequence -- missing closing `}`"));
                    }
                    let substring = input.source_for_span(span);
                    let Ok(codepoint) = u32::from_str_radix(substring, 16) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    let Some(codepoint) = char::from_u32(codepoint) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    content.push(codepoint);
                }
                'x' => {
                    let span = input.spanner();
                    let _ = std::iter::from_fn(|| input.advance())
                        .take(2)
                        .filter(char::is_ascii_hexdigit)
                        .count();
                    let span = input.span(span);
                    let substring = input.source_for_span(span);
                    let Ok(codepoint) = u32::from_str_radix(substring, 16) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    let Some(codepoint) = char::from_u32(codepoint) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    content.push(codepoint);
                }
                _ => return Err(input.error("invalid escape sequence")),
            }
        }
        if !input.take_char('"') {
            return Err(input.error("unterminated string literal"));
        }

        Ok(Self {
            content,
            span: input.span(span),
        })
    }
}

