use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

use super::*;

use tynm::type_name;
use yoke::{erased::ErasedRcCart, Yoke, Yokeable};

pub struct Ref<T>(pub Yoke<T, ErasedRcCart>)
where
    T: for<'a> Yokeable<'a>;

impl<T> Ref<T>
where
    T: for<'a> Yokeable<'a>,
{
    pub fn get(&self) -> &<T as Yokeable<'_>>::Output {
        self.0.get()
    }
}

impl<T> Ref<&'static T> {
    pub fn type_name() -> String {
        type_name::<T>()
    }
}

impl<T> Debug for Ref<T>
where
    T: for<'a> Yokeable<'a, Output: Debug>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<T, U> Deref for Ref<T>
where
    T: for<'a> Yokeable<'a, Output: Deref<Target = U>>,
{
    type Target = U;

    fn deref(&self) -> &Self::Target {
        self.0.get().deref()
    }
}

impl<T, U> AsRef<U> for Ref<T>
where
    T: for<'a> Yokeable<'a, Output: AsRef<U>>,
{
    fn as_ref(&self) -> &U {
        self.get().as_ref()
    }
}

impl<T> Ref<T>
where
    T: for<'a> Yokeable<'a, Output: Clone>,
{
    pub fn map<U>(&self, f: impl MapFn<T, U>) -> Ref<U>
    where
        U: for<'a> Yokeable<'a>,
    {
        Ref(self.0.clone().map_project(|t, p| f(t, p)))
    }

    pub fn clone(t: &Self) -> Self {
        Ref(t.0.clone())
    }
}

pub trait MapFn<T, U>
where
    T: for<'a> Yokeable<'a>,
    U: for<'a> Yokeable<'a>,
    Self: for<'a> FnOnce(<T as Yokeable<'a>>::Output, PhantomData<&'a ()>) -> <U as Yokeable<'a>>::Output,
{
}

impl<T, U, F> MapFn<T, U> for F
where
    T: for<'a> Yokeable<'a>,
    U: for<'a> Yokeable<'a>,
    F: for<'a> FnOnce(<T as Yokeable<'a>>::Output, PhantomData<&'a ()>) -> <U as Yokeable<'a>>::Output,
{
}

pub trait IntoLuaRef: Sized {
    fn into_lua_ref(self) -> Ref<&'static Self> {
        let cart = Rc::new(self);
        let yoke = Yoke::<&_, _>::attach_to_cart(cart, |x| x);
        Ref(yoke.erase_rc_cart())
    }
}

impl<T> IntoLuaRef for T {}
