use super::runtime::*;
use super::{ParseError, Token};
use regex::{Regex, RegexBuilder};

use nom::{
    bytes::complete::{is_a, take_till},
    character::complete::one_of,
    combinator::opt,
    IResult,
};

pub(crate) trait Parsable: Sized {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError>;
}

impl Parsable for DispatchCase {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let (tokens, cond) = Condition::parse(tokens)?;

        let (tokens, n) = next(tokens)?;

        if !matches!(n, Token::Begin { sym: '{' }) {
            return Err(ParseError::InvalidExpectation {
                expect: "{".to_string(),
                reality: format!("{:?}", n),
            });
        }

        let mut actions = Vec::new();

        let mut loop_tokens = tokens;
        loop {
            let (t, n) = next(loop_tokens)?;

            if matches!(n, Token::End { sym: '}' }) {
                loop_tokens = t;
                break;
            }

            let (tokens, action_gen) = ActionGen::parse(loop_tokens)?;

            actions.push(action_gen);
            loop_tokens = tokens;
        }

        Ok((loop_tokens, DispatchCase { cond, actions }))
    }
}

impl Parsable for Condition {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        Condition::parse_or(tokens)
    }
}

impl Condition {
    fn parse_or<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let (tokens, lh) = Condition::parse_and(tokens)?;

        match next(tokens)? {
            (tokens, Token::Ident { content: "or" }) => {
                let (tokens, rh) = Condition::parse(tokens)?;

                Ok((tokens, Condition::Or(Box::new(lh), Box::new(rh))))
            }

            _ => Ok((tokens, lh)),
        }
    }

    fn parse_and<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let (tokens, lh) = Condition::parse_not(tokens)?;

        match next(tokens)? {
            (tokens, Token::Ident { content: "and" }) => {
                let (tokens, rh) = Condition::parse(tokens)?;

                Ok((tokens, Condition::And(Box::new(lh), Box::new(rh))))
            }

            _ => Ok((tokens, lh)),
        }
    }

    fn parse_not<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        match next(tokens)? {
            (tokens, Token::Ident { content: "not" }) => {
                let (tokens, sub) = Condition::parse_op(tokens)?;

                Ok((tokens, Condition::Not(Box::new(sub))))
            }

            _ => Condition::parse_op(tokens),
        }
    }

    fn parse_op<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        match next(tokens)? {
            (tokens, Token::Begin { sym: '(' }) => {
                let (tokens, op) = Op::parse(tokens)?;
                let (tokens, end) = next(tokens)?;

                if !matches!(end, Token::Begin { sym: ')' }) {
                    return Err(ParseError::InvalidExpectation {
                        reality: format!("{:?}", end),
                        expect: "end parenthesis".to_string(),
                    });
                }

                Ok((tokens, Condition::Op(op)))
            }

            _ => {
                let (tokens, op) = Op::parse(tokens)?;

                Ok((tokens, Condition::Op(op)))
            }
        }
    }
}

impl Parsable for ActionGen {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let (tokens, n) = next(tokens)?;

        match n {
            Token::Ident { content: "notify" } => Ok((tokens, ActionGen::Notify)),
            Token::Ident { content: "record" } => Ok((tokens, ActionGen::Record)),
            Token::Ident { content: "drop" } => Ok((tokens, ActionGen::Drop)),
            Token::Ident { content: "exec" } => {
                let (tokens, si) = StringInterpol::parse(tokens)?;

                Ok((tokens, ActionGen::Exec(si)))
            }

            _ => {
                return Err(ParseError::InvalidExpectation {
                    expect: "action keyword".to_string(),
                    reality: format!("{:?}", n),
                })
            }
        }
    }
}

impl Parsable for Op {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let (tokens, lh) = Expr::parse(tokens)?;

        let (tokens, token) = next(tokens)?;

        match token {
            Token::Ident { content: "is" } => {
                let (tokens, rh) = Expr::parse(tokens)?;

                Ok((tokens, Op::Is(lh, rh)))
            }

            Token::Ident { content: "in" } => {
                let (tokens, rh) = Expr::parse(tokens)?;

                Ok((tokens, Op::In(lh, rh)))
            }

            Token::Ident { content: "matches" } => {
                let (tokens, rh) = Regex::parse(tokens)?;

                Ok((tokens, Op::Matches(lh, rh)))
            }

            _ => {
                return Err(ParseError::InvalidExpectation {
                    reality: format!("{:?}", token),
                    expect: "token should be is, in, matches".to_string(),
                })
            }
        }
    }
}

impl Parsable for Regex {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let (tokens, t) = next(tokens)?;
        match t {
            Token::Regex { content, flags } => {
                let mut builder = RegexBuilder::new(&content);

                if flags.find('i').is_some() {
                    builder.case_insensitive(true);
                }

                if flags.find('m').is_some() {
                    builder.multi_line(true);
                }

                if flags.find('s').is_some() {
                    builder.dot_matches_new_line(true);
                }

                if flags.find('U').is_some() {
                    builder.swap_greed(true);
                }

                if flags.find('x').is_some() {
                    builder.ignore_whitespace(true);
                }

                let regex = builder.build().or(Err(ParseError::InvalidExpectation {
                    expect: "valid regex".to_string(),
                    reality: format!("{}", content),
                }))?;

                Ok((tokens, regex))
            }

            _ => Err(ParseError::InvalidExpectation {
                reality: format!("{:?}", t),
                expect: "a regex".to_string(),
            }),
        }
    }
}

impl Parsable for Expr {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let s = StringInterpol::parse(tokens).map(|(tokens, s)| (tokens, Expr::Str(s)));
        let v = Var::parse(tokens).map(|(tokens, v)| (tokens, Expr::Var(v)));

        let n = next(tokens).and_then(|(tokens, t)| match t {
            Token::Ident { content: "null" } => Ok((tokens, Expr::Null)),
            _ => Err(ParseError::InvalidExpectation {
                expect: "null".to_string(),
                reality: format!("{:?}", t),
            }),
        });

        match s.or(v).or(n) {
            Ok((tokens, e)) => Ok((tokens, e)),
            a => a,
        }
    }
}

impl Parsable for Var {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let (tokens, t) = next(tokens)?;

        let var = match t {
            Token::Ident { content } => match *content {
                "url" => Var::URL,
                "title" => Var::Title,
                "categories" => Var::Categories,
                "description" => Var::Description,

                "feed_url" => Var::FeedURL,
                "feed_title" => Var::FeedTitle,
                "feed_categories" => Var::FeedCategories,

                _ => {
                    return Err(ParseError::InvalidExpectation {
                        reality: format!("{:?}", t),
                        expect: "a variable ident".to_string(),
                    })
                }
            },

            _ => {
                return Err(ParseError::InvalidExpectation {
                    reality: format!("{:?}", t),
                    expect: "a variable ident".to_string(),
                })
            }
        };

        Ok((tokens, var))
    }
}

impl Parsable for StringInterpol {
    fn parse<'input, 'tokens>(
        tokens: &'tokens [Token<'input>],
    ) -> Result<(&'tokens [Token<'input>], Self), ParseError> {
        let (tokens, tok) = next(tokens)?;

        let s = match tok {
            Token::Str {
                content,
                interpolated: false,
            } => StringInterpol::Inert(escapes(&content, false)),

            Token::Str {
                content,
                interpolated: true,
            } => {
                parse_interpol_str(&content)
                    .or(Err(ParseError::InvalidExpectation {
                        reality: format!("{:?}", content),
                        expect: "a valid string".to_string(),
                    }))?
                    .1
            }

            _ => {
                return Err(ParseError::InvalidExpectation {
                    reality: format!("{:?}", tok),
                    expect: "string token".to_string(),
                })
            }
        };

        Ok((tokens, s))
    }
}

fn next<'input, 'tokens>(
    tokens: &'tokens [Token<'input>],
) -> Result<(&'tokens [Token<'input>], &'tokens Token<'input>), ParseError> {
    if tokens.len() > 0 {
        Ok((&tokens[1..], &tokens[0]))
    } else {
        Err(ParseError::EndOfTokens)
    }
}

fn parse_interpol_str(mut input: &str) -> IResult<&str, StringInterpol> {
    let mut lits = Vec::new();
    let mut vars = Vec::new();

    loop {
        let (i, lit) = parse_lit(input)?;
        lits.push(lit);

        if i.is_empty() {
            return Ok((i, StringInterpol::Live { lits, vars }));
        }

        let (i, var) = parse_inter_var(i)?;
        vars.push(var);

        input = i;
    }
}

fn parse_lit(input: &str) -> IResult<&str, String> {
    let mut buf = String::new();
    let (mut input, mut part) = take_till(|c| c == '$')(input)?;
    buf.push_str(part);

    while count_end(part, '\\') & 1 == 1 {
        let (i, p) = take_till(|c| c == '$')(input)?;

        input = i;
        part = p;

        buf.push_str(part);
    }

    Ok((input, escapes(&buf, true)))
}

fn parse_inter_var(input: &str) -> IResult<&str, Var> {
    let (input, _) = one_of("$")(input)?;

    let (t, bracket) = opt(one_of("{"))(input)?;

    let (input, content) = if bracket.is_some() {
        let (input, content) = take_till(|c| c == '}')(t)?;
        let (input, _) = one_of("}")(input)?;

        (input, content)
    } else {
        is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")(input)?
    };

    let var = match content {
        "url" => Var::URL,
        "title" => Var::Title,
        "categories" => Var::Categories,
        "description" => Var::Description,

        "feed_url" => Var::FeedURL,
        "feed_title" => Var::FeedTitle,
        "feed_categories" => Var::FeedCategories,

        _ => {
            return Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Fix,
            )))
        }
    };

    Ok((input, var))
}

fn count_end(input: &str, ch: char) -> usize {
    let mut cnt = 0;

    for c in input.chars() {
        if c == ch {
            cnt += 1;
        } else {
            cnt = 0;
        }
    }

    cnt
}

fn escapes(s: &str, interpolated: bool) -> String {
    let n = s.replace("\\n", "\n");
    let n = n.replace("\\\\", "\\");
    if interpolated {
        n.replace("\\", "$")
    } else {
        n
    }
}
