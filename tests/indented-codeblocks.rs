use smarkdown::parse;

/// Simple Indented Codeblock
#[test]
fn example_107() {
  let example_string = "    a simple\n      indented code block";
  let expected_html = "<pre><code>a simple\n  indented code block\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// If there is any ambiguity between an interpretation of indentation as a code block and as indicating that material belongs to a list item, the list item interpretation takes precedence:
#[test]
fn example_108() {
  let example_string = "  - foo\n\n    bar";
  let expected_html = "<ul>\n<li>\n<p>foo</p>\n<p>bar</p>\n</li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// If there is any ambiguity between an interpretation of indentation as a code block and as indicating that material belongs to a list item, the list item interpretation takes precedence:
#[test]
fn example_109() {
  let example_string = "1.  foo\n\n    - bar";
  let expected_html = "<ol>\n<li>\n<p>foo</p>\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The contents of a code block are literal text, and do not get parsed as Markdown:
#[test]
fn example_110() {
  let example_string = "    <a/>\n    *hi*\n\n    - one";
  let expected_html = "<pre><code>&lt;a/&gt;\n*hi*\n\n- one\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Here we have three chunks separated by blank lines:
#[test]
fn example_111() {
  let example_string = "    chunk1\n\n    chunk2\n  \n  \n\n    chunk3";
  let expected_html = "<pre><code>chunk1\n\nchunk2\n\n\n\nchunk3\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Any initial spaces or tabs beyond four spaces of indentation will be included in the content, even in interior blank lines:
#[test]
fn example_112() {
  let example_string = "    chunk1\n      \n      chunk2";
  let expected_html = "<pre><code>chunk1\n  \n  chunk2\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// An indented code block cannot interrupt a paragraph. (This allows hanging indents and the like.)
#[test]
fn example_113() {
  let example_string = "Foo\n    bar";
  let expected_html = "<p>Foo\nbar</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// However, any non-blank line with fewer than four spaces of indentation ends the code block immediately. So a paragraph may occur immediately after indented code:
#[test]
fn example_114() {
  let example_string = "    foo\nbar";
  let expected_html = "<pre><code>foo\n</code></pre>\n<p>bar</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// And indented code can occur immediately before and after other kinds of blocks:
#[test]
fn example_115() {
  let example_string = "# Heading\n    foo\nHeading\n------\n    foo\n----";
  let expected_html = "<h1>Heading</h1>\n<pre><code>foo\n</code></pre>\n<h2>Heading</h2>\n<pre><code>foo\n</code></pre>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The first line can be preceded by more than four spaces of indentation:
#[test]
fn example_116() {
  let example_string = "        foo\n    bar";
  let expected_html = "<pre><code>    foo\nbar\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Blank lines preceding or following an indented code block are not included in it:
#[test]
fn example_117() {
  let example_string = "\n    \n    foo\n    ";
  let expected_html = "<pre><code>foo\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Trailing spaces or tabs are included in the code block’s content:
#[test]
fn example_118() {
  let example_string = "    foo  ";
  let expected_html = "<pre><code>foo  \n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}
