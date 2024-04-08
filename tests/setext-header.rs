use smarkdown::parse;

/// Simple examples:
#[test]
fn example_80() {
  let example_string = "Foo *bar*\n=========\n\nFoo *bar*\n---------";
  let expected_html = "<h1>Foo <em>bar</em></h1>\n<h2>Foo <em>bar</em></h2>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The content of the header may span more than one line:
#[test]
fn example_81() {
  let example_string = "Foo *bar\nbaz*\n====";
  let expected_html = "<h1>Foo <em>bar\nbaz</em></h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The contents are the result of parsing the headings’s raw content as inlines.
/// The heading’s raw content is formed by concatenating the lines and removing initial and final spaces or tabs.
#[test]
fn example_82() {
  let example_string = "  Foo *bar\nbaz*→\n====";
  let expected_html = "<h1>Foo <em>bar\nbaz</em></h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The underlining can be any length:
#[test]
fn example_83() {
  let example_string = "Foo\n-------------------------\n\nFoo\n=";
  let expected_html = "<h2>Foo</h2>\n<h1>Foo</h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The heading content can be preceded by up to three spaces of indentation, and need not line up with the underlining:
#[test]
fn example_84() {
  let example_string = "   Foo\n---\n\n  Foo\n-----\n\n\n  Foo\n  ===";
  let expected_html = "<h2>Foo</h2>\n<h2>Foo</h2>\n<h1>Foo</h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Four spaces of indentation is too many:
#[test]
fn example_85() {
  let example_string = "    Foo\n    ---\n\n    Foo\n---";
  let expected_html = "<pre><code>Foo\n---\n\nFoo\n</code></pre>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The setext heading underline can be preceded by up to three spaces of indentation, and may have trailing spaces or tabs:
#[test]
fn example_86() {
  let example_string = "Foo\n   ----      ";
  let expected_html = "<h2>Foo</h2>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Four spaces of indentation is too many:
#[test]
fn example_87() {
  let example_string = "Foo\n    ---";
  let expected_html = "<p>Foo\n---</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The setext heading underline cannot contain internal spaces or tabs:
#[test]
fn example_88() {
  let example_string = "Foo\n= =\n\nFoo\n--- -";
  let expected_html = "<p>Foo\n= =</p>\n<p>Foo</p>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Trailing spaces or tabs in the content line do not cause a hard line break:
/// TODO: Fix handling of trimming with takeovers
#[test]
fn example_89() {
  let example_string = "Foo  \n-----";
  let expected_html = "<h2>Foo</h2>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Nor does a backslash at the end:
#[test]
fn example_90() {
  let example_string = "Foo\\\n----";
  let expected_html = "<h2>Foo\\</h2>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Since indicators of block structure take precedence over indicators of inline structure, the following are setext headings:
/// TODO: Fix handling of html &char syntax
#[test]
fn example_91() {
  let example_string = "`Foo\n----\n`\n\n<a title=\"a lot\n---\nof dashes\"/>";
  let expected_html =
    "<h2>`Foo</h2>\n<p>`</p>\n<h2>&lt;a title=&quot;a lot</h2>\n<p>of dashes&quot;/&gt;</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The setext heading underline cannot be a lazy continuation line in a list item or block quote:
#[test]
fn example_92() {
  let example_string = "> Foo\n---";
  let expected_html = "<blockquote>\n<p>Foo</p>\n</blockquote>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The setext heading underline cannot be a lazy continuation line in a list item or block quote:
#[test]
fn example_93() {
  let example_string = "> foo\nbar\n===";
  let expected_html = "<blockquote>\n<p>foo\nbar\n===</p>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The setext heading underline cannot be a lazy continuation line in a list item or block quote:
#[test]
fn example_94() {
  let example_string = "- Foo\n---";
  let expected_html = "<ul>\n<li>Foo</li>\n</ul>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// A blank line is needed between a paragraph and a following setext heading, since otherwise the paragraph becomes part of the heading’s content:
#[test]
fn example_95() {
  let example_string = "Foo\nBar\n---";
  let expected_html = "<h2>Foo\nBar</h2>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// A blank line is needed between a paragraph and a following setext heading, since otherwise the paragraph becomes part of the heading’s content:
#[test]
fn example_96() {
  let example_string = "---\nFoo\n---\nBar\n---\nBaz";
  let expected_html = "<hr />\n<h2>Foo</h2>\n<h2>Bar</h2>\n<p>Baz</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Setext headings cannot be empty:
#[test]
fn example_97() {
  let example_string = "\n====";
  let expected_html = "<p>====</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Setext heading text lines must not be interpretable as block constructs other than paragraphs.
/// So, the line of dashes in these examples gets interpreted as a thematic break:
#[test]
fn example_98() {
  let example_string = "---\n---";
  let expected_html = "<hr />\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Setext heading text lines must not be interpretable as block constructs other than paragraphs.
/// So, the line of dashes in these examples gets interpreted as a thematic break:
#[test]
fn example_99() {
  let example_string = "- foo\n-----";
  let expected_html = "<ul>\n<li>foo</li>\n</ul>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Setext heading text lines must not be interpretable as block constructs other than paragraphs.
/// So, the line of dashes in these examples gets interpreted as a thematic break:
#[test]
fn example_100() {
  let example_string = "    foo\n---";
  let expected_html = "<pre><code>foo\n</code></pre>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Setext heading text lines must not be interpretable as block constructs other than paragraphs.
/// So, the line of dashes in these examples gets interpreted as a thematic break:
#[test]
fn example_101() {
  let example_string = "> foo\n-----";
  let expected_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// If you want a heading with `> foo` as its literal text, you can use backslash escapes:
#[test]
fn example_102() {
  let example_string = "\\> foo\n------";
  let expected_html = "<h2>&gt; foo</h2>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

// The following tests are all related to setext headings spaning multiple lines. Confirming
// that we follow CommonMark as outside of CommonMark there is no consensus about how to interpret them.

#[test]
fn example_103() {
  let example_string = "Foo\n\nbar\n---\nbaz";
  let expected_html = "<p>Foo</p>\n<h2>bar</h2>\n<p>baz</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_104() {
  let example_string = "Foo\nbar\n\n---\n\nbaz";
  let expected_html = "<p>Foo\nbar</p>\n<hr />\n<p>baz</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_105() {
  let example_string = "Foo\nbar\n* * *\nbaz";
  let expected_html = "<p>Foo\nbar</p>\n<hr />\n<p>baz</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_106() {
  let example_string = "Foo\nbar\n\\---\nbaz";
  let expected_html = "<p>Foo\nbar\n---\nbaz</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}
