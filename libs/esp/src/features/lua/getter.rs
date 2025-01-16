use super::*;

pub trait Getter: 'static + Sized {
    fn getter<Owner>(fields: &mut impl UserDataFields<Owner>, name: impl ToString, get: impl Get<Owner, Self>);
}

#[macro_export]
macro_rules! impl_getter {
    () => {
        fn getter<Owner>(
            fields: &mut impl LuaUserDataFields<Owner>,
            name: impl ToString,
            get: impl Get<Owner, Self>, //
        ) {
            fields.add_field_method_get(name, move |_, this| Ok(get(this)))
        }
    };
}

impl<X, Y> Getter for (X, Y)
where
    X: 'static + LuaPrimitive,
    Y: 'static + LuaPrimitive,
    Ref<&'static Self>: UserData,
{
    impl_getter!();
}

impl<X, Y, Z> Getter for (X, Y, Z)
where
    X: 'static + LuaPrimitive,
    Y: 'static + LuaPrimitive,
    Z: 'static + LuaPrimitive,
    Ref<&'static Self>: UserData,
{
    impl_getter!();
}

impl<T> Getter for Vec<T>
where
    T: 'static + LuaPrimitive,
    Ref<&'static Self>: UserData,
{
    impl_getter!();
}

impl<X, Y> Getter for Vec<(X, Y)>
where
    X: 'static,
    Y: 'static,
    Ref<&'static Self>: UserData,
{
    impl_getter!();
}

impl<K, V> Getter for HashMap<K, V>
where
    K: 'static,
    V: 'static,
    Ref<&'static Self>: UserData,
{
    impl_getter!();
}

impl<T, const N: usize> Getter for [T; N]
where
    T: 'static + LuaPrimitive,
    Ref<&'static Self>: UserData,
{
    impl_getter!();
}

//

impl<T> Getter for T
where
    T: 'static + LuaPrimitive,
{
    fn getter<Owner>(
        fields: &mut impl LuaUserDataFields<Owner>,
        name: impl ToString,
        get: impl Get<Owner, Self>, //
    ) {
        fields.add_field_method_get(name, move |lua, this| get(this).clone_into_lua(lua))
    }
}

impl<T, const N: usize> Getter for Box<[T; N]>
where
    T: 'static,
    Ref<&'static [T; N]>: UserData,
{
    fn getter<Owner>(
        fields: &mut impl UserDataFields<Owner>,
        name: impl ToString,
        get: impl Get<Owner, Self>, //
    ) {
        fields.add_field_method_get(name, move |_, this| {
            let value = get(this);
            let remap = value.map::<&_>(|this, _| this.as_ref());
            Ok(remap)
        })
    }
}

impl<T> Getter for Option<T>
where
    T: 'static,
    Ref<&'static T>: UserData,
{
    fn getter<Owner>(
        fields: &mut impl UserDataFields<Owner>,
        name: impl ToString,
        get: impl Get<Owner, Self>, //
    ) {
        fields.add_field_method_get(name, move |lua, this| {
            let value = get(this);
            if value.is_none() {
                Ok(Nil)
            } else {
                let remap = value.map::<&_>(|this, _| this.as_ref().unwrap());
                remap.into_lua(lua)
            }
        })
    }
}

// Convenience Aliases

pub trait Get<Owner, T>
where
    T: 'static,
    Self: 'static + Fn(&Owner) -> Ref<&'static T>,
{
}

impl<Owner, T, F> Get<Owner, T> for F
where
    T: 'static,
    F: 'static + Fn(&Owner) -> Ref<&'static T>,
{
}

pub fn getter<Owner, T: Getter>(
    fields: &mut impl UserDataFields<Owner>,
    name: impl ToString,
    get: impl Get<Owner, T>, //
) {
    <T>::getter(fields, name, get)
}
