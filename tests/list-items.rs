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
