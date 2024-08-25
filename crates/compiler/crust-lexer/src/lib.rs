use logos::Logos;

// Goal here is to turn token defintions into a single DFA,
// allowing for a faster lexer.

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Keywords
    #[token("fn")]
    Function,
    #[token("let")]
    Let,
    #[token("if")]
    If,
    #[token("else")]
    Else,

    // Identifiers (variable names, function names, etc.)
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Literals
    #[regex(r"[0-9]+")]
    Number,

    // Operators and punctuation
    #[token("=")]
    Equals,
    #[token(";")]
    Semicolon,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,

    // Skip whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    // No need for catch all, as we are using the `filter_map` when tokenizing
}

pub fn tokenize(source: &str) -> Vec<Token> {
    Token::lexer(source)
        .filter_map(Result::ok) // Only keep successful tokens
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let source_code = "fn main() { let x = 42; }";
        let tokens = tokenize(source_code);

        assert_eq!(
            tokens,
            vec![
                Token::Function,
                Token::Identifier,
                Token::OpenParen,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Let,
                Token::Identifier,
                Token::Equals,
                Token::Number,
                Token::Semicolon,
                Token::CloseBrace,
            ]
        );
    }
}
