// external imports
use float_eq::float_eq;
use nalgebra::{Matrix3, UnitQuaternion};

// internal imports
use crate::prelude::*;

pub trait InsertKeys {
    /// Evaluate and insert animation keys for the given times.
    ///
    /// If evaluation is impossible, insert the provided default value.
    ///
    fn evaluate_and_insert(&mut self, times: &[f32], default: &[f32]);
}

impl InsertKeys for NiFloatKey {
    fn evaluate_and_insert(&mut self, times: &[f32], default: &[f32]) {
        match self {
            NiFloatKey::LinKey(keys) => keys.evaluate_and_insert(times, default),
            NiFloatKey::BezKey(keys) => keys.evaluate_and_insert(times, default),
            NiFloatKey::TCBKey(keys) => keys.evaluate_and_insert(times, default),
        }
    }
}

impl InsertKeys for NiPosKey {
    fn evaluate_and_insert(&mut self, times: &[f32], default: &[f32]) {
        match self {
            NiPosKey::LinKey(keys) => keys.evaluate_and_insert(times, default),
            NiPosKey::BezKey(keys) => keys.evaluate_and_insert(times, default),
            NiPosKey::TCBKey(keys) => keys.evaluate_and_insert(times, default),
        }
    }
}

impl InsertKeys for NiRotKey {
    fn evaluate_and_insert(&mut self, times: &[f32], default: &[f32]) {
        let mat = Matrix3::from_column_slice(default);
        let quat = UnitQuaternion::from_matrix(&mat);
        let default = quat.coords.as_slice();
        match self {
            NiRotKey::LinKey(keys) => keys.evaluate_and_insert(times, default),
            NiRotKey::BezKey(keys) => keys.evaluate_and_insert(times, default),
            NiRotKey::TCBKey(keys) => keys.evaluate_and_insert(times, default),
            NiRotKey::EulerKey(keys) => {
                assert_eq!(keys.euler_axis_order, AxisOrder::XYZ);
                let (x, y, z) = quat.euler_angles();
                for (data, angle) in keys.euler_data.iter_mut().zip([x, y, z]) {
                    data.keys.evaluate_and_insert(times, &[angle]);
                }
            }
        }
    }
}

trait InsertKeysGeneric<const K: usize, const V: usize>: Sized {
    /// Evaluate and insert animation keys for the given times.
    ///
    /// If evaluation is impossible, insert the provided default value.
    ///
    fn evaluate_and_insert(&mut self, times: &[f32], default: &[f32]);
}

impl<T, const K: usize, const V: usize> InsertKeysGeneric<K, V> for T
where
    T: KeysExt<K, V>,
{
    fn evaluate_and_insert(&mut self, times: &[f32], default: &[f32]) {
        if times.is_empty() {
            return;
        }

        // if the original keys data is empty, fill with defaults
        if self.is_empty() {
            *self = Self::zeros(times.len());
            // fill in times
            self.times_mut().copy_from_slice(times);
            // fill in values
            self.values_mut()
                .column_iter_mut()
                .for_each(|mut v| v.copy_from_slice(default));
            return;
        }

        // evaluate new keys for all times not already present
        let mut keys = Self::zeros(self.num_keys() + times.len());
        let max_time = times.last().unwrap().max(self.last_time());

        let mut new_index = 0;
        let mut old_index = 0;
        let mut tan_scale = 0.0;

        for time in times.iter().copied().chain([max_time]) {
            // first insert any old keys that had a prior timing
            while old_index < self.num_keys() {
                let key = self.data().column(old_index);
                if time < key[0] {
                    break;
                }

                // insert the key
                keys.key_mut(new_index).copy_from(&key);

                // scale tangents
                if Self::HAS_BEZ_TANGENTS {
                    keys.in_tan_mut(new_index).scale_mut(1.0 - tan_scale);
                    tan_scale = 0.0;
                }

                // offset indices
                new_index += 1;
                old_index += 1;
            }

            // skip times that are the same as the previous time
            if new_index != 0 {
                let prev_time = *keys.time(new_index - 1);
                if float_eq!(time, prev_time, abs <= 1e-6) {
                    continue;
                }
            }

            // skip times that are the same as the next old time
            if old_index < self.num_keys() {
                let next_time = *self.time(old_index);
                if float_eq!(time, next_time, abs <= 1e-6) {
                    continue;
                }
            }

            // evaluate and insert the result for the given time
            let start_index = self.prev_index(old_index);
            let (_, t, key) = self.evaluate(time, start_index).unwrap();

            // insert the key
            keys.key_mut(new_index).copy_from(key.data());

            // scale tangents
            if Self::HAS_BEZ_TANGENTS {
                if new_index != 0 {
                    // apply any stored in_tan
                    keys.in_tan_mut(new_index).scale_mut(1.0 - tan_scale);
                    // adjust previous out_tan
                    keys.out_tan_mut(new_index - 1).scale_mut(t);
                }
                tan_scale = t;
            }

            // offset indices
            new_index += 1;
        }

        // truncate if it had duplicates
        if keys.num_keys() > new_index {
            keys.data_mut().resize_horizontally_mut(new_index, 0.0);
        }

        *self = keys;
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::dmatrix;

    #[test]
    fn insert_defaults() {
        let mut keys = LinFloatKeys::default();

        keys.evaluate_and_insert(&[0.0, 1.0, 2.0], &[7.0]);

        assert_eq!(keys.times(), dmatrix![0.0, 1.0, 2.0]);
        assert_eq!(keys.values(), dmatrix![7.0, 7.0, 7.0]);
    }

    #[test]
    fn insert_with_duplicates() {
        let mut keys = LinFloatKeys::from_vec(vec![
            /*time*/ 1.0, /*value*/ 3.0,
            /*time*/ 2.0, /*value*/ 4.0,
        ]);

        keys.evaluate_and_insert(&[0.0, 0.0, 1.0-1e-7, 1.0+1e-7, 2.0, 3.0, 3.0], &[0.0]);

        assert_eq!(keys.times(),  dmatrix![0.0, 1.0, 2.0, 3.0]);
        assert_eq!(keys.values(),  dmatrix![3.0, 3.0, 4.0, 4.0]);
    }

    #[test]
    fn insert_only_duplicates() {
        let mut keys = LinFloatKeys::from_vec(vec![
            /*time*/ 1.0, /*value*/ 3.0,
            /*time*/ 2.0, /*value*/ 4.0,
        ]);

        keys.evaluate_and_insert(&[1.0, 2.0], &[0.0]);

        assert_eq!(keys.times(), dmatrix![1.0, 2.0]);
        assert_eq!(keys.values(), dmatrix![3.0, 4.0]);
    }

    #[test]
    fn insert_with_trailing_start() {
        let mut keys = LinFloatKeys::from_vec(vec![
            /*time*/ 0.0, /*value*/ 3.0,
            /*time*/ 1.0, /*value*/ 4.0,
            /*time*/ 2.0, /*value*/ 5.0,
        ]);

        keys.evaluate_and_insert(&[1.5, 2.5], &[0.0]);

        assert_eq!(keys.times(),  dmatrix![0.0, 1.0, 1.5, 2.0, 2.5]);
        assert_eq!(keys.values(),  dmatrix![3.0, 4.0, 4.5, 5.0, 5.0]);
    }

    #[test]
    fn insert_with_trailing_end() {
        let mut keys = LinFloatKeys::from_vec(vec![
            /*time*/ 1.0, /*value*/ 3.0,
            /*time*/ 2.0, /*value*/ 4.0,
            /*time*/ 3.0, /*value*/ 5.0,
        ]);

        keys.evaluate_and_insert(&[0.0, 1.5], &[0.0]);

        assert_eq!(keys.times(),  dmatrix![0.0, 1.0, 1.5, 2.0, 3.0]);
        assert_eq!(keys.values(),  dmatrix![3.0, 3.0, 3.5, 4.0, 5.0]);
    }
}
