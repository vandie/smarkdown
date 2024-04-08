use super::blocks::{BlockType, ListType};
use crate::lex::{Bracket, Token, TokenType, VecNum};

const TAB_SIZE: usize = 4;

#[derive(Debug, PartialEq)]
pub struct Line(pub Vec<Token>);
impl Line {
  // Returns true if the Line is empty
  pub fn is_empty(&self) -> bool {
    self.0.iter().fold(true, |empty, token| match token {
      Token::Space => empty,
      Token::Tab => empty,
      _ => false,
    })
  }

  /// Removes all indentation from a line while returning the indentation level the line was previously
  pub(crate) fn remove_all_indentation(&mut self) -> usize {
    let mut space_count: usize = 0;
    if self.is_empty() {
      self.0 = vec![];
      return 0;
    }

    for (i, token) in self.0.iter().enumerate() {
      match token {
        Token::Space => space_count += 1,
        Token::Tab => space_count += TAB_SIZE,
        _ => {
          self.0.drain(..i);
          return space_count / TAB_SIZE;
        }
      }
    }

    // This shouldn't ever happen but the compiler can't seem to realise that
    space_count / TAB_SIZE
  }

  /// Counts the amount of leading spaces (up to 3) at the start of a line
  pub fn leading_spaces(&self) -> usize {
    let mut count = 0;
    while self.0.get(count) == Some(&Token::Space) && count < 4 {
      count += 1;
    }
    // Acording to spec, up to 3 leading spaces can be ignore
    if count == 4 {
      count = 0;
    }
    count
  }

  /// Gets the block type for a line so that it can be parsed
  ///
  /// None Lazy fallbacks will be ignored
  pub fn line_type(&self, fallback: BlockType) -> BlockType {
    // We should only overwrite the fallback if the type is lazy
    let fallback = match fallback {
      BlockType::List(..) => fallback,
      BlockType::BlockQuote => fallback,
      _ => BlockType::Paragraph,
    };

    let mut current: Vec<TokenType> = vec![];
    for (i, token) in self.0.iter().enumerate() {
      current.push(token.clone().into());
      match current.as_slice() {
        [TokenType::Dash] => {
          if fallback == BlockType::Paragraph && self.is_all(&TokenType::Dash, 1, false, true) {
            return BlockType::SetextHeader(2);
          }
          if self.is_all(&TokenType::Dash, 3, true, true) {
            return BlockType::ThematicBreak;
          }
        }
        [TokenType::Underscore] => {
          if self.is_all(&TokenType::Underscore, 3, true, true) {
            return BlockType::ThematicBreak;
          }
        }
        [TokenType::Star] => {
          if self.is_all(&TokenType::Star, 3, true, true) {
            return BlockType::ThematicBreak;
          }
        }
        [TokenType::Hash] => {
          if self.is_space(i + 1) {
            return BlockType::Header(1);
          }
        }
        [TokenType::Hash, TokenType::Hash] => {
          if self.is_space(i + 1) {
            return BlockType::Header(2);
          }
        }
        [TokenType::Hash, TokenType::Hash, TokenType::Hash] => {
          if self.is_space(i + 1) {
            return BlockType::Header(3);
          }
        }
        [TokenType::Hash, TokenType::Hash, TokenType::Hash, TokenType::Hash] => {
          if self.is_space(i + 1) {
            return BlockType::Header(4);
          }
        }
        [TokenType::Hash, TokenType::Hash, TokenType::Hash, TokenType::Hash, TokenType::Hash] => {
          if self.is_space(i + 1) {
            return BlockType::Header(5);
          }
        }
        [TokenType::Hash, TokenType::Hash, TokenType::Hash, TokenType::Hash, TokenType::Hash, TokenType::Hash] => {
          if self.is_space(i + 1) {
            return BlockType::Header(6);
          }
        }
        [TokenType::Equals] => {
          if fallback == BlockType::Paragraph && self.is_all(&TokenType::Equals, 1, false, true) {
            return BlockType::SetextHeader(1);
          }
        }
        [TokenType::CloseBracket(Bracket::Angle), TokenType::Space] => {
          return BlockType::BlockQuote
        }
        [TokenType::Dash, TokenType::Space] => return BlockType::List(ListType::Dash),
        [TokenType::Plus, TokenType::Space] => return BlockType::List(ListType::Plus),
        [TokenType::Star, TokenType::Space] => return BlockType::List(ListType::Star),
        [TokenType::Number, TokenType::CloseBracket(Bracket::Parenthesis), TokenType::Space] => {
          return BlockType::List(ListType::BracketedNumber(grab_number(&self, 0).to_usize()))
        }
        [TokenType::Number, TokenType::Dot, TokenType::Space] => {
          return BlockType::List(ListType::Number(grab_number(&self, 0).to_usize()))
        }
        _ => {}
      }
    }

    fallback
  }

  fn is_space(&self, i: usize) -> bool {
    let token = self.0.get(i);
    token == Some(&Token::Space) || token == Some(&Token::Tab) || token == None
  }

  fn to_token_types(&self) -> Vec<TokenType> {
    self
      .0
      .iter()
      .map(|t| Into::<TokenType>::into(t.clone()))
      .collect()
  }

  /// Checks if a line is entirely made up of a single token with a few exceptions.
  ///
  /// if `allow_inline_blanks` is `true` then spaces and tabs between chars will be ignored
  ///
  /// if `allow_inline_blanks` is `false`, spaces and tabs between chars will triger a failure
  ///
  /// if `allow_trailing_blanks` is `true`, and `allow_inline_blanks` is `false` then spaces and tabs at
  /// the end of a line will be ignored but only if no other characters exist within them
  ///
  /// if `allow_inline_blanks` is `true` then `allow_trailing_blanks` has no effect
  fn is_all(
    &self,
    needle: &TokenType,
    min_number: usize,
    allow_inline_blanks: bool,
    allow_trailing_blanks: bool,
  ) -> bool {
    let tokens = self.to_token_types();
    let mut count = 0;
    let mut on_trail: bool = false;
    tokens.iter().all(|token| {
      if token == needle {
        count += 1
      }
      if allow_trailing_blanks && !allow_inline_blanks {
        if token == &TokenType::Space || token == &TokenType::Tab {
          on_trail = true;
          return true;
        }
        if on_trail && !(token == &TokenType::Space || token == &TokenType::Tab) {
          return false;
        }
      }
      return token == needle
        || allow_inline_blanks && (token == &TokenType::Space || token == &TokenType::Tab);
    }) && count >= min_number
  }

  /// Remove the starting chars that label a block as a given type
  pub fn remove_type_chars(&mut self, line_type: &BlockType) {
    let leading_spaces = self.leading_spaces();
    match line_type {
      BlockType::Paragraph => {
        self.remove_all_indentation();
      }
      BlockType::BlockQuote => {
        if self.0.get(leading_spaces) == Some(&Token::CloseBracket(Bracket::Angle)) {
          self.trim_line_start(leading_spaces + 1);
        } else {
          // as this is a continuation, we need to do some weirdness to stop certain types from being converted when parsed within a blockquote
          if matches!(
            self.line_type(BlockType::Paragraph),
            BlockType::SetextHeader(..)
          ) {
            self.stringify_line();
          }
        }
      }
      BlockType::Header(level) => {
        let end = leading_spaces + *level as usize;
        if self.0.get(end - 1) == Some(&Token::Hash) {
          // This is horrible but the only way to handle blank headers right now
          self.trim_line_start(end);
        }

        self.remove_ending_blanks();
        if self.0.len() == 0 {
          return;
        }
        let mut end_to_remove = self.0.len();
        while end_to_remove > 1 && self.0.get(end_to_remove - 1) == Some(&Token::Hash) {
          end_to_remove -= 1;
        }
        if self.is_space(end_to_remove - 1) {
          self.0.drain(end_to_remove..);
          self.remove_ending_blanks();
        }
        self.remove_all_indentation();
      }
      BlockType::SetextHeader(_) => {
        let is_l1 = self.is_all(&TokenType::Equals, 1, false, true);
        let is_l2 = self.is_all(&TokenType::Dash, 1, false, true);
        if is_l1 || is_l2 {
          self.0 = vec![];
        }
      }
      _ => {}
    }
  }

  pub fn remove_ending_blanks(&mut self) {
    let mut end = self.0.len();
    if self.0.len() == 0 {
      return;
    }
    while self.is_space(end - 1) && end > 1 {
      end -= 1;
    }
    self.0.drain(end..);
  }

  /// Removed 1 level of indentation from a line (1 Tab or 4 Spaces)
  pub fn unindent(&mut self) {
    let indent = self.remove_all_indentation();
    if indent == 0 {
      return;
    }

    let mut new = vec![Token::Tab; indent - 1];
    new.append(&mut self.0);
    self.0 = new;
  }

  /// Removes a number of characters from the start of a line
  pub fn trim_line_start(&mut self, chars: usize) {
    self.0.drain(..chars);
  }

  pub fn find_first(&self, needle: TokenType) -> Option<usize> {
    for (i, token) in self.0.iter().enumerate() {
      if Into::<TokenType>::into(token.clone()) == needle {
        return Some(i);
      }
    }
    None
  }

  pub fn stringify_line(&mut self) {
    self.0 = vec![Token::Text(
      self
        .0
        .iter()
        .map(|i| Into::<String>::into(i.clone()))
        .collect::<Vec<String>>()
        .join(""),
    )];
  }
}

fn grab_number(line: &Line, index: usize) -> VecNum {
  match line.0.get(index) {
    Some(Token::Number(num)) => num.clone(),
    _ => VecNum(vec![0]),
  }
}
