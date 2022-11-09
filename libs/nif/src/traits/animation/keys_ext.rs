#![allow(clippy::many_single_char_names, clippy::wildcard_imports)]

// external imports
use nalgebra as na;
use nalgebra::constraint::{DimEq, ShapeConstraint};
use nalgebra::Const;

// internal imports
use crate::prelude::*;

// aliased generics
mod private {
    use nalgebra::*;
    pub(crate) type Key<const K: usize> = OVector<f32, Const<K>>;
    pub(crate) type KeySlice<'a, const K: usize, const V: usize> = MatrixSlice<'a, f32, Const<V>, U1, U1, Const<K>>;
    pub(crate) type KeySliceMut<'a, const K: usize, const V: usize> = MatrixSliceMut<'a, f32, Const<V>, U1, U1, Const<K>>;
    pub(crate) type Keys<const K: usize, const V: usize> = OMatrix<f32, Const<K>, Dynamic>;
    pub(crate) type KeysSlice<'a, const K: usize, const V: usize> = MatrixSlice<'a, f32, Const<V>, Dynamic, U1, Const<K>>;
    pub(crate) type KeysSliceMut<'a, const K: usize, const V: usize> =
        MatrixSliceMut<'a, f32, Const<V>, Dynamic, U1, Const<K>>;
}
use private::*;

pub trait KeysExt<const KEY_SIZE: usize, const VALUE_SIZE: usize>
where
    Self: Default + From<NiAnimationKeys<KEY_SIZE, VALUE_SIZE>>,
{
    // Associated Types

    type Key: KeyExt<KEY_SIZE, VALUE_SIZE>;

    // Associated Constants

    const KEY_TYPE: KeyType;
    const KEY_CONTENT: KeyContent;

    // Derived Constants

    const TIMES_ROW: usize = 0;
    const VALUES_ROW: usize = 1;
    const IN_TANS_ROW: usize = 1 + VALUE_SIZE;
    const OUT_TANS_ROW: usize = 1 + VALUE_SIZE * 2;
    const TCB_PARAMS_ROW: usize = 1 + VALUE_SIZE;

    // Domain Constants

    #[rustfmt::skip]
    const HERMITE_BASIS: na::Matrix4<f32> = na::Matrix4::new(
        2.0, -3.0, 0.0, 1.0,
        -2.0, 3.0, 0.0, 0.0,
        1.0, -2.0, 1.0, 0.0,
        1.0, -1.0, 0.0, 0.0,
    );

    // Helper Constants

    const HAS_TCB_PARAMS: bool = matches!(Self::KEY_TYPE, KeyType::TCBKey);

    const HAS_BEZ_TANGENTS: bool = matches!(
        (Self::KEY_TYPE, Self::KEY_CONTENT),
        (KeyType::BezKey, KeyContent::FloatKey | KeyContent::PosKey)
    );

    // Required Methods

    fn data(&self) -> &Keys<KEY_SIZE, VALUE_SIZE>;

    fn data_mut(&mut self) -> &mut Keys<KEY_SIZE, VALUE_SIZE>;

    // Constructors

    fn zeros(num_keys: usize) -> Self {
        Self::from(Keys::zeros(num_keys).into())
    }

    fn from_vec(data: Vec<f32>) -> Self {
        assert_eq!(data.len() % KEY_SIZE, 0);
        Self::from(Keys::from_vec(data).into())
    }

    //

    fn default_key(&self) -> Self::Key {
        Self::Key::default()
    }

    fn num_keys(&self) -> usize {
        self.data().ncols()
    }

    fn is_empty(&self) -> bool {
        self.data().is_empty()
    }

    fn first_time(&self) -> f32 {
        *self.time(0)
    }

    fn last_time(&self) -> f32 {
        *self.time(self.num_keys() - 1)
    }

    //

    fn key(&self, i: usize) -> Self::Key {
        Self::Key::from(self.data().column(i).clone_owned().into())
    }

    fn time(&self, i: usize) -> &f32 {
        self.data().index((Self::TIMES_ROW, i))
    }

    fn value(&self, i: usize) -> KeySlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_slice(Self::VALUES_ROW, i)
    }

    fn in_tan(&self, i: usize) -> KeySlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_slice(Self::IN_TANS_ROW, i)
    }

    fn out_tan(&self, i: usize) -> KeySlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_slice(Self::OUT_TANS_ROW, i)
    }

    fn tcb_param(&self, i: usize) -> KeySlice<'_, KEY_SIZE, 3> {
        self.data().fixed_slice(Self::TCB_PARAMS_ROW, i)
    }

    //

    fn key_mut(&mut self, i: usize) -> KeySliceMut<'_, KEY_SIZE, KEY_SIZE> {
        self.data_mut().column_mut(i)
    }

    fn time_mut(&mut self, i: usize) -> &mut f32 {
        self.data_mut().index_mut((Self::TIMES_ROW, i))
    }

    fn value_mut(&mut self, i: usize) -> KeySliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_slice_mut(Self::VALUES_ROW, i)
    }

    fn in_tan_mut(&mut self, i: usize) -> KeySliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_slice_mut(Self::IN_TANS_ROW, i)
    }

    fn out_tan_mut(&mut self, i: usize) -> KeySliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_slice_mut(Self::OUT_TANS_ROW, i)
    }

    fn tcb_param_mut(&mut self, i: usize) -> KeySliceMut<'_, KEY_SIZE, 3> {
        self.data_mut().fixed_slice_mut(Self::TCB_PARAMS_ROW, i)
    }

    //

    fn keys(&self) -> KeysSlice<'_, KEY_SIZE, KEY_SIZE> {
        self.data().into()
    }

    fn times(&self) -> KeysSlice<'_, KEY_SIZE, 1> {
        self.data().fixed_rows(Self::TIMES_ROW)
    }

    fn values(&self) -> KeysSlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_rows(Self::VALUES_ROW)
    }

    fn in_tans(&self) -> KeysSlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_rows(Self::IN_TANS_ROW)
    }

    fn out_tans(&self) -> KeysSlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_rows(Self::OUT_TANS_ROW)
    }

    fn tcb_params(&self) -> KeysSlice<'_, KEY_SIZE, 3> {
        self.data().fixed_rows(Self::TCB_PARAMS_ROW)
    }

    //

    fn keys_mut(&mut self) -> KeysSliceMut<'_, KEY_SIZE, KEY_SIZE> {
        self.data_mut().into()
    }

    fn values_mut(&mut self) -> KeysSliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_rows_mut(Self::VALUES_ROW)
    }

    fn times_mut(&mut self) -> KeysSliceMut<'_, KEY_SIZE, 1> {
        self.data_mut().fixed_rows_mut(Self::TIMES_ROW)
    }

    fn in_tans_mut(&mut self) -> KeysSliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_rows_mut(Self::IN_TANS_ROW)
    }

    fn out_tans_mut(&mut self) -> KeysSliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_rows_mut(Self::OUT_TANS_ROW)
    }

    fn tcb_params_mut(&mut self) -> KeysSliceMut<'_, KEY_SIZE, 3> {
        self.data_mut().fixed_rows_mut(Self::TCB_PARAMS_ROW)
    }

    //

    fn tcb_in_tan(&self, i: usize) -> Key<VALUE_SIZE> {
        let p = self.prev_index(i);
        let n = self.next_index(i);

        let prev_time = self.time(p);
        let this_time = self.time(i);
        let next_time = self.time(n);

        let prev_value = self.value(p);
        let this_value = self.value(i);
        let next_value = self.value(n);

        let prev_len = this_value - prev_value;
        let next_len = next_value - this_value;

        let [t, c, b] = self.tcb_param(i).as_ref().to_owned();
        let ts = (1.0 - t) * (1.0 - c) * (1.0 + b) * prev_len;
        let td = (1.0 - t) * (1.0 + c) * (1.0 - b) * next_len;

        (ts + td) * (this_time - prev_time) / (next_time - prev_time)
    }

    fn tcb_out_tan(&self, i: usize) -> Key<VALUE_SIZE> {
        let p = self.prev_index(i);
        let n = self.next_index(i);

        let prev_time = self.time(p);
        let this_time = self.time(i);
        let next_time = self.time(n);

        let prev_value = self.value(p);
        let this_value = self.value(i);
        let next_value = self.value(n);

        let prev_len = this_value - prev_value;
        let next_len = next_value - this_value;

        let [t, c, b] = self.tcb_param(i).as_ref().to_owned();
        let ts = (1.0 - t) * (1.0 + c) * (1.0 + b) * prev_len;
        let td = (1.0 - t) * (1.0 - c) * (1.0 - b) * next_len;

        (ts + td) * (this_time - next_time) / (prev_time - next_time)
    }

    //

    #[inline]
    fn prev_index(&self, i: usize) -> usize {
        i.saturating_sub(1)
    }

    #[inline]
    fn next_index(&self, i: usize) -> usize {
        i.saturating_add(1).min(self.num_keys() - 1)
    }

    /// Search for the position in which an animation key for the given time would appear.
    /// Returns the previous and following indices, or None if there were no keys present.
    ///
    /// Returns (0, 0) or (len-1, len-1) if the key was out of bounds.
    ///
    fn position(&self, time: f32, start_index: usize) -> Option<(usize, usize)> {
        let length = self.num_keys();
        if length == 0 {
            return None;
        }
        if start_index >= length || time >= self.last_time() {
            return Some((length - 1, length - 1));
        }
        let times = self.data().slice_range(Self::TIMES_ROW, start_index..);
        let next_index = start_index + times.iter().position(|t| *t >= time)?;
        let prev_index = self.prev_index(next_index);
        Some((prev_index, next_index))
    }

    /// Evaluate the animation key for the given time.
    ///
    /// Returns a tuple of `(index, fraction, key)`.
    ///
    /// `index` is the index in the keys array where the evaluated key would be positioned.
    ///
    /// `fraction` defines the fractional position between the previous and following keys.
    ///
    /// `key` is the evaluated key.
    ///
    fn evaluate(&self, time: f32, start_index: usize) -> Option<(usize, f32, Self::Key)> {
        // KEY_TYPE is known at compile time, so the branching here gets optimized away
        match Self::KEY_TYPE {
            KeyType::LinKey => self.lin_interp(time, start_index),
            KeyType::BezKey => self.bez_interp(time, start_index),
            KeyType::TCBKey => self.tcb_interp(time, start_index),
            _ => None,
        }
    }

    fn lin_interp(&self, time: f32, start_index: usize) -> Option<(usize, f32, Self::Key)> {
        let (i, j) = self.position(time, start_index)?;
        if i == j {
            let mut key = self.key(i);
            key.set_time(time);
            // TODO: this stuff gets copy pasted every where and looks stupid
            // make a helper func returns either this or Self::Key::default()
            return Some((j, 1.0, key));
        }

        let prev_time = self.time(i);
        let next_time = self.time(j);

        let prev_value = self.value(i);
        let next_value = self.value(j);

        let t = (time - prev_time) / (next_time - prev_time);
        let value = prev_value.lerp(&next_value, t);

        let mut key = Self::Key::default();
        key.set_time(time);
        key.set_value(value);

        Some((j, t, key))
    }

    fn bez_interp(&self, time: f32, start_index: usize) -> Option<(usize, f32, Self::Key)> {
        let (i, j) = self.position(time, start_index)?;
        if i == j {
            let mut key = self.key(i);
            key.set_time(time);
            return Some((j, 1.0, key));
        }

        let prev_time = self.time(i);
        let next_time = self.time(j);

        let prev_value = self.value(i);
        let next_value = self.value(j);

        let in_tan = self.out_tan(i);
        let out_tan = self.in_tan(j);

        let t = (time - prev_time) / (next_time - prev_time);
        let t2 = t * t;
        let t3 = t2 * t;

        let v = na::Vector4::<f32>::new(t3, t2, t, 1.0);
        let s = na::Vector4::<f32>::new(3.0 * t2, 2.0 * t, 1.0, 0.0);
        let c = na::SMatrix::<f32, VALUE_SIZE, 4>::from_columns(&[prev_value, next_value, in_tan, out_tan]);

        let value = c * (Self::HERMITE_BASIS * v);
        let tan_u = c * (Self::HERMITE_BASIS * s);

        let in_tan = tan_u.scale(t);
        let out_tan = tan_u.scale(1.0 - t);

        let mut key = Self::Key::default();
        key.set_time(time);
        key.set_value(value);
        key.set_in_tan(in_tan);
        key.set_out_tan(out_tan);

        Some((j, t, key))
    }

    fn tcb_interp(&self, time: f32, start_index: usize) -> Option<(usize, f32, Self::Key)> {
        let (i, j) = self.position(time, start_index)?;
        if i == j {
            let mut key = self.key(i);
            key.set_time(time);
            return Some((j, 1.0, key));
        }

        let prev_time = self.time(i);
        let next_time = self.time(j);

        let prev_value = self.value(i).into();
        let next_value = self.value(j).into();

        let in_tan = self.tcb_out_tan(i);
        let out_tan = self.tcb_in_tan(j);

        let t = (time - prev_time) / (next_time - prev_time);
        let t2 = t * t;
        let t3 = t2 * t;

        let v = na::Vector4::<f32>::new(t3, t2, t, 1.0);
        let c = na::SMatrix::<f32, VALUE_SIZE, 4>::from_columns(&[prev_value, next_value, in_tan, out_tan]);

        let value = c * (Self::HERMITE_BASIS * v);

        let mut key = Self::Key::default();
        key.set_time(time);
        key.set_value(value);

        Some((j, t, key))
    }

    // Load + Save

    fn load(stream: &mut Reader<'_>, num_keys: usize) -> io::Result<Self> {
        let data = stream.load_matrix(KEY_SIZE, num_keys)?;
        let keys = NiAnimationKeys { data };
        Ok(keys.into())
    }

    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save_as::<_, u32>(self.num_keys())?;
        if !self.is_empty() {
            stream.save(&Self::KEY_TYPE)?;
            stream.save_matrix(self.data())?;
        }
        Ok(())
    }

    // Load + Save (Quaternions)

    #[doc(hidden)]
    fn load_q(stream: &mut Reader<'_>, num_keys: usize) -> io::Result<Self>
    where
        ShapeConstraint: DimEq<Const<VALUE_SIZE>, Const<4>>,
    {
        let mut this = Self::load(stream, num_keys)?;
        this.swap_quaternion_layout(true);
        Ok(this)
    }

    #[doc(hidden)]
    fn save_q(&self, stream: &mut Writer) -> io::Result<()>
    where
        ShapeConstraint: DimEq<Const<VALUE_SIZE>, Const<4>>,
    {
        let data = self.data().clone_owned();
        let mut temp = Self::from(NiAnimationKeys { data });
        temp.swap_quaternion_layout(false);
        temp.save(stream)?;
        Ok(())
    }

    #[doc(hidden)]
    /// Swap quaternion layout from wxyz to xyzw, or vice-versa.
    fn swap_quaternion_layout(&mut self, reverse: bool)
    where
        ShapeConstraint: DimEq<Const<VALUE_SIZE>, Const<4>>,
    {
        let (w, x, y, z) = if reverse { (4, 3, 2, 1) } else { (1, 2, 3, 4) };

        for mut v in self.data_mut().column_iter_mut() {
            // Safety: ensured by the ShapeConstraint bound
            unsafe {
                use nalgebra::RawStorageMut;
                v.data.swap_unchecked_linear(w, x); // wxyz -> xwyz | xyzw -> xywz
                v.data.swap_unchecked_linear(x, y); // xwyz -> xywz | xywz -> xwyz
                v.data.swap_unchecked_linear(y, z); // xywz -> xyzw | xwyz -> wxyz
            }
        }
    }
}

impl KeysExt<2, 1> for LinFloatKeys {
    type Key = NiAnimationKey<2, 1>;

    const KEY_TYPE: KeyType = KeyType::LinKey;
    const KEY_CONTENT: KeyContent = KeyContent::FloatKey;

    fn data(&self) -> &Keys<2, 1> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<2, 1> {
        &mut self.data
    }
}

impl KeysExt<4, 1> for BezFloatKeys {
    type Key = NiAnimationKey<4, 1>;

    const KEY_TYPE: KeyType = KeyType::BezKey;
    const KEY_CONTENT: KeyContent = KeyContent::FloatKey;

    fn data(&self) -> &Keys<4, 1> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<4, 1> {
        &mut self.data
    }
}

impl KeysExt<5, 1> for TCBFloatKeys {
    type Key = NiAnimationKey<5, 1>;

    const KEY_TYPE: KeyType = KeyType::TCBKey;
    const KEY_CONTENT: KeyContent = KeyContent::FloatKey;

    fn data(&self) -> &Keys<5, 1> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<5, 1> {
        &mut self.data
    }
}

impl KeysExt<4, 3> for LinPosKeys {
    type Key = NiAnimationKey<4, 3>;

    const KEY_TYPE: KeyType = KeyType::LinKey;
    const KEY_CONTENT: KeyContent = KeyContent::PosKey;

    fn data(&self) -> &Keys<4, 3> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<4, 3> {
        &mut self.data
    }
}

impl KeysExt<10, 3> for BezPosKeys {
    type Key = NiAnimationKey<10, 3>;
    const KEY_TYPE: KeyType = KeyType::BezKey;
    const KEY_CONTENT: KeyContent = KeyContent::PosKey;

    fn data(&self) -> &Keys<10, 3> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<10, 3> {
        &mut self.data
    }
}

impl KeysExt<7, 3> for TCBPosKeys {
    type Key = NiAnimationKey<7, 3>;

    const KEY_TYPE: KeyType = KeyType::TCBKey;
    const KEY_CONTENT: KeyContent = KeyContent::PosKey;

    fn data(&self) -> &Keys<7, 3> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<7, 3> {
        &mut self.data
    }
}

impl KeysExt<5, 4> for LinColKeys {
    type Key = NiAnimationKey<5, 4>;

    const KEY_TYPE: KeyType = KeyType::LinKey;
    const KEY_CONTENT: KeyContent = KeyContent::ColorKey;

    fn data(&self) -> &Keys<5, 4> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<5, 4> {
        &mut self.data
    }
}

impl KeysExt<5, 4> for LinRotKeys {
    type Key = NiAnimationKey<5, 4>;

    const KEY_TYPE: KeyType = KeyType::LinKey;
    const KEY_CONTENT: KeyContent = KeyContent::RotKey;

    fn data(&self) -> &Keys<5, 4> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<5, 4> {
        &mut self.data
    }

    fn lin_interp(&self, time: f32, start_index: usize) -> Option<(usize, f32, Self::Key)> {
        let (i, j) = self.position(time, start_index)?;
        if i == j {
            let mut key = self.key(i);
            key.set_time(time);
            return Some((j, 1.0, key));
        }

        let prev_time = self.time(i);
        let next_time = self.time(j);

        let prev_value = self.value(i).into_owned().into();
        let next_value = self.value(j).into_owned().into();

        let t = (time - prev_time) / (next_time - prev_time);
        let value = slerp(prev_value, next_value, t);

        let mut key = Self::Key::default();
        key.set_time(time);
        key.set_value(value.coords);
        Some((j, t, key))
    }
}

impl KeysExt<5, 4> for BezRotKeys {
    type Key = NiAnimationKey<5, 4>;

    const KEY_TYPE: KeyType = KeyType::BezKey;
    const KEY_CONTENT: KeyContent = KeyContent::RotKey;

    fn data(&self) -> &Keys<5, 4> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<5, 4> {
        &mut self.data
    }

    fn bez_interp(&self, time: f32, start_index: usize) -> Option<(usize, f32, Self::Key)> {
        let (i, j) = self.position(time, start_index)?;
        if i == j {
            let mut key = self.key(i);
            key.set_time(time);
            return Some((j, 1.0, key));
        }

        let h = self.prev_index(i);
        let k = self.next_index(j);

        let prev_time = self.time(i);
        let next_time = self.time(j);

        let h_value = self.value(h).into_owned().into();
        let i_value = self.value(i).into_owned().into();
        let j_value = self.value(j).into_owned().into();
        let k_value = self.value(k).into_owned().into();

        let i0 = intermediate(h_value, i_value, j_value);
        let i1 = intermediate(i_value, j_value, k_value);

        let t = (time - prev_time) / (next_time - prev_time);
        let value = squad(i_value, i0, i1, j_value, t);

        let mut key = Self::Key::default();
        key.set_time(time);
        key.set_value(value.coords);
        Some((j, t, key))
    }
}

impl KeysExt<8, 4> for TCBRotKeys {
    type Key = NiAnimationKey<8, 4>;

    const KEY_TYPE: KeyType = KeyType::TCBKey;
    const KEY_CONTENT: KeyContent = KeyContent::RotKey;

    fn data(&self) -> &Keys<8, 4> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Keys<8, 4> {
        &mut self.data
    }

    fn tcb_in_tan(&self, i: usize) -> Key<4> {
        let p = self.prev_index(i);
        let n = self.next_index(i);

        let prev_time = self.time(p);
        let this_time = self.time(i);
        let next_time = self.time(n);

        let prev_value: na::Quaternion<f32> = self.value(p).into_owned().into();
        let this_value: na::Quaternion<f32> = self.value(i).into_owned().into();
        let next_value: na::Quaternion<f32> = self.value(n).into_owned().into();

        let prev_len = (prev_value.conjugate() * this_value).ln();
        let next_len = (this_value.conjugate() * next_value).ln();

        let [t, c, b] = self.tcb_param(i).as_ref().to_owned();
        let ts = (1.0 - t) * (1.0 - c) * (1.0 + b) * prev_len;
        let td = (1.0 - t) * (1.0 + c) * (1.0 - b) * next_len;

        let r = (ts + td) * (next_time - this_time) / (next_time - prev_time);

        (this_value * (0.5 * (prev_len - r)).exp()).coords
    }

    fn tcb_out_tan(&self, i: usize) -> Key<4> {
        let p = self.prev_index(i);
        let n = self.next_index(i);

        let prev_time = self.time(p);
        let this_time = self.time(i);
        let next_time = self.time(n);

        let prev_value: na::Quaternion<f32> = self.value(p).into_owned().into();
        let this_value: na::Quaternion<f32> = self.value(i).into_owned().into();
        let next_value: na::Quaternion<f32> = self.value(n).into_owned().into();

        let prev_len = (prev_value.conjugate() * this_value).ln();
        let next_len = (this_value.conjugate() * next_value).ln();

        let [t, c, b] = self.tcb_param(i).as_ref().to_owned();
        let ts = (1.0 - t) * (1.0 + c) * (1.0 + b) * prev_len;
        let td = (1.0 - t) * (1.0 - c) * (1.0 - b) * next_len;

        let r = (ts + td) * (this_time - prev_time) / (next_time - prev_time);

        (this_value * (0.5 * (r - next_len)).exp()).coords
    }

    fn tcb_interp(&self, time: f32, start_index: usize) -> Option<(usize, f32, Self::Key)> {
        let (i, j) = self.position(time, start_index)?;
        if i == j {
            let mut key = self.key(i);
            key.set_time(time);
            return Some((j, 1.0, key));
        }

        let prev_time = self.time(i);
        let next_time = self.time(j);

        let prev_value = self.value(i).into_owned().into();
        let next_value = self.value(j).into_owned().into();

        let in_tan = self.tcb_out_tan(i).into();
        let out_tan = self.tcb_in_tan(j).into();

        let t = (time - prev_time) / (next_time - prev_time);
        let value = squad(prev_value, in_tan, out_tan, next_value, t);

        let mut key = Self::Key::default();
        key.set_time(time);
        key.set_value(value.coords);
        Some((j, t, key))
    }
}

pub trait KeyExt<const KEY_SIZE: usize, const VALUE_SIZE: usize>
where
    Self: Default + From<NiAnimationKey<KEY_SIZE, VALUE_SIZE>>,
{
    const TIMES_ROW: usize = 0;
    const VALUES_ROW: usize = 1;
    const IN_TANS_ROW: usize = 1 + VALUE_SIZE;
    const OUT_TANS_ROW: usize = 1 + VALUE_SIZE * 2;
    const TCB_PARAMS_ROW: usize = 1 + VALUE_SIZE;

    fn data(&self) -> &Key<KEY_SIZE>;

    fn data_mut(&mut self) -> &mut Key<KEY_SIZE>;

    //

    fn time(&self) -> &f32 {
        self.data().index(Self::TIMES_ROW)
    }

    fn value(&self) -> KeySlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_rows(Self::VALUES_ROW)
    }

    fn in_tan(&self) -> KeySlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_rows(Self::IN_TANS_ROW)
    }

    fn out_tan(&self) -> KeySlice<'_, KEY_SIZE, VALUE_SIZE> {
        self.data().fixed_rows(Self::OUT_TANS_ROW)
    }

    fn tcb_param(&self) -> KeySlice<'_, KEY_SIZE, 3> {
        self.data().fixed_rows(Self::TCB_PARAMS_ROW)
    }

    //

    fn time_mut(&mut self) -> &mut f32 {
        self.data_mut().index_mut(Self::TIMES_ROW)
    }

    fn value_mut(&mut self) -> KeySliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_rows_mut(Self::VALUES_ROW)
    }

    fn in_tan_mut(&mut self) -> KeySliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_rows_mut(Self::IN_TANS_ROW)
    }

    fn out_tan_mut(&mut self) -> KeySliceMut<'_, KEY_SIZE, VALUE_SIZE> {
        self.data_mut().fixed_rows_mut(Self::OUT_TANS_ROW)
    }

    fn tcb_param_mut(&mut self) -> KeySliceMut<'_, KEY_SIZE, 3> {
        self.data_mut().fixed_rows_mut(Self::TCB_PARAMS_ROW)
    }

    //

    fn set_time(&mut self, value: f32) {
        *self.time_mut() = value;
    }

    fn set_value(&mut self, value: Key<VALUE_SIZE>) {
        self.value_mut().copy_from(&value);
    }

    fn set_in_tan(&mut self, in_tan: Key<VALUE_SIZE>) {
        self.in_tan_mut().copy_from(&in_tan);
    }

    fn set_out_tan(&mut self, out_tan: Key<VALUE_SIZE>) {
        self.out_tan_mut().copy_from(&out_tan);
    }

    fn set_tcb_param(&mut self, tcb_param: Key<3>) {
        self.tcb_param_mut().copy_from(&tcb_param);
    }
}

impl<const K: usize, const V: usize> KeyExt<K, V> for NiAnimationKey<K, V>
where
    Self: Default + From<NiAnimationKey<K, V>>,
{
    fn data(&self) -> &Key<K> {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Key<K> {
        &mut self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn position_works() {
        #[rustfmt::skip]
        let keys = LinFloatKeys::from_vec(vec![
            /*time*/ 2.0, /*value*/0.0,
            /*time*/ 4.0, /*value*/0.0,
            /*time*/ 6.0, /*value*/0.0,
            /*time*/ 8.0, /*value*/0.0,
        ]);

        // clamps to first index if before first start time
        let (prev_index, next_index) = keys.position(0.0, 0).unwrap();
        assert_eq!(prev_index, 0);
        assert_eq!(next_index, 0);

        // returns previous/next indices if inbetween times
        let (prev_index, next_index) = keys.position(3.0, 0).unwrap();
        assert_eq!(prev_index, 0);
        assert_eq!(next_index, 1);

        // returns previous/next inclusive on direct matches
        let (prev_index, next_index) = keys.position(6.0, 0).unwrap();
        assert_eq!(prev_index, 1);
        assert_eq!(next_index, 2);

        // clamps to last index if beyond last start time
        let (prev_index, next_index) = keys.position(9.0, 0).unwrap();
        assert_eq!(prev_index, 3);
        assert_eq!(next_index, 3);

        // returned indices are independent of start_index
        assert_eq!(keys.position(1.0, 0).unwrap(), keys.position(1.0, 0).unwrap());
        assert_eq!(keys.position(3.0, 0).unwrap(), keys.position(3.0, 0).unwrap());
        assert_eq!(keys.position(7.0, 0).unwrap(), keys.position(7.0, 2).unwrap());
        assert_eq!(keys.position(9.0, 0).unwrap(), keys.position(9.0, 3).unwrap());
    }

    #[test_case("tests/assets/eval_pos_lin.nif")]
    #[test_case("tests/assets/eval_pos_bez.nif")]
    #[test_case("tests/assets/eval_pos_tcb.nif")]
    fn evaluate_and_insert_translations(path: &str) {
        let mut stream = NiStream::from_path(path).unwrap();
        let mut objects = stream.objects_of_type_mut::<NiKeyframeData>();

        let target = &mut objects.next().unwrap().translations.keys;
        let expect = &mut objects.next().unwrap().translations.keys;

        target.evaluate_and_insert(&[2.0], &[0.0; 3]);

        assert_eq!(target, expect);
    }

    #[test_case("tests/assets/eval_rot_lin.nif")]
    #[test_case("tests/assets/eval_rot_bez.nif")]
    #[test_case("tests/assets/eval_rot_tcb.nif")]
    fn evaluate_and_insert_rotations(path: &str) {
        let mut stream = NiStream::from_path(path).unwrap();
        let mut objects = stream.objects_of_type_mut::<NiKeyframeData>();

        let target = &mut objects.next().unwrap().rotations.keys;
        let expect = &mut objects.next().unwrap().rotations.keys;

        target.evaluate_and_insert(&[2.0], &[0.0; 9]);

        assert_eq!(target, expect);
    }

    #[test_case("tests/assets/eval_sca_lin.nif")]
    #[test_case("tests/assets/eval_sca_bez.nif")]
    #[test_case("tests/assets/eval_sca_tcb.nif")]
    fn evaluate_and_insert_scales(path: &str) {
        let mut stream = NiStream::from_path(path).unwrap();
        let mut objects = stream.objects_of_type_mut::<NiKeyframeData>();

        let target = &mut objects.next().unwrap().scales.keys;
        let expect = &mut objects.next().unwrap().scales.keys;

        target.evaluate_and_insert(&[2.0], &[0.0]);

        assert_eq!(target, expect);
    }
}
