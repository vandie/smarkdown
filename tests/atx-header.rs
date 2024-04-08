use smarkdown::parse;

/// Simple headings:
#[test]
fn example_62() {
  let example_string = "# foo\n## foo\n### foo\n#### foo\n##### foo\n###### foo";
  let expected_html =
    "<h1>foo</h1>\n<h2>foo</h2>\n<h3>foo</h3>\n<h4>foo</h4>\n<h5>foo</h5>\n<h6>foo</h6>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// More than six `#` characters is not a heading:
#[test]
fn example_63() {
  let example_string = "####### foo";
  let expected_html = "<p>####### foo</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// At least one space or tab is required between the # characters and the heading’s contents, unless the heading is empty.
/// Note that many implementations currently do not require the space.
/// However, the space was required by the original ATX implementation, and it helps prevent things like the following from being parsed as headings:
#[test]
fn example_64() {
  let example_string = "#5 bolt\n\n#hashtag";
  let expected_html = "<p>#5 bolt</p>\n<p>#hashtag</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// This is not a heading, because the first # is escaped:
#[test]
fn example_65() {
  let example_string = "\\## foo";
  let expected_html = "<p>## foo</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Contents are parsed as inlines:
#[test]
fn example_66() {
  let example_string = "# foo *bar* \\*baz\\*";
  let expected_html = "<h1>foo <em>bar</em> *baz*</h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Leading and trailing spaces or tabs are ignored in parsing inline content:
#[test]
fn example_67() {
  let example_string = "#                  foo                     ";
  let expected_html = "<h1>foo</h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Up to three spaces of indentation are allowed:
#[test]
fn example_68() {
  let example_string = " ### foo\n  ## foo\n   # foo";
  let expected_html = "<h3>foo</h3>\n<h2>foo</h2>\n<h1>foo</h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Four spaces of indentation is too many:
#[test]
fn example_69() {
  let example_string = "    # foo";
  let expected_html = "<pre><code># foo\n</code></pre>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Four spaces of indentation is too many:
#[test]
fn example_70() {
  let example_string = "foo\n    # bar";
  let expected_html = "<p>foo\n# bar</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// A closing sequence of # characters is optional:
#[test]
fn example_71() {
  let example_string = "## foo ##\n  ###   bar    ###";
  let expected_html = "<h2>foo</h2>\n<h3>bar</h3>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// It need not be the same length as the opening sequence:
#[test]
fn example_72() {
  let example_string = "# foo ##################################\n##### foo ##";
  let expected_html = "<h1>foo</h1>\n<h5>foo</h5>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Spaces or tabs are allowed after the closing sequence:
#[test]
fn example_73() {
  let example_string = "### foo ###     ";
  let expected_html = "<h3>foo</h3>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// A sequence of `#` characters with anything but spaces or tabs following it is not a closing sequence, but counts as part of the contents of the heading:
#[test]
fn example_74() {
  let example_string = "### foo ### b";
  let expected_html = "<h3>foo ### b</h3>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The closing sequence must be preceded by a space or tab:
#[test]
fn example_75() {
  let example_string = "# foo#";
  let expected_html = "<h1>foo#</h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// The closing sequence must be preceded by a space or tab:
#[test]
fn example_76() {
  let example_string = "### foo \\###\n## foo #\\##\n# foo \\#";
  let expected_html = "<h3>foo ###</h3>\n<h2>foo ###</h2>\n<h1>foo #</h1>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// Backslash-escaped `#` characters do not count as part of the closing sequence:
#[test]
fn example_77() {
  let example_string = "****\n## foo\n****";
  let expected_html = "<hr />\n<h2>foo</h2>\n<hr />";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// ATX headings need not be separated from surrounding content by blank lines, and they can interrupt paragraphs:
#[test]
fn example_78() {
  let example_string = "Foo bar\n# baz\nBar foo";
  let expected_html = "<p>Foo bar</p>\n<h1>baz</h1>\n<p>Bar foo</p>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}

/// ATX headings can be empty:
#[test]
fn example_79() {
  let example_string = "## \n#\n### ###";
  let expected_html = "<h2></h2>\n<h1></h1>\n<h3></h3>";
  assert_eq!(parse(example_string).as_html(), expected_html);
}
