/// The recognised types of bracket
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Bracket {
  Parenthesis,
  Square,
  Brace,
  Angle,
}

/// A lexical Token used by the Parser to understand the markdown
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
  Hash,
  Bang,
  OpenBracket(Bracket),
  CloseBracket(Bracket),
  Star,
  Dash,
  Plus,
  Equals,
  Dot,
  Underscore,
  BackTick,
  Tilde,
  Tab, // Handling for tab in the parser
  NewLine,
  Space,
  Text(String),
  Number(VecNum), // This is for handling series of numbers without loosing leading 0s. a u8 is likely still to big
  Escape,         // Used to handle escape chars
}

/// A lexical Token used by the Parser to understand the markdown
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenType {
  Hash,
  Bang,
  OpenBracket(Bracket),
  CloseBracket(Bracket),
  Star,
  Dash,
  Plus,
  Equals,
  Dot,
  Underscore,
  BackTick,
  Tilde,
  Tab,
  NewLine,
  Space,
  Text,
  Number,
  Escape, // Used to handle escape chars
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VecNum(pub Vec<u8>);
impl VecNum {
  pub fn to_string(&self) -> String {
    self
      .0
      .iter()
      .fold(String::new(), |str, val| format!("{str}{val}"))
  }

  pub fn to_usize(&self) -> usize {
    let mut total: usize = 0;
    for num in self.0.iter() {
      total = (total * 10) + (*num as usize);
    }
    total
  }
}

impl Into<TokenType> for Token {
  fn into(self) -> TokenType {
    match self {
      Token::Hash => TokenType::Hash,
      Token::Bang => TokenType::Bang,
      Token::OpenBracket(bracket) => TokenType::OpenBracket(bracket),
      Token::CloseBracket(bracket) => TokenType::CloseBracket(bracket),
      Token::Star => TokenType::Star,
      Token::Dash => TokenType::Dash,
      Token::Plus => TokenType::Plus,
      Token::Equals => TokenType::Equals,
      Token::Dot => TokenType::Dot,
      Token::Underscore => TokenType::Underscore,
      Token::BackTick => TokenType::BackTick,
      Token::Tilde => TokenType::Tilde,
      Token::Tab => TokenType::Tab,
      Token::NewLine => TokenType::NewLine,
      Token::Space => TokenType::Space,
      Token::Text(..) => TokenType::Text,
      Token::Escape => TokenType::Escape,
      Token::Number(..) => TokenType::Number,
    }
  }
}

impl Into<String> for Token {
  fn into(self) -> String {
    match self {
      Token::Hash => "#".to_string(),
      Token::Bang => "!".to_string(),
      Token::OpenBracket(bracket) => match bracket {
        Bracket::Angle => "<".to_string(),
        Bracket::Brace => "{".to_string(),
        Bracket::Parenthesis => "(".to_string(),
        Bracket::Square => "[".to_string(),
      },
      Token::CloseBracket(bracket) => match bracket {
        Bracket::Angle => ">".to_string(),
        Bracket::Brace => "}".to_string(),
        Bracket::Parenthesis => ")".to_string(),
        Bracket::Square => "]".to_string(),
      },
      Token::Star => "*".to_string(),
      Token::Dash => "-".to_string(),
      Token::Plus => "+".to_string(),
      Token::Equals => "=".to_string(),
      Token::Dot => ".".to_string(),
      Token::Underscore => "_".to_string(),
      Token::BackTick => "`".to_string(),
      Token::Tilde => "~".to_string(),
      Token::Tab => "\t".to_string(),
      Token::NewLine => "\n".to_string(),
      Token::Space => " ".to_string(),
      Token::Text(text) => text,
      Token::Escape => "\\".to_string(),
      Token::Number(val) => val.to_string(),
    }
  }
}
