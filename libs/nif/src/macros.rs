/// Internal macro that generates properties for flags.
///
macro_rules! flag_props {
    ($($name:ident @ (mask = $mask:expr) -> bool),*) => {
        $(
            #[inline]
            pub fn $name(&self) -> bool {
                self.flags & $mask != 0
            }
            paste::paste! {
                #[inline]
                pub fn [<set_ $name>](&mut self, value: bool) {
                    if value {
                        self.flags |= $mask;
                    } else {
                        self.flags &= !$mask;
                    }
                }
            }
        )*
    };
    ($($name:ident @ (mask = $mask:expr, pos = $pos:expr) -> $type:ty),*) => {
        $(
            #[inline]
            pub fn $name(&self) -> $type {
                let value = (self.flags & $mask) >> $pos;
                value.try_into().unwrap_or_default()
            }
            paste::paste! {
                #[inline]
                pub fn [<set_ $name>](&mut self, value: $type) {
                    self.flags = (self.flags & !$mask) | ((value as u16) << $pos);
                }
            }
        )*
    };
    (
        $name:ident @ (mask = $mask:expr) -> bool,
        $($rest:tt)*
    ) => {
        flag_props!($name @ (mask = $mask) -> bool);
        flag_props!($($rest)*);
    };
    (
        $name:ident @ (mask = $mask:expr, pos = $pos:expr) -> $type:ty,
        $($rest:tt)*
    ) => {
        flag_props!($name @ (mask = $mask, pos = $pos) -> $type);
        flag_props!($($rest)*);
    };
}
pub(crate) use flag_props;
