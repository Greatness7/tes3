// internal imports
use crate::prelude::*;

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum FileType {
    #[default]
    Esp = 0,
    Esm = 1,
    Ess = 32,
}

#[esp_meta]
#[repr(u8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum GlobalType {
    #[default]
    Float = b'f',
    Long = b'l',
    Short = b's',
}

#[esp_meta]
#[repr(i32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum AttributeId {
    #[default]
    None = -1,
    Strength = 0,
    Intelligence = 1,
    Willpower = 2,
    Agility = 3,
    Speed = 4,
    Endurance = 5,
    Personality = 6,
    Luck = 7,
}

#[esp_meta]
#[repr(i8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum AttributeId2 {
    #[default]
    None = -1,
    Strength = 0,
    Intelligence = 1,
    Willpower = 2,
    Agility = 3,
    Speed = 4,
    Endurance = 5,
    Personality = 6,
    Luck = 7,
}

#[esp_meta]
#[repr(i32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum Specialization {
    #[default]
    None = -1,
    Combat = 0,
    Magic = 1,
    Stealth = 2,
}

#[esp_meta]
#[repr(i32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum SkillId {
    #[default]
    None = -1,
    Block = 0,
    Armorer = 1,
    MediumArmor = 2,
    HeavyArmor = 3,
    BluntWeapon = 4,
    LongBlade = 5,
    Axe = 6,
    Spear = 7,
    Athletics = 8,
    Enchant = 9,
    Destruction = 10,
    Alteration = 11,
    Illusion = 12,
    Conjuration = 13,
    Mysticism = 14,
    Restoration = 15,
    Alchemy = 16,
    Unarmored = 17,
    Security = 18,
    Sneak = 19,
    Acrobatics = 20,
    LightArmor = 21,
    ShortBlade = 22,
    Marksman = 23,
    Mercantile = 24,
    Speechcraft = 25,
    HandToHand = 26,
}

#[esp_meta]
#[repr(i8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum SkillId2 {
    #[default]
    None = -1,
    Block = 0,
    Armorer = 1,
    MediumArmor = 2,
    HeavyArmor = 3,
    BluntWeapon = 4,
    LongBlade = 5,
    Axe = 6,
    Spear = 7,
    Athletics = 8,
    Enchant = 9,
    Destruction = 10,
    Alteration = 11,
    Illusion = 12,
    Conjuration = 13,
    Mysticism = 14,
    Restoration = 15,
    Alchemy = 16,
    Unarmored = 17,
    Security = 18,
    Sneak = 19,
    Acrobatics = 20,
    LightArmor = 21,
    ShortBlade = 22,
    Marksman = 23,
    Mercantile = 24,
    Speechcraft = 25,
    HandToHand = 26,
}

#[esp_meta]
#[repr(i32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum EffectId {
    #[default]
    None = -1,
    WaterBreathing = 0,
    SwiftSwim = 1,
    WaterWalking = 2,
    Shield = 3,
    FireShield = 4,
    LightningShield = 5,
    FrostShield = 6,
    Burden = 7,
    Feather = 8,
    Jump = 9,
    Levitate = 10,
    SlowFall = 11,
    Lock = 12,
    Open = 13,
    FireDamage = 14,
    ShockDamage = 15,
    FrostDamage = 16,
    DrainAttribute = 17,
    DrainHealth = 18,
    DrainMagicka = 19,
    DrainFatigue = 20,
    DrainSkill = 21,
    DamageAttribute = 22,
    DamageHealth = 23,
    DamageMagicka = 24,
    DamageFatigue = 25,
    DamageSkill = 26,
    Poison = 27,
    WeaknessToFire = 28,
    WeaknessToFrost = 29,
    WeaknessToShock = 30,
    WeaknessToMagicka = 31,
    WeaknessToCommonDisease = 32,
    WeaknessToBlightDisease = 33,
    WeaknessToCorprus = 34,
    WeaknessToPoison = 35,
    WeaknessToNormalWeapons = 36,
    DisintegrateWeapon = 37,
    DisintegrateArmor = 38,
    Invisibility = 39,
    Chameleon = 40,
    Light = 41,
    Sanctuary = 42,
    NightEye = 43,
    Charm = 44,
    Paralyze = 45,
    Silence = 46,
    Blind = 47,
    Sound = 48,
    CalmHumanoid = 49,
    CalmCreature = 50,
    FrenzyHumanoid = 51,
    FrenzyCreature = 52,
    DemoralizeHumanoid = 53,
    DemoralizeCreature = 54,
    RallyHumanoid = 55,
    RallyCreature = 56,
    Dispel = 57,
    SoulTrap = 58,
    Telekinesis = 59,
    Mark = 60,
    Recall = 61,
    DivineIntervention = 62,
    AlmsiviIntervention = 63,
    DetectAnimal = 64,
    DetectEnchantment = 65,
    DetectKey = 66,
    SpellAbsorption = 67,
    Reflect = 68,
    CureCommonDisease = 69,
    CureBlightDisease = 70,
    CureCorprus = 71,
    CurePoison = 72,
    CureParalyzation = 73,
    RestoreAttribute = 74,
    RestoreHealth = 75,
    RestoreMagicka = 76,
    RestoreFatigue = 77,
    RestoreSkill = 78,
    FortifyAttribute = 79,
    FortifyHealth = 80,
    FortifyMagicka = 81,
    FortifyFatigue = 82,
    FortifySkill = 83,
    FortifyMagickaMultiplier = 84,
    AbsorbAttribute = 85,
    AbsorbHealth = 86,
    AbsorbMagicka = 87,
    AbsorbFatigue = 88,
    AbsorbSkill = 89,
    ResistFire = 90,
    ResistFrost = 91,
    ResistShock = 92,
    ResistMagicka = 93,
    ResistCommonDisease = 94,
    ResistBlightDisease = 95,
    ResistCorprus = 96,
    ResistPoison = 97,
    ResistNormalWeapons = 98,
    ResistParalysis = 99,
    RemoveCurse = 100,
    TurnUndead = 101,
    SummonScamp = 102,
    SummonClannfear = 103,
    SummonDaedroth = 104,
    SummonDremora = 105,
    SummonGhost = 106,
    SummonSkeleton = 107,
    SummonLeastBonewalker = 108,
    SummonGreaterBonewalker = 109,
    SummonBonelord = 110,
    SummonTwilight = 111,
    SummonHunger = 112,
    SummonGoldenSaint = 113,
    SummonFlameAtronach = 114,
    SummonFrostAtronach = 115,
    SummonStormAtronach = 116,
    FortifyAttackBonus = 117,
    CommandCreature = 118,
    CommandHumanoid = 119,
    BoundDagger = 120,
    BoundLongsword = 121,
    BoundMace = 122,
    BoundBattleAxe = 123,
    BoundSpear = 124,
    BoundLongbow = 125,
    ExtraSpell = 126,
    BoundCuirass = 127,
    BoundHelm = 128,
    BoundBoots = 129,
    BoundShield = 130,
    BoundGloves = 131,
    Corprus = 132,
    Vampirism = 133,
    SummonCenturionSphere = 134,
    SunDamage = 135,
    StuntedMagicka = 136,
    SummonFabricant = 137,
    SummonWolf = 138,
    SummonBear = 139,
    SummonBoneWolf = 140,
    Summon04 = 141,
    Summon05 = 142,
}

#[esp_meta]
#[repr(i16)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum EffectId2 {
    #[default]
    None = -1,
    WaterBreathing = 0,
    SwiftSwim = 1,
    WaterWalking = 2,
    Shield = 3,
    FireShield = 4,
    LightningShield = 5,
    FrostShield = 6,
    Burden = 7,
    Feather = 8,
    Jump = 9,
    Levitate = 10,
    SlowFall = 11,
    Lock = 12,
    Open = 13,
    FireDamage = 14,
    ShockDamage = 15,
    FrostDamage = 16,
    DrainAttribute = 17,
    DrainHealth = 18,
    DrainMagicka = 19,
    DrainFatigue = 20,
    DrainSkill = 21,
    DamageAttribute = 22,
    DamageHealth = 23,
    DamageMagicka = 24,
    DamageFatigue = 25,
    DamageSkill = 26,
    Poison = 27,
    WeaknessToFire = 28,
    WeaknessToFrost = 29,
    WeaknessToShock = 30,
    WeaknessToMagicka = 31,
    WeaknessToCommonDisease = 32,
    WeaknessToBlightDisease = 33,
    WeaknessToCorprus = 34,
    WeaknessToPoison = 35,
    WeaknessToNormalWeapons = 36,
    DisintegrateWeapon = 37,
    DisintegrateArmor = 38,
    Invisibility = 39,
    Chameleon = 40,
    Light = 41,
    Sanctuary = 42,
    NightEye = 43,
    Charm = 44,
    Paralyze = 45,
    Silence = 46,
    Blind = 47,
    Sound = 48,
    CalmHumanoid = 49,
    CalmCreature = 50,
    FrenzyHumanoid = 51,
    FrenzyCreature = 52,
    DemoralizeHumanoid = 53,
    DemoralizeCreature = 54,
    RallyHumanoid = 55,
    RallyCreature = 56,
    Dispel = 57,
    SoulTrap = 58,
    Telekinesis = 59,
    Mark = 60,
    Recall = 61,
    DivineIntervention = 62,
    AlmsiviIntervention = 63,
    DetectAnimal = 64,
    DetectEnchantment = 65,
    DetectKey = 66,
    SpellAbsorption = 67,
    Reflect = 68,
    CureCommonDisease = 69,
    CureBlightDisease = 70,
    CureCorprus = 71,
    CurePoison = 72,
    CureParalyzation = 73,
    RestoreAttribute = 74,
    RestoreHealth = 75,
    RestoreMagicka = 76,
    RestoreFatigue = 77,
    RestoreSkill = 78,
    FortifyAttribute = 79,
    FortifyHealth = 80,
    FortifyMagicka = 81,
    FortifyFatigue = 82,
    FortifySkill = 83,
    FortifyMagickaMultiplier = 84,
    AbsorbAttribute = 85,
    AbsorbHealth = 86,
    AbsorbMagicka = 87,
    AbsorbFatigue = 88,
    AbsorbSkill = 89,
    ResistFire = 90,
    ResistFrost = 91,
    ResistShock = 92,
    ResistMagicka = 93,
    ResistCommonDisease = 94,
    ResistBlightDisease = 95,
    ResistCorprus = 96,
    ResistPoison = 97,
    ResistNormalWeapons = 98,
    ResistParalysis = 99,
    RemoveCurse = 100,
    TurnUndead = 101,
    SummonScamp = 102,
    SummonClannfear = 103,
    SummonDaedroth = 104,
    SummonDremora = 105,
    SummonGhost = 106,
    SummonSkeleton = 107,
    SummonLeastBonewalker = 108,
    SummonGreaterBonewalker = 109,
    SummonBonelord = 110,
    SummonTwilight = 111,
    SummonHunger = 112,
    SummonGoldenSaint = 113,
    SummonFlameAtronach = 114,
    SummonFrostAtronach = 115,
    SummonStormAtronach = 116,
    FortifyAttackBonus = 117,
    CommandCreature = 118,
    CommandHumanoid = 119,
    BoundDagger = 120,
    BoundLongsword = 121,
    BoundMace = 122,
    BoundBattleAxe = 123,
    BoundSpear = 124,
    BoundLongbow = 125,
    ExtraSpell = 126,
    BoundCuirass = 127,
    BoundHelm = 128,
    BoundBoots = 129,
    BoundShield = 130,
    BoundGloves = 131,
    Corprus = 132,
    Vampirism = 133,
    SummonCenturionSphere = 134,
    SunDamage = 135,
    StuntedMagicka = 136,
    SummonFabricant = 137,
    SummonWolf = 138,
    SummonBear = 139,
    SummonBoneWolf = 140,
    Summon04 = 141,
    Summon05 = 142,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum EffectRange {
    #[default]
    OnSelf = 0,
    OnTouch = 1,
    OnTarget = 2,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum EffectSchool {
    #[default]
    Alteration = 0,
    Conjuration = 1,
    Destruction = 2,
    Illusion = 3,
    Mysticism = 4,
    Restoration = 5,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum ApparatusType {
    #[default]
    MortarAndPestle = 0,
    Alembic = 1,
    Calcinator = 2,
    Retort = 3,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum ArmorType {
    #[default]
    Helmet = 0,
    Cuirass = 1,
    LeftPauldron = 2,
    RightPauldron = 3,
    Greaves = 4,
    Boots = 5,
    LeftGauntlet = 6,
    RightGauntlet = 7,
    Shield = 8,
    LeftBracer = 9,
    RightBracer = 10,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum BookType {
    #[default]
    Book = 0,
    Scroll = 1,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum ClothingType {
    #[default]
    Pants = 0,
    Shoes = 1,
    Shirt = 2,
    Belt = 3,
    Robe = 4,
    RightGlove = 5,
    LeftGlove = 6,
    Skirt = 7,
    Ring = 8,
    Amulet = 9,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum CreatureType {
    #[default]
    Normal = 0,
    Daedra = 1,
    Undead = 2,
    Humanoid = 3,
}

#[esp_meta]
#[repr(u16)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum WeaponType {
    #[default]
    ShortBladeOneHand = 0,
    LongBladeOneHand = 1,
    LongBladeTwoClose = 2,
    BluntOneHand = 3,
    BluntTwoClose = 4,
    BluntTwoWide = 5,
    SpearTwoWide = 6,
    AxeOneHand = 7,
    AxeTwoHand = 8,
    MarksmanBow = 9,
    MarksmanCrossbow = 10,
    MarksmanThrown = 11,
    Arrow = 12,
    Bolt = 13,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum SpellType {
    #[default]
    Spell = 0,
    Ability = 1,
    Blight = 2,
    Disease = 3,
    Curse = 4,
    Power = 5,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum EnchantType {
    #[default]
    CastOnce = 0,
    CastOnStrike = 1,
    CastWhenUsed = 2,
    ConstantEffect = 3,
}

#[esp_meta]
#[repr(u8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum BodypartId {
    #[default]
    Head = 0,
    Hair = 1,
    Neck = 2,
    Chest = 3,
    Groin = 4,
    Hand = 5,
    Wrist = 6,
    Forearm = 7,
    UpperArm = 8,
    Foot = 9,
    Ankle = 10,
    Knee = 11,
    UpperLeg = 12,
    Clavicle = 13,
    Tail = 14,
}

#[esp_meta]
#[repr(u8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum BodypartType {
    #[default]
    Skin = 0,
    Clothing = 1,
    Armor = 2,
}

#[esp_meta]
#[repr(u8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum BipedObjectType {
    #[default]
    Head = 0,
    Hair = 1,
    Neck = 2,
    Chest = 3,
    Groin = 4,
    Skirt = 5,
    RightHand = 6,
    LeftHand = 7,
    RightWrist = 8,
    LeftWrist = 9,
    Shield = 10,
    RightForearm = 11,
    LeftForearm = 12,
    RightUpperArm = 13,
    LeftUpperArm = 14,
    RightFoot = 15,
    LeftFoot = 16,
    RightAnkle = 17,
    LeftAnkle = 18,
    RightKnee = 19,
    LeftKnee = 20,
    RightUpperLeg = 21,
    LeftUpperLeg = 22,
    RightPauldron = 23,
    LeftPauldron = 24,
    Weapon = 25,
    Tail = 26,
}

#[esp_meta]
#[repr(i8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum Sex {
    #[default]
    Any = -1,
    Male = 0,
    Female = 1,
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum DialogueType {
    #[default]
    Topic = 0,
    Voice = 1,
    Greeting = 2,
    Persuasion = 3,
    Journal = 4,
}

#[esp_meta]
#[repr(u8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum DialogueType2 {
    #[default]
    Topic = 0,
    Voice = 1,
    Greeting = 2,
    Persuasion = 3,
    Journal = 4,
}

#[esp_meta]
#[repr(u8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum FilterType {
    #[default]
    None = b'0',
    Function = b'1',
    Global = b'2',
    Local = b'3',
    Journal = b'4',
    Item = b'5',
    Dead = b'6',
    NotId = b'7',
    NotFaction = b'8',
    NotClass = b'9',
    NotRace = b'A',
    NotCell = b'B',
    NotLocal = b'C',
}

#[rustfmt::skip]
#[esp_meta]
#[repr(u16)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum FilterFunction {
    #[default]
    ReactionLow = 12336,           // b"00",
    ReactionHigh = 12592,          // b"01",
    RankRequirement = 12848,       // b"02",
    Reputation = 13104,            // b"03",
    HealthPercent = 13360,         // b"04",
    PcReputation = 13616,          // b"05",
    PcLevel = 13872,               // b"06",
    PcHealthPercent = 14128,       // b"07",
    PcMagicka = 14384,             // b"08",
    PcFatigue = 14640,             // b"09",
    PcStrength = 12337,            // b"10",
    PcBlock = 12593,               // b"11",
    PcArmorer = 12849,             // b"12",
    PcMediumArmor = 13105,         // b"13",
    PcHeavyArmor = 13361,          // b"14",
    PcBluntWeapon = 13617,         // b"15",
    PcLongBlade = 13873,           // b"16",
    PcAxe = 14129,                 // b"17",
    PcSpear = 14385,               // b"18",
    PcAthletics = 14641,           // b"19",
    PcEnchant = 12338,             // b"20",
    PcDestruction = 12594,         // b"21",
    PcAlteration = 12850,          // b"22",
    PcIllusion = 13106,            // b"23",
    PcConjuration = 13362,         // b"24",
    PcMysticism = 13618,           // b"25",
    PcRestoration = 13874,         // b"26",
    PcAlchemy = 14130,             // b"27",
    PcUnarmored = 14386,           // b"28",
    PcSecurity = 14642,            // b"29",
    PcSneak = 12339,               // b"30",
    PcAcrobatics = 12595,          // b"31",
    PcLightArmor = 12851,          // b"32",
    PcShortBlade = 13107,          // b"33",
    PcMarksman = 13363,            // b"34",
    PcMercantile = 13619,          // b"35",
    PcSpeechcraft = 13875,         // b"36",
    PcHandToHand = 14131,          // b"37",
    PcSex = 14387,                 // b"38",
    PcExpelled = 14643,            // b"39",
    PcCommonDisease = 12340,       // b"40",
    PcBlightDisease = 12596,       // b"41",
    PcClothingModifier = 12852,    // b"42",
    PcCrimeLevel = 13108,          // b"43",
    SameSex = 13364,               // b"44",
    SameRace = 13620,              // b"45",
    SameFaction = 13876,           // b"46",
    FactionRankDifference = 14132, // b"47",
    Detected = 14388,              // b"48",
    Alarmed = 14644,               // b"49",
    Choice = 12341,                // b"50",
    PcIntelligence = 12597,        // b"51",
    PcWillpower = 12853,           // b"52",
    PcAgility = 13109,             // b"53",
    PcSpeed = 13365,               // b"54",
    PcEndurance = 13621,           // b"55",
    PcPersonality = 13877,         // b"56",
    PcLuck = 14133,                // b"57",
    PcCorprus = 14389,             // b"58",
    Weather = 14645,               // b"59",
    PcVampire = 12342,             // b"60",
    Level = 12598,                 // b"61",
    Attacked = 12854,              // b"62",
    TalkedToPc = 13110,            // b"63",
    PcHealth = 13366,              // b"64",
    CreatureTarget = 13622,        // b"65",
    FriendHit = 13878,             // b"66",
    Fight = 14134,                 // b"67",
    Hello = 14390,                 // b"68",
    Alarm = 14646,                 // b"69",
    Flee = 12343,                  // b"70",
    ShouldAttack = 12599,          // b"71",
    Werewolf = 12855,              // b"72",
    WerewolfKills = 13111,         // b"73",
    NotClass = 22595,              // b"CX",
    DeadType = 22596,              // b"DX",
    NotFaction = 22598,            // b"FX",
    ItemType = 22601,              // b"IX",
    JournalType = 22602,           // b"JX",
    NotCell = 22604,               // b"LX",
    NotRace = 22610,               // b"RX",
    NotIdType = 22616,             // b"XX",
    Global = 22630,                // b"fX",
    PcGold = 22636,                // b"lX",
    CompareGlobal = 22578,         // b"2X",
    CompareLocal = 22579,          // b"3X",
    VariableCompare = 22643,       // b"sX",
}

#[esp_meta]
#[repr(u8)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum FilterComparison {
    #[default]
    Equal = b'0',
    NotEqual = b'1',
    Greater = b'2',
    GreaterEqual = b'3',
    Less = b'4',
    LessEqual = b'5',
}

#[esp_meta]
#[repr(u32)]
#[derive(LoadSave, NoUninit, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum SoundGenType {
    #[default]
    LeftFoot = 0,
    RightFoot = 1,
    SwimLeft = 2,
    SwimRight = 3,
    Moan = 4,
    Roar = 5,
    Scream = 6,
    Land = 7,
}
