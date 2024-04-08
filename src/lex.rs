mod token;
pub use token::{Bracket, Token, TokenType, VecNum};

/// Parses a single token onto a given token stack
fn parse_token(token_list: &mut Vec<Token>, latest_char: char) {
  let mut last_token = token_list.last_mut();
  let mut token: Token;

  // As per [spec](https://spec.commonmark.org/0.31.2/#backslash-escapes) Any ASCII punctuation character may be backslash-escaped
  // but Backslashes before other characters are treated as literal backslashes
  let mut escaped = false;
  if last_token == Some(&mut Token::Escape) {
    for escapable_char in "!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~".chars() {
      if latest_char == escapable_char {
        token_list.pop(); // Remove the escape from the stack as it's been used
        last_token = token_list.last_mut(); // Grab the new last token
        escaped = true;
      }
    }
  }

  // What token is this?
  token = match latest_char {
    '#' => Token::Hash,
    '!' => Token::Bang,
    '(' => Token::OpenBracket(Bracket::Parenthesis),
    ')' => Token::CloseBracket(Bracket::Parenthesis),
    '[' => Token::OpenBracket(Bracket::Square),
    ']' => Token::CloseBracket(Bracket::Square),
    '{' => Token::OpenBracket(Bracket::Brace),
    '}' => Token::CloseBracket(Bracket::Brace),
    '<' => Token::OpenBracket(Bracket::Angle),
    '>' => Token::CloseBracket(Bracket::Angle),
    '*' => Token::Star,
    '-' => Token::Dash,
    '+' => Token::Plus,
    '=' => Token::Equals,
    '.' => Token::Dot,
    '_' => Token::Underscore,
    '`' => Token::BackTick,
    '~' => Token::Tilde,
    '\\' => Token::Escape,
    '\n' => Token::NewLine,
    '\t' => Token::Tab,
    ' ' => Token::Space,
    // Below this point are tokens with a bit of weirdness to them as they aren't direct one to one mappings of characters

    // Numbers
    '0'..='9' => match last_token {
      Some(Token::Text(..)) => Token::Text(latest_char.to_string()), // If the previous token is a string then the number is part of it
      _ => Token::Number(VecNum(vec![latest_char.to_string().parse::<u8>().unwrap()])), // otherwise attempt to parse the number
    },

    // This is a regular text char
    _ => {
      let mut text = String::new();
      // replace Unicode character `U+0000` with `U+FFFD` per https://spec.commonmark.org/0.31.2/#insecure-characters
      let mut char = latest_char;
      if char == '\u{0000}' {
        char = '\u{FFFD}';
      }

      // Convert the character to a text token
      text.push_str(&char.to_string());
      Token::Text(text)
    }
  };

  // If this char should be escaped and is not already being treated as text
  if escaped && (matches!(token, Token::Text(..)) == false) {
    token = Token::Text(latest_char.to_string()); // Escape the char that needs to be escaped
  }

  // If the last token was text or number and this token is the same type then we should merge this token into the previous token
  // rather than creating a whole new token. We should then return early to avoiding adding our new token to the stack
  match last_token {
    Some(Token::Text(last_string)) => match token {
      Token::Text(char) => {
        last_string.push_str(&char.to_string());
        return;
      }
      _ => {} // Do Nothing
    },
    Some(Token::Number(num_list)) => match token {
      Token::Number(num) => {
        num_list.0.push(num.0[0]);
        return;
      }
      _ => {} // Do Nothing
    },
    _ => {} // Do Nothing
  };

  // Add the new token to the token stack
  token_list.push(token);
}

/// Parses a markdown string into Tokens
pub fn lex(markdown: &str) -> Vec<Token> {
  let mut tokens = vec![];
  for char in markdown.chars() {
    parse_token(&mut tokens, char);
  }
  tokens
}

#[cfg(test)]
mod tests;
