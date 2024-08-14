// Represents all possible error types after lexical analysis

pub enum Error<'a> {
    InvalidCharacter {
        character: char,
        location: SourceIndex,
        source: &'a str,
    },
    UnterminatedString {
        location: SourceIndex,
        source: &'a str,
    },
    UnterminatedComment {
        location: SourceIndex,
        source: &'a str,
    },
    InvalidEscapeSequence {
        location: SourceIndex,
        source: &'a str,
    },
    InvalidNumber {
        location: SourceIndex,
        source: &'a str,
    },
    InvalidIdentifier {
        location: SourceIndex,
        source: &'a str,
    },
    InvalidToken {
        location: SourceIndex,
        source: &'a str,
    },
    // Add more error types as needed
}
