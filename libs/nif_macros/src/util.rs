#[derive(Clone, Copy, Default)]
pub struct DeterministicState;

impl std::hash::BuildHasher for DeterministicState {
    type Hasher = std::collections::hash_map::DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::default()
    }
}

pub type HashMap<K, V> = std::collections::HashMap<K, V, DeterministicState>;
