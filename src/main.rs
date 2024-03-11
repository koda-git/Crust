use nom::{character::complete::alphanumeric1, IResult};

fn parse_alphanumeric(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn main() {
    let result = parse_alphanumeric("Hello123");
    println!("{:?}", result)
}


