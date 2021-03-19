use super::Parsable;
use crate::dispatch::{runtime::*, token::Token};

#[test]
fn test_cond() {
    let mut tokens = Token::tokenize_significant(
        "feed_title is 'Lobsters' and feed_title is 'Youtube' or feed_title is 'Hacker News'",
    )
    .unwrap();

    let (rest, condition) = Condition::parse(&tokens).unwrap();

    assert!(rest.is_empty());

    assert_eq!(
        condition,
        Condition::Or(
            Box::new(Condition::And(
                Box::new(Condition::Op(Op::Is(
                    Expr::Var(Var::FeedTitle),
                    Expr::Str(StringInterpol::Inert("Lobsters".to_string()))
                ))),
                Box::new(Condition::Op(Op::Is(
                    Expr::Var(Var::FeedTitle),
                    Expr::Str(StringInterpol::Inert("Youtube".to_string()))
                )))
            )),
            Box::new(Condition::Op(Op::Is(
                Expr::Var(Var::FeedTitle),
                Expr::Str(StringInterpol::Inert("Hacker News".to_string()))
            )))
        ),
    );
}
