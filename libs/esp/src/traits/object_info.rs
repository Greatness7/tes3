use crate::prelude::*;

pub trait ObjectInfo {
    fn object_flags(&self) -> &ObjectFlags;
    fn object_flags_mut(&mut self) -> &mut ObjectFlags;

    #[inline(always)]
    fn modified(&self) -> bool {
        self.object_flags().contains(ObjectFlags::MODIFIED)
    }

    #[inline(always)]
    fn set_modified(&mut self, state: bool) {
        self.object_flags_mut().set(ObjectFlags::MODIFIED, state);
    }

    #[inline(always)]
    fn deleted(&self) -> bool {
        self.object_flags().contains(ObjectFlags::DELETED)
    }

    #[inline(always)]
    fn set_deleted(&mut self, state: bool) {
        self.object_flags_mut().set(ObjectFlags::DELETED, state);
    }

    #[inline(always)]
    fn persistent(&self) -> bool {
        self.object_flags().contains(ObjectFlags::PERSISTENT)
    }

    #[inline(always)]
    fn set_persistent(&mut self, state: bool) {
        self.object_flags_mut().set(ObjectFlags::PERSISTENT, state);
    }

    #[inline(always)]
    fn ignored(&self) -> bool {
        self.object_flags().contains(ObjectFlags::IGNORED)
    }

    #[inline(always)]
    fn set_ignored(&mut self, state: bool) {
        self.object_flags_mut().set(ObjectFlags::IGNORED, state);
    }

    #[inline(always)]
    fn blocked(&self) -> bool {
        self.object_flags().contains(ObjectFlags::BLOCKED)
    }

    #[inline(always)]
    fn set_blocked(&mut self, state: bool) {
        self.object_flags_mut().set(ObjectFlags::BLOCKED, state);
    }
}

delegate! {
    impl ObjectInfo {
        #[inline(always)]
        fn object_flags(&self) -> &ObjectFlags {
            &self.flags
        }
        #[inline(always)]
        fn object_flags_mut(&mut self) -> &mut ObjectFlags {
            &mut self.flags
        }
    }
}

impl ObjectInfo for TES3Object {
    fn object_flags(&self) -> &ObjectFlags {
        delegate! {
            match self {
                inner => inner.object_flags()
            }
        }
    }
    fn object_flags_mut(&mut self) -> &mut ObjectFlags {
        delegate! {
            match self {
                inner => inner.object_flags_mut()
            }
        }
    }
}
