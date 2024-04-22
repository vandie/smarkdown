use std::usize;

use crate::{parse::tokens_to_lines, tokeniser::Token};

use super::{
  document::DocContext,
  helpers::trim_empty_lines,
  inlines::Inline,
  list::{parse_line_items, ListType},
  parse_inlines, parse_tokens_with_context,
};

/// Types of block that appear in the stack
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum BlockType {
  Paragraph,
  BlockQuote,
  List(ListType),
  /// An item in a list
  LineItem,
  /// An `<hr>` tag
  ThematicBreak,
  /// A header value from `h1`-`h6`
  Header(u8),
  /// Setext Header is a special case as it becomes a regular header once generated
  SetextHeader(u8),
  IndentedCodeBlock,
}

impl BlockType {
  /// Can this block have no content?
  pub fn allow_no_content(&self) -> bool {
    match self {
      Self::Header(_) => true,
      Self::SetextHeader(_) => true, // In block creation we'll switch this into a ThematicBreak
      _ => false,
    }
  }

  /// Can this block take over the passed block if after it in the stack
  pub fn allow_takeover(&self, block_type: BlockType) -> bool {
    match self {
      Self::SetextHeader(_) => block_type == BlockType::Paragraph,
      _ => false,
    }
  }
}

/// A renderable Block of content
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
  Header(u8, Vec<Inline>),
  IndentedCodeBlock(String),
}

impl Block {
  /// Creates a new renderable block after calling the correct parse method on inner tokens for that block
  pub fn new(block_type: BlockType, inner: Vec<Token>, context: &mut DocContext) -> Block {
    match block_type {
      BlockType::Paragraph => Block::Paragraph(parse_inlines(&inner, context)),
      BlockType::BlockQuote => Block::BlockQuote(parse_tokens_with_context(&inner, context)),
      BlockType::List(list_type) => {
        let inner_blocks = parse_line_items(&inner, context, list_type);
        let lines = tokens_to_lines(&inner);
        let loose = lines
          .iter()
          .position(|line| line.is_empty())
          .is_some_and(|pos| pos + 1 != lines.len()); // there is an edge case where the last line is accidentally a new line, this is not a loose paragraph
        println!("line: \"{inner:?}\" is looks - {loose}");
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
      BlockType::Header(level) => Block::Header(level, parse_inlines(&inner, context)),
      BlockType::SetextHeader(level) => {
        // Due to weirdness with parsing where until it hit the Setext Line, this was a paragraph,
        // there is a chance that a new line token made it through. It should be nipped in the bud here
        let mut true_inner = inner.clone();
        if true_inner.last() == Some(&Token::NewLine) {
          true_inner.pop();
        }

        // This is a catch for when a thematic break is wrongly parsed as a setext header
        if true_inner.len() == 0 {
          return Block::new(BlockType::ThematicBreak, true_inner, context);
        }
        Block::Header(level, parse_inlines(&true_inner, context))
      }
      BlockType::IndentedCodeBlock => Block::IndentedCodeBlock(
        trim_empty_lines(inner)
          .iter()
          .map(|i| Into::<String>::into(i.clone()))
          .collect::<Vec<String>>()
          .join(""),
      ),
    }
  }

  /// Convert a Renderable Block to HTML
  ///
  /// `loose_mode` should almost always be `true` as it determins if `<p>` tags
  /// should be rendered or just spat out as plain text (as is required in lists at times)
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
        let mut blocks = Block::vec_as_html(inner, loose_mode);
        if loose_mode == false && matches!(inner[0], Block::Paragraph(..)) {
          if matches!(inner.last(), Some(Block::List { .. })) {
            blocks += "\n";
          }
          return format!("<li>{blocks}</li>");
        }
        return format!("<li>\n{blocks}\n</li>");
      }
      Block::ThematicBreak => "<hr />".to_string(),
      Block::Header(level, inner) => {
        format!("<h{level}>{}</h{level}>", Inline::vec_as_html(inner))
      }
      Block::IndentedCodeBlock(inner) => format!("<pre><code>{inner}\n</code></pre>"),
    }
  }

  /// Converts an array of Renderable Blocks into a single HTML String
  pub fn vec_as_html(blocks: &Vec<Self>, loose_mode: bool) -> String {
    let mut html: Vec<String> = vec![];
    for block in blocks {
      html.push(block.as_html(loose_mode))
    }
    html.join("\n")
  }
}
