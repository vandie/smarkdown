use smarkdown::parse;

/// Blockquote Example:
#[test]
fn example_228() {
  let example_string = "> # Foo\n> bar\n> baz";
  let expected_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The space or tab after the `>` characters can be omitted:
#[test]
fn example_229() {
  let example_string = "># Foo\n>bar\n> baz";
  let expected_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The `>` characters can be preceded by up to three spaces of indentation:
#[test]
fn example_230() {
  let example_string = "   > # Foo\n   > bar\n > baz";
  let expected_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Four spaces of indentation is too many:
#[test]
fn example_231() {
  let example_string = "    > # Foo\n    > bar\n    > baz";
  let expected_html = "<pre><code>&gt; # Foo\n&gt; bar\n&gt; baz\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The Laziness clause allows us to omit the `>` before [paragraph continuation text](https://spec.commonmark.org/0.31.2/#paragraph-continuation-text):
#[test]
fn example_232() {
  let example_string = "> # Foo\n> bar\nbaz";
  let expected_html = "<blockquote>\n<h1>Foo</h1>\n<p>bar\nbaz</p>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// A block quote can contain some lazy and some non-lazy continuation lines:
#[test]
fn example_233() {
  let example_string = "> bar\nbaz\n> foo";
  let expected_html = "<blockquote>\n<p>bar\nbaz\nfoo</p>\n</blockquote>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Laziness only applies to lines that would have been continuations of paragraphs had they been prepended with block quote markers.
/// For example, the `>` cannot be omitted in the second line of
/// ```markdown
/// > foo
/// > ---
/// ```
/// without changing the meaning:
#[test]
fn example_234() {
  let example_string = "> foo\n---";
  let expected_html = "<blockquote>\n<p>foo</p>\n</blockquote>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Similarly, if we omit the `>` in the second line of
/// ```
/// > - foo
/// > - bar
/// ```
///then the block quote ends after the first line:
#[test]
fn example_235() {
  let example_string = "> - foo\n- bar";
  let expected_html =
    "<blockquote>\n<ul>\n<li>foo</li>\n</ul>\n</blockquote>\n<ul>\n<li>bar</li>\n</ul>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// For the same reason, we can’t omit the `>` in front of subsequent lines of an indented or fenced code block:
#[test]
fn example_236() {
  let example_string = ">     foo\nbar";
  let expected_html =
    "<blockquote>\n<pre><code>foo\n</code></pre>\n</blockquote>\n<pre><code>bar\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}
