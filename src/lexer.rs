use nom::branch::*;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0};
use nom::combinator::{map, map_res, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair};
use nom::*;

use std::str;
use std::str::FromStr;
use std::str::Utf8Error;

macro_rules! syntax {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name<'a>(s: &'a [u8]) -> IResult<&[u8], Token> {
            map(tag($tag_string), |_| $output_token)(s)
        }
    };
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,
    // identifier and literals
    Identifier(String),
    // operations
    Assign,
    Select,
    // reserved words
    Sigma,
    // punctuations
    Comma,
    LParen,
    RParen,
    LBracket,
    RBracket,
}

// operators
syntax! {assign_operator, "=", Token::Assign}
syntax! {select_operator, ".", Token::Select}
syntax! {comma, ",", Token::Comma}
syntax! {lparen, "(", Token::LParen}
syntax! {rparen, ")", Token::RParen}
syntax! {lbracket, "[", Token::LBracket}
syntax! {rbracket, "]", Token::RBracket}
syntax! {sigma, "sigma", Token::Sigma}

fn identifier(input: &[u8]) -> IResult<&[u8], Token> {
    map_res(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s| {
            let c = str::from_utf8(s);
            c.map(|syntax| match syntax {
                _ => Token::Identifier(syntax.to_string()),
            })
        },
    )(input)
}

fn punctuation(input: &[u8]) -> IResult<&[u8], Token> {
    alt((comma, lparen, rparen, rbracket, lbracket))(input)
}

fn keywords(input: &[u8]) -> IResult<&[u8], Token> {
    sigma(input)
}

fn operators(input: &[u8]) -> IResult<&[u8], Token> {
    alt((assign_operator, select_operator))(input)
}

fn lex_token(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        punctuation,
        keywords,
        operators,
        identifier,
        lex_illegal,
    ))(input)
}

fn complete_byte_slice_str_from_utf8(c: &[u8]) -> Result<&str, Utf8Error> {
    str::from_utf8(c)
}

// Illegal tokens
fn lex_illegal(input: &[u8]) -> IResult<&[u8], Token> {
    map(take(1usize), |_| Token::Illegal)(input)
}

pub fn lex_tokens2(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(bytes: &[u8]) -> IResult<&[u8], Vec<Token>> {
        lex_tokens2(bytes)
            .map(|(slice, result)| (slice, [&result[..], &vec![Token::EOF][..]].concat()))
        //????????
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

    #[test]
    fn test_lexer1() {
        let input = &b"[a = sigma(x) x]"[..];
        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::LBracket,
            Token::Identifier("a".to_owned()),
            Token::Assign,
            Token::Sigma,
            Token::LParen,
            Token::Identifier("x".to_owned()),
            Token::RParen,
            Token::Identifier("x".to_owned()),
            Token::RBracket,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }


    #[test]
    fn test_lexer2() {
        let input = &b"[a = sigma(x) x]"[..];
        let (_, result) = lex_tokens2(input).unwrap();

        let expected_results = vec![
            Token::LBracket,
            Token::Identifier("a".to_owned()),
            Token::Assign,
            Token::Sigma,
            Token::LParen,
            Token::Identifier("x".to_owned()),
            Token::RParen,
            Token::Identifier("x".to_owned()),
            Token::RBracket,
        ];

        assert_eq!(result, expected_results);
    } 
// }
