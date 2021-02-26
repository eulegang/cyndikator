use super::ParseError;

use nom::{
    branch::alt,
    bytes::complete::{escaped, is_a, is_not, tag},
    character::complete::{char, multispace1, one_of},
    combinator::{all_consuming, opt},
    multi::many0,
    IResult,
};

#[derive(Debug)]
pub(crate) enum Token<'input> {
    Comment {
        content: &'input str,
    },

    Str {
        content: &'input str,
        interpolated: bool,
    },

    Ident {
        content: &'input str,
    },

    Regex {
        content: &'input str,
        flags: &'input str,
    },

    Begin {
        sym: char,
    },

    End {
        sym: char,
    },

    Space,
}

impl<'a> Token<'a> {
    pub(crate) fn tokenize(input: &'a str) -> Result<Vec<Token<'a>>, ParseError> {
        all_consuming(many0(alt((
            parse_comment,
            parse_str,
            parse_ident,
            parse_context,
            parse_space,
            parse_regex,
        ))))(input)
        .or(Err(ParseError::Tokenize))
        .map(|s| s.1)
    }

    pub(crate) fn is_significant(&self) -> bool {
        !matches!(self, Token::Space | Token::Comment { .. })
    }
}

fn parse_comment(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("#")(input)?;
    let (input, content) = is_not("\r\n")(input)?;

    Ok((input, Token::Comment { content }))
}

fn parse_str(input: &str) -> IResult<&str, Token> {
    let (input, ch) = one_of("\"'")(input)?;

    let (input, content) = match ch {
        '\'' => {
            let (input, content) = is_not("'")(input)?;
            let (input, _) = tag("'")(input)?;

            (input, content)
        }

        '"' => {
            let (input, content) = is_not("\"")(input)?;
            let (input, _) = tag("\"")(input)?;

            (input, content)
        }

        _ => unreachable!(),
    };

    let interpolated = ch == '"';

    Ok((
        input,
        Token::Str {
            content,
            interpolated,
        },
    ))
}

fn parse_ident(input: &str) -> IResult<&str, Token> {
    let (input, content) = is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")(input)?;

    Ok((input, Token::Ident { content }))
}

fn parse_context(input: &str) -> IResult<&str, Token> {
    let (input, sym) = one_of("{}()")(input)?;

    match sym {
        '{' => Ok((input, Token::Begin { sym })),
        '}' => Ok((input, Token::End { sym })),
        '(' => Ok((input, Token::End { sym })),
        ')' => Ok((input, Token::End { sym })),

        _ => unreachable!(),
    }
}

fn parse_space(input: &str) -> IResult<&str, Token> {
    let (input, _) = multispace1(input)?;

    Ok((input, Token::Space))
}

fn parse_regex(input: &str) -> IResult<&str, Token> {
    let (input, _) = char('/')(input)?;

    let (input, content) = escaped(is_not("/\\"), '\\', one_of("/dwsDWS\\"))(input)?;

    let (input, _) = char('/')(input)?;

    let (input, flags) = opt(is_a("i"))(input)?;
    let flags = flags.unwrap_or_default();

    Ok((input, Token::Regex { content, flags }))
}
