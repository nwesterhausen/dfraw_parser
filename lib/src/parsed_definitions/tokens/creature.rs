//! Contains the `CreatureTag` enum and associated implementations.

use crate::custom_types::TileCharacter;

/// An enum representing a creature tag.
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
pub enum CreatureToken {
    /// If set, the creature will blink between its `[Tile]` and its `[AltTile]`.
    ///
    /// Arguments:
    ///
    /// - the 'character' or tile number
    ///
    /// Appears as `ALTTILE:123`
    AltTile {
        /// The character or tile number
        character: TileCharacter,
    },
    /// Applies the specified creature variation with the given arguments to the creature. See `[ApplyCreatureVariation]` for more information.
    ///
    /// Appears as `APPLY_CREATURE_VARIATION:SOME_VARIATION` or `APPLY_CREATURE_VARIATION:SOME_VARIATION:ARG1:ARG2:ARG3`
    ApplyCreatureVariation {
        /// Creature variation ID to apply
        id: String,
        /// (Optional) any number of arguments to pass to the creature variation
        args: Vec<String>,
    },
    /// Applies the effects of all pending `[CV_ADD_TAG]` and `[CV_REMOVE_TAG]` tokens that have been defined in the current creature (so far).
    ///
    /// Appears as `APPLY_CURRENT_CREATURE_VARIATION`
    ApplyCurrentCreatureVariation,
    /// Enables the creature to be kept in artificial hives by beekeepers.
    ///
    /// Appears as `ARTIFICIAL_HIVEABLE`
    ArtificialHiveable,
    /// Select a biome the creature may appear in.
    ///
    /// Appears as `BIOME:SomeBiomeId`
    Biome {
        /// Biome identifier
        id: String,
    },
    /// Defines a caste
    Caste {
        /// The name of the caste
        name: String,
    },
    /// Multiplies frequency by a factor of (integer)%.
    ///
    /// Appears as `CHANGE_FREQUENCY_PERC:100`
    ChangeFrequencyPercent {
        /// The percentage to change the frequency by
        percent: u32,
    },
    /// The minimum/maximum numbers of how many creatures per spawned cluster. Vermin fish with this token in
    /// combination with temperate ocean and river biome tokens will perform seasonal migrations.
    ///
    /// Defaults to 1:1 if not specified.
    ///
    /// Appears as `CLUSTER_NUMBER:1:1`
    ClusterNumber {
        /// The minimum number of creatures per spawned cluster
        min: u32,
        /// The maximum number of creatures per spawned cluster
        max: u32,
    },
    /// Copies another specified creature. This will override any definitions made before it; essentially, it makes this creature identical to the other one, which can then
    /// be modified. Often used in combination with `[APPLY_CREATURE_VARIATION]` to import standard variations from a file. The vanilla giant animals and animal peoples are
    /// examples of this token combination.
    ///
    /// Arguments:
    ///
    /// * `creature`: The identifier of the creature to copy
    ///
    /// Appears as `COPY_TAGS_FROM:SomeCreature`
    CopyTagsFrom {
        /// The identifier of the creature to copy
        creature: String,
    },
    /// Creatures active in their civilization's military will use this tile instead.
    ///
    /// Appears as `CREATURE_SOLDIER_TILE:123`
    CreatureSoldierTile {
        /// The character or tile number
        character: TileCharacter,
    },
    /// The symbol of the creature in ASCII mode.
    ///
    /// Appears as `CREATURE_TILE:123`
    CreatureTile {
        /// The character or tile number
        character: TileCharacter,
    },
    /// The color of the creature's tile.
    ///
    /// Arguments:
    ///
    /// * `foreground`: The foreground color
    /// * `background`: The background color
    /// * `brightness`: The brightness of the color
    ///
    /// Appears as `COLOR:0:0:0`
    Color {
        /// The foreground color
        foreground: u32,
        /// The background color
        background: u32,
        /// The brightness of the color
        brightness: u32,
    },
    /// Adding this token to a creature prevents it from appearing in generated worlds (unless it's marked as always present for a particular
    /// civilization). For example, adding it to dogs will lead to worlds being generated without dogs in them. Also removes the creature from the
    /// object testing arena's spawn list. If combined with [`CreatureTag::Fanciful`], artistic depictions of the creature will occur regardless. Used by centaurs,
    /// chimeras and griffons in the vanilla game.
    ///
    /// Appears as `DOES_NOT_EXIST`
    DoesNotExist,
    /// Makes the creature appear as a large 3x3 wagon responsible for carrying trade goods, pulled by two `[WAGON_PULLER]` creatures and driven by a merchant.
    ///
    /// Appears as `EQUIPMENT_WAGON`
    EquipmentWagon,
    /// The creature is considered evil and will only show up in evil biomes. Civilizations with `[EntityToken::UseEvilAnimals]` can domesticate them
    /// regardless of exotic status. Has no effect on cavern creatures except to restrict taming. A civilization with evil creatures can colonize evil areas.
    ///
    /// Appears as `EVIL`
    Evil,
    /// The creature is a thing of legend and known to all civilizations. Its materials cannot be requested or preferred. The tag also adds some art value modifiers.
    /// Used by a number of creatures. Conflicts with `[CasteToken::CommonDomestic]`.
    Fanciful,
    /// Determines the chances of a creature appearing within its environment, with higher values resulting in more frequent appearance. Also affects the chance of a
    /// creature being brought in a caravan for trading. The game effectively considers all creatures that can possibly appear and uses the FREQUENCY value as a weight
    ///
    /// For example, if there are three creatures with frequencies 10/25/50, the creature with `[FREQUENCY:50]` will appear approximately 58.8% of the time.
    ///
    /// Defaults to 50 if not specified. Not to be confused with `[PopulationRatio]`.
    ///
    /// Appears as `FREQUENCY:50`
    Frequency {
        /// The frequency of the creature, a number between 0 and 100 (inclusive)
        frequency: u32,
    },
    /// Name of the creatures baby form. Applies to all castes but can be overridden by `[CasteToken::BabyName]`.
    ///
    /// Appears as `GENERAL_BABY_NAME:BabyName:BabyNames`
    GeneralBabyName {
        /// The name of the baby
        singular: String,
        /// The plural name of the baby
        plural: String,
    },
    /// Name of the creatures child form. Applies to all castes but can be overridden by `[CasteToken::ChildName]`.
    ///
    /// Appears as `GENERAL_CHILD_NAME:ChildName:ChildNames`
    GeneralChildName {
        /// The name of the child
        singular: String,
        /// The plural name of the child
        plural: String,
    },
    /// Found on procedurally generated creatures like forgotten beasts, titans, demons, angels, and night creatures. Cannot be specified in user-defined raws.
    ///
    /// Appears as `GENERATED`
    Generated,
    /// The color of the creature's glow tile.
    ///
    /// Arguments:
    ///
    /// * `foreground`: The foreground color
    /// * `background`: The background color
    /// * `brightness`: The brightness of the color
    ///
    /// Appears as `GLOWCOLOR:0:0:0`
    GlowColor {
        /// The foreground color
        foreground: u32,
        /// The background color
        background: u32,
        /// The brightness of the color
        brightness: u32,
    },
    /// The creature's tile when it is glowing.
    ///
    /// Arguments:
    ///
    /// * `character`: The character or tile number
    ///
    /// Appears as `GLOWTILE:123`
    GlowTile {
        /// The character or tile number
        character: TileCharacter,
    },
    /// Creature is considered good and will only show up in good biomes - unicorns, for example. Civilizations with `[EntityToken::UseGoodAnimals]` can
    /// domesticate them regardless of exotic status. Has no effect on cavern creatures except to restrict taming. A civilization that has good
    /// creatures can colonize good areas in world-gen.
    ///
    /// Appears as `GOOD`
    Good,
    /// When using tags from an existing creature, inserts new tags at the end of the creature.
    ///
    /// Appears as `GO_TO_END`
    GoToEnd,
    /// When using tags from an existing creature, inserts new tags at the beginning of the creature.
    ///
    /// Appears as `GO_TO_START`
    GoToStart,
    /// When using tags from an existing creature, inserts new tags after the specified tag.
    ///
    /// Arguments:
    ///
    /// * `tag`: The tag to insert after
    ///
    /// Appears as `GO_TO_TAG:TAG`
    GoToTag {
        /// The tag to insert after
        tag: String,
    },
    /// What product is harvested from beekeeping.
    ///
    /// Arguments:
    ///
    /// * `number`: The number of products harvested
    /// * `time`: The time it takes before the next harvest
    /// * `item tokens`: The item tokens that are harvested (some arbitrary list of items)
    ///
    /// Appears as `HARVEST_PRODUCT:1:1:ITEM_TOKENS`
    HarvestProduct {
        /// The number of products harvested
        number: u32,
        /// The time it takes before the next harvest
        time: u32,
        /// The item tokens that are harvested (some arbitrary list of items)
        item_tokens: Vec<String>,
    },
    /// This is the core requisite tag allowing the creature to spawn as a wild animal in the appropriate biomes. Requires specifying a [`crate::tokens::BiomeTag`] in which the creature will spawn.
    /// Does not require specifying a frequency, population number, or cluster number.
    ///
    /// This tag stacks with `[CasteToken::Megabeast]`, `[CasteToken::SemiMegabeast]`, or `[CasteToken::NightCreatureHunter]`; if used with one of these tags, the creature will spawn
    /// as both a boss and as a wild animal. This tag does not stack with `[CasteToken::FeatureBeast]` and if both are used the creature will not spawn. This tag is unaffected by
    /// `[CasteToken::Demon]`.
    ///
    /// Appears as `LARGE_ROAMING`
    LargeRoaming,
    /// Allows you to play as a wild animal of this species in adventurer mode. Prevents trading of (tame) instances of this creature in caravans.
    ///
    /// Appears as `LOCAL_POPS_CONTROLLABLE`
    LocalPopsControllable,
    /// Wild animals of this species may occasionally join a civilization. Prevents trading of (tame) instances of this creature in caravans.
    ///
    /// Appears as `LOCAL_POPS_PRODUCE_HEROES`
    LocalPopsProduceHeroes,
    /// The creatures will scatter if they have this tag, or form tight packs if they don't.
    ///
    /// Appears as `LOOSE_CLUSTERS`
    LooseClusters,
    /// Marks if the creature is an actual real-life creature. Only used for age-names at present.
    Mundane,
    /// The generic name for any creature of this type - will be used when distinctions between caste are unimportant. For names for specific castes, use `[CASTE_NAME]` instead.
    /// If left undefined, the creature will be labeled as "nothing" by the game.
    ///
    /// Appears as `NAME:Name:Names:NameAdj`
    Name {
        /// The name of the creature
        name: String,
        /// The plural name of the creature
        plural_name: String,
        /// The adjective form of the creature's name
        adjective: String,
    },
    /// Adds a material to selected materials. Used immediately after `[SELECT_MATERIAL]`.
    ///
    /// Appears as `PLUS_MATERIAL:Material`
    PlusMaterial {
        /// The material to add
        material: String,
    },
    /// The minimum/maximum numbers of how many of these creatures are present in each world map tile of the appropriate region. Defaults to 1:1 if not specified.
    ///
    /// Appears as `POPULATION_NUMBER:1:1`
    PopulationNumber {
        /// The minimum number of creatures per spawned cluster
        min: u32,
        /// The maximum number of creatures per spawned cluster
        max: u32,
    },
    /// Sets what other creatures prefer about this creature.
    ///
    /// "Urist likes dwarves for their beards."
    ///
    /// Multiple entries will be chosen from at random. Creatures lacking a PREFSTRING token will never appear under another's preferences.
    ///
    /// Appears as `PREFSTRING:PrefString`
    PrefString {
        /// The preference string
        pref_string: String,
    },
    /// The generic name for members of this profession, at the creature level. In order to give members of specific castes different names for professions,
    /// use `[CASTE_PROFESSION_NAME]` instead.
    ///
    /// Appears as `PROFESSION_NAME:ProfessionId:ProfessionName:ProfessionNames`
    ProfessionName {
        /// The profession id
        id: String,
        /// The name of the profession
        name: String,
        /// The plural name of the profession
        plural_name: String,
    },
    /// Removes a material from the creature.
    ///
    /// Appears as `REMOVE_MATERIAL:Material`
    RemoveMaterial {
        /// The material to remove
        material: String,
    },
    /// Removes a tissue from the creature.
    ///
    /// Appears as `REMOVE_TISSUE:Tissue`
    RemoveTissue {
        /// The tissue to remove
        tissue: String,
    },
    /// The creature will only show up in "savage" biomes. Has no effect on cavern creatures. Cannot be combined with `[GOOD]` or `[EVIL]`.
    ///
    /// Appears as `SAVAGE`
    Savage,
    /// Adds an additional previously defined caste to the selection. Used after `[SELECT_CASTE]`.
    ///
    /// Appears as `SELECT_ADDITIONAL_CASTE:Caste`
    SelectAdditionalCaste {
        /// The caste to add
        caste: String,
    },
    /// Selects a previously defined caste
    ///
    /// Appears as `SELECT_CASTE:Caste`
    SelectCaste {
        /// The caste to select
        caste: String,
    },
    /// Selects a locally defined material. Can be ALL.
    ///
    /// Appears as `SELECT_MATERIAL:Material`
    SelectMaterial {
        /// The material to select
        material: String,
    },
    /// Selects a tissue for editing.
    ///
    /// Appears as `SELECT_TISSUE:Tissue`
    SelectTissue {
        /// The tissue to select
        tissue: String,
    },
    /// Boasting speeches relating to killing this creature. Examples include `text_dwarf.txt` and `text_elf.txt` in `data\vanilla\vanilla_creatures\objects`.
    ///
    /// Appears as `SLAIN_CASTE:SomeSpeechSet`
    SlainSpeech {
        /// The speech set to use
        slain_speech: String,
    },
    /// Determines how keen a creature's sense of smell is - lower is better. At 10000, a creature cannot smell at all.
    ///
    /// Appears as `SMELL_TRIGGER:10000`
    SmellTrigger {
        /// The smell trigger
        smell_trigger: u32,
    },
    /// If this creature is active in its civilization's military, it will blink between its default tile and this one.
    ///
    /// Appears as `SOLDIER_ALTTILE:SomeTile`
    SoldierAltTile {
        /// The tile to use
        tile: String,
    },
    /// Found on generated angels. This is the historical figure ID of the deity with which the angel is associated. Since HFIDs are not predictable before worldgen,
    /// this isn't terribly usable in mods.
    ///
    /// Appears as `SOURCE_HFID:123`
    SourceHfid {
        /// The historical figure ID
        hfid: u32,
    },
    /// Sets what religious spheres the creature is aligned to, for purposes of being worshipped via the `[POWER]` token. Also affects the layout of hidden fun stuff,
    /// and the creature's name.
    ///
    /// Appears as `SPHERE:SomeSphere`
    Sphere {
        /// The sphere to use
        sphere: String,
    },
    /// Begins defining a tissue in the creature file. Follow this with standard tissue definition tokens to define the tissue properties.
    ///
    /// Arguments:
    ///
    /// * `name`: The name of the tissue
    ///
    /// Appears as `TISSUE:SomeTissue`
    Tissue {
        /// The name of the tissue
        name: String,
    },
    /// A large swarm of vermin can be disturbed, usually in adventurer mode.
    ///
    /// Appears as `TRIGGERABLE_GROUP:5:10`
    TriggerableGroup {
        /// The minimum number of vermin in the swarm
        min: u32,
        /// The maximum number of vermin in the swarm
        max: u32,
    },
    /// Creature will occur in every region with the correct biome. Does not apply to `[EVIL]`/`[GOOD]` tags.
    ///
    /// Appears as `UBIQUITOUS`
    Ubiquitous,
    /// Depth that the creature appears underground. Numbers can be from 0 to 5. 0 is actually 'above ground' and can be used if the creature is to appear both above and below ground.
    /// Values from 1-3 are the respective cavern levels, 4 is the magma sea and 5 is the HFS. A single argument may be used instead of min and max. Demons use only 5:5;
    /// user-defined creatures with both this depth and `[FLIER]` will take part in the initial wave from the HFS alongside generated demons, but without `[FLIER]` they will only spawn from
    /// the map edges. Civilizations that can use underground plants or animals will only export (via the embark screen or caravans) things that are available at depth 1.
    ///
    /// Arguments:
    ///
    /// * `min`: The minimum depth
    /// * `max`: The maximum depth
    ///
    /// Appears as `UNDERGROUND_DEPTH:1:3`
    UndergroundDepth {
        /// The minimum depth
        min: u32,
        /// The maximum depth
        max: u32,
    },
    /// Defines a new caste derived directly from a previous caste. The new caste inherits all properties of the old one. The effect of this tag is automatic if one has not yet defined any castes:
    /// "Any caste-level tag that occurs before castes are explicitly declared is saved up and placed on any caste that is declared later, unless the caste is explicitly derived from another caste."
    ///
    /// "When DF detects duplicate tokens in the raws of the same object, a failsafe seems to kick in; it takes the bottom-most of the duplicates, and disregards the others. In the case of tokens
    /// added by a mod, it prioritizes the duplicate in the mod." This means that if a tag is defined in the base-caste and redefined in the derived caste, the derived tag overwrites the base tag.
    ///
    /// Arguments:
    ///
    /// * `caste`: The name of the new caste
    /// * `original_caste`: The name of the original caste to copy
    ///
    /// Appears as `USE_CASTE:SomeCaste:SomeOriginalCaste`
    UseCaste {
        /// The name of the new caste
        caste: String,
        /// The name of the original caste to copy
        original_caste: String,
    },
    /// Defines a new local creature material and populates it with all properties defined in the specified local creature material.
    ///
    /// Arguments:
    ///
    /// * `material`: The name of the new material
    /// * `original_material`: The name of the original material to copy
    ///
    /// Appears as `USE_MATERIAL:SomeMaterial:SomeOriginalMaterial`
    UseMaterial {
        /// The name of the new material
        material: String,
        /// The name of the original material to copy
        original_material: String,
    },
    /// Defines a new local creature material and populates it with all properties defined in the specified template. There seems to be a limit of 200 materials per creature.
    ///
    /// Arguments:
    ///
    /// * `material`: The name of the new material
    /// * `template`: The name of the template to copy
    ///
    /// Appears as `USE_MATERIAL_TEMPLATE:SomeMaterial:SomeTemplate`
    UseMaterialTemplate {
        /// The name of the new material
        material: String,
        /// The name of the template to copy
        template: String,
    },
    /// Defines a new local creature tissue and populates it with all properties defined in the local tissue specified in the second argument.
    ///
    /// Arguments:
    ///
    /// * `tissue`: The name of the new tissue
    /// * `original_tissue`: The name of the original tissue to copy
    ///
    /// Appears as `USE_TISSUE:SomeTissue:SomeOriginalTissue`
    UseTissue {
        /// The name of the new tissue
        tissue: String,
        /// The name of the original tissue to copy
        original_tissue: String,
    },
    /// Loads a tissue template listed in `OBJECT:TISSUE_TEMPLATE` files, such as `tissue_template_default.txt`.
    ///
    /// Arguments:
    ///
    /// * `tissue`: The name of the new tissue
    /// * `template`: The name of the template to copy
    ///
    /// Appears as `USE_TISSUE_TEMPLATE:SomeTissue:SomeTemplate`
    UseTissueTemplate {
        /// The name of the new tissue
        tissue: String,
        /// The name of the template to copy
        template: String,
    },
    /// Changes the language of the creature into unintelligible 'kobold-speak', which creatures of other species will be unable to understand. If a civilized creature has this and is not
    /// part of a `[SKULKING]` civ, it will tend to start wars with all nearby civilizations and will be unable to make peace treaties due to 'inability to communicate'.
    ///
    /// Appears as `UTTERNANCES`
    Utterances,
    /// The vermin creature will attempt to eat exposed food. See `[PENETRATEPOWER]`. Distinct from `[VERMIN_ROTTER]`.
    ///
    /// Appears as `VERMIN_EATER`
    VerminEater,
    /// The vermin appears in water and will attempt to swim around.
    ///
    /// Appears as `VERMIN_FISH`
    VerminFish,
    /// The creature appears in "general" surface ground locations. Note that this doesn't stop the creature from flying if it can (most vermin birds have this tag).
    ///
    /// Appears as `VERMIN_GROUNDER`
    VerminGrounder,
    /// The vermin are attracted to rotting stuff and loose food left in the open and cause unhappy thoughts to dwarves who encounter them. Present on flies, knuckle worms,
    /// acorn flies, and blood gnats. Speeds up decay?
    ///
    /// Appears as `VERMIN_ROTTER`
    VerminRotter,
    /// The creature randomly appears near dirt or mud, and may be uncovered by creatures that have the `[ROOT_AROUND]` interaction such as geese and chickens.
    /// Dwarves will ignore the creature when given the "Capture live land animal" task.
    ///
    /// Appears as `VERMIN_SOIL`
    VerminSoil,
    /// The vermin will appear in a single tile cluster of many vermin, such as a colony of ants.
    ///
    /// Appears as `VERMIN_SOIL_COLONY`
    VerminSoilColony,
    /// An unknown tag.
    #[default]
    Unknown,
    // Tokens found in the legends xml exports but not in the raws
    /// Mates to breed
    MatesToBreed,
    /// Has two genders
    TwoGenders,
    /// All castes are alive
    AllCastesAlive,
    /// Is a small race
    SmallRace,
    /// Occurs as an entity
    OccursAsEntityRace,
    /// Equipment used
    Equipment,
}

impl std::fmt::Display for CreatureToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
