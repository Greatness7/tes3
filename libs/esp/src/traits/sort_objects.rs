use crate::prelude::*;

impl Plugin {
    pub fn sort_objects(&mut self) {
        let mut indices = vec![];

        // Objects will be ordered by type, sort hint, and editor id.
        // Dialogue/DialogueInfo sorting is not currently implemented.
        #[rustfmt::skip]
        isort(&mut indices, &self.objects, |object| {
            match object {
                TES3Object::Header(obj)           => ( 0, obj.sort_hint(), obj.editor_id()),
                TES3Object::GameSetting(obj)      => ( 1, obj.sort_hint(), obj.editor_id()),
                TES3Object::GlobalVariable(obj)   => ( 2, obj.sort_hint(), obj.editor_id()),
                TES3Object::Class(obj)            => ( 3, obj.sort_hint(), obj.editor_id()),
                TES3Object::Faction(obj)          => ( 4, obj.sort_hint(), obj.editor_id()),
                TES3Object::Race(obj)             => ( 5, obj.sort_hint(), obj.editor_id()),
                TES3Object::Sound(obj)            => ( 6, obj.sort_hint(), obj.editor_id()),
                TES3Object::Skill(obj)            => ( 7, obj.sort_hint(), obj.editor_id()),
                TES3Object::MagicEffect(obj)      => ( 8, obj.sort_hint(), obj.editor_id()),
                TES3Object::Script(obj)           => ( 9, obj.sort_hint(), obj.editor_id()),
                TES3Object::Region(obj)           => (10, obj.sort_hint(), obj.editor_id()),
                TES3Object::Birthsign(obj)        => (11, obj.sort_hint(), obj.editor_id()),
                TES3Object::StartScript(obj)      => (12, obj.sort_hint(), obj.editor_id()),
                TES3Object::LandscapeTexture(obj) => (13, obj.sort_hint(), obj.editor_id()),
                TES3Object::Spell(obj)            => (14, obj.sort_hint(), obj.editor_id()),
                TES3Object::Static(obj)           => (15, obj.sort_hint(), obj.editor_id()),
                TES3Object::Door(obj)             => (16, obj.sort_hint(), obj.editor_id()),
                TES3Object::MiscItem(obj)         => (17, obj.sort_hint(), obj.editor_id()),
                TES3Object::Weapon(obj)           => (18, obj.sort_hint(), obj.editor_id()),
                TES3Object::Container(obj)        => (19, obj.sort_hint(), obj.editor_id()),
                TES3Object::Creature(obj)         => (20, obj.sort_hint(), obj.editor_id()),
                TES3Object::Bodypart(obj)         => (21, obj.sort_hint(), obj.editor_id()),
                TES3Object::Light(obj)            => (22, obj.sort_hint(), obj.editor_id()),
                TES3Object::Enchanting(obj)       => (23, obj.sort_hint(), obj.editor_id()),
                TES3Object::Npc(obj)              => (24, obj.sort_hint(), obj.editor_id()),
                TES3Object::Armor(obj)            => (25, obj.sort_hint(), obj.editor_id()),
                TES3Object::Clothing(obj)         => (26, obj.sort_hint(), obj.editor_id()),
                TES3Object::RepairItem(obj)       => (27, obj.sort_hint(), obj.editor_id()),
                TES3Object::Activator(obj)        => (28, obj.sort_hint(), obj.editor_id()),
                TES3Object::Apparatus(obj)        => (29, obj.sort_hint(), obj.editor_id()),
                TES3Object::Lockpick(obj)         => (30, obj.sort_hint(), obj.editor_id()),
                TES3Object::Probe(obj)            => (31, obj.sort_hint(), obj.editor_id()),
                TES3Object::Ingredient(obj)       => (32, obj.sort_hint(), obj.editor_id()),
                TES3Object::Book(obj)             => (33, obj.sort_hint(), obj.editor_id()),
                TES3Object::Alchemy(obj)          => (34, obj.sort_hint(), obj.editor_id()),
                TES3Object::LeveledItem(obj)      => (35, obj.sort_hint(), obj.editor_id()),
                TES3Object::LeveledCreature(obj)  => (36, obj.sort_hint(), obj.editor_id()),
                TES3Object::Cell(obj)             => (37, obj.sort_hint(), obj.editor_id()),
                TES3Object::Landscape(obj)        => (38, obj.sort_hint(), obj.editor_id()),
                TES3Object::PathGrid(obj)         => (39, obj.sort_hint(), obj.editor_id()),
                TES3Object::SoundGen(obj)         => (40, obj.sort_hint(), obj.editor_id()),
                TES3Object::Dialogue(obj)         => (41, obj.sort_hint(), "".into()), // Preserve DIAL/INFO order
                TES3Object::DialogueInfo(obj)     => (41, obj.sort_hint(), "".into()), // ^
            }
        });
        unsafe { apply_isort(&mut indices, &mut self.objects) };

        // sort inventories
        for object in &mut self.objects {
            let inventory = match object {
                TES3Object::Npc(obj) => &mut obj.inventory,
                TES3Object::Creature(obj) => &mut obj.inventory,
                TES3Object::Container(obj) => &mut obj.inventory,
                _ => continue,
            };
            isort(&mut indices, inventory, |(_, name)| name.as_str());
            unsafe { apply_isort(&mut indices, inventory) };
        }
    }
}

/// Internal helper trait for [`Plugin::sort_objects`] implementation.
/// Some types override the method to influence sort logic.
trait SortHint: Sized {
    fn sort_hint(&self) -> i64;
}

#[allow(clippy::cast_lossless)]
const _: () = {
    impl<T> SortHint for &T {
        fn sort_hint(&self) -> i64 {
            0
        }
    }

    impl SortHint for Cell {
        fn sort_hint(&self) -> i64 {
            let (x, y) = self.exterior_coords().unwrap_or_default();
            ((x as i64) << 32) | (y as i64)
        }
    }

    impl SortHint for PathGrid {
        fn sort_hint(&self) -> i64 {
            let (x, y) = self.data.grid;
            ((x as i64) << 32) | (y as i64)
        }
    }

    impl SortHint for Landscape {
        fn sort_hint(&self) -> i64 {
            let (x, y) = self.grid;
            ((x as i64) << 32) | (y as i64)
        }
    }

    impl SortHint for LandscapeTexture {
        fn sort_hint(&self) -> i64 {
            self.index as i64
        }
    }

    impl SortHint for MagicEffect {
        fn sort_hint(&self) -> i64 {
            self.effect_id as i64
        }
    }

    impl SortHint for Skill {
        fn sort_hint(&self) -> i64 {
            self.skill_id as i64
        }
    }
};

/// Calculate the `indices` for indirectly sorting `subject` according to `function`.
///
fn isort<'a, T, F, K>(indices: &mut Vec<usize>, subject: &'a [T], function: F)
where
    F: Fn(&'a T) -> K,
    K: Ord,
{
    indices.clear();
    indices.extend(0..subject.len());
    indices.sort_by_key(|&i| function(&subject[i]));
}

/// Sort `subject` in place using the order specified by `indices`.
///
/// Note: use `isort` function to compose a valid `indices` slice.
///
/// Does not allocate, but instead uses `indices` as scratch space.\
/// As such the order of `indices` after completion is unspecified.
///
/// Safety:
///     - Both `indices` and `subject` must be of equal length.
///     - Values of `indices` must be valid for indexing `subject`.
///     - The `indices` slice must not contain any repeated values.
///
unsafe fn apply_isort<T>(indices: &mut [usize], subject: &mut [T]) {
    let this = indices.as_mut_ptr();
    let data = subject.as_mut_ptr();
    for i in 0..indices.len().saturating_sub(1) {
        let mut curr_idx = i;
        loop {
            let next_ptr = this.add(curr_idx);
            let next_idx = *next_ptr;
            *next_ptr = curr_idx;
            if next_idx == i {
                break;
            }
            std::ptr::swap(data.add(curr_idx), data.add(next_idx));
            curr_idx = next_idx;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_identity() {
        let mut indices = vec![0, 1, 2, 3, 4];
        let mut data = ['a', 'b', 'c', 'd', 'e'];
        unsafe { apply_isort(&mut indices, &mut data) };
        assert_eq!(['a', 'b', 'c', 'd', 'e'], data);
    }

    #[test]
    fn sort_reverse() {
        let mut indices = vec![4, 3, 2, 1, 0];
        let mut data = ['a', 'b', 'c', 'd', 'e'];
        unsafe { apply_isort(&mut indices, &mut data) };
        assert_eq!(['e', 'd', 'c', 'b', 'a'], data);
    }

    #[test]
    fn sort_cycle() {
        let mut indices = vec![1, 2, 3, 4, 0];
        let mut data = ['a', 'b', 'c', 'd', 'e'];
        unsafe { apply_isort(&mut indices, &mut data) };
        assert_eq!(['b', 'c', 'd', 'e', 'a'], data);
    }

    #[test]
    fn sort_swap() {
        let mut indices = vec![4, 1, 0, 3, 2];
        let mut data = ['a', 'b', 'c', 'd', 'e'];
        unsafe { apply_isort(&mut indices, &mut data) };
        assert_eq!(['e', 'b', 'a', 'd', 'c'], data);
    }
}
