#[derive(Debug, PartialEq)]
pub(crate) enum Inline {
  Text(String),
}

impl Inline {
  pub fn as_html(&self) -> String {
    match self {
      Inline::Text(text) => text.clone(),
    }
  }

  pub fn vec_as_html(inlines: &Vec<Self>) -> String {
    let mut html: Vec<String> = vec![];
    for inline in inlines {
      html.push(inline.as_html())
    }
    html.join("")
  }
}
