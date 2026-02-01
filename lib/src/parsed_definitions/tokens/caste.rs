//! Tags that can be used to define a creature's caste.

use crate::{
    custom_types::{BodySize, Color, Name, TileCharacter},
    parsed_definitions::custom_types::HabitCount,
    tokens::ObjectType,
};

/// Tokens that can be found in a creature's caste definitions.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    strum_macros::EnumIter,
)]
pub enum CasteToken {
    /// Prevents tamed creature from being made available for adoption, instead allowing it to automatically adopt
    /// whoever it wants. The basic requirements for adoption are intact, and the creature will only adopt individuals
    /// who have a preference for their species. Used by cats in the vanilla game. When viewing a tame creature with
    /// this token, the message "This animal isn't interested in your wishes" will appear instead of "This adorable
    /// animal can't work" or "This animal is waiting to be trained".
    ///
    /// Appears as `ADOPTS_OWNER`
    AdoptsOwner,
    /// Makes the creature need alcohol to get through the working day; it will choose to drink booze instead of water
    /// if possible. Going sober for too long reduces speed.
    ///
    /// Appears as `ALCOHOL_DEPENDENT`
    AlcoholDependent,
    /// Sets the creature to be active during the day, night, and twilight in Adventurer Mode. Seems to be a separate
    /// value from `[DIURNAL]/[NOCTURNAL]/[CREPUSCULAR]`, rather than implying them.
    ///
    /// Appears as `ALL_ACTIVE`
    AllActive,
    /// Caste-specific version of `[Creature::AltTile]`. Requires `[Tile]`.
    ///
    /// Appears as `CASTE_ALTTILE:SomeTile`
    AltTile {
        /// The tile to use
        tile: TileCharacter,
    },
    /// Found on `[LargePredator]`s who ambush their prey. Instead of charging relentlessly at prey, the predator will
    /// wait till the prey is within a few squares before charging. May or may not work on other creatures.
    ///
    /// Appears as `AMBUSHPREDATOR`
    AmbushPredator,
    /// Allows a creature to breathe both in and out of water (unlike `[Aquatic]`) - does not prevent drowning in magma.
    ///
    /// Appears as `AMPHIBIOUS`
    Amphibious,
    /// Applies the specified creature variation with the given arguments to the creature.
    ///
    /// Appears as `APPLY_CREATURE_VARIATION:SOME_VARIATION` or `APPLY_CREATURE_VARIATION:SOME_VARIATION:ARG1:ARG2:ARG3`
    ApplyCreatureVariation {
        /// Creature variation ID to apply
        id: String,
        /// (Optional) any number of arguments to pass to the creature variation
        args: Vec<u32>,
    },
    /// Applies the effects of all pending `[CV_ADD_TAG]` and `[CV_REMOVE_TAG]` tokens that have been defined in the
    /// current creature (so far).
    ///
    /// Appears as `APPLY_CURRENT_CREATURE_VARIATION`
    ApplyCurrentCreatureVariation,
    /// Enables the creature to breathe in water, but causes it to air-drown on dry land.
    ///
    /// Appears as `AQUATIC`
    Aquatic,
    /// Causes the creature to be excluded from the object testing arena's creature spawning list. Typically applied to
    /// spoileriffic creatures.
    ///
    /// Appears as `ARENA_RESTRICTED`
    ArenaRestricted,
    /// Prevents the creature from attacking or frightening creatures with the `[Natural]` tag.
    ///
    /// Appears as `AT_PEACE_WITH_WILDLIFE`
    AtPeaceWithWildlife,
    /// Defines the attack name, and the body part used.
    ///
    /// Appears as `ATTACK:NAME:BODYPART:BY_CATEGORY:HORN` or similar
    Attack {
        /// The verb for the attack
        verb: String,
        /// The body part selector used for the attack
        selector: Vec<String>,
    },
    /// Specifies when a megabeast or semi-megabeast will attack the fortress. The attacks will start occurring when at
    /// least one of the requirements is met. Setting a value to 0 disables the trigger.
    ///
    /// Appears as `ATTACK_TRIGGER:0:1:2`
    AttackTrigger {
        /// Population trigger
        population: u32,
        /// Exported wealth trigger
        exported_wealth: u32,
        /// Created wealth trigger
        created_wealth: u32,
    },
    /// Age at which creature is considered a child, the default is zero. One can think of this as the duration of the
    /// baby stage.
    ///
    /// Appears as `BABY:12`
    Baby {
        /// The age at which the creature is considered a child
        age: u32,
    },
    /// Defines a new name for a creature in baby state at the caste level. For non-caste-specific baby names, see
    /// [`crate::tokens::CreatureTag::GeneralBabyName`].
    ///
    /// Appears as `BABYNAME:SomeName:SomeNames`
    BabyName {
        /// Name of the baby
        name: Name,
    },
    /// Creature may be subject to beaching, becoming stranded on shores, where they will eventually air-drown. The
    /// number indicates the frequency of the occurrence. Presumably requires the creature to be `[Aquatic]`. Used by
    /// orcas, sperm whales and sea nettle jellyfish in the vanilla game.
    ///
    /// Appears as `BEACH_FREQUENCY:100`
    BeachFrequency {
        /// Frequency of beaching
        frequency: u32,
    },
    /// The creature is non-aggressive by default, and will never automatically be engaged by companions or soldiers,
    /// running away from any creatures that are not friendly to it, and will only defend itself if it becomes enraged.
    /// Can be thought of as the counterpoint of the `[LargePredator]` tag.
    ///
    /// When tamed, animals with this tag will be useless for fortress defense.
    ///
    /// Appears as `BENIGN`
    Benign,
    /// Specifies what the creature's blood is made of.
    ///
    /// Appears as `BLOOD:SomeMaterial:SubMaterial?:SomeToken`
    Blood {
        /// Blood material
        material: Vec<String>,
        /// Blood token
        state: String,
    },
    /// Causes vampire-like behavior; the creature will suck the blood of unconscious victims when its thirst for blood
    /// grows sufficiently large. When controlling the creature in adventure mode, this can be done at will. Seems to be
    /// required to make the creature denouncable (in-world) as a creature of the night.
    ///
    /// Appears as `BLOODSUCKER`
    BloodSucker,
    /// Draws body parts from `OBJECT:BODY` files (such as `body_default.txt`)
    ///
    /// Appears as `BODY:BODY_WITH_HEAD_FLAG:HEART:GUTS:BRAIN:MOUTH`
    Body {
        /// Body parts arguments
        body_parts: Vec<String>,
    },
    /// These body modifiers give individual creatures different characteristics. In the case of `HEIGHT`, `BROADNESS`
    /// and `LENGTH`, the modifier is also a percentage change to the `BODY_SIZE` of the individual creature. The seven
    /// numbers afterward give a distribution of ranges. Each interval has an equal chance of occurring.
    ///
    /// Example: `BODY_APPEARANCE_MODIFIER:HEIGHT:90:95:98:100:102:105:110`
    ///
    /// * `HEIGHT`: marks the height to be changed
    /// * `90:95:98:100:102:105:110`: sets the range from the shortest (90% of the average height) to the
    ///   tallest (110% of the average height) creature variation.
    BodyAppearanceModifier {
        /// Body part to modify
        attribute: String,
        /// Range of values, spread from lowest to median to highest
        values: [u32; 7],
    },
    /// Loads a plan from listed `OBJECT:BODY_DETAIL_PLAN` files, such as `b_detail_plan_default.txt`. Mass applies
    /// `USE_MATERIAL_TEMPLATE`, mass alters RELSIZE, alters body part positions, and will allow tissue layers to be
    /// defined. Tissue layers are defined in order of skin to bone here.
    ///
    /// Example: `BODY_DETAIL_PLAN:VERTEBRATE_TISSUE_LAYERS:SKIN:FAT:MUSCLE:BONE:CARTILAGE`
    ///
    /// This creates the detailed body of a fox, the skin, fat, muscle, bones and cartilage out of the vertebrate
    /// tissues. A maggot would only need:
    ///
    /// `BODY_DETAIL_PLAN:EXOSKELETON_TISSUE_LAYERS:SKIN:FAT:MUSCLE`
    BodyDetailPlan {
        /// Body detail plan to load
        body_plan: String,
        /// Body detail plan arguments
        arguments: Vec<String>,
    },
    /// Sets size at a given age. Size is in cubic centimeters, and for normal body materials, is roughly equal to the
    /// creature's average weight in grams.
    ///
    /// Appears as `BODY_SIZE:0:0:1000`
    BodySize {
        /// The body size descriptor
        size: BodySize,
    },
    /// Substitutes body part text with replacement text. Draws gloss information from `OBJECT:BODY`files
    /// (such as `body_default.txt`)
    ///
    /// Appears as `BODYGLOSS:SomeGloss`
    BodyGloss {
        /// The gloss to use on the body part
        gloss: String,
    },
    /// Creature eats bones. Implies `[Carnivore]`. Adventurers with this token are currently unable to eat bones.
    ///
    /// Appears as `BONECARN`
    BoneCarn,
    /// Adds a type to a body part - used with `[SetBodyPartGroup]`. In vanilla DF, this is used for adding the type
    /// `[Geldable]` to the lower body of certain creatures.
    ///
    /// Appears as `BP_ADD_TYPE:SomeBodyPartType`
    BodyPartAddType {
        /// The body part type to add
        body_part_type: String,
    },
    /// Sets up the breadth of possibilities for appearance qualities for a selected `BP` group. e.g.:
    ///
    /// * Eyes (`CLOSE_SET`, `DEEP_SET`, `ROUND_VS_NARROW`, `LARGE_IRIS`)
    /// * Lips (`THICKNESS`)
    /// * Nose (`BROADNESS`, `LENGTH`, `UPTURNED`, `CONVEX`)
    /// * Ear (`SPLAYED_OUT`, `HANGING_LOBES`, `BROADNESS`, `HEIGHT`)
    /// * Tooth (`GAPS`)
    /// * Skull (`HIGH_CHEEKBONES`, `BROAD_CHIN`, `JUTTING CHIN`, `SQUARE_CHIN`)
    /// * Neck (`DEEP_VOICE`, `RASPY_VOICE`)
    /// * Head (`BROADNESS`, `HEIGHT`)
    ///
    /// Appears as `BP_APPEARANCE_MODIFIER:SomeQuality:0:0:0:0:0:0:0`
    BodyPartAppearanceModifier {
        /// The quality that can appear
        quality: String,
        /// The spread of the quality, from lowest to median to highest
        spread: [u32; 7],
    },
    /// Removes a type from a body part. Used with `[SetBodyPartGroup]`.
    ///
    /// Appears as `BP_REMOVE_TYPE:SomeBodyPartType`
    BodyPartRemoveType {
        /// The body part type to remove
        body_part_type: String,
    },
    /// Allows a creature to destroy furniture and buildings. Value `1` targets mostly doors, hatches, furniture and
    /// the like. Value `2` targets anything not made with the b + C commands.
    ///
    /// Appears as `BUILDINGDESTROYER:1`
    BuildingDestroyer {
        /// Whether the creature focuses on doors, hatches, furniture, etc. (`1`) or anything not made with
        /// the b + C commands (`2`)
        door_and_furniture_focused: bool,
    },
    /// The creature can perform an interaction.
    ///
    /// Appears as `CAN_DO_INTERACTION:SomeInteraction`
    CanDoInteraction {
        /// Interaction to allow
        interaction: String,
    },
    /// The creature gains skills and can have professions. If a member of a civilization (even a pet) has this token,
    /// it'll need to eat, drink and sleep. Note that this token makes the creature unable to be eaten by an adventurer,
    /// so it is not recommended for uncivilized monsters. Adventurers lacking this token can allocate but not increase
    /// attributes and skills. Skills allocated will disappear on start.
    ///
    /// Appears as `CAN_LEARN`
    CanLearn,
    /// Can talk. Note that this is not necessary for a creature to gain social skills.
    ///
    /// Appears as `CAN_SPEAK`
    CanSpeak,
    /// Creature cannot climb, even if it has free grasp parts.
    ///
    /// Appears as `CANNOT_CLIMB`
    CannotClimb,
    /// Creature cannot jump.
    ///
    /// Appears as `CANNOT_JUMP`
    CannotJump,
    /// Acts like `[NotLiving]`, except that `[OpposedToLife]` creatures will attack them.
    ///
    /// Appears as `CANNOT_UNDEAD`
    CannotUndead,
    /// Allows the creature to open doors that are set to be unpassable for pets. In adventure mode, creatures lacking
    /// this token will be unable to pass through door tiles except whilst these are occupied by other creatures. Not
    /// currently useful in Fortress mode as doors can no longer be set unpassable for pets.
    ///
    /// Appears as `CANOPENDOORS`
    CanOpenDoors,
    /// Creature only eats meat. If the creature goes on rampages in worldgen, it will often devour the
    /// people/animals it kills.
    /// Does not seem to affect the diet of the adventurer in adventure mode.
    ///
    /// Appears as `CARNIVORE`
    Carnivore,
    /// Gives the creature a bonus in caves. Also causes cave adaptation.
    ///
    /// Appears as `CAVE_ADAPT`
    CaveAdaptation,
    /// Multiplies body size by a factor of (integer)%. 50 halves size, 200 doubles.
    ///
    /// Appears as `CHANGE_BODY_SIZE_PERC:100`
    ChangeBodySizePercent {
        /// The percentage to change the body size by
        percent: u32,
    },
    /// Age at which creature is considered an adult - one can think of this as the duration of the child stage.
    /// Allows the creature's offspring to be rendered fully tame if trained during their childhood.
    ///
    /// Appears as `CHILD:12`
    Child {
        /// The age at which the creature is considered an adult
        age: u32,
    },
    /// Defines a name for the creature in child state at the caste level. For non-caste-specific child names, see
    /// `[crate::tokens::CreatureToken::GeneralChildName]`.
    ///
    /// Appears as `CHILDNAME:SomeName:SomeNames`
    ChildName {
        /// Name of the child
        name: Name,
    },
    /// Number of eggs laid in one sitting.
    ///
    /// Appears as `CLUTCH_SIZE:1:1`
    ClutchSize {
        /// Minimum number of eggs laid in one sitting
        min: u32,
        /// Maximum number of eggs laid in one sitting
        max: u32,
    },
    /// Caste-specific color
    ///
    /// Arguments:
    ///
    /// * `foreground`: The foreground color
    /// * `background`: The background color
    /// * `brightness`: The brightness of the color
    ///
    /// Appears as `CASTE_COLOR:0:0:0`
    Color {
        /// Color
        color: Color,
    },
    /// When combined with any of `[Pet]`, `[PackAnimal]`, `[WagonPuller]` and/or `[Mount]`, the creature is guaranteed
    /// to be domesticated by any civilization with `[crate::tokens::EntityToken::CommonDomesticPet]`,
    /// `[crate::tokens::EntityToken::CommonDomesticPackAnimal]`, `[crate::tokens::EntityToken::CommonDomesticWagonPuller]`
    /// and/or `[crate::tokens::EntityToken::CommonDomesticMount]` respectively. Such civilizations will always have
    /// access to the creature, even in the absence of wild populations. This token is invalid on
    /// `[crate::tokens::CreatureToken::Fanciful]` creatures.
    ///
    /// Appears as `COMMON_DOMESTIC`
    CommonDomestic,
    /// Creatures of this caste's species with the [`CasteTag::SpouseConverter`] and [`CasteTag::NightCreatureHunter`] tokens will kidnap
    /// `[SpouseConversionTarget]`s of an appropriate
    /// sex and convert them into castes with `CONVERTED_SPOUSE`.
    ///
    /// Appears as `CONVERTED_SPOUSE`
    ConvertedSpouse,
    /// Set this to allow the creature to be cooked in meals without first being butchered/cleaned. Used by some water-dwelling vermin such as mussels, nautiluses and oysters.
    ///
    /// Appears as `COOKABLE_LIVE`
    CookableLive,
    /// Creature is 'berserk' and will attack all other creatures, except members of its own species that also have the CRAZED tag. It will show Berserk in the unit list.
    /// Berserk creatures go on rampages during worldgen much more frequently than non-berserk ones.
    ///
    /// Appears as `CRAZED`
    Crazed,
    /// An arbitrary creature classification. Can be set to anything, but the only vanilla uses are `GENERAL_POISON` (used in syndromes), `EDIBLE_GROUND_BUG`
    /// (used as targets for `[GobbleVerminClass]`), `MAMMAL`, and `POISONOUS` (both used for kobold pet eligibility). A single creature can have multiple classes.
    ///
    /// Appears as `CREATURE_CLASS:SomeClass`
    CreatureClass {
        /// The creature class
        class: String,
    },
    /// Sets the creature to be active at twilight in adventurer mode.
    ///
    /// Appears as `CREPUSCULAR`
    Crepuscular,
    /// Allows a creature to steal and eat edible items from a site. It will attempt to grab a food item and immediately make its way to the map's edge,
    /// where it will disappear with it. If the creature goes on rampages during worldgen, it will often steal food instead of attacking. Trained and tame instances
    /// of the creature will no longer display this behavior.
    ///
    /// Appears as `CURIOUSBEAST_EATER`
    CuriousBeastEater,
    /// Allows a creature to (very quickly) drink your alcohol. Or spill the barrel to the ground. Also affects undead versions of the creature. Unlike food or item thieves,
    /// drink thieves will consume your alcohol on the spot rather than run away with one piece of it. Trained and tame instances of the creature will no longer display this behavior.
    ///
    /// Appears as `CURIOUSBEAST_GUZZLER`
    CuriousBeastGuzzler,
    /// Allows a creature to steal things (apparently, of the highest value it can find). It will attempt to grab an item of value and immediately make its way to the map's edge,
    /// where it will disappear with it. If a creature with any of the CURIOUSBEAST tokens carries anything off the map, even if it is a caravan's pack animal, it will be reported
    /// as stealing everything it carries. If the creature goes on rampages in worldgen, it will often steal items instead of attacking - kea birds are infamous for this.
    /// Trained and tame instances of the creature will no longer display this behavior. Also, makes the creature unable to drop hauled items until it enters combat.
    ///
    /// Appears as `CURIOUSBEAST_ITEM`
    CuriousBeastItem,
    /// Adds a tag. Used in conjunction with creature variation templates.
    ///
    /// Appears as `CV_ADD_TAG:SomeTag`
    CreatureVariationAddTag {
        /// The tag to add
        tag: String,
    },
    /// Removes a tag. Used in conjunction with creature variation templates.
    ///
    /// Appears as `CV_REMOVE_TAG:SomeTag`
    CreatureVariationRemoveTag {
        /// The tag to remove
        tag: String,
    },
    /// Found on generated demons. Marks the caste to be used in the initial wave after breaking into the underworld, and by the demon civilizations created during world-gen breachings
    ///
    /// Appears as `DEMON`
    Demon,
    /// A brief description of the creature type, as displayed when viewing the creature's description/thoughts & preferences screen.
    Description {
        /// The description to use
        description: String,
    },
    /// Causes the creature to die upon attacking. Used by honey bees to simulate them dying after using their stingers.
    ///
    /// Appears as `DIE_WHEN_VERMIN_BITE`
    DieWhenVerminBite,
    /// Increases experience gain during adventure mode. Creatures with a difficulty of 11 or higher are not assigned for quests in adventure mode.
    ///
    /// Appears as `DIFFICULTY:10`
    Difficulty {
        /// The difficulty of the creature
        difficulty: u32,
    },
    /// Sets the creature to be active during the day in Adventurer Mode.
    ///
    /// Appears as `DIURNAL`
    Diurnal,
    /// The creature hunts vermin by diving from the air. On tame creatures, it has the same effect as `[HuntsVermin]`. Found on peregrine falcons.
    ///
    /// Appears as `DIVE_HUNTS_VERMIN`
    DiveHuntsVermin,
    /// Defines the item that the creature drops upon being butchered. Used with `[ExtraButcherObject]`.
    ///
    /// Appears as `EBO_ITEM:SomeItem:SomeMaterial`
    ExtraButcherObjectItem {
        /// The item to add
        item: String,
        /// The material of the item
        material: Vec<String>,
    },
    /// The shape of the creature's extra butchering drop. Used with `[ExtraButcherObject]`.
    ///
    /// Appears as `EBO_SHAPE:SomeShape`
    ExtraButcherObjectShape {
        /// The shape to add
        shape: String,
    },
    /// Defines the material composition of eggs laid by the creature. Egg-laying creatures in the default game define this 3 times, using `LOCAL_CREATURE_MAT:EGGSHELL`,
    /// `LOCAL_CREATURE_MAT:EGG_WHITE`, and then `LOCAL_CREATURE_MAT:EGG_YOLK`. Eggs will be made out of eggshell. Edibility is determined by tags on whites or yolk,
    /// but they otherwise do not exist.
    ///
    /// Appears as `EGG_MATERIAL:SomeMaterial:SomeState`
    EggMaterial {
        /// The material to use
        material: Vec<String>,
        /// The state of the material
        state: String,
    },
    /// Determines the size of laid eggs. Doesn't affect hatching or cooking, but bigger eggs will be heavier, and may take longer to be hauled depending on the hauler's strength.
    ///
    /// Appears as `EGG_SIZE:100`
    EggSize {
        /// The size of the egg
        size: u32,
    },
    /// Allows the creature to wear or wield items.
    ///
    /// Appears as `EQUIPS`
    Equips,
    /// The creature drops an additional object when butchered, as defined by `[ExtraButcherObjectItem]` and `[ExtraButcherObjectShape]`.
    /// Used for gizzard stones in default creatures. For some materials, needs to be defined after caste definitions with `SELECT_CASTE:ALL`
    ///
    /// Appears as `EXTRA_BUTCHER_OBJECT`
    ExtraButcherObject {
        /// Details about the extra butcher object
        object_type: String,
        /// Arguments for the extra butcher object
        arguments: Vec<String>,
    },
    /// Defines a creature extract which can be obtained via small animal dissection.
    ///
    /// Appears as `EXTRACT:SomeExtract`
    Extract {
        /// The extract material
        material: String,
    },
    /// The creature can see regardless of whether it has working eyes and has full 360 degree vision, making it impossible to strike the creature from a blind spot
    /// in combat. Invisible creatures will also be seen, namely intelligent undead using a "vanish" power.
    ///
    /// Appears as `EXTRAVISION`
    Extravision,
    /// Found on subterranean animal-man tribals. Currently defunct. In previous versions, it caused these creatures to crawl out of chasms and underground rivers.
    ///
    /// Appears as `FEATURE_ATTACK_GROUP`
    FeatureAttackGroup,
    /// Found on forgotten beasts. Presumably makes it act as such, initiating underground attacks on fortresses, or leads to the pop-up message upon encountering one.
    /// Hides the creature from displaying in a `world_sites_and_pops.txt` file. Does not create historical figures like generated forgotten beasts do.
    ///
    /// Requires specifying a `[Biome]` in which the creature will live, and both surface and subterranean biomes are allowed. Does not stack with `[LargeRoaming]` and if
    /// both are used the creature will not spawn. Appears to be incompatible with `[Demon]` even if used in separate castes.
    ///
    /// Appears as `FEATURE_BEAST`
    FeatureBeast,
    /// Makes the creature biologically female, enabling her to bear young. Usually specified inside a caste.
    ///
    /// Appears as `FEMALE`
    Female,
    /// Makes the creature immune to FIREBALL and FIREJET attacks, and allows it to path through high temperature zones, like lava or fires. Does not, by itself,
    /// make the creature immune to the damaging effects of burning in fire, and does not prevent general heat damage or melting on its own (this would require adjustments
    /// to be made to the creature's body materials - see the dragon raws for an example).
    ///
    /// Appears as `FIREIMMUNE`
    FireImmune,
    /// Like `[FireImmune]`, but also renders the creature immune to `DRAGONFIRE` attacks.
    ///
    /// Appears as `FIREIMMUNE_SUPER`
    FireImmuneSuper,
    /// The creature's corpse is a single `FISH_RAW` food item that needs to be cleaned (into a FISH item) at a fishery to become edible. Before being cleaned the item is
    /// referred to as "raw". The food item is categorized under "fish" on the food and stocks screens, and when uncleaned it is sorted under "raw fish" in the stocks
    /// (but does not show up on the food screen).
    ///
    /// Without this or `[CookableLive]`, fished vermin will turn into food the same way as non-vermin creatures, resulting in multiple units of food (meat, brain, lungs,
    /// eyes, spleen etc.) from a single fished vermin. These units of food are categorized as meat by the game.
    ///
    /// Appears as `FISHITEM`
    FishItem,
    /// The creature's body is constantly at this temperature, heating up or cooling the surrounding area. Alters the temperature of the creature's inventory and all
    /// adjacent tiles, with all the effects that this implies - may trigger wildfires at high enough values. Also makes the creature immune to extreme heat or cold, as
    /// long as the temperature set is not harmful to the materials that the creature is made from. Corpses and body parts of creatures with a fixed temperature maintain
    /// their temperature even after death.
    ///
    /// Note that temperatures of 12000 and higher may cause pathfinding issues in fortress mode.
    ///
    /// Appears as `FIXED_TEMP:10000`
    FixedTemp {
        /// The temperature of the creature
        temperature: i32,
    },
    /// If engaged in combat, the creature will flee at the first sign of resistance. Used by kobolds in the vanilla game.
    ///
    /// Appears as `FLEEQUICK`
    FleeQuick,
    /// Allows a creature to fly, independent of it having wings or not. Fortress Mode pathfinding only partially incorporates flying - flying creatures need a land path
    /// to exist between them and an area in order to access it, but as long as one such path exists, they do not need to use it, instead being able to fly over intervening
    /// obstacles. Winged creatures with this token can lose their ability to fly if their wings are crippled or severed. Winged creatures without this token will be unable
    /// to fly. (A 'wing' in this context refers to any body part with its own FLIER token).
    ///
    /// Appears as `FLIER`
    Flier,
    /// Defines a gait by which the creature can move. Typically defined by using `APPLY_CREATURE_VARIATION:STANDARD_GAIT:xxx` in the creature's raws, instead of
    /// by using this token directly. See `[Gait]` for more detailed information.
    ///
    /// Since it's a bit complicated, we let [`crate::Gait`] `from_value()` handle parsing this token.
    ///
    /// Appears (typically) as `CV_NEW_TAG:GAIT:WALK:Sprint:!ARG4:10:3:!ARG2:50:LAYERS_SLOW:STRENGTH:AGILITY:STEALTH_SLOWS:50`
    Gait {
        /// The value of the token
        gait_values: Vec<String>,
    },
    /// Has the same function as `[MaterialForceMultiplier]`, but applies to all attacks instead of just those involving a specific material. Appears to be overridden by
    /// `[MaterialForceMultiplier]` (werebeasts, for example, use both tokens to provide resistance to all materials, with one exception to which they are especially vulnerable).
    ///
    /// When struck with a weapon made of the any material, the force exerted will be multiplied by A/B.
    ///
    /// Appears as `GENERAL_MATERIAL_FORCE_MULTIPLIER:1:1`
    GeneralMaterialForceMultiplier {
        /// The material to apply the multiplier to
        value_a: u32,
        /// The multiplier to apply
        value_b: u32,
    },
    /// Makes the creature get infections from necrotic tissue.
    ///
    /// Appears as `GETS_INFECTIONS_FROM_ROT`
    GetsInfectionsFromRot,
    /// Makes the creature's wounds become infected if left untreated for too long.
    ///
    /// Appears as `GETS_WOUND_INFECTIONS`
    GetsWoundInfections,
    /// Caste-specific glow color.
    ///
    /// Arguments:
    ///
    /// * `foreground`: The foreground color
    /// * `background`: The background color
    /// * `brightness`: The brightness of the color
    ///
    /// Appears as `CASTE_GLOWCOLOR:0:0:0`
    GlowColor {
        /// Color
        color: Color,
    },
    /// Caste-specific glow tile.
    ///
    /// Appears as `CASTE_GLOWTILE:SomeTile`
    GlowTile {
        /// The tile to use
        tile: TileCharacter,
    },
    /// The creature can and will gnaw its way out of animal traps and cages using the specified verb, depending on the material from which it is made (normally wood).
    ///
    /// Appears as `GNAWER:SomeVerb`
    Gnawer {
        /// The verb to use
        verb: String,
    },
    /// The creature eats vermin of the specified class.
    ///
    /// Appears as `GOBBLE_VERMIN_CLASS:SomeVerminClass`
    GobbleVerminClass {
        /// The vermin class to eat
        vermin_class: String,
    },
    /// The creature eats a specific vermin.
    ///
    /// Appears as `GOBBLE_VERMIN_CREATURE:SomeVerminCreature:SomeVerminCaste`
    GobbleVerminCreature {
        /// The vermin creature to eat
        vermin_creature: String,
        /// The vermin caste to eat
        vermin_caste: String,
    },
    /// The value determines how rapidly grass is trampled when a creature steps on it - a value of 0 causes the creature to never damage grass,
    /// while a value of 100 causes grass to be trampled as rapidly as possible.
    ///
    /// Defaults to 5.
    ///
    /// Appears as `GRASS_TRAMPLE:5`
    GrassTrample {
        /// The trample value
        trample: u32,
    },
    /// Used in Creature Variants. This token changes the adult body size to the average of the old adult body size and the target value and scales all intermediate
    /// growth stages by the same factor.
    ///
    /// Appears as `GRAVITATE_BODY_SIZE:25`
    GravitateBodySize {
        /// The target body size of the creature when it is an adult (fully grown)
        target: u32,
    },
    /// The creature is a grazer - if tamed in fortress mode, it needs a pasture to survive. The higher the number, the less frequently it needs to eat in order to live.
    ///
    /// Not used since 0.40.12, replaced by `[StandardGrazer]` to fix Bug 4113.
    ///
    /// Appears as `GRAZER:100`
    Grazer {
        /// The grazer value
        grazer: u32,
    },
    /// Defines certain behaviors for the creature. The habit types are:
    ///
    /// * `COLLECT_TROPHIES`
    /// * `COOK_PEOPLE`
    /// * `COOK_VERMIN`
    /// * `GRIND_VERMIN`
    /// * `COOK_BLOOD`
    /// * `GRIND_BONE_MEAL`
    /// * `EAT_BONE_PORRIDGE`
    /// * `USE_ANY_MELEE_WEAPON`
    /// * `GIANT_NEST`
    /// * `COLLECT_WEALTH`
    ///
    /// These require the creature to have a `[Lair]` to work properly, and also don't seem to work on creatures who are not a `[SemiMegabeast]`, `[Megabeast]`, or `[NightCreatureHunter]`.
    ///
    /// Appears as `HABIT:SomeHabit`
    Habit {
        /// The habit to add
        habit: String,
    },
    /// "If you set `HABIT_NUM` to a number, it should give you that exact number of habits according to the weights.". All lists of `HABIT`s are preceded by `[HABIT_NUM:TEST_ALL]`
    ///
    /// Appears as `HABIT_NUM:2` or `HABIT_NUM:TEST_ALL`
    HabitNumber {
        /// The number of habits to add. A value of `TEST_ALL` will add all habits and will cause number to be 0.
        number: HabitCount,
    },
    /// The creature has nerves in its muscles. Cutting the muscle tissue can sever motor and sensory nerves.
    ///
    /// Appears as `HAS_NERVES`
    HasNerves,
    /// The creature has a shell. Seemingly no longer used - holdover from previous versions.
    ///
    /// Appears as `HASSHELL`
    HasShell,
    /// Default 'NONE'. The creature's normal body temperature. Creature ceases maintaining temperature on death unlike fixed material temperatures.
    /// Provides minor protection from environmental temperature to the creature.
    ///
    /// Appears as `HOMEOTHERM:10000`
    Homeotherm {
        /// The temperature of the creature, as number or `NONE` (zero) which is the default
        temperature: u32,
    },
    /// Creature hunts and kills nearby vermin, randomly walking between places with food laying on the ground or in stockpiles, to check for possible `[VerminEater]` vermin,
    /// but they'll kill any other vermin too.
    HuntsVermin,
    /// The creature cannot move. Found on sponges. Will also stop a creature from breeding in fortress mode (MALE and FEMALE are affected, if one is IMMOBILE no breeding will happen).
    ///
    /// Appears as `IMMOBILE`
    Immobile,
    /// The creature is immobile while on land. Only works on `[Aquatic]` creatures which can't breathe on land.
    ///
    /// Appears as `IMMOBILE_LAND`
    ImmobileLand,
    /// The creature radiates fire. It will ignite, and potentially completely destroy, items the creature is standing on. Also gives the vermin a high chance of escaping from animal
    /// traps and cages made of any flammable materials (specifically ones that could be ignited by magma).
    ///
    /// Appears as `IMMOLATE`
    Immolate,
    /// Alias for `[CanSpeak]` + `[CanLearn]`.
    ///
    /// Appears as `INTELLIGENT`
    Intelligent,
    /// Specifies interaction details following a `[CanDoInteraction]` token.
    ///
    /// Appears as `[CDI:TYPE:SomeArgs..]`:
    ///
    /// * `[CDI:TOKEN:SPIT]`
    /// * `[CDI:ADV_NAME:Spit]`
    /// * `[CDI:USAGE_HINT:NEGATIVE_SOCIAL_RESPONSE]`
    /// * etc.
    InteractionDetail {
        /// The type of detail described
        label: String,
        /// Arbitrary arguments for the interaction
        args: Vec<String>,
    },
    /// Determines if the creature leaves behind a non-standard corpse (i.e. wood, statue, bars, stone, pool of liquid, etc.). Ethics may prevent actually using the item in jobs or reactions.
    ///
    /// Appears as `ITEMCORPSE:ItemToken:MaterialToken`
    ItemCorpse {
        /// The item token to use
        item: String,
        /// The material token to use
        material: Vec<String>,
    },
    /// The quality of an item-type corpse left behind. Valid values are: 0 for ordinary, 1 for well-crafted, 2 for finely-crafted, 3 for superior, 4 for exceptional, 5 for masterpiece.
    ///
    /// Appears as `ITEMCORPSE_QUALITY:3`
    ItemCorpseQuality {
        /// The quality of the item
        quality: u32,
    },
    /// Found on megabeasts, semimegabeasts, and night creatures. The creature will seek out sites of this type and take them as lairs. The lair types are:
    ///
    /// * `SIMPLE_BURROW`
    /// * `SIMPLE_MOUND`
    /// * `WILDERNESS_LOCATION`
    /// * `SHRINE`
    /// * `LABYRINTH`
    ///
    /// Appears as `LAIR:SomeLair:Probability`
    Lair {
        /// The lair type
        lair: String,
        /// The probability of the lair
        probability: u32,
    },
    /// Defines certain features of the creature's lair. The only valid characteristic is `HAS_DOORS`.
    ///
    /// Appears as `LAIR_CHARACTERISTIC:SomeCharacteristic`
    LairCharacteristic {
        /// The characteristic to add
        characteristic: String,
    },
    /// This creature will actively hunt adventurers in its lair.
    ///
    /// Appears as `LAIR_HUNTER`
    LairHunter,
    /// What this creature says while hunting adventurers in its lair.
    ///
    /// Appears as `LAIR_HUNTER_SPEECH:SomeSpeech`
    LairHunterSpeech {
        /// The file containing what the creature says
        speech_file: String,
    },
    /// Will attack things that are smaller than it (like dwarves). Only one group of "large predators" (possibly two groups on "savage" maps) will appear on any given map.
    /// In adventure mode, large predators will try to ambush and attack you (and your party will attack them back). When tamed, large predators tend to be much more
    /// aggressive to enemies than non-large predators, making them a good choice for an animal army. They may go on rampages in worldgen, and adventurers may receive quests
    /// to kill them. Also, they can be mentioned in the intro paragraph when starting a fortress e.g. "ere the wolves get hungry."
    ///
    /// Appears as `LARGE_PREDATOR`
    LargePredator,
    /// Creature lays eggs instead of giving birth to live young.
    ///
    /// Appears as `LAYS_EGGS`
    LaysEggs,
    /// Creature lays the specified item instead of regular eggs.
    ///
    /// Appears as `LAYS_UNUSUAL_EGGS:SomeItem:SomeMaterial`
    LaysUnusualEggs {
        /// The item to lay
        item: String,
        /// The material of the item
        material: Vec<String>,
    },
    /// The creature has ligaments in its `[CONNECTIVE_TISSUE_ANCHOR]` tissues (bone or chitin by default). Cutting the bone/chitin tissue severs the ligaments,
    /// disabling motor function if the target is a limb.
    ///
    /// Appears as `LIGAMENTS:SomeMaterial:HealingRate`
    Ligaments {
        /// The material to use
        material: Vec<String>,
        /// The healing rate
        healing_rate: u32,
    },
    /// The creature will generate light, such as in adventurer mode at night.
    ///
    /// Appears as `LIGHT_GEN`
    LightGen,
    /// The creature will attack enemies rather than flee from them. This tag has the same effect on player-controlled creatures - including modded dwarves.
    /// Retired as of v0.40.14 in favor of `[LargePredator]`.
    ///
    /// Appears as `LIKES_FIGHTING`
    LikesFighting,
    /// Creature uses "sssssnake talk" (multiplies 'S' when talking - "My name isss Recisssiz."). Used by serpent men and reptile men in the vanilla game.
    /// C's with the same pronunciation (depending on the word) are not affected by this token.
    ///
    /// Appears as `LISP`
    Lisp,
    /// Determines the number of offspring per one birth; default 1-3, not used in vanilla raws.
    ///
    /// Appears as `LITTERSIZE:1:3`
    LitterSize {
        /// The minimum number of offspring
        min: u32,
        /// The maximum number of offspring
        max: u32,
    },
    /// Lets a creature open doors that are set to forbidden in fortress mode.
    ///
    /// Appears as `LOCKPICKER`
    LockPicker,
    /// Determines how well a creature can see in the dark - higher is better. Dwarves have 10000, which amounts to perfect nightvision.
    ///
    /// Appears as `LOW_LIGHT_VISION:10000`
    LowLightVision {
        /// The vision value
        vision: u32,
    },
    /// According to Toady One, this is completely interchangeable with `[AtPeaceWithWildlife]` and might have been used in very early versions of the game by
    /// wandering wizards or the ent-type tree creatures that used to be animated by elves.
    ///
    /// Appears as `MAGICAL`
    Magical,
    /// The creature is able to see while submerged in magma.
    ///
    /// Appears as `MAGMA_VISION`
    MagmaVision,
    /// Makes the creature biologically male.
    ///
    /// Appears as `MALE`
    Male,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_LAUGH`
    MannerismLaugh,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_SMILE`
    MannerismSmile,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_WALK`
    MannerismWalk,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_SIT`
    MannerismSit,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_BREATH`
    MannerismBreath,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_POSTURE`
    MannerismPosture,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_STRETCH`
    MannerismStretch,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_EYELIDS`
    MannerismEyelids,
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_FINGERS:SomeFinger:SomeFingers`
    MannerismFingers {
        /// The finger mannerism to add
        finger: String,
        /// The fingers mannerism to add
        fingers: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_NOSE:SomeNose`
    MannerismNose {
        /// The nose mannerism to add
        nose: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_EAR:SomeEar`
    MannerismEar {
        /// The ear mannerism to add
        ear: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_HEAD:SomeHead`
    MannerismHead {
        /// The head mannerism to add
        head: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_EYES:SomeEyes`
    MannerismEyes {
        /// The eyes mannerism to add
        eyes: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_MOUTH:SomeMouth`
    MannerismMouth {
        /// The mouth mannerism to add
        mouth: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_HAIR:SomeHair`
    MannerismHair {
        /// The hair mannerism to add
        hair: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_KNUCKLES:SomeKnuckles`
    MannerismKnuckles {
        /// The knuckles mannerism to add
        knuckles: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_LIPS:SomeLips`
    MannerismLips {
        /// The lips mannerism to add
        lips: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_CHEEK:SomeCheek`
    MannerismCheek {
        /// The cheek mannerism to add
        cheek: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_NAILS:SomeNails`
    MannerismNails {
        /// The nails mannerism to add
        nails: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_FEET:SomeFeet`
    MannerismFeet {
        /// The feet mannerism to add
        feet: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_ARMS:SomeArms`
    MannerismArms {
        /// The arms mannerism to add
        arms: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded.
    ///
    /// Appears as `MANNERISM_HANDS:SomeHands`
    MannerismHands {
        /// The hands mannerism to add
        hands: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded. Appears to be unused.
    ///
    /// Appears as `MANNERISM_TONGUE:SomeTongue`
    MannerismTongue {
        /// The tongue mannerism to add
        tongue: String,
    },
    /// Adds a possible mannerism to the creature's profile. These are not defined in raws but hardcoded. Appears to be unused.
    ///
    /// Appears as `MANNERISM_LEG:SomeLeg`
    MannerismLeg {
        /// The leg mannerism to add
        leg: String,
    },
    /// Sets the creature to be active at dawn in adventurer mode.
    ///
    /// Appears as `MATUTINAL`
    Matutinal,
    /// Determines the creature's natural lifespan, using the specified minimum and maximum age values (in years). Each individual creature with this token is generated with a
    /// predetermined date (calculated down to the exact tick!) between these values, at which it is destined to die of old age, should it live long enough. Note that the
    /// probability of death at any given age does not increase as the creature gets older.
    ///
    /// Creatures which lack this token are naturally immortal. The `NO_AGING` syndrome tag will prevent death by old age from occurring. Also note that, among civilized creatures,
    /// castes which lack this token will refuse to marry others with it, and vice versa.
    ///
    /// Appears as `MAXAGE:100:150`
    MaxAge {
        /// The minimum age of the creature
        min: u32,
        /// The maximum age of the creature
        max: u32,
    },
    /// Makes the creature slowly stroll around, unless it's in combat or performing a job. If combined with `[CanLearn]`, will severely impact their pathfinding and lead the creature
    /// to move extremely slowly when not performing any task. Problematically applies to animal people based on the animal and war trained animals.
    ///
    /// Appears as `MEANDERER`
    Meanderer,
    /// A 'boss' creature. A small number of the creatures are created during worldgen, their histories and descendants (if any) will be tracked in worldgen (as opposed to simply 'spawning'),
    /// and they will occasionally go on rampages, potentially leading to worship if they attack the same place multiple times. Their presence and number will also influence age names.
    /// When appearing in fortress mode, they will have a pop-up message announcing their arrival. They will remain hostile to military even after being tamed.
    ///
    /// Requires specifying a `[Biome]` in which the creature will live. Subterranean biomes appear to not be allowed. Does stack with `[LargeRoaming]` and if both are used the creature will spawn
    /// as both historical bosses and as wild animals.
    ///
    /// Appears as `MEGABEAST`
    Megabeast,
    /// Default is 200. This means you can increase your attribute to 200% of its starting value (or the average value + your starting value if that is higher).
    ///
    /// Arguments:
    ///
    /// * `attribute`: The attribute to modify
    /// * `percentage`: The percentage to modify the attribute by
    ///
    /// Appears as `MENT_ATT_CAP_PERC:Attribute:200`
    MentalAttributeCapPercentage {
        /// The attribute to modify
        attribute: String,
        /// The percentage to modify the attribute by
        percentage: u32,
    },
    /// Sets up a mental attribute's range of values (0-5000). All mental attribute ranges default to 200:800:900:1000:1100:1300:2000.
    ///
    /// Arguments:
    ///
    /// * `attribute`: The attribute to modify
    /// * `ranges`: The ranges from lowest to highest with 7 steps
    ///
    /// Appears as `MENT_ATT_RANGE:Attribute:200:800:900:1000:1100:1300:2000`
    MentalAttributeRange {
        /// The attribute to modify
        attribute: String,
        /// The ranges from lowest to highest with 7 steps
        ranges: [u32; 7],
    },
    /// Mental attribute gain/decay rates. Lower numbers in the last three slots make decay occur faster. Defaults are 500:4:5:4.
    ///
    /// Arguments:
    ///
    /// * `attribute`: The attribute to modify
    /// * `improvement_cost`: The cost to improve the attribute
    /// * `decay_rate_unused`: The decay rate of the attribute when it is unused
    /// * `decay_rate_rusty`: The decay rate of the attribute when it is rusty
    /// * `decay_rate_demotion`: The decay rate of the attribute when it is demoted
    ///
    /// Appears as `MENT_ATT_RATE:Attribute:500:4:5:4`
    MentalAttributeRate {
        /// The attribute to modify
        attribute: String,
        /// The cost to improve the attribute
        improvement_cost: u32,
        /// The decay rate of the attribute when it is unused
        decay_rate_unused: u32,
        /// The decay rate of the attribute when it is rusty
        decay_rate_rusty: u32,
        /// The decay rate of the attribute when it is demoted
        decay_rate_demotion: u32,
    },
    /// Allows the creature to be milked in the farmer's workshop. The frequency is the amount of ticks the creature needs to "recharge" (i.e. how much time needs to pass before
    /// it can be milked again). Does not work on sentient creatures, regardless of ethics.
    ///
    /// Arguments:
    ///
    /// * `material`: The material of the milk
    /// * `frequency`: The frequency the creature can be milked
    ///
    /// Appears as `MILKABLE:SomeMaterial:1000`
    Milkable {
        /// The material of the milk
        material: Vec<String>,
        /// The frequency the creature can be milked
        frequency: u32,
    },
    /// The creature spawns stealthed and will attempt to path into the fortress, pulling any levers it comes across. It will be invisible on the map and unit list until spotted by a citizen,
    /// at which point the game will pause and recenter on the creature.
    ///
    /// Used by gremlins in the vanilla game. "They go on little missions to mess with various fortress buildings, not just levers."
    ///
    /// Appears as `MISCHIEVOUS` or `MISCHIEVIOUS` (sic)
    Mischievous,
    /// Seemingly no longer used.
    ///
    /// Appears as `MODVALUE:SomeValue`
    ModValue {
        /// The value to modify
        value: String,
    },
    /// Creature may be used as a mount. No use for the player in fortress mode,
    /// but enemy sieging forces may arrive with cavalry. Mounts are usable in adventure mode.
    ///
    /// Appears as `MOUNT`
    Mount,
    /// Creature may be used as a mount, but civilizations cannot domesticate it in worldgen without certain exceptions.
    ///
    /// Appears as `MOUNT_EXOTIC`
    MountExotic,
    /// Allows the creature to have all-around vision as long as it has multiple heads that can see.
    ///
    /// Appears as `MULTIPART_FULL_VISION`
    MultipartFullVision,
    /// Makes the species usually produce a single offspring per birth, with a 1/500
    /// chance of using the `[LITTERSIZE]` as usual. Requires `[FEMALE]`.
    ///
    /// Appears as `MULTIPLE_LITTER_RARE`
    MultipleLitterRare,
    /// Name of the caste
    ///
    /// Arguments:
    ///
    /// * `singular`: The singular name of the caste
    /// * `plural`: The plural name of the caste
    /// * `adjective`: The adjective form of the caste
    ///
    /// Appears as `CASTE_NAME:SomeName:SomeNames:SomeAdjective`
    Name {
        /// Name for the caste
        name: Name,
    },
    /// Animal is considered to be natural. NATURAL animals will not engage creatures tagged with `[AtPeaceWitHWildlife]` in combat unless they are
    /// members of a hostile entity and vice-versa.
    ///
    /// Appears as `NATURAL` or `NATURAL_ANIMAL`
    Natural,
    /// The creature possesses the specified skill at this level inherently - that is, it begins with the skill at this level, and the skill may never
    /// rust below that. A value of 15 is legendary.
    ///
    /// Arguments:
    ///
    /// * `skill`: The skill token to add
    /// * `level`: The level of the skill
    ///
    /// Appears as `NATURAL_SKILL:SomeSkill:15`
    NaturalSkill {
        /// The skill token to add
        skill: String,
        /// The level of the skill
        level: u32,
    },
    /// Creatures with this token can appear in bogeyman ambushes in adventure mode, where they adopt classical bogeyman traits such as stalking the adventurer
    /// and vaporising when dawn breaks. Such traits do not manifest if the creature is encountered outside of a bogeyman ambush (for instance, as a megabeast
    /// or a civilised being). In addition, their corpses and severed body parts turn into smoke after a short while. Note that setting the "Number of Bogeyman Types"
    /// in advanced world generation to 0 will only remove randomly-generated bogeymen.
    ///
    /// Appears as `NIGHT_CREATURE_BOGEYMAN`
    NightCreatureBogeyman,
    /// Found on some necromancers. Creatures with this tag may periodically "perform horrible experiments" offscreen, during which they can use creature-targeting
    /// interactions with an `[I_SOURCE:EXPERIMENT]` tag on living creatures in their area. Worlds are generated with a list of procedurally-generated experiments,
    /// allowing necromancers to turn living people and animals into ghouls and other experimental creatures, and these will automatically be available to all experimenters;
    /// it does not appear possible to prevent this. You can mod in your own custom experiment interactions, but these are used very infrequently due to the large number
    /// of generated experiments.
    ///
    /// Appears as `NIGHT_CREATURE_EXPERIMENTER`
    NightCreatureExperimenter,
    /// Found on night trolls and werebeasts. Implies that the creature is a night creature, and shows its description in legends mode entry. The creature is always hostile and
    /// will start no quarter combat with any nearby creatures, except for members of its own race. Note that this tag does not override the creature's normal behavior in fortress
    /// mode except for the aforementioned aggression, and doesn't prevent the creature from fleeing the battles it started. It also removes the creature's materials from stockpile
    /// settings list, making them be stored there regardless of settings.
    ///
    /// Does stack with `[LARGE_ROAMING]` and if both are used the creature will spawn as both historical hunters and as wild animals; this requires specifying a `[BIOME]` in which the
    /// creature will live, and subterranean biomes are allowed.
    ///
    /// This tag causes the usual behaviour of werebeasts in worldgen, that is, fleeing towns upon being cursed and conducting raids from a lair. If this tag is absent from a deity
    /// curse, the accursed will simply be driven out of towns in a similar manner to vampires. When paired with `SPOUSE_CONVERTER`, a very small population of the creature will be
    /// created during worldgen (sometimes only a single individual will be created), and their histories will be tracked (that is, they will not spawn spontaneously later, they must
    /// either have children or convert other creatures to increase their numbers). The creature will settle in a lair and go on rampages during worldgen. It will actively attempt to
    /// seek out potential conversion targets to abduct, convert, and have children with (if possible).
    ///
    /// Appears as `NIGHT_CREATURE_HUNTER`
    NightCreatureHunter,
    /// Found on nightmares. Corpses and severed body parts derived from creatures with this token turn into smoke after a short while.
    ///
    /// Appears as `NIGHT_CREATURE_NIGHTMARE`
    NightCreatureNightmare,
    /// Creature doesn't require connected body parts to move; generally used on undead creatures with connections that have rotted away.
    ///
    /// Appears as `NO_CONNECTIONS_FOR_MOVEMENT`
    NoConnectionsForMovement,
    /// Creature cannot become dizzy.
    ///
    /// Appears as `NO_DIZZINESS`
    NoDizziness,
    /// Creature does not need to drink.
    ///
    /// Appears as `NO_DRINK`
    NoDrink,
    /// Creature does not need to eat.
    ///
    /// Appears as `NO_EAT`
    NoEat,
    /// The creature caste does not appear in autumn.
    ///
    /// Appears as `NO_AUTUMN`
    NoFall,
    /// Creature cannot suffer fevers.
    ///
    /// Appears as `NO_FEVERS`
    NoFevers,
    /// The creature is biologically sexless. Makes the creature unable to breed.
    ///
    /// Appears as `NO_GENDER`
    NoGender,
    /// The creature cannot raise any physical attributes.
    ///
    /// Appears as `NO_PHYS_ATT_GAIN`
    NoPhysicalAttributeGain,
    /// The creature cannot lose any physical attributes.
    ///
    /// Appears as `NO_PHYS_ATT_RUST`
    NoPhysicalAttributeRust,
    /// Creature does not need to sleep. Can still be rendered unconscious by other means.
    ///
    /// Appears as `NO_SLEEP`
    NoSleep,
    /// The creature caste does not appear in spring.
    ///
    /// Appears as `NO_SPRING`
    NoSpring,
    /// The creature caste does not appear in summer.
    ///
    /// Appears as `NO_SUMMER`
    NoSummer,
    /// Creature doesn't require an organ with the `[THOUGHT]` tag to survive or attack; generally used on creatures that don't have brains.
    ///
    /// Appears as `NO_THOUGHT_CENTER_FOR_MOVEMENT`
    NoThoughtCenterForMovement,
    /// Prevents creature from selecting its color based on its profession (e.g. Miner, Hunter, Wrestler).
    ///
    /// Appears as `NO_UNIT_TYPE_COLOR`
    NoUnitTypeColor,
    /// Likely prevents the creature from leaving broken vegetation tracks
    ///
    /// Appears as `NO_VEGETATION_PERTURB`
    NoVegetationDisturbance,
    /// The creature caste does not appear in winter.
    ///
    /// Appears as `NO_WINTER`
    NoWinter,
    /// Creature has no bones.
    ///
    /// Appears as `NOBONES`
    NoBones,
    /// Creature doesn't need to breathe or have `[BREATHE]` parts in body, nor can it drown or be strangled. Creatures living in magma must have this tag,
    /// otherwise they will drown.
    ///
    /// Appears as `NOBREATHE`
    NoBreathe,
    /// Sets the creature to be active at night in adventure mode.
    ///
    /// Appears as `NOCTURNAL`
    Nocturnal,
    /// Creature has no emotions. It is immune to the effects of stress and unable to rage, and its needs cannot be fulfilled in any way.
    /// Used on undead in the vanilla game.
    ///
    /// Appears as `NOEMOTION`
    NoEmotion,
    /// Creature can't become tired or over-exerted from taking too many combat actions or moving at full speed for extended periods of time.
    ///
    /// Appears as `NOEXERT`
    NoExert,
    /// Creature doesn't feel fear and will never flee from battle, and will be immune to ghosts' attempts to 'scare it to death'.
    /// Additionally, it causes bogeymen and nightmares to become friendly towards the creature.
    ///
    /// Appears as `NOFEAR`
    NoFear,
    /// Creature will not be hunted or fed to wild beasts.
    ///
    /// Appears as `NOMEAT`
    NoMeat,
    /// Creature isn't nauseated by gut hits and cannot vomit.
    ///
    /// Appears as `NONAUSEA`
    NoNausea,
    /// Creature doesn't feel pain.
    ///
    /// Appears as `NOPAIN`
    NoPain,
    /// Creature will not drop a hide when butchered.
    ///
    /// Appears as `NOSKIN`
    NoSkin,
    /// Creature will not drop a skull on butchering, rot, or decay of severed head.
    ///
    /// Appears as `NOSKULL`
    NoSkull,
    /// Does not produce miasma when rotting.
    ///
    /// Appears as `NOSMELLYROT`
    NoSmellyRot,
    /// Weapons can't get stuck in the creature.
    ///
    /// Appears as `NOSTUCKINS`
    NoStuckIns,
    /// Creature can't be stunned and knocked unconscious by pain or head injuries. Creatures with this tag never wake up from sleep in Fortress Mode.
    /// If this creature needs to sleep while playing, it will die.
    ///
    /// Appears as `NOSTUN`
    NoStun,
    /// Cannot be butchered.
    ///
    /// Appears as `NOT_BUTCHERABLE`
    NotButcherable,
    /// Cannot be raised from the dead by necromancers or evil clouds. Implies the creature is not a normal living being. Used by vampires, mummies and
    /// inorganic creatures like the amethyst man and bronze colossus. Creatures who are `[OPPOSED_TO_LIFE]` (undead) will be docile towards creatures with this token.
    ///
    /// Appears as `NOT_LIVING`
    NotLiving,
    /// Creature doesn't require a `[THOUGHT]` body part to survive. Has the added effect of preventing speech, though directly controlling creatures that would otherwise
    /// be capable of speaking allows them to engage in conversation.
    ///
    /// Appears as `NOTHOUGHT`
    NoThought,
    /// How easy the creature is to smell. The higher the number, the easier the creature is to sniff out. Defaults to 50.
    /// Vanilla creatures have values from 0 (undetectable) to 90 (noticeable by humans and dwarves).
    ///
    /// Appears as `ODOR_LEVEL:50`
    OdorLevel {
        /// The odor level, defaults to 50
        odor_level: u32,
    },
    /// What the creature smells like. If no odor string is defined, the creature name (not the caste name) is used.
    ///
    /// Appears as `ODOR_STRING:SomeOdor`
    OdorString {
        /// The odor string to use
        odor_string: String,
    },
    /// Is hostile to all creatures except undead and other non-living ones and will show Opposed to life in the unit list. Used by undead in the vanilla game.
    /// Functions without the `[NOT_LIVING]` token, and seems to imply said token as well. Undead will not be hostile to otherwise-living creatures given this token.
    /// Living creatures given this token will attack living creatures that lack it, while ignoring other living creatures that also have this token.
    ///
    /// Appears as `OPPOSED_TO_LIFE`
    OpposedToLife,
    /// Determines caste's likelihood of having sexual attraction to certain sexes. Values default to 75:20:5 for the same sex and 5:20:75 for the opposite sex.
    /// The first value indicates how likely to be entirely uninterested in the sex, the second decides if the creature will be able to become lovers with that sex,
    /// the third decides whether they will be able to marry in worldgen and post-worldgen world activities (which implies being able to become lovers). Marriage seems
    /// to be able to happen in fort mode play regardless, as long as they are lovers first.
    ///
    /// Arguments:
    ///
    /// * `caste`: The caste to set orientation to (MALE or FEMALE typically)
    /// * `disinterested_chance`: The chance of being disinterested in `caste`
    /// * `casual_chance`: The chance of being casually interested in `caste`
    /// * `strong_chance`: The chance of being strongly interested in `caste`
    ///
    /// Appears as `ORIENTATION:SomeCaste:75:20:5`
    Orientation {
        /// The caste to set orientation to
        caste: String,
        /// The chance of being disinterested in `caste`
        disinterested_chance: u32,
        /// The chance of being casually interested in `caste`
        casual_chance: u32,
        /// The chance of being strongly interested in `caste`
        strong_chance: u32,
    },
    /// Lets you play as an outsider of this species in adventure mode.
    ///
    /// Appears as `OUTSIDER_CONTROLLABLE`
    OutsiderControllable,
    /// Allows the creature to be used as a pack animal. Used by merchants without wagons and adventurers. Also prevents creature from dropping hauled items on its own
    ///
    /// Note: do not use for player-controllable creatures! May lead to the creature being domesticated during worldgen, even if it doesn't have `[COMMON_DOMESTIC]`.
    ///
    /// Appears as `PACK_ANIMAL`
    PackAnimal,
    /// The creature is immune to all paralyzing special attacks.
    ///
    /// Appears as `PARALYZEIMMUNE`
    ParalyzeImmune,
    /// Used to control the bat riders with paralyze-dart blowguns that flew through the 2D chasm. Doesn't do anything now.
    ///
    /// Appears as `PATTERNFLIER`
    PatternFlier,
    /// In earlier versions, creature would generate pearls. Does nothing in the current version
    ///
    /// Appears as `PEARL`
    Pearl,
    /// Controls the ability of vermin to find a way into containers when they are eating food from your stockpiles.
    ///
    /// Objects made of most materials (e.g. metal) roll a number from 0-100, and if the resulting number is greater than the penetrate power, their contents escape for the time
    /// being. Objects made of wood, leather, amber, or coral roll 0-95, and items made of cloth roll 0-90.
    ///
    /// Appears as `PENETRATEPOWER:100`
    PenetratePower {
        /// The penetration power
        penetrate_power: u32,
    },
    /// Determines the range and chance of personality traits. Standard is 0:50:100.
    ///
    /// Arguments:
    ///
    /// * `personality_trait`: The trait to modify
    /// * `low`: The lowest chance of having the trait
    /// * `median`: The median chance of having the trait
    /// * `high`: The highest chance of having the trait
    ///
    /// Appears as `PERSONALITY:SomeTrait:0:50:100`
    Personality {
        /// The trait to modify
        personality_trait: String,
        /// The lowest chance of having the trait
        low: u32,
        /// The median chance of having the trait
        median: u32,
        /// The highest chance of having the trait
        high: u32,
    },
    /// Allows the creature to be tamed in Fortress mode. Prerequisite for all other working animal roles. Civilizations that encounter it in worldgen
    /// will tame and domesticate it for their own use. Adding this to civilization members will classify them as pets instead of citizens, with all the
    /// problems that entails. However, you can solve these problems using the popular plugin Dwarf Therapist, which is completely unaffected by the tag.
    ///
    /// Appears as `PET`
    Pet,
    /// Allows the creature to be tamed in Fortress mode. Prerequisite for all other working animal roles. Civilizations cannot domesticate it in worldgen,
    /// with certain exceptions. Adding this to civilization members will classify them as pets instead of citizens, with all the problems that entails.
    ///
    /// Appears as `PET_EXOTIC`
    PetExotic,
    /// How valuable a tamed animal is. Actual cost in points in the embarking screen is 1+(PETVALUE/2) for an untrained animal, 1+PETVALUE for a war/hunting one.
    ///
    /// Appears as `PETVALUE:100`
    PetValue {
        /// The pet value
        pet_value: u32,
    },
    /// Divides the creature's `[PETVALUE]` by the specified number. Used by honey bees to prevent a single hive from being worth a fortune.
    ///
    /// Appears as `PETVALUE_DIVISOR:2`
    PetValueDivisor {
        /// The divisor
        divisor: u32,
    },
    /// Default is 200. This means you can increase your attribute to 200% of its starting value (or the average value + your starting value if that is higher).
    ///
    /// Appears as `PHYS_ATT_CAP_PERC:Attribute:200`
    PhysicalAttributeCapPercentage {
        /// The attribute to modify
        attribute: String,
        /// The percentage to modify the attribute by
        percentage: u32,
    },
    /// Sets up a physical attribute's range of values (0-5000). All physical attribute ranges default to 200:700:900:1000:1100:1300:2000.
    ///
    /// Appears as `PHYS_ATT_RANGE:Attribute:200:700:900:1000:1100:1300:2000`
    PhysicalAttributeRange {
        /// The attribute to modify
        attribute: String,
        /// The ranges from lowest to highest with 7 steps
        ranges: [u32; 7],
    },
    /// Physical attribute gain/decay rates. Lower numbers in the last three slots make decay occur faster. Defaults for `STRENGTH`, `AGILITY`, `TOUGHNESS`, and `ENDURANCE`
    /// are `500:3:4:3`, while `RECUPERATION` and `DISEASE_RESISTANCE` default to `500:NONE:NONE:NONE`.
    ///
    /// Arguments:
    ///
    /// * `attribute`: The attribute to modify
    /// * `improvement_cost`: The cost to improve the attribute
    /// * `decay_rate_unused`: The decay rate of the attribute when it is unused
    /// * `decay_rate_rusty`: The decay rate of the attribute when it is rusty
    /// * `decay_rate_demotion`: The decay rate of the attribute when it is demoted
    ///
    /// Appears as `PHYS_ATT_RATE:Attribute:500:3:4:3`
    PhysicalAttributeRate {
        /// The attribute to modify
        attribute: String,
        /// The cost to improve the attribute
        improvement_cost: u32,
        /// The decay rate of the attribute when it is unused
        decay_rate_unused: u32,
        /// The decay rate of the attribute when it is rusty
        decay_rate_rusty: u32,
        /// The decay rate of the attribute when it is demoted
        decay_rate_demotion: u32,
    },
    /// Adds a body part group to selected body part group. Presumably used immediately after `[SET_BP_GROUP]`.
    ///
    /// Arguments:
    ///
    /// * `selector`: the selector for the specific body part
    ///
    /// Appears as `PLUS_BP_GROUP:SomeBodyPartSelector:SomeBodyPartGroup`
    PlusBodyPartGroup {
        /// The body part selector
        selector: Vec<String>,
    },
    /// Weighted population of caste; Lower is rarer. Not to be confused with `[FREQUENCY]`.
    ///
    /// Appears as `POP_RATIO:100`
    PopulationRatio {
        /// The population ratio
        pop_ratio: u32,
    },
    /// Allows the being to represent itself as a deity, allowing it to become the leader of a civilized group. Used by unique demons in the vanilla game.
    /// Requires `[CAN_SPEAK]` to actually do anything more than settle at a location (e.g. write books, lead armies, profane temples). Doesn't appear to do anything
    /// for creatures that are already civilized. Once the creature ascends to a position of leadership, it will proceed to act as a standard ruler for their
    /// entity and fulfill the same functions (hold tournaments, tame creatures, etc.).
    ///
    /// Appears as `POWER`
    Power,
    /// Caste-specific profession name.
    ///
    /// Arguments:
    ///
    /// * `profession`: The profession name / unit type token ID
    /// * `singular`: The singular name of the profession
    /// * `plural`: The plural name of the profession
    ///
    /// Appears as `CASTE_PROFESSION_NAME:SomeProfession:SomeName:SomeNames`
    ProfessionName {
        /// The profession name / unit type token ID
        profession: String,
        /// Name used for the profression
        name: Name,
    },
    /// Creature has a percentage chance to flip out at visible non-friendly creatures. Enraged creatures attack anything regardless of timidity and get a
    /// strength bonus to their hits. This is what makes badgers so hardcore.
    ///
    /// Appears as `PRONE_TO_RAGE:100`
    ProneToRage {
        /// The rage chance
        rage_chance: u32,
    },
    /// The creature has pus. Specifies the stuff secreted by infected wounds.
    ///
    /// Arguments:
    ///
    /// * `material`: The material of the pus
    /// * `material_state`: The material state of the pus
    ///
    /// Appears as `PUS:SomeMaterial:SomeMaterialState`
    Pus {
        /// The material of the pus
        material: Vec<String>,
        /// The material state of the pus
        state: String,
    },
    /// Specifies a new relative size for a part than what is stated in the body plan. For example, dwarves have larger livers.
    ///
    /// Arguments:
    ///
    /// * `selector`: the selector for the specific body part
    /// * `relative_size`: The relative size of the body part (by percentage?)
    ///
    /// Appears as `RELATIVE_SIZE:SomeBodyPartSelector:SomeBodyPart:100`
    RelativeSize {
        /// The body part selector
        selector: Vec<String>,
        /// The relative size of the body part (by percentage?)
        relative_size: u32,
    },
    /// What the creature's remains are called.
    ///
    /// Appears as `REMAINS:SomeRemain:SomeRemains`
    Remains {
        /// The singular name of the remains
        singular: String,
        /// The plural name of the remains
        plural: String,
    },
    /// What color the creature's remains are.
    ///
    /// Appears as `REMAINS_COLOR:SomeColor`
    RemainsColor {
        /// The color of the remains
        remains_color: String,
    },
    /// Goes with `[VERMIN_BITE]` and `[DIE_WHEN_VERMIN_BITE]`, the vermin creature will leave remains on death when biting.
    /// Leaving this tag out will cause the creature to disappear entirely after it bites.
    ///
    /// Appears as `REMAINS_ON_VERMIN_BITE_DEATH`
    RemainsOnVerminBiteDeath,
    /// Unknown remains variation.
    ///
    /// Appears as `REMAINS_UNDETERMINED`
    RemainsUndetermined,
    /// The creature will retract into the specified body part(s) when threatened. It will be unable to move or attack, but enemies will only be able to attack the specified body part(s). When one of the specified body part is severed off, the creature automatically unretracts and cannot retract anymore. More than one body part can be selected by using `BY_TYPE` or `BY_CATEGORY`.
    ///
    /// Second-person descriptions are used for adventurer mode natural ability. `"<pro_pos>"` can be used in the descriptions, being replaced with the proper pronoun (or lack thereof) in-game.
    ///
    /// Undead curled up creatures are buggy, specifically those that retract into their upper bodies: echidnas, hedgehogs and pangolins. The upper body is prevented from collapsing by a separate body part (the middle spine), which cannot be attacked when the creature is retracted. See `[PREVENTS_PARENT_COLLAPSE]`. Living creatures eventually succumb to blood loss, but undead creatures do not. Giant creatures also take a very long time to bleed out.
    ///
    /// Arguments:
    ///
    /// * `body_part_selector`: The body part selector to use
    /// * `body_part`: The body part to retract
    /// * `second_person`: Description using "you" and "your"
    /// * `third_person`: Description using "it" and "its"
    /// * `second_person_cancel`: Description using "you" and "your" when the creature is no longer retracted
    /// * `third_person_cancel`: Description using "it" and "its" when the creature is no longer retracted
    ///
    /// Appears as `RETRACT_INTO_BP:SomeBodyPartSelector:SomeBodyPart:SomeSecondPerson:SomeThirdPerson:SomeSecondPersonCancel:SomeThirdPersonCancel`
    RetractIntoBodyPart {
        /// The body part selector to use
        body_part_selector: String,
        /// The body part to retract
        body_part: String,
        /// Description using "you" and "your"
        second_person: String,
        /// Description using "it" and "its"
        third_person: String,
        /// Description using "you" and "your" when the creature is no longer retracted
        second_person_cancel: String,
        /// Description using "it" and "its" when the creature is no longer retracted
        third_person_cancel: String,
    },
    /// Cat behavior. If it kills a vermin creature and has an owner, it carries the remains in its mouth and drops them at their feet. Requires `[HUNTS_VERMIN]`.
    ///
    /// Appears as `RETURNS_VERMIN_KILLS_TO_OWNER`
    ReturnsVerminKillsToOwner,
    /// Creature will occasionally root around in the grass, looking for insects. Used for flavor in Adventurer Mode, spawns vermin edible for this creature in Fortress Mode. Creatures missing the specified body part will be unable to perform this action. The action produces a message (visible in adventure mode) in the form:
    ///
    /// `[creature] [verb text] the [description of creature's location]`
    ///
    /// In adventure mode, the "rooting around" ability will be included in the "natural abilities" menu, represented by its second person verb text.
    ///
    /// Arguments:
    ///
    /// * `body_part_selector`: the selector for the specific body part
    /// * `second_person_verb`: Verb to use in second person tense ("you")
    /// * `third_person_verb`: Verb to use in third person tense ("it")
    ///
    /// Appears as `ROOT_AROUND:SomeBodyPartSelector:SomeBodyPart:SomeSecondPersonVerb:SomeThirdPersonVerb`
    RootAround {
        /// The body part selector
        body_part_selector: Vec<String>,
        /// Verb to use in second person tense ("you")
        second_person_verb: String,
        /// Verb to use in third person tense ("it")
        third_person_verb: String,
    },
    /// Causes the specified tissue layer(s) of the indicated body part(s) to secrete the designated material. A size 100 ('covering') contaminant is created over the affected body part(s) in its specified material state (and at the temperature appropriate to this state) when the trigger condition is met, as long as one of the secretory tissue layers is still intact. Valid triggers are:
    ///
    /// * `CONTINUOUS`: Secretion occurs once every 40 ticks in fortress mode, and every tick in adventurer mode.
    /// * `EXERTION`: Secretion occurs continuously (at the rate described above) whilst the creature is at minimum Tired following physical exertion. Note that this cannot occur if the creature has `[NOEXERT]`.
    /// * `EXTREME_EMOTION`:  Secretion occurs continuously (as above) whilst the creature is distressed. Cannot occur in creatures with `[NOEMOTION]`.
    ///
    /// Arguments:
    ///
    /// * `material`: The material of the secretion
    /// * `material_state`: The material state of the secretion
    /// * `body_part_selector`: the selector for the specific body part
    /// * `tissue_layer`: The tissue layer to use
    /// * `trigger`: The trigger to use (`CONTINUOUS`, `EXERTION`, `EXTREME_EMOTION`)
    ///
    /// Appears as `SECRETION:SomeMaterial:SomeMaterialState:SomeBodyPartSelector:SomeBodyPart:SomeTissueLayer:SomeTrigger`
    Secretion {
        /// The material of the secretion
        material: Vec<String>,
        /// The material state of the secretion
        material_state: String,

        /// The body part selector
        body_part_selector: Vec<String>,

        /// The tissue layer to use
        tissue_layer: String,
        /// The trigger to use (`CONTINUOUS`, `EXERTION`, `EXTREME_EMOTION`)
        trigger: String,
    },
    /// Essentially the same as `[MEGABEAST]`, but more of them are created during worldgen. See the semi-megabeast page for details.
    ///
    /// Appears as `SEMIMEGABEAST`
    SemiMegabeast,
    /// Gives the creature the ability to sense creatures belonging to the specified creature class even when they lie far beyond line of sight, including through walls and floors.
    /// It also appears to reduce or negate the combat penalty of blind units when fighting creatures they can sense. In adventure mode, the specified tile will be used to represent
    /// sensed creatures when they cannot be seen directly.
    ///
    /// Arguments:
    ///
    /// * `creature_class`: The creature class to sense
    /// * `tile`: The tile to use
    /// * `color`: via foreground, background, and brightness values
    ///
    /// Appears as `SENSE_CREATURE_CLASS:SomeCreatureClass:SomeTile:0:0:0`
    SenseCreatureClass {
        /// The creature class to sense
        creature_class: String,
        /// The tile to use
        tile: String,
        /// The foreground color to use
        foreground: u32,
        /// The background color to use
        background: u32,
        /// The brightness to use
        brightness: u32,
    },
    /// Begins a selection of body parts.
    ///
    /// Arguments:
    ///
    /// * `body_part_selector`: the selector for the specific body part
    ///
    /// Appears as `SET_BP_GROUP:SomeBodyPartSelector:SomeBodyPart`
    SetBodyPartGroup {
        /// The body part selector
        body_part_selector: Vec<String>,
    },
    /// The rate at which this creature learns this skill. Requires `[CAN_LEARN]` or `[INTELLIGENT]` to function.
    ///
    /// Arguments:
    ///
    /// * `skill`: The skill to modify
    /// * `rate`: The rate to modify the skill by (percentage)
    ///
    /// Appears as `SKILL_LEARN_RATE:SomeSkill:100`
    SkillLearnRate {
        /// The skill to modify
        skill: String,
        /// The rate to modify the skill by
        rate: u32,
    },
    /// The rate at which this creature learns all skills. Requires `[CAN_LEARN]` or `[INTELLIGENT]` to function.
    ///
    /// Arguments:
    ///
    /// * `rate`: The rate to modify the skill by (percentage)
    ///
    /// Appears as `SKILL_LEARN_RATES:100`
    SkillLearnRates {
        /// The rate to modify the skill by
        rate: u32,
    },
    /// Like `[SKILL_RATES]`, but applies to individual skills instead. Requires `[CAN_LEARN]` or `[INTELLIGENT]` to function.
    ///
    /// Arguments:
    ///
    /// * `skill`: The skill to modify
    /// * `improvement_rate`: The improvement rate to modify the skill by (percentage)
    /// * `decay_rate_unused`: The decay rate of the skill when it is unused
    /// * `decay_rate_rusty`: The decay rate of the skill when it is rusty
    /// * `decay_rate_demotion`: The decay rate of the skill when it is demoted
    ///
    /// Appears as `SKILL_RATE:SomeSkill:100:3:4:3`
    SkillRate {
        /// The skill to modify
        skill: String,
        /// The improvement rate to modify the skill by
        improvement_rate: u32,
        /// The decay rate of the skill when it is unused
        decay_rate_unused: u32,
        /// The decay rate of the skill when it is rusty
        decay_rate_rusty: u32,
        /// The decay rate of the skill when it is demoted
        decay_rate_demotion: u32,
    },
    /// Affects skill gain and decay. Lower numbers in the last three slots make decay occur faster (`[SKILL_RATES:100:1:1:1]` would cause rapid decay).
    /// The counter (decay) rates may also be replaced with NONE.
    ///
    /// Default is `[SKILL_RATES:100:8:16:16]`. Requires `[CAN_LEARN]` or `[INTELLIGENT]` to function.
    ///
    /// Arguments:
    ///
    /// * `improvement_rate`: The improvement rate to modify the skill by (percentage)
    /// * `decay_rate_unused`: The decay rate of the skill when it is unused
    /// * `decay_rate_rusty`: The decay rate of the skill when it is rusty
    /// * `decay_rate_demotion`: The decay rate of the skill when it is demoted
    ///
    /// Appears as `SKILL_RATES:100:8:16:16`
    SkillRates {
        /// The improvement rate to modify the skill by
        improvement_rate: u32,
        /// The decay rate of the skill when it is unused
        decay_rate_unused: u32,
        /// The decay rate of the skill when it is rusty
        decay_rate_rusty: u32,
        /// The decay rate of the skill when it is demoted
        decay_rate_demotion: u32,
    },
    /// The rate at which this skill decays. Lower values cause the skill to decay faster. Requires `[CAN_LEARN]` or `[INTELLIGENT]` to function.
    ///
    /// Arguments:
    ///
    /// * `skill`: The skill to modify
    /// * `decay_rate_unused`: The decay rate of the skill when it is unused
    /// * `decay_rate_rusty`: The decay rate of the skill when it is rusty
    /// * `decay_rate_demotion`: The decay rate of the skill when it is demoted
    ///
    /// Appears as `SKILL_RUST_RATE:SomeSkill:3:4:3`
    SkillRustRate {
        /// The skill to modify
        skill: String,
        /// The decay rate of the skill when it is unused
        decay_rate_unused: u32,
        /// The decay rate of the skill when it is rusty
        decay_rate_rusty: u32,
        /// The decay rate of the skill when it is demoted
        decay_rate_demotion: u32,
    },
    /// The rate at which all skills decay. Lower values cause the skills to decay faster. Requires `[CAN_LEARN]` or `[INTELLIGENT]` to function.
    ///
    /// Arguments:
    ///
    /// * `decay_rate_unused`: The decay rate of the skill when it is unused
    /// * `decay_rate_rusty`: The decay rate of the skill when it is rusty
    /// * `decay_rate_demotion`: The decay rate of the skill when it is demoted
    ///
    /// Appears as `SKILL_RUST_RATES:3:4:3`
    SkillRustRates {
        /// The decay rate of the skill when it is unused
        decay_rate_unused: u32,
        /// The decay rate of the skill when it is rusty
        decay_rate_rusty: u32,
        /// The decay rate of the skill when it is demoted
        decay_rate_demotion: u32,
    },
    /// Caste-specific `[SLAIN_SPEECH]`.
    ///
    /// Appears as `SLAIN_CASTE_SPEECH:SomeSpeechSet`
    SlainSpeech {
        /// The speech set to use
        speech_file: String,
    },
    /// Shorthand for `[CAN_LEARN]` + `[SKILL_LEARN_RATES:50]`. Used by a number of 'primitive' creatures (like ogres, giants and troglodytes) in the vanilla game.
    /// Applicable to player races. Prevents a player from recruiting nobility, even basic ones. Subterranean creatures with this token combined with `[EVIL]` will become
    /// servants of goblins in their civilizations, in the style of trolls.
    ///
    /// Appears as `SLOW_LEARNER`
    SlowLearner,
    /// Creature leaves "remains" instead of a corpse. Used by vermin.
    ///
    /// Appears as `SMALL_REMAINS`
    SmallRemains,
    /// Caste-specific solider tile.
    ///
    /// Appears as `CASTE_SOLDIER_TILE:SomeTile`
    SoldierTile {
        /// The tile to use
        tile: TileCharacter,
    },
    /// Caste-specific solider alt tile.
    ///
    /// Appears as `CASTE_SOLDIER_ALTTILE:SomeTile`
    SoldierAltTile {
        /// The tile to use
        tile: TileCharacter,
    },
    /// Creature makes sounds periodically, which can be heard in Adventure mode.
    ///
    /// For example, with `SOUND:PEACEFUL_INTERMITTENT:100:1000:VOCALIZATION:bark:barks:a loud bark`
    ///
    /// * First-person reads "You 'bark'"
    /// * Third-person reads "The capybara 'barks'"
    /// * Out of sight reads "You hear 'a loud bark'"
    ///
    /// Arguments:
    ///
    /// * `sound_type`: The sound type to use (`ALERT` or `PEACEFUL_INTERMITTENT`)
    /// * `sound_range`: The range of the sound (in tiles)
    /// * `sound_interval`: A delay before the sound is produced again (in ticks)
    /// * `requires_breathing`: Whether the creature needs to breathe to make the sound
    ///   (indicated by `VOCALIZATION` for true or `NONE` for false)
    /// * `first_person`: The first-person description of the sound
    /// * `third_person`: The third-person description of the sound
    /// * `out_of_sight`: The out-of-sight description of the sound
    ///
    /// Appears as `SOUND:SomeSoundType:100:1000:SomeFirstPerson:SomeThirdPerson:SomeOutOfSight`
    Sound {
        /// The sound type to use (`ALERT` or `PEACEFUL_INTERMITTENT`)
        sound_type: String,
        /// The range of the sound (in tiles)
        sound_range: u32,
        /// A delay before the sound is produced again (in ticks)
        sound_interval: u32,
        /// Whether the creature needs to breathe to make the sound
        requires_breathing: bool,
        /// The first-person description of the sound
        first_person: String,
        /// The third-person description of the sound
        third_person: String,
        /// The out-of-sight description of the sound
        out_of_sight: String,
    },
    /// Creature will only appear in biomes with this plant or creature available.
    /// Grazers given a specific type of grass (such as pandas and bamboo) will only eat that grass and nothing else, risking starvation if there's none available.
    ///
    /// Arguments:
    ///
    /// * `food_type`: The type of the required food
    /// * `identifier`: The identifier of the required plant or creature
    ///
    /// Appears as `SPECIFIC_FOOD:PLANT:Bamboo` or `SPECIFIC_FOOD:CREATURE:Tiger`
    SpecificFood {
        /// The type of the required food
        food_type: ObjectType,
        /// The identifier of the required plant or creature
        identifier: String,
    },
    /// This creature can be converted by a night creature with `[SPOUSE_CONVERTER]`.
    ///
    /// Appears as `SPOUSE_CONVERSION_TARGET`
    SpouseConversionTarget,
    /// If the creature has the `[NIGHT_CREATURE_HUNTER]` tag, it will kidnap `[SPOUSE_CONVERSION_TARGET]`s and transform them into the caste of its species
    /// with the `[CONVERTED_SPOUSE]` tag during worldgen. It may also start families this way.
    ///
    /// Appears as `SPOUSE_CONVERTER`
    SpouseConverter,
    /// If the creature rules over a site, it will cause the local landscape to be corrupted into evil surroundings associated with the creature's spheres.
    /// The creature must have at least one of the following spheres for this to take effect: BLIGHT, DEATH, DISEASE, DEFORMITY, NIGHTMARES. The first three kill vegetation,
    /// while the others sometimes do. The last two get evil plants and evil animals sometimes. NIGHTMARES gets bogeymen. Used by demons in the vanilla game.
    ///
    /// Appears as `SPREAD_EVIL_SPHERES_IF_RULER`
    SpreadEvilSpheresIfRuler,
    /// Caste does not require `[GRASP]` body parts to climb -- it can climb with `[STANCE]` parts instead.
    ///
    /// Appears as `STANCE_CLIMBER`
    StanceClimber,
    /// Acts as `[GRAZER]` but set to 20000*G*(max size)^(-3/4), where G defaults to 100 but can be set in `d_init`, and the whole thing is trapped between 150 and 3 million.
    /// Used for all grazers in the default creature raws.
    ///
    /// Appears as `STANDARD_GRAZER`
    StandardGrazer,
    /// The creature will get strange moods in fortress mode and can produce artifacts.
    ///
    /// Appears as `STRANGE_MOODS`
    StrangeMoods,
    /// Gives the creature knowledge of any secrets with `[SUPERNATURAL_LEARNING_POSSIBLE]` that match its spheres and also prevents it from becoming a vampire or werebeast.
    /// Other effects are unknown.
    ///
    /// Appears as `SUPERNATURAL`
    Supernatural,
    /// The creature naturally knows how to swim perfectly and does not use the swimmer skill, as opposed to `[SWIMS_LEARNED]` below.
    /// However, Fortress mode AI never paths into water anyway, so it's less useful there.
    ///
    /// Appears as `SWIMS_INNATE`
    SwimsInnate,
    /// The creature swims only as well as their present swimming skill allows them to.
    ///
    /// Appears as `SWIMS_LEARNED`
    SwimsLearned,
    /// Dilutes the effects of syndromes which have the specified identifier. A percentage of 100 is equal to the regular syndrome effect severity, higher percentages reduce severity.
    ///
    /// Arguments:
    ///
    /// * `syndrome`: The syndrome to modify
    /// * `percentage`: The percentage to modify the syndrome by
    ///
    /// Appears as `SYNDROME_DILUTION_FACTOR:SomeSyndrome:100`
    SyndromeDilutionFactor {
        /// The syndrome to modify
        syndrome: String,
        /// The percentage to modify the syndrome by
        percentage: u32,
    },
    /// The creature has tendons in its `[CONNECTIVE_TISSUE_ANCHOR]` tissues (bone or chitin by default).
    /// Cutting the bone/chitin tissue severs the tendons, disabling motor function if the target is a limb.
    ///
    /// Arguments:
    ///
    /// * `material`: The material of the tendons
    /// * `healing_rate`: The rate at which the tendons heal (lower is faster)
    ///
    /// Appears as `TENDONS:SomeMaterial:100`
    Tendons {
        /// The material of the tendons
        material: Vec<String>,
        /// The rate at which the tendons heal (lower is faster)
        healing_rate: u32,
    },
    /// The creature's webs can catch larger creatures.
    ///
    /// Appears as `THICKWEB`
    ThickWeb,
    /// Caste-specific tile.
    ///
    /// Appears as `CASTE_TILE:SomeTile`
    Tile {
        /// The tile to use
        tile: TileCharacter,
    },
    /// Adds the tissue layer to wherever it is required.
    ///
    /// Non-argument Locations can be FRONT, RIGHT, LEFT, TOP, BOTTOM. Argument locations are AROUND and CLEANS, requiring a further body part and a % of coverage/cleansing
    ///
    /// Arguments:
    ///
    /// * `body_part_selector`: the selector for the specific body part
    /// * `tissue`: The name of the tissue to use
    /// * `location`: The location to use (`FRONT`, `RIGHT`, `LEFT`, `TOP`, `BOTTOM`) or with an additional argument, (`AROUND`, `CLEANS`) with a body part and a percentage
    ///
    /// Appears as `[TISSUE_LAYER:SomeBodyPartSelector:SomeBodyPart:SomeTissue:SomeLocation]` or `[TISSUE_LAYER:SomeBodyPartSelector:SomeBodyPart:SomeTissue:SomeLocation:SomeBodyPart:100]`
    /// ALSO appears as `[TISSUE_LAYER_OVER:SomeBodyPartSelector:SomeBodyPart:SomeTissue:SomeLocation]` or `[TISSUE_LAYER_OVER:SomeBodyPartSelector:SomeBodyPart:SomeTissue:SomeLocation:SomeBodyPart:100]`
    TissueLayer {
        /// The body part selector
        body_part_selector: Vec<String>,
        /// The tissue to apply (e.g. NAIL)
        tissue: String,
        /// The remaining tokens defining location/positioning.
        /// e.g. ["FRONT"] or ["ABOVE", "BY_CATEGORY", "EYE"] or []
        positioning: Vec<String>,
    },
    /// Adds the tissue layer under a given part.
    ///
    /// For example, an iron man has a gaseous poison within, and this tissue (GAS is its name) has the token `[TISSUE_LEAKS]` and its state is GAS, so when you puncture the iron outside and
    /// damage this tissue it leaks gas (can have a syndrome by using a previous one in the creature sample.)
    /// `[TISSUE_LAYER_UNDER:BY_CATEGORY:ALL:{tissue}]`
    ///
    /// `{tissue}` is what will be under the `TISSUE_LAYER`; here is an example Tissue from the Iron Man:
    ///
    /// `[TISSUE:GAS] [TISSUE_NAME:gas:NP] [TISSUE_MATERIAL:LOCAL_CREATURE_MAT:GAS] [TISSUE_MAT_STATE:GAS] [RELATIVE_THICKNESS:50] [TISSUE_LEAKS] [TISSUE_SHAPE:LAYER]`
    ///
    /// Arguments:
    ///
    /// * `body_part_selector`: The body part selector to use (`BY_TYPE`, `BY_CATEGORY`, `BY_TOKEN`)
    /// * `body_part`: The body part to use (via category, type or token)
    /// * `tissue`: The name of the tissue to use
    ///
    /// Appears as `TISSUE_LAYER_UNDER:SomeBodyPartSelector:SomeBodyPart:SomeTissue`
    TissueLayerUnder {
        /// The body part selector to use (`BY_TYPE`, `BY_CATEGORY`, `BY_TOKEN`)
        body_part_selector: String,
        /// The body part to use (via category, type or token)
        body_part: String,
        /// The name of the tissue to use
        tissue: String,
    },
    /// Found on titans. Cannot be specified in user-defined raws.
    ///
    /// Appears as `TITAN`
    Titan,
    /// How much the creature can carry when used by merchants. 1000 by default. If a civilization uses a custom pack animal via `ALWAYS_PACK`, you must manually add a capacity to the raws of that
    /// creature itself. Capacity defaults to null leading to empty caravans.
    ///
    /// Arguments:
    ///
    /// * `capacity`: The capacity of the creature
    ///
    /// Appears as `TRADE_CAPACITY:1000`
    TradeCapacity {
        /// The capacity of the creature
        capacity: u32,
    },
    /// Shortcut for `[TRAINABLE_HUNTING]` + `[TRAINABLE_WAR]`.
    ///
    /// Appears as `TRAINABLE`
    Trainable,
    /// Can be trained as a hunting beast, increasing speed.
    ///
    /// Appears as `TRAINABLE_HUNTING`
    TrainableHunting,
    /// Can be trained as a war beast, increasing strength and endurance.
    ///
    /// Appears as `TRAINABLE_WAR`
    TrainableWar,
    /// Allows the creature to go into martial trances. Used by dwarves in the vanilla game.
    ///
    /// Appears as `TRANCES`
    Trances,
    /// The creature will never trigger traps it steps on. Used by a number of creatures. Doesn't make the creature immune to remotely activated traps (like retractable spikes being triggered
    /// while the creature is standing over them). TRAPAVOID creatures lose this power if they're immobilized while standing in a trap, be it by stepping on thick web, being paralyzed or being
    /// knocked unconscious.
    ///
    /// Appears as `TRAPAVOID`
    TrapAvoid,
    /// The creature is displayed as blue when in 7/7 water. Used on fish and amphibious creatures which swim under the water.
    ///
    /// Appears as `UNDERSWIM`
    UnderSwim,
    /// Found on generated demons; causes the game to create a single named instance of the demon which will emerge from the underworld and take over civilizations during worldgen.
    ///
    /// Appears as `UNIQUE_DEMON`
    UniqueDemon,
    /// Like `[AT_PEACE_WITH_WILDLIFE]`, but also makes the creature more valued in artwork by civilisations with the "plant" sphere. Used by grimelings in the vanilla game.
    ///
    /// Appears as `VEGETATION`
    Vegetation,
    /// Enables vermin to bite other creatures, injecting the specified material. See `[SPECIALATTACK_INJECT_EXTRACT]` for details about injection - this token presumably works in a similar manner.
    ///
    /// Arguments:
    ///
    /// * `chance`: The chance to inject the material
    /// * `verb`: The verb to use (e.g. "bitten, stung")
    /// * `material`: The material to inject
    /// * `material_state`: The material state to inject
    ///
    /// Appears as `VERMIN_BITE:100:bitten:SomeMaterial:SomeMaterialState`
    VerminBite {
        /// The chance to inject the material
        chance: u32,
        /// The verb to use (e.g. "bitten, stung")
        verb: String,
        /// The material to inject
        material: Vec<String>,
        /// The material state to inject
        material_state: String,
    },
    /// Some dwarves will hate the creature and get unhappy thoughts when around it. See the list of hateable vermin for details.
    ///
    /// Appears as `VERMIN_HATEABLE`
    VerminHateable,
    /// This makes the creature move in a swarm of creatures of the same race as it (e.g. swarm of flies, swarm of ants).
    ///
    /// Appears as `VERMIN_MICRO`
    VerminMicro,
    /// The creature cannot be caught by fishing.
    ///
    /// Appears as `VERMIN_NOFISH`
    VerminNoFish,
    /// The creature will not be observed randomly roaming about the map.
    ///
    /// Appears as `VERMIN_NOROAM`
    VerminNoRoam,
    /// The creature cannot be caught in baited animal traps; however, a "catch live land animal" task may still be able to capture one if a dwarf finds one roaming around.
    ///
    /// Appears as `VERMIN_NOTRAP`
    VerminNoTrap,
    /// Old shorthand for "does cat stuff". Contains `[AT_PEACE_WITH_WILDLIFE]` + `[RETURNS_VERMIN_KILLS_TO_OWNER]` + `[HUNTS_VERMIN]` + `[ADOPTS_OWNER]`.
    ///
    /// Appears as `VERMINHUNTER`
    VerminHunter,
    /// Sets the creature to be active during the evening in adventurer mode.
    ///
    /// Appears as `VESPERTINE`
    Vespertine,
    /// Value should determine how close you have to get to a critter before it attacks (or prevents adv mode travel etc.) Default is 20.
    ///
    /// Appears as `VIEWRANGE:20`
    ViewRange {
        /// The view range of the creature, default is 20
        view_range: u32,
    },
    /// The width of the creature's vision arcs, in degrees (i.e. 0 to 360). The first number is binocular vision, the second is non-binocular vision.
    /// Binocular vision has a minimum of about 10 degrees, monocular, a maximum of about 350 degrees. Values past these limits will be accepted, but will default to ~10 degrees
    /// and ~350 degrees respectively.
    ///
    /// Defaults are 60:120.
    ///
    /// Appears as `VISION_ARC:60:120`
    VisionArc {
        /// The binocular vision arc of the creature, default is 60
        binocular: u32,
        /// The non-binocular vision arc of the creature, default is 120
        non_binocular: u32,
    },
    /// Allows the creature to pull caravan wagons. If a civilization doesn't have access to any, it is restricted to trading with pack animals.
    ///
    /// Appears as `WAGON_PULLER`
    WagonPuller,
    /// Allows the creature to create webs, and defines what the webs are made of.
    ///
    /// Arguments:
    ///
    /// * `material`: The material of the webs
    ///
    /// Appears as `WEBBER:SomeMaterial`
    Webber {
        /// The material of the webs
        material: Vec<String>,
    },
    /// The creature will not get caught in thick webs. Used by creatures who can shoot thick webs (such as giant cave spiders) in order to make them immune to their own attacks.
    ///
    /// Appears as `WEBIMMUNE`
    WebImmune,
    /// An unknown token.
    #[default]
    Unknown,
    // Tokens found in the XML exports..
    /// A night creature
    NightCreature,
    /// Not fire immune
    NotFireImmune,
    /// Has blood
    HasBlood,
    /// Can grasp
    Grasp,
    /// The gait of the race
    RaceGait,
    /// Cannot breathe water
    CannotBreatheWater,
    /// Is a natural animal
    NaturalAnimal,
    /// Is a curious beast
    CuriousBeast,
    /// Is a flying curious beast
    CannotBreatheAir,
}

impl std::fmt::Display for CasteToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
