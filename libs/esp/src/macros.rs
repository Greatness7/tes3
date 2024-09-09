/// A cursed macro that generates a macro!
///
macro_rules! make_delegate {
    ($D:tt $($T:ident)*) => {
        /// A convenience macro repeating code across all `TES3Object` variants.
        ///
        /// Supports two forms:
        ///
        /// 1. Implementing a trait for all `TES3Object` variants.
        ///
        /// ```ignore
        /// delegate! {
        ///     impl MyTrait {
        ///         fn my_method(&self) {
        ///             // code here must be valid for all `TES3Object` variants
        ///         }
        ///     }
        /// }
        /// ```
        ///
        /// 2. Repeating match statements for all `TES3Object` variants.
        ///
        /// ```ignore
        /// delegate! {
        ///    match object {
        ///       inner => {
        ///          // code here must be valid for all `TES3Object` variants
        ///       }
        ///    }
        /// }
        /// ```
        ///
        ///
        #[macro_export]
        macro_rules! delegate {
            (impl $D name:path { $D($D body:item)*} ) => {
                $(
                    impl $D name for $T { $D($D body)* }
                )*
            };
            (match $D object:ident { $D name:ident => $D body:expr $D (,)? }) => {
                match $D object {
                    $(
                        TES3Object::$T($D name) => $D body,
                    )*
                }
            }
        }
    };
}
make_delegate!(
    $
    Header
    GameSetting
    GlobalVariable
    Class
    Faction
    Race
    Sound
    Skill
    MagicEffect
    Script
    Region
    Birthsign
    StartScript
    LandscapeTexture
    Spell
    Static
    Door
    MiscItem
    Weapon
    Container
    Creature
    Bodypart
    Light
    Enchanting
    Npc
    Armor
    Clothing
    RepairItem
    Activator
    Apparatus
    Lockpick
    Probe
    Ingredient
    Book
    Alchemy
    LeveledItem
    LeveledCreature
    Cell
    Landscape
    PathGrid
    SoundGen
    Dialogue
    DialogueInfo
);
