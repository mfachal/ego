use nom::IResult;
mod parser;

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

fn main() -> () {
    let (remaining_input, output) = do_nothing_parser("my_input").unwrap();
    assert_eq!(remaining_input, "my_input");
    assert_eq!(output, "");
}
