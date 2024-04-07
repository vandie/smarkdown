use smarkdown::parse;

/// A simple example with two paragraphs:
#[test]
fn example_219() {
  let example_string = "aaa\n\nbbb";
  let expected_html = "<p>aaa</p>\n<p>bbb</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Paragraphs can contain multiple lines, but no blank lines:
#[test]
fn example_220() {
  let example_string = "aaa\nbbb\n\nccc\nddd";
  let expected_html = "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Multiple blank lines between paragraphs have no effect:
#[test]
fn example_221() {
  let example_string = "aaa\n\n\nbbb";
  let expected_html = "<p>aaa</p>\n<p>bbb</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Leading spaces or tabs are skipped:
#[test]
fn example_222() {
  let example_string = "  aaa\n bbb";
  let expected_html = "<p>aaa\nbbb</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Lines after the first may be indented any amount, since indented code blocks cannot interrupt paragraphs.
#[test]
fn example_223() {
  let example_string = "aaa\n             bbb\n                                       ccc";
  let expected_html = "<p>aaa\nbbb\nccc</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The first line may be preceded by up to three spaces of indentation. Four spaces of indentation is too many:
#[test]
fn example_224() {
  let example_string = "   aaa\nbbb";
  let expected_html = "<p>aaa\nbbb</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The first line may be preceded by up to three spaces of indentation. Four spaces of indentation is too many:
#[test]
fn example_225() {
  let example_string = "    aaa\nbbb";
  let expected_html = "<pre><code>aaa\n</code></pre>\n<p>bbb</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Final spaces or tabs are stripped before inline parsing, so a paragraph that ends with two or more spaces will not end with a [hard line break](https://spec.commonmark.org/0.31.2/#hard-line-break):
#[test]
fn example_226() {
  let example_string = "aaa     \nbbb     ";
  let expected_html = "<p>aaa<br />\nbbb</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}
