use std::usize;

use crate::{lex::Token, parse::tokens_to_lines};

use super::{
  document::DocContext, inlines::Inline, list::parse_line_items, parse_inlines,
  parse_tokens_with_context,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum BlockType {
  Paragraph,
  BlockQuote,
  List(ListType),
  LineItem,
  ThematicBreak,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Block {
  Paragraph(Vec<Inline>),
  BlockQuote(Vec<Block>),
  List {
    list_type: ListType,
    inner: Vec<Block>,
    loose: bool,
  },
  LineItem {
    inner: Vec<Block>,
  },
  ThematicBreak,
}

impl Block {
  pub fn new(block_type: BlockType, inner: Vec<Token>, context: &mut DocContext) -> Block {
    match block_type {
      BlockType::Paragraph => Block::Paragraph(parse_inlines(&inner, context)),
      BlockType::BlockQuote => Block::BlockQuote(parse_tokens_with_context(&inner, context)),
      BlockType::List(list_type) => {
        let inner_blocks = parse_line_items(&inner, context, list_type);
        let loose = tokens_to_lines(&inner).iter().any(|line| line.is_empty());
        Block::List {
          list_type,
          inner: inner_blocks,
          loose,
        }
      }
      BlockType::LineItem => Block::LineItem {
        inner: parse_tokens_with_context(&inner, context),
      },
      BlockType::ThematicBreak => Block::ThematicBreak,
    }
  }

  pub fn as_html(&self, loose_mode: bool) -> String {
    match self {
      Block::Paragraph(inlines) => match loose_mode {
        true => format!("<p>{}</p>", Inline::vec_as_html(inlines)),
        false => Inline::vec_as_html(inlines),
      },
      Block::BlockQuote(blocks) => {
        format!(
          "<blockquote>\n{}\n</blockquote>",
          Block::vec_as_html(blocks, true)
        )
      }
      Block::List {
        list_type,
        inner,
        loose,
      } => {
        let mut start = String::new();
        let list_name = match list_type {
          ListType::Number(start_num) => {
            if *start_num != 1 {
              start = format!(" start=\"{start_num}\"");
            }
            "ol"
          }
          _ => "ul",
        };
        format!(
          "<{list_name}{start}>\n{}\n</{list_name}>",
          Block::vec_as_html(inner, *loose)
        )
      }
      Block::LineItem { inner, .. } => {
        format!("<li>{}</li>", Block::vec_as_html(inner, loose_mode))
      }
      Block::ThematicBreak => "<hr />".to_string(),
    }
  }

  pub fn vec_as_html(blocks: &Vec<Self>, loose_mode: bool) -> String {
    let mut html: Vec<String> = vec![];
    for block in blocks {
      html.push(block.as_html(loose_mode))
    }
    html.join("\n")
  }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum ListType {
  Number(usize),
  BracketedNumber(usize),
  Dash,
  Star,
  Plus,
}
