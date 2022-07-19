// rust std imports
use std::marker::PhantomData;

// external imports
use slotmap::{DefaultKey, Key, KeyData};

// internal imports
use crate::prelude::*;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct NiLink<T> {
    pub key: DefaultKey,
    phantom: PhantomData<*const T>,
}

impl<T> NiLink<T> {
    pub const fn new(key: DefaultKey) -> Self {
        Self {
            key,
            phantom: PhantomData,
        }
    }

    pub fn null() -> Self {
        Self::new(DefaultKey::null())
    }

    pub fn is_null(&self) -> bool {
        self.key.is_null()
    }

    pub const fn cast<U>(&self) -> NiLink<U> {
        NiLink::new(self.key)
    }
}

impl<T> Clone for NiLink<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for NiLink<T> {}

impl<T> Load for NiLink<T>
where
    T: Load,
{
    #[allow(clippy::cast_sign_loss)]
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let idx: i32 = stream.load()?;
        let key = match idx {
            i if (i < 0) => DefaultKey::null(),
            i => KeyData::from_ffi((1 << 32) | (i as u64 + 1)).into(),
        };
        Ok(Self::new(key))
    }
}

impl<T> Save for NiLink<T>
where
    T: Save,
{
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        if self.is_null() {
            stream.save(&-1i32)?;
        } else {
            let key = self.key.data().as_ffi();
            stream.save_as::<_, i32>(stream.context[&key])?;
        };
        Ok(())
    }
}

//
// Visitor
//

pub trait Visitor {
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(DefaultKey);
}

impl<T> Visitor for &T {
    #[inline]
    fn visitor<F>(&self, _: &mut F)
    where
        F: FnMut(DefaultKey),
    {
    }
}

impl<V: Visitor> Visitor for Option<V> {
    #[inline]
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(DefaultKey),
    {
        if let Some(inner) = self {
            inner.visitor(f);
        }
    }
}

impl<V: Visitor> Visitor for Vec<V> {
    #[inline]
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(DefaultKey),
    {
        for item in self.iter().rev() {
            item.visitor(f);
        }
    }
}

impl<T> Visitor for NiLink<T> {
    #[inline]
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(DefaultKey),
    {
        f(self.key);
    }
}

impl Visitor for TextureMap {
    #[inline]
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(DefaultKey),
    {
        match self {
            TextureMap::Map(inner) => inner.visitor(f),
            TextureMap::BumpMap(inner) => inner.visitor(f),
        }
    }
}

impl Visitor for Source {
    #[inline]
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(DefaultKey),
    {
        match self {
            Source::External(inner) => inner.visitor(f),
            Source::Internal(inner) => inner.visitor(f),
        }
    }
}
