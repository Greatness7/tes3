// rust std imports
use std::marker::PhantomData;

// external imports
use slotmap::{Key, KeyData};

// internal imports
use crate::prelude::*;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct NiLink<T> {
    pub key: NiKey,
    phantom: PhantomData<fn() -> T>,
}

impl<T> NiLink<T> {
    #[inline]
    pub const fn new(key: NiKey) -> Self {
        Self {
            key,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn null() -> Self {
        Self::new(NiKey::null())
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.key.is_null()
    }

    #[inline]
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
            i if (i < 0) => NiKey::null(),
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
            stream.save_as::<i32>(stream.context[&key])?;
        }
        Ok(())
    }
}

//
// Visitor
//

pub trait Visitor {
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(NiKey);
}

impl<T> Visitor for &T {
    #[inline]
    fn visitor<F>(&self, _: &mut F)
    where
        F: FnMut(NiKey),
    {
    }
}

impl<V: Visitor> Visitor for Option<V> {
    #[inline]
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(NiKey),
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
        F: FnMut(NiKey),
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
        F: FnMut(NiKey),
    {
        f(self.key);
    }
}

impl Visitor for TextureMap {
    #[inline]
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(NiKey),
    {
        match self {
            TextureMap::Map(inner) => inner.visitor(f),
            TextureMap::BumpMap(inner) => inner.visitor(f),
        }
    }
}

impl Visitor for TextureSource {
    #[inline]
    fn visitor<F>(&self, f: &mut F)
    where
        F: FnMut(NiKey),
    {
        match self {
            TextureSource::External(inner) => inner.visitor(f),
            TextureSource::Internal(inner) => inner.visitor(f),
        }
    }
}
