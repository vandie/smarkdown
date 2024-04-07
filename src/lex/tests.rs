use crate::lex::{Bracket, VecNum};

use super::{lex, Token};

#[test]
fn lex_test() {
  let example_string = "# Title\nsome paragraph content with a [![alt text](example.png)](http://example.org)\n\nperhaps we'll throw in an ~~strikeout~~\n019";
  let expected = vec![
    Token::Hash,
    Token::Space,
    Token::Text("Title".to_string()),
    Token::NewLine,
    Token::Text("some paragraph content with a ".to_string()),
    Token::OpenBracket(Bracket::Square),
    Token::Bang,
    Token::OpenBracket(Bracket::Square),
    Token::Text("alt text".to_string()),
    Token::CloseBracket(Bracket::Square),
    Token::OpenBracket(Bracket::Parenthesis),
    Token::Text("example".to_string()),
    Token::Dot,
    Token::Text("png".to_string()),
    Token::CloseBracket(Bracket::Parenthesis),
    Token::CloseBracket(Bracket::Square),
    Token::OpenBracket(Bracket::Parenthesis),
    Token::Text("http://example".to_string()),
    Token::Dot,
    Token::Text("org".to_string()),
    Token::CloseBracket(Bracket::Parenthesis),
    Token::NewLine,
    Token::NewLine,
    Token::Text("perhaps we'll throw in an ".to_string()),
    Token::Tilde,
    Token::Tilde,
    Token::Text("strikeout".to_string()),
    Token::Tilde,
    Token::Tilde,
    Token::NewLine,
    Token::Number(VecNum(vec![0, 1, 9])),
  ];
  assert_eq!(lex(example_string), expected);
}
