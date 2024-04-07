use super::{blocks::Block, inlines::Inline};

pub struct DocContext {}
pub struct Document {
  blocks: Vec<Block>,
  pub context: DocContext,
}

impl Document {
  pub fn new() -> Self {
    Self {
      blocks: vec![],
      context: DocContext {},
    }
  }

  pub fn as_html(&self) -> String {
    println!("final: {:?}", self.blocks);
    let mut html: Vec<String> = vec![];
    for block in self.blocks.iter() {
      html.push(block.as_html(true))
    }
    html.join("\n")
  }

  pub(crate) fn add_block(&mut self, block: Block) {
    self.blocks.push(block);
  }
}
