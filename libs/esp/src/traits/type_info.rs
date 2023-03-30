use crate::prelude::*;

pub trait TypeInfo {
    fn tag(&self) -> &'static [u8; 4];
    fn tag_str(&self) -> &'static str;
    fn type_name(&self) -> &'static str;
}

delegate! {
    impl TypeInfo {
        #[inline(always)]
        fn tag(&self) -> &'static [u8; 4] {
            Self::TAG
        }
        #[inline(always)]
        fn tag_str(&self) -> &'static str {
            Self::TAG_STR
        }
        #[inline(always)]
        fn type_name(&self) -> &'static str {
            Self::TYPE_NAME
        }
    }
}

impl TypeInfo for TES3Object {
    fn tag(&self) -> &'static [u8; 4] {
        delegate! {
            match self {
                inner => inner.tag()
            }
        }
    }
    fn tag_str(&self) -> &'static str {
        delegate! {
            match self {
                inner => inner.tag_str()
            }
        }
    }
    fn type_name(&self) -> &'static str {
        delegate! {
            match self {
                inner => inner.tag_str()
            }
        }
    }
}
