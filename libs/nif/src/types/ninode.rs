// rust std imports
use std::collections::VecDeque;

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
        let mut stack: VecDeque<_> = self.children.iter().copied().collect();
        std::iter::from_fn(move || {
            while let Some(link) = stack.pop_front() {
                if !link.is_null() {
                    if let Some(node) = stream.get_as::<_, NiNode>(link) {
                        for child in &node.children {
                            stack.push_front(*child);
                        }
                    }
                    return Some(link);
                }
            }
            None
        })
    }

    pub fn children_of_type<'a, T>(&'a self, stream: &'a NiStream) -> impl 'a + Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.children.iter().filter_map(move |child| stream.get_as::<_, T>(*child))
    }

    pub fn children_of_type_recursive<'a, T>(&'a self, stream: &'a NiStream) -> impl 'a + Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.children_recursive(stream)
            .filter_map(move |child| stream.get_as::<_, T>(child))
    }
}
