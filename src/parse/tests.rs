use crate::{
  parse::{self, parse, tokens_to_lines},
  tokeniser::Token,
};

use super::line::Line;

#[test]
fn line_test() {
  let input = vec![
    Token::Text("Title".to_string()),
    Token::NewLine,
    Token::Text("some paragraph content".to_string()),
    Token::NewLine,
    Token::NewLine,
    Token::Text("some paragraph content 2".to_string()),
  ];
  let expected_lines = vec![
    Line(vec![Token::Text("Title".to_string())]),
    Line(vec![Token::Text("some paragraph content".to_string())]),
    Line(vec![]),
    Line(vec![Token::Text("some paragraph content 2".to_string())]),
  ];
  assert_eq!(tokens_to_lines(&input), expected_lines);
}

#[test]
fn indentation_test() {
  let mut lines = vec![
    Line(vec![]),
    Line(vec![Token::Space, Token::Text("Title".to_string())]),
    Line(vec![
      Token::Space,
      Token::Space,
      Token::Space,
      Token::Space,
      Token::Text("some paragraph content".to_string()),
    ]),
    Line(vec![
      Token::Tab,
      Token::Text("some paragraph content".to_string()),
    ]),
    Line(vec![
      Token::Tab,
      Token::Space,
      Token::Text("some paragraph content".to_string()),
    ]),
    Line(vec![
      Token::Tab,
      Token::Tab,
      Token::Text("some paragraph content".to_string()),
    ]),
  ];
  let expected_indents: Vec<(Vec<Token>, usize)> = vec![
    (vec![], 0),
    (vec![Token::Text("Title".to_string())], 0),
    (vec![Token::Text("some paragraph content".to_string())], 1),
    (vec![Token::Text("some paragraph content".to_string())], 1),
    (vec![Token::Text("some paragraph content".to_string())], 1),
    (vec![Token::Text("some paragraph content".to_string())], 2),
  ];
  for i in 0..lines.len() {
    assert_eq!(lines[i].remove_all_indentation(), expected_indents[i].1);
    assert_eq!(lines[i].0, expected_indents[i].0);
  }
}

#[test]
fn remove_blank_end() {
  let mut line = Line(vec![
    Token::Text("Hey".to_string()),
    Token::Space,
    Token::Space,
  ]);
  line.remove_ending_blanks();
  assert_eq!(line, Line(vec![Token::Text("Hey".to_string())]));
}

#[test]
fn remove_blank_start() {
  let mut line = Line(vec![
    Token::Space,
    Token::Space,
    Token::Text("Hey".to_string()),
  ]);
  line.trim_line_start(2);
  assert_eq!(line, Line(vec![Token::Text("Hey".to_string())]));
}

#[test]
fn paragraph_parse() {
  let example_string = "this is an example with two paragraphs.\nthis is part of the first paragraph still.\n\nWelcome to paragraph 2.";
  let expected_html = "<p>this is an example with two paragraphs.\nthis is part of the first paragraph still.</p>\n<p>Welcome to paragraph 2.</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn blockquote_basic() {
  let example_string = "> This is the text\n\nWelcome to a paragraph.";
  let expected_html =
    "<blockquote>\n<p>This is the text</p>\n</blockquote>\n<p>Welcome to a paragraph.</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn list_basic() {
  let example_string = "- item 1\n- item 2\n\t- sub item 1";
  let expected_html =
    "<ul>\n<li>item 1</li>\n<li>item 2\n<ul>\n<li>sub item 1</li>\n</ul></li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn loose_list() {
  let example_string = "- item 1\n\n- item 2\n\t- sub item 1";
  let expected_html =
    "<ul>\n<li>\n<p>item 1</p>\n</li>\n<li>\n<p>item 2</p>\n<ul>\n<li>sub item 1</li>\n</ul>\n</li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn single_level_list() {
  let example_string = "- item 1\n\n- item 2";
  let expected_html = "<ul>\n<li>\n<p>item 1</p>\n</li>\n<li>\n<p>item 2</p>\n</li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn num_list() {
  let example_string = "1. item 1\n2. item 2";
  let expected_html = "<ol>\n<li>item 1</li>\n<li>item 2</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn num_list_none_1() {
  let example_string = "3. item 1\n4. item 2";
  let expected_html = "<ol start=\"3\">\n<li>item 1</li>\n<li>item 2</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn thematic_break_basic() {
  let example_string = "this is an example\n***\nand so it was";
  let expected_html = "<p>this is an example</p>\n<hr />\n<p>and so it was</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn header_basic() {
  let example_string = "## Header 2\nthis is an example";
  let expected_html = "<h2>Header 2</h2>\n<p>this is an example</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}
