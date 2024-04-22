use smarkdown::parse;

#[test]
fn example_253() {
  let example_string = "A paragraph\nwith two lines.\n\n    indented code\n\n> A block quote.";
  let expected_html = "<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_254() {
  let example_string =
    "1.  A paragraph\n    with two lines.\n\n        indented code\n\n\n    > A block quote.";
  let expected_html = "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_255() {
  let example_string = "- one\n\n two";
  let expected_html = "<ul>\n<li>one</li>\n</ul>\n<p>two</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_256() {
  let example_string = "- one\n\n  two";
  let expected_html = "<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_257() {
  let example_string = " -    one\n\n     two";
  let expected_html = "<ul>\n<li>one</li>\n</ul>\n<pre><code> two\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_258() {
  let example_string = " -    one\n\n      two";
  let expected_html = "<ul>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_259() {
  let example_string = "   > > 1.  one\n>>\n>>     two";
  let expected_html = "<blockquote>\n<blockquote>\n<ol>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ol>\n</blockquote>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_260() {
  let example_string = ">>- one\n>>\n  >  > two";
  let expected_html = "<blockquote>\n<blockquote>\n<ul>\n<li>one</li>\n</ul>\n<p>two</p>\n</blockquote>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_261() {
  let example_string = " 1.  A paragraph\n    with two lines.";
  let expected_html = "<ol>\n<li>A paragraph\nwith two lines.</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_262() {
  let example_string = "> 1. > Blockquote\ncontinued here.";
  let expected_html = "<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_263() {
  let example_string = "> 1. > Blockquote\n> continued here.";
  let expected_html = "<blockquote>\n<ol>\n<li>\n<blockquote>\n<p>Blockquote\ncontinued here.</p>\n</blockquote>\n</li>\n</ol>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_264() {
  let example_string = "- foo\n  - bar\n    - baz\n      - boo";
  let expected_html = "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz\n<ul>\n<li>boo</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_265() {
  let example_string = "123456789. ok";
  let expected_html = "<ol start=\"123456789\">\n<li>ok</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_266() {
  let example_string = "1234567890. not ok";
  let expected_html = "<p>1234567890. not ok</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_267() {
  let example_string = "0. ok";
  let expected_html = "<ol start=\"0\">\n<li>ok</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_268() {
  let example_string = "003. ok";
  let expected_html = "<ol start=\"3\">\n<li>ok</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_269() {
  let example_string = "-1. not ok";
  let expected_html = "<p>-1. not ok</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_270() {
  let example_string = "- foo\n\n      bar";
  let expected_html = "<ul>\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

#[test]
fn example_271() {
  let example_string = "  10.  foo\n\n           bar";
  let expected_html =
    "<ol start=\"10\">\n<li>\n<p>foo</p>\n<pre><code>bar\n</code></pre>\n</li>\n</ol>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}
