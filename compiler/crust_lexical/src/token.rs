#[derive(Debug)]
enum TokenType {
    Keyword,
    Identifier,
    Comments,
    Space,
    Punctuation,
    Literal,
    EndOfFile,
    // Add more token types as needed
}

// Token struct representing a single token word
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token<'a> {
    token_type: TokenType,
    range: Range<SourceIndex>,
    lexeme: &'a str,
}

fn main() {
    // Your lexicon analysis code goes here
}
