
use nom::{
    IResult,
    branch::alt,
    multi::{many0, many0_count},
    combinator::recognize,
    sequence::{pair, delimited, tuple},
    character::complete::{alpha1, alphanumeric1},
    bytes::complete::tag,
  };
use std::vec::Vec;

  pub fn obj_start (i: &str) -> IResult<&str, &str> {tag("[")(i)}
  pub fn obj_end (i: &str) -> IResult<&str, &str> {tag("]")(i)}
  pub fn sigma (i: &str) -> IResult<&str, &str> {tag("sigma")(i)}
  pub fn bracket_start (i: &str) -> IResult<&str, &str> {tag("(")(i)}
  pub fn bracket_end (i: &str) -> IResult<&str, &str> {tag(")")(i)}
  pub fn equals (i: &str) -> IResult<&str, &str> {tag("=")(i)}

  pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(
      pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"))))
      )
    )(input)
  }

  pub fn nametag(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (tag, _, _, _, selfname, _, expr)) = 
        tuple((identifier, equals, sigma, bracket_start, identifier, bracket_end, expression))(input)?;
    Ok((input, (tag, selfname, expr)))
  }

  pub fn object(input: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
    let tags: Vec<(&str, &str, &str)> = vec![];
    let (input, tags) = delimited(obj_start ,many0(nametag), obj_end)(input)?;
    Ok((input, tags))
  }

  pub fn expression(_input: &str) -> IResult<&str, &str>{
    //TODO
    Ok(("", "x"))
  }

  #[test]
  fn tag_test(){
    //TODO: whitespace
    assert_eq!(nametag("a=sigma(x)x"), Ok(("", ("a", "x", "x"))));
  }

  #[test]
  fn obj_test(){}