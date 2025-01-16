use std::fmt::Debug;

use super::*;

#[macro_export]
macro_rules! impl_meta_method {
    ($methods:ident, "__tostring") => {
        $methods.add_meta_method("__tostring", |_, this, _: ()| {
            Ok(format!("{:#?}", this)) //
        });
    };
    ($methods:ident, "__len") => {
        $methods.add_meta_method("__len", |_, this, _: ()| {
            Ok(this.len()) //
        });
    };
    ($methods:ident, "__index") => {
        $methods.add_meta_method("__index", |lua, this, i: usize| match i.checked_sub(1) {
            Some(i) if i < this.len() => {
                let value = this.map::<&_>(|this, _| &this[i]);
                value.into_lua(lua)
            }
            _ => Ok(Nil),
        });
    };
    ($methods:ident, "__ipairs") => {
        $methods.add_meta_method("__ipairs", |lua, this, _: ()| {
            let iter = lua.create_function(move |lua, (this, i): (LuaUserDataRef<Self>, usize)| {
                let index = i + 1;
                if i < this.len() {
                    let index = index.into_lua(lua)?;
                    let value = this.map::<&_>(|this, _| &this[i]).into_lua(lua)?;
                    Ok((index, value))
                } else {
                    Ok((Nil, Nil))
                }
            })?;

            let this = Ref::clone(this);

            (iter, this, 0).into_lua_multi(lua)
        });
    };
    ($methods:ident, "__pairs") => {
        impl_meta_method!($methods, "__ipairs");
    };
}

impl<T> UserData for Ref<&'static T>
where
    T: LuaPrimitive,
    Self: Debug,
{
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field("type", Self::type_name());
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        impl_meta_method!(methods, "__tostring");
    }
}

impl<X, Y> UserData for Ref<&'static (X, Y)>
where
    X: Getter,
    Y: Getter,
    Self: Debug,
{
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field("type", Self::type_name());

        getter(fields, "x", |s: &Self| s.map(|v, _| &v.0));
        getter(fields, "y", |s: &Self| s.map(|v, _| &v.1));
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        impl_meta_method!(methods, "__tostring");
    }
}

impl<X, Y, Z> UserData for Ref<&'static (X, Y, Z)>
where
    X: Getter,
    Y: Getter,
    Z: Getter,
    Self: Debug,
{
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field("type", Self::type_name());

        getter(fields, "x", |s: &Self| s.map(|v, _| &v.0));
        getter(fields, "y", |s: &Self| s.map(|v, _| &v.1));
        getter(fields, "z", |s: &Self| s.map(|v, _| &v.2));
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        impl_meta_method!(methods, "__tostring");
    }
}

impl<T> UserData for Ref<&'static Vec<T>>
where
    Ref<&'static T>: UserData,
    Self: Debug,
{
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field("type", Self::type_name());
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        impl_meta_method!(methods, "__tostring");
        impl_meta_method!(methods, "__len");
        impl_meta_method!(methods, "__index");
        impl_meta_method!(methods, "__ipairs");
    }
}

impl<const N: usize, T> UserData for Ref<&'static [T; N]>
where
    Ref<&'static T>: UserData,
    Self: Debug,
{
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field("type", Self::type_name());
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        impl_meta_method!(methods, "__tostring");
        impl_meta_method!(methods, "__len");
        impl_meta_method!(methods, "__index");
        impl_meta_method!(methods, "__ipairs");
    }
}

impl UserData for Ref<&'static HashMap<(u32, u32), Reference>>
where
    Self: Debug,
{
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field("type", Self::type_name());
    }
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        impl_meta_method!(methods, "__tostring");
        impl_meta_method!(methods, "__len");
        impl_meta_method!(methods, "__index");
        impl_meta_method!(methods, "__ipairs");
        impl_meta_method!(methods, "__pairs");
    }
}
