// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiNode {
    pub base: NiAVObject,
    pub children: Vec<NiLink<NiAVObject>>,
    pub effects: Vec<NiLink<NiDynamicEffect>>,
}

impl Load for NiNode {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let children = stream.load()?;
        let effects = stream.load()?;
        Ok(Self { base, children, effects })
    }
}

impl Save for NiNode {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.children)?;
        stream.save(&self.effects)?;
        Ok(())
    }
}

impl NiNode {
    pub fn children_recursive<'a>(&'a self, stream: &'a NiStream) -> impl 'a + Iterator<Item = NiLink<NiAVObject>> {
        let mut stack = self.children.clone();
        stack.reverse();

        std::iter::from_fn(move || {
            while let Some(link) = stack.pop() {
                if !link.is_null() {
                    if let Some(node) = stream.get_as::<_, NiNode>(link) {
                        stack.extend(node.children.iter().rev().copied());
                    }
                    return Some(link);
                }
            }
            None
        })
    }
}
