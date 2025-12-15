//! Tokens that can be found in an entity raw file.

/// Tokens that can be found in an entity raw file.
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type,
)]
pub enum EntityTag {
    /// Allows adventure mode for entities with sites.
    AllMainPopsControllable,
    /// Allows fortress mode. If multiple entities have the `SITE_CONTROLLABLE` token, then at embark the specific civs can be chosen on
    /// the civ list screen. At least one civilization must have this token.
    SiteControllable,
    /// Arguments: creature
    ///
    /// The type of creature that will inhabit the civilization. If multiple creature types are specified, each civilization will randomly
    /// choose one of the creatures. In entities with multiple possible creatures, you can manipulate the chance of one creature being
    /// chosen by adding multiple identical creature tags. For instance adding `[CREATURE:DWARF][CREATURE:DWARF][CREATURE:DWARF][CREATURE:ELF]`
    /// to the same entity will make the civs created about 75% dwarven, 25% elven. It should be noted that civilizations are in general
    /// weighted by this token.
    ///
    /// For example, if you have one entity with three `[CREATURE:DWARF]` entries and another separate entity with a single `[CREATURE:ELF]` entry,
    /// then you can expect to see three times as many of the former placed as the latter.
    Creature,
    /// Arguments: number (integer)
    ///
    /// Found on generated angel entities. Appears to draw from creatures with this HFID, which associates the entity with a historical
    /// figure of the same ID corresponding to a deity.
    SourceHfid,
    // # Placement Tokens #
    /// Arguments: biome, frequency
    ///
    /// Controls the expansion of the civilization's territory. The higher the number is relative to other `BIOME_SUPPORT` tokens in the entity,
    /// the faster it can spread through the biome. These numbers are evaluated relative to each other, i.e. if one biome is 1 and the other is 2,
    /// the spread will be the same as if one was 100 and the other was 200. Civs can spread out over biomes they cannot actually build in;
    ///
    /// For example, humans spread quickly over oceans but cannot actually build in them.
    ///
    /// e.g. `[BIOME_SUPPORT:ANY_GRASSLAND:4]`
    BiomeSupport,
    /// Arguments: biome
    ///
    /// If the civ's territory crosses over this biome, it can build settlements here.
    ///
    /// e.g. `[SETTLEMENT_BIOME:ANY_GRASSLAND]`
    SettlementBiome,
    /// Arguments: biome
    ///
    /// Combination of `EXCLUSIVE_START_BIOME` and `SETTLEMENT_BIOME`; allows the civ to start in and create settlements in the biome.
    ///
    /// e.g. `[START_BIOME:ANY_FOREST]`
    StartBiome,
    /// Arguments: biome
    ///
    /// The birth of the civilization can occur in this biome, but cannot (necessarily) build in it.
    /// If the civ does not have `SETTLEMENT_BIOME` or `START_BIOME` for the biome in question, it will only construct a single settlement there.
    ///
    /// e.g. `[EXCLUSIVE_START_BIOME:MOUNTAIN]`
    ExclusiveStartBiome,
    /// Arguments: site type
    ///
    /// Valid site types are `DARK_FORTRESS` (π), `CAVE` (•), `CAVE_DETAILED` (Ω), `TREE_CITY` (î), and `CITY` (#).
    /// Also recognizes `PLAYER_FORTRESS` (creates a civ of hillocks only), and `MONUMENT` (creates a civ without visible sites
    /// (except tombs and castles), but may cause worldgen crashes). `FORTRESS` is no longer a valid entry, castles are
    /// currently controlled by `BUILDS_OUTDOOR_FORTIFICATIONS`. Defaults to `CITY`. Selecting `CAVE` causes the classic kobold behavior
    /// of not showing up on the "neighbors" section of the site selection screen. Selecting `DARK_FORTRESS` also allows generation
    /// of certain other structures. It also gives the civ a special overlord.
    ///
    /// `CAVE_DETAILED` civilizations will create fortresses in mountainous regions and hillocks in non-mountainous regions.
    ///
    /// e.g. `[DEFAULT_SITE_TYPE:CAVE_DETAILED]`
    DefaultSiteType,
    /// Arguments: site type
    ///
    /// Most residents will try to move to this site type, unless already at one.
    ///
    /// e.g. `[LIKES_SITE:CAVE_DETAILED]`
    LikesSite,
    /// Arguments: site type
    ///
    /// Some residents will try to move to this site type, unless already at one.
    ///
    /// e.g. `[TOLERATES_SITE:CITY]`
    ToleratesSite,
    /// Arguments: construction
    ///
    /// Controls which constructions the civ will build on the world map. Valid constructions are ROAD, TUNNEL, BRIDGE, and WALL.
    ///
    /// e.g. `[WORLD_CONSTRUCTION:BRIDGE] [WORLD_CONSTRUCTION:ROAD] [WORLD_CONSTRUCTION:TUNNEL] [WORLD_CONSTRUCTION:WALL]`
    WorldConstruction,
    //# Population Tokens #
    /// Arguments: number
    ///
    /// Max historical population per entity. Multiply this by max starting civ to get the total maximum historical population of the species.
    ///
    /// Defaults to 500.
    ///
    /// e.g. `[MAX_POP_NUMBER:500]`
    MaxPopNumber,
    /// Arguments: number
    ///
    /// Max historical population per individual site.
    ///
    /// Defaults to 50.
    ///
    /// e.g. `[MAX_SITE_POP_NUMBER:200]`
    MaxSitePopNumber,
    /// Arguments: number
    ///
    /// Max number of civ to spawn at world generation. Worldgen picks entities in some sequential order from the raws,
    /// and once it reaches the end of the list, it will begin again at the top. Setting this number lower than 100,
    /// like say, 7, will cause worldgen to skip over the civ for placement if there are already 7 civs of this type.
    ///
    /// Note that if all civs are set to lower numbers, and the number of starting civs is set higher than the
    /// maximum possible amount of civs in total, it will gracefully stop placing civs and get down to the history
    /// aspect of worldgen.
    ///
    /// Defaults to 3.
    ///
    /// e.g `[MAX_STARTING_CIV_NUMBER:3]`
    MaxStartingCivNumber,
    // # Flavor Tokens #
    /// Arguments: building name
    ///
    /// The named, custom building can be built by a civilization in Fortress Mode.
    ///
    /// e.g. `[PERMITTED_BUILDING:SOAP_MAKER]`
    PermittedBuilding,
    /// Arguments: profession
    ///
    /// Allows this job type to be selected. This applies to worldgen creatures, in the embark screen, and in play.
    /// Certain professions also influence the availability of materials for trade.
    ///
    /// e.g. `[PERMITTED_JOB:MINER]`
    PermittedJob,
    /// Arguments: reaction name
    ///
    /// Allows this reaction to be used by a civilization. It is used primarily in Fortress Mode,
    /// but also allows certain resources, such as steel, to be available to a race. When creating custom reactions,
    /// this token must be present or the player will not be able to use the reaction in Fortress Mode.
    ///
    /// e.g. `[PERMITTED_REACTION:TAN_A_HIDE]`
    PermittedReaction,
    /// Causes the civ's currency to be numbered with the year it was minted.
    CurrencyByYear,
    /// Arguments: inorganic material, value
    ///
    /// What kind of metals the civ uses for coin minting, as well as the value of the coin.
    /// Due to the Dwarven economy having been disabled since version 0.31, the value doesn't actually do anything.
    ///
    /// e.g `[CURRENCY:SILVER:5]`
    Currency,
    /// Arguments: type, number
    ///
    /// `OWN_RACE`, `FANCIFUL`, `EVIL`, `GOOD`
    ///
    /// Number goes from `0` to `25_600` where `256` is the default.
    ///
    /// e.g. `[ART_FACET_MODIFIER:OWN_RACE:512]`
    ArtFacetModifier,
    /// Arguments: item, number
    ///
    /// Allowed item: CREATURE, PLANT, TREE, SHAPE, ITEM
    ///
    /// Allowed number: 0-25600
    ///
    /// Determines the chance of each image occurring in that entity's artwork, such as engravings and on artifacts,
    /// for default (non-historical) artwork.
    ///
    /// e.g. `[ART_IMAGE_ELEMENT_MODIFIER:TREE:512]`
    ArtImageElementModifier,
    /// Arguments: item, number
    ///
    /// Allowed item: `ART_IMAGE`, `COVERED` or `GLAZED`, `RINGS_HANGING`, `BANDS`, `SPIKES`, `ITEMSPECIFIC`, `THREAD`, `CLOTH`, `SEWN_IMAGE`
    ///
    /// Allowed number: 0-25600
    ///
    /// Determines the chance of the entity using that particular artwork method, such as "encircled with bands" or "menaces with spikes".
    ///
    /// `[ITEM_IMPROVEMENT_MODIFIER:SPIKES:0]`
    ///
    /// This also seems to change the amount that the entity will pay for items that are improved in these ways in their tokens.
    ItemImprovementModifier,
    /// Arguments: language
    ///
    /// What language raw the entity uses.
    ///
    /// - If an entity lacks this tag, translations are drawn randomly from all translation files. Multiple translation tags will only
    ///   result in the last one being used. Migrants will sometimes arrive with no name.
    /// - If `GEN_DIVINE` is entered, the entity will use a generated divine language, that is, the same language that is used for the names of angels.
    ///
    /// e.g. `[TRANSLATION:DWARF]`
    Translation,
    /// Arguments: noun, symbol
    ///
    /// Allowed Values:
    /// `ALL`, `REMAINING`, `BATTLE`, `BRIDGE`, `CIV`, `CRAFT_GUILD`, `FESTIVAL`, `LIBRARY`, `MERCHANT_COMPANY`, `MILITARY_UNIT`,
    /// `OTHER`, `RELIGION`, `ROAD`, `SIEGE`, `SITE`, `TEMPLE`, `TUNNEL`, `VESSEL`, `WALL`, `WAR`
    ///
    /// Causes the entity to more often use these symbols in the particular SYM set.
    ///
    /// REMAINING will select all symbols that have not already been declared above it.
    ///
    /// e.g. `[SELECT_SYMBOL:ALL:PEACE]`
    SelectSymbol,
    /// Arguments: noun, symbol
    ///
    /// Causes the symbol set to be preferred as adjectives by the civilization. Used in vanilla to put violent names in sieges and battles.
    ///
    /// e.g. `[SELECT_SYMBOL:SIEGE:NAME_SIEGE] [SUBSELECT_SYMBOL:SIEGE:VIOLENT]`
    SubselectSymbol,
    /// Arguments: noun, symbol
    ///
    /// Causes the entity to not use the words in these SYM sets.
    ///
    /// e.g. `[CULL_SYMBOL:ALL:UGLY]`
    CullSymbol,
    /// Arguments: color
    ///
    /// The color of this entity's civilization settlements in the world gen and embark screens, also used when announcing arrival of their caravan.
    ///
    /// Defaults to 7:0:1.
    ///
    /// e.g. `[FRIENDLY_COLOR:1:0:1]`
    FriendlyColor,
    // # Religion Tokens #
    /// Arguments: type
    ///
    /// - `REGIONAL_FORCE`: The creatures will worship a single force associated with the terrain of their initial biome.
    /// - `PANTHEON`: The creatures will worship a group of gods, each aligned with their spheres and other appropriate ones as well.
    ///
    /// e.g. `[RELIGION:PANTHEON]`
    Religion,
    /// Arguments: sphere
    ///
    /// Can be any available sphere - multiple entries are possible. Choosing a religious sphere will automatically make
    /// its opposing sphere not possible for the species to have: adding WATER, for example, means civilizations of this entity will
    /// never get FIRE as a religious sphere. Note that the DEATH sphere favours the appearance of necromancers
    /// (and therefore, towers) "in" the entity.
    ///
    /// e.g. `[RELIGION_SPHERE:FORTRESSES]`
    ReligionSphere,
    /// Arguments: sphere, number
    ///
    /// This token forces an entity to favor or disfavor particular religious spheres, causing them to acquire those spheres more
    /// often when generating a pantheon.
    ///
    /// Default is 256, minimum is 0, maximum is 25600.
    ///
    /// e.g. `[SPHERE_ALIGNMENT:TREES:512]`
    SphereAlignment,
    // # Position Tokens #
    /// Defines a leader/noble position for a civilization. These replace previous tags such as `[MAYOR]` and `[CAN_HAVE_SITE_LEADER]` and so on.
    ///
    /// To define a position further, see Position token.
    Position,
    /// Arguments: land  holder ID, population, wealth exported, wealth created
    ///
    /// Defines when a particular land-holding noble (baron, count, duke in vanilla) will arrive at a fortress.
    ///
    /// As of version 0.44.11, however, this is obsolete due to the changes in how sites are elevated in status.
    LandHolderTrigger,
    /// Arguments: position responsibility or 'ALL'
    ///
    /// Allows a site responsibility to be taken up by a dynamically generated position (lords, hearth-persons, etc.).
    /// Any defined positions holding a given responsibility will take precedence over generated positions for that responsibility.
    /// Also appears to cause site disputes.
    SiteVariablePositions,
    /// Arguments: position responsibility or 'ALL'
    ///
    /// Allows a responsibility to be taken up by a dynamically generated position (such as Law-maker).
    /// Any defined positions holding a given responsibility will take precedence over generated positions for that responsibility.
    VariablePositions,
    // # Behavior Tokens #
    /// Arguments: behavior, action
    ///
    /// Sets the civ's view of ethics (certain behaviors), from capital punishment to completely acceptable.
    /// This also causes the civ to look upon opposing ethics with disfavor if their reaction to it is opposing,
    /// and when at extremes (one ACCEPTABLE, another civ UNTHINKABLE; for example) they will often go to war over it.
    ///
    /// e.g. `[ETHIC:EAT_SAPIENT_KILL:ACCEPTABLE]`
    Ethic,
    /// Arguments: value, number
    ///
    /// Sets the civ's cultural values. Numbers range from -50 (complete anathema) to 0 (neutral) to 50 (highly valued).
    ///
    /// e.g. `[VALUE:CRAFTSMANSHIP:50]`
    ///
    /// Certain values must be set to 15 or more for civilizations to create structures and form entities during history gen:
    ///
    /// - 15+ KNOWLEDGE for libraries
    /// - 15+ COOPERATION and 15+ CRAFTSMANSHIP for craft guilds
    ///
    /// Guilds also need guild-valid professions (see `PERMITTED_JOB`)
    Value,
    /// Arguments: value or `ALL`, min, max
    ///
    /// Makes values randomized rather than specified.
    ///
    /// This tag overrides the VALUE tag. Using `[VARIABLE_VALUE:ALL:x:y]` and then overwriting single values with further
    ///
    /// e.g. `[VARIABLE_VALUE:value:x:y]` tags works
    VariableValue,
    /// Makes the civ's traders accept offered goods.
    WillAcceptTribute,
    /// The civ will send out Wanderer adventurers in worldgen, which seems to increase Tracker skill.
    ///
    /// These types of adventurers will sometimes be seen leading a battle (instead of war leaders or generals) in remote locations during world-gen, in charge of the defenders.
    ///
    /// Mercenaries and monster hunters from the civ may visit player's fortress and petition for residency there to enlist in the military or hunt monsters in caverns, respectively.
    Wanderer,
    /// The civ will send out `BeastHunter` adventurers in worldgen, which seems to increase Tracker skill.
    ///
    /// These types of adventurers will sometimes be seen leading a battle (instead of war leaders or generals) in remote locations during world-gen, in charge of the defenders.
    ///
    /// Mercenaries and monster hunters from the civ may visit player's fortress and petition for residency there to enlist in the military or hunt monsters in caverns, respectively.
    BeastHunter,
    /// The civ will send out Scout adventurers in worldgen, which seems to increase Tracker skill.
    ///
    /// These types of adventurers will sometimes be seen leading a battle (instead of war leaders or generals) in remote locations during world-gen, in charge of the defenders.
    ///
    /// Mercenaries and monster hunters from the civ may visit player's fortress and petition for residency there to enlist in the military or hunt monsters in caverns, respectively.
    Scout,
    /// The civ will send out Mercenary adventurers in worldgen, which seems to increase Tracker skill.
    ///
    /// These types of adventurers will sometimes be seen leading a battle (instead of war leaders or generals) in remote locations during world-gen, in charge of the defenders.
    ///
    /// Mercenaries and monster hunters from the civ may visit player's fortress and petition for residency there to enlist in the military or hunt monsters in caverns, respectively.
    Mercenary,
    /// The civilization will mutilate bodies when they are the victors in history-gen warfare, such as hanging bodies from trees, putting them on spikes, and so forth.
    /// Adventurers killed in Adventurer mode will sometimes be impaled on spikes wherever they died, with or without this token,
    /// and regardless of whether they actually antagonized the townspeople.
    AbuseBodies,
    /// Arguments: season
    ///
    /// The season when the civ is most active: when they will trade, interact with you via diplomats, and/or invade you.
    /// Civilizations can have multiple season entries. Note: If multiple caravans arrive at the same time, you are able to select
    /// which civ to trade with at the depot menu. `ACTIVE_SEASON` tags may be changed for a currently active fort.
    ///
    /// e.g. `[ACTIVE_SEASON:SUMMER]`
    ActiveSeason,
    /// When invading, sneaks around and shoots at straggling members of your society. They will spawn on the edge of the map and will only be visible when
    /// one of their party are spotted; this can be quite dangerous to undefended trade depots. If the civilization also has the SIEGER token,
    /// they will eventually ramp it up to less subtle means of warfare.
    Ambusher,
    /// Will not attack wildlife, and will not be attacked by them, even if you have them in your party. This can be somewhat disconcerting when attacked
    /// by bears in the forest, and your elven ally sits back and does nothing. Additionally, this token determines if the entity can settle in savage biomes.
    AtPeaceWithWildlife,
    /// Sends thieves to steal babies. Without this tag (or `AMBUSHER`, or `ITEM_THIEF`), enemy civilizations will only siege (if capable),
    /// and will siege as early as they would otherwise babysnatch. This can happen as early as the first year of the fort!
    /// In addition, babysnatcher civilizations will snatch children during worldgen, allowing them to become part of the civ if they do not escape.
    ///
    /// Note: If the playable civ in fortress mode has this tag (e.g. you add BABYSNATCHER to the dwarf entity) then the roles will be reversed ==>
    /// elves and humans will siege and ambush and goblins will be friendly to you.
    /// However, animals traded away to one's own caravan will count as snatched, reported upon the animal leaving the map,
    /// and the animal will not count as having been exported.
    BabySnatcher,
    /// Makes the civilization build castles from mead halls. Only functions when the type of site built is a hamlet/town.
    /// This, combined with the correct type of position associated with a site, is why adventurers can only lay claim to human sites.
    BuildsOutdoorFortifications,
    /// Makes the civilization build tombs.
    BuildsOutdoorTombs,
    /// Arguments: percentage
    ///
    /// Sets a percentage of the entity population to be used as bandits.
    Banditry,
    /// Visiting diplomats are accompanied by a pair of soldiers.
    DiplomatBodyguards,
    /// Found on generated divine "HF Guardian Entities". Cannot be used in user-defined raws.
    Generated,
    /// Causes invaders to ignore visiting caravans and other neutral creatures
    InvadersIgnoreNeutrals,
    /// Sends thieves to steal items. This will also occur in history generation, and thieves will have the "thief" profession.
    /// Items stolen in history gen will be scattered around that creature's home.
    ///
    /// Also causes that civ to be hostile to any entity without this token. Without this tag (or AMBUSHER, or BABYSNATCHER), enemy civs will only siege
    /// (if capable), and will siege as early as they would otherwise steal.
    ///
    /// Note: If the playable civ in Fortress Mode has this tag (e.g. you add `ITEM_THIEF` to the Dwarf entity) then the roles will be reversed ==>
    /// elves and humans will siege and ambush and kobolds will be friendly to you. However, ALL items traded away to one's own caravan will count as stolen,
    /// reported when the items leave the map, and the stolen items will not count as exported
    ItemThief,
    /// Causes the entity to send out patrols that can ambush adventurers. Said patrols will be hostile to any adventurers they encounter,
    /// regardless of race or nationality.
    LocalBanditry,
    /// Caravan merchants are accompanied by soldiers.
    MerchantBodyguards,
    /// Merchants will engage in cross-civ trading and form companies.
    ///
    /// In previous versions, this resulted in the civ having a Guild Representative / Merchant Baron / Merchant Prince,
    /// but now this is controlled solely by positions
    MerchantNobility,
    /// Arguments: level
    ///
    /// 0 to 5, civ will come to site once population at site has reached that level. If multiple progress triggers exist for a civ,
    /// it will come when any one of them is fulfilled instead of waiting for all of them to be reached.
    ///
    /// - A value of 0 disables the trigger.
    /// - 1 corresponds to 20 dwarves,
    /// - 2 to 50 dwarves,
    /// - 3 to 80,
    /// - 4 to 110, and
    /// - 5 to 140.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    ///
    /// Note: hostile civs require that this be fulfilled as well as at least one other non-siege trigger before visiting for non-siege activities.
    ProgressTriggerPopulation,
    /// Arguments: level
    ///
    /// 0 to 5, civ will come to site once created wealth has reached that level. If multiple progress triggers exist for a civ,
    /// it will come when any one of them is fulfilled instead of waiting for all of them to be reached.
    ///
    /// - A value of 0 disables the trigger.
    /// - 1 corresponds to 5000☼ created wealth,
    /// - 2 to 25000☼,
    /// - 3 to 100000☼,
    /// - 4 to 200000☼, and
    /// - 5 to 300000☼.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    ProgressTriggerProduction,
    /// Arguments: level
    ///
    /// 0 to 5, civ will come to site once exported goods has reached that level. If multiple progress triggers exist for a civ,
    /// it will come when any one of them is fulfilled instead of waiting for all of them to be reached.
    ///
    /// - A value of 0 disables the trigger.
    /// - 1 corresponds to 500☼ exported wealth,
    /// - 2 to 2500☼,
    /// - 3 to 10000☼,
    /// - 4 to 20000☼, and
    /// - 5 to 30000☼.
    ///
    /// Progress triggers may be changed, added, or deleted for a currently active fort.
    ProgressTriggerTrade,
    /// Arguments: level
    ///
    /// 0 to 5, civ will begin to send sieges against the player civ when this level is reached if it is hostile.
    ///
    /// If multiple progress triggers exist for a civ, it will come when any one of them is fulfilled instead of
    /// waiting for all of them to be reached. A value of 0 disables the trigger
    ProgressTriggerPopulationSiege,
    /// Arguments: level
    ///
    /// 0 to 5, civ will begin to send sieges against the player civ when this level is reached if it is hostile.
    ///
    /// If multiple progress triggers exist for a civ, it will come when any one of them is fulfilled instead of
    /// waiting for all of them to be reached. A value of 0 disables the trigger
    ProgressTriggerProductionSiege,
    /// Arguments: level
    ///
    /// 0 to 5, civ will begin to send sieges against the player civ when this level is reached if it is hostile.
    ///
    /// If multiple progress triggers exist for a civ, it will come when any one of them is fulfilled instead of
    /// waiting for all of them to be reached. A value of 0 disables the trigger
    ProgressTriggerTradeSiege,
    /// Will start campfires and wait around at the edge of your map for a month or two before rushing in to attack.
    /// This will occur when the progress triggers for sieging are reached. If the civ lacks smaller methods of conflict
    /// (`AMBUSHER`, `BABYSNATCHER`, `ITEM_THIEF`), they will instead send smaller-scale sieges when their triggers for
    /// "first contact" are reached.
    Sieger,
    /// Guards certain special sites, such as a vault belonging to a demon allied with a deity. Used in generated divine entities.
    SiteGuardian,
    /// This makes the severity of attacks depend on the extent of item/baby thievery rather than the passage of time.
    /// Designed to go with `ITEM_THIEF`, may or may not work with BABYSNATCHER. Prevents the civ from engaging in diplomacy
    /// or ending up at war.
    Skulking,
    /// Visiting diplomats impose tree cutting quotas; without this, they will simply compliment your fortress and leave.
    /// Also causes the diplomat to make unannounced first contact at the very beginning of the first spring after your
    /// fortress becomes a land holder.
    TreeCapDiplomacy,
    /// Defines if a civilization is a hidden subterranean entity, such as bat man civilizations.
    /// May spawn in any of the three caverns; cavern dweller raids due to agitation will pull from these.
    /// If you embark as this civ, you have access to pets and trees from all three layers, not only the first.
    LayerLinked,
    /// Makes civilizations generate keyboard instruments
    GenerateKeyboardInstruments,
    /// Makes civilizations generate percussion instruments
    GeneratePercussionInstruments,
    /// Makes civilizations generate stringed instruments
    GenerateStringedInstruments,
    /// Makes civilizations generate wind instruments
    GenerateWindInstruments,
    /// Makes civilizations generate dance forms.
    GenerateDanceForms,
    /// Makes civilizations generate musical forms.
    GenerateMusicalForms,
    /// Makes civilizations generate poetic forms.
    GeneratePoeticForms,
    /// Arguments: scholar type
    ///
    /// `ALL`, `ASTRONOMER`, `CHEMIST`, `DOCTOR`, `ENGINEER`, `GEOGRAPHER`, `HISTORIAN`, `MATHEMATICIAN`, `NATURALIST`, `PHILOSOPHER`
    Scholar,
    /// Generates scholars based on the values generated with the `VARIABLE_VALUE` tag.
    SetScholarsOnValuesAndJobs,
    /// Used for kobolds.
    NoArtifactClaims,
    /// The civilization can breach the Underworld during world generation.
    MiningUnderworldDisasters,

    // # Available Resources #
    /// Arguments: `item_token`
    ///
    /// Used before a ranged weapon type.
    ///
    /// e.g. `[AMMO:ITEM_AMMO_BOLTS]`
    Ammo,
    /// Arguments: `item_token`, rarity
    ///
    /// Rarity is optional, and valid values are FORCED, COMMON, UNCOMMON, and RARE (anything else is treated as COMMON).
    /// FORCED items will be available 100% of the time, COMMON items 50%, UNCOMMON items 10%, and RARE items 1%.
    /// If certain armor types are lacking after performing one pass of randomized checks, the game will repeat random checks
    /// until an option is successfully chosen.
    ///
    /// e.g. `[ARMOR:ITEM_ARMOR_PLATEMAIL:COMMON]`
    Armor,
    /// Arguments: `item_token`
    ///
    /// Causes the selected weapon to fall under the "digging tools" section of the embark screen.
    /// Also forces the weapon to be made out of metal, which can cause issues if a modded entity has access to picks without
    /// access to metal - for those cases, listing the pick under the `[WEAPON]` token works just as well. Note that this tag is
    /// neither necessary nor sufficient to allow use of that item as a mining tool -–
    /// for that, the item itself needs to be a weapon with `[SKILL:MINING]`.
    ///
    /// e.g. `[DIGGER:ITEM_WEAPON_PICK]`
    Digger,
    /// Arguments: `item_token`, `rarity`
    ///
    /// Rarity is optional, and valid values are FORCED, COMMON, UNCOMMON, and RARE (anything else is treated as COMMON).
    /// Uses the same rarity values and methods as outlined in ARMOR.
    ///
    /// e.g. `[GLOVES:ITEM_GLOVES_GAUNTLETS:COMMON]`
    Gloves,
    /// Arguments: `item_token`, `rarity`
    ///
    /// Rarity is optional, and valid values are FORCED, COMMON, UNCOMMON, and RARE (anything else is treated as COMMON).
    /// Uses the same rarity values and methods as outlined in ARMOR.
    ///
    /// e.g. `[HELM:ITEM_HELM_HELM:COMMON]`
    Helm,
    /// Arguments: `item_token`
    ///
    /// No longer used as of Version 0.42.01 due to the ability to generate instruments in world generation.
    ///
    /// It is still usable if pre-defined instruments are modded in, and generated musical forms are capable
    /// of selecting pre-defined instruments to use. However, reactions for making instruments, instrument parts,
    /// and/or assembling such instruments need to be added as well, as this token no longer adds such instruments
    /// to the craftsdwarf workshop menu.
    ///
    /// e.g. `[INSTRUMENT:ITEM_INSTRUMENT_FLUTE]`
    Instrument,
    /// Arguments: `item_token`, `rarity`
    ///
    /// Rarity is optional, and valid values are FORCED, COMMON, UNCOMMON, and RARE (anything else is treated as COMMON).
    /// Uses the same rarity values and methods as outlined in ARMOR.
    ///
    /// e.g. `[PANTS:ITEM_PANTS_LEGGINGS:COMMON]`
    Pants,
    /// Arguments: `item_token`
    ///
    /// e.g. `[SHIELD:ITEM_SHIELD_BUCKLER]`
    Shield,
    /// Arguments: `item_token`, `rarity`
    ///
    /// Rarity is optional, and valid values are FORCED, COMMON, UNCOMMON, and RARE (anything else is treated as COMMON).
    /// Uses the same rarity values and methods as outlined in ARMOR.
    ///
    /// e.g. `[SHOES:ITEM_SHOES_BOOTS:COMMON]`
    Shoes,
    /// Arguments: `item_token`
    ///
    /// e.g. `[SIEGEAMMO:ITEM_SIEGEAMMO_BALLISTA]`
    SiegeAmmo,
    /// Arguments: `item_token`
    ///
    /// e.g. `[TOOL:ITEM_TOOL_NEST_BOX]`
    Tool,
    /// Arguments: `item_token`
    ///
    /// e.g. `[TOY:ITEM_TOY_PUZZLEBOX]`
    Toy,
    /// Arguments: `item_token`
    ///
    /// e.g. `[TRAPCOMP:ITEM_TRAPCOMP_GIANTAXEBLADE]`
    TrapComponent,
    /// Arguments: `item_token`
    ///
    /// While this does not accept a rarity value, something similar can be achieved by having multiple variations of a weapon type
    /// with small differences and specifying each of them.
    ///
    /// e.g. `[WEAPON:ITEM_WEAPON_AXE_BATTLE]`
    Weapon,
    /// Allows use of products made from animals. All relevant creatures will be able to provide wool, silk, and extracts (including milk and venom)
    /// for trade, and non-sentient creatures (unless ethics state otherwise) will be able to provide eggs, caught fish, meat, leather, bone,
    /// shell, pearl, horn, and ivory.
    UseAnimalProducts,
    /// Any creature in the civilization's list of usable's (from the surrounding 7x7 or so of squares and map features in those squares)
    /// which has `PET` or `PET_EXOTIC` will be available as a pet, pack animal (with `PACK_ANIMAL`),
    /// wagon puller (with `WAGON_PULLER`),
    /// mount (with `MOUNT` or `MOUNT_EXOTIC`), or
    /// siege minion (with `TRAINABLE_WAR ` and without `CAN_LEARN`).
    ///
    /// This notion of the initial usable creature list, which then gets pared down or otherwise considered, applies below as well.
    ///
    /// All common domestic and equipment creatures are also added to the initial list.
    UseAnyPetRace,
    /// Without this, creatures with exclusively subterranean biomes are skipped.
    ///
    /// If they have it, cave creatures with PET will also be available as pets, pack animals (with `PACK_ANIMAL`), wagon pullers (with `WAGON_PULLER`),
    /// mounts (with `MOUNT` or `MOUNT_EXOTIC`), and siege minions (with `TRAINABLE_WAR` and without `CAN_LEARN`).
    UseCaveAnimals,
    /// Without this, `EVIL` creatures are skipped.
    ///
    /// Otherwise, evil creatures with `SLOW_LEARNER` or without `CAN_LEARN` will be also available as pets (with `PET`),
    /// pack animals (with `PACK_ANIMAL`), wagon pullers (with `WAGON_PULLER`), mounts (with `MOUNT` or `MOUNT_EXOTIC`),
    /// and siege minions (with `TRAINABLE_WAR` or `SLOW_LEARNER`), even the normally untameable species.
    UseEvilAnimals,
    /// Same as `USE_EVIL_ANIMALS` for all uses of plants.
    UseEvilPlants,
    /// Same as `USE_EVIL_ANIMALS` for all uses of wood.
    UseEvilWood,
    /// Without this `GOOD` creatures are skipped, otherwise, good creatures without `CAN_LEARN` will also be available as pets (with `PET`),
    /// pack animals (with `PACK_ANIMAL`), wagon pullers (with `WAGON_PULLER`), mounts (with `MOUNT` or `MOUNT_EXOTIC`), and siege minions
    /// (with `TRAINABLE_WAR`), even the normally untameable species.
    UseGoodAnimals,
    /// Same as `USE_GOOD_ANIMALS` for all uses of plants.
    UseGoodPlants,
    /// Same as `USE_GOOD_ANIMALS` for all uses of wood.
    UseGoodWood,
    /// If the relevant professions are permitted, controls availability of lye (`LYE_MAKING`), charcoal (`BURN_WOOD`), and potash (`POTASH_MAKING`).
    UseMiscProcessedWoodProducts,
    /// Makes the civilization use all locally available non-exotic pets.
    UseNoneExoticPetRace,
    /// Gives the civilization access to creatures with `COMMON_DOMESTIC` and `MOUNT`. Additionally, all available
    /// (based on `USE_ANY_PET_RACE`, `USE_CAVE_ANIMALS`, `USE_GOOD_ANIMALS`, and `USE_EVIL_ANIMALS`) creature with `MOUNT` and `PET`
    /// will be allowed for use as mounts during combat.
    CommonDomesticMount,
    /// Gives the civilization access to creatures with `COMMON_DOMESTIC` and `PACK_ANIMAL`.
    /// Additionally, all available (see above) creatures with `PACK_ANIMAL` and `PET` will be allowed for use during trade as pack animals.
    CommonDomesticPackAnimal,
    /// Gives the civilization access to creatures with `COMMON_DOMESTIC` and `PET`.
    /// Additionally, all available (see above) creatures with PET will be allowed for use as pets.
    CommonDomesticPet,
    /// Gives the civilization access to creatures with `COMMON_DOMESTIC` and `WAGON_PULLER`. Additionally, all
    /// available (see above) creatures with `WAGON_PULLER` and PET will be allowed for use during trade to pull wagons.
    CommonDomesticPullAnimal,
    /// Allow use of river products in the goods available for trade.
    RiverProducts,
    /// Allow use of ocean products (including amber and coral) in the goods available for trade.
    /// Without `OCEAN_PRODUCTS`, civilizations will not be able to trade ocean fish even if they are also
    /// available from other sources (e.g. sturgeons and stingrays).
    OceanProducts,
    /// Allow use of underground plant products in the goods available for trade.
    /// Lack of suitable vegetation in the caverns will cause worldgen rejections.
    IndoorFarming,
    /// Allow use of outdoor plant products in the goods available for trade.
    /// Lack of suitable vegetation in this civilization's starting area will cause worldgen rejections.
    OutdoorFarming,
    /// Allow use of underground plant growths (quarry bush leaves, in unmodded games) in the goods available for trade.
    IndoorGardens,
    /// Allow use of outdoor plant growths in the goods available for trade.
    OutdoorGardens,
    /// Allows use of indoor tree growths in the goods available for trade.
    /// Not used in vanilla entities, as vanilla underground trees do not grow fruit.
    ///
    /// Needs `INDOOR_WOOD` to function.
    ///
    /// Will cause rejections, if growths are unavailable.
    IndoorOrchards,
    /// Allows use of outdoor tree growths in the goods available for trade.
    ///
    /// Needs `OUTDOOR_WOOD` to function.
    OutdoorOrchards,
    /// Civilization members will attempt to wear clothing.
    Clothing,
    /// Will wear things made of spider silk and other subterranean materials.
    SubterraneanClothing,
    /// Adds decorations to equipment based on the level of the generated unit. Also improves item quality.
    EquipmentImprovements,
    /// Adds decorations to weapons generated for bowman and master bowman.
    ImprovedBows,
    /// Allows metal materials to be used to make cages (inexpensive metals only) and crafts.
    MetalPref,
    /// Allows the civilization to make use of nearby stone types. If the `FURNACE_OPERATOR` job is permitted,
    /// also allows ore-bearing stones to be smelted into metals.
    StonePref,
    /// The civilization can make traditionally metallic weapons such as swords and spears from wood.
    WoodWeapons,
    /// The civilization can make traditionally metallic armor such as mail shirts and helmets from wood.
    WoodArmor,
    /// Enables creatures of this entity to bring gems in trade.
    GemPref,
    /// Allow use of subterranean wood types, such as tower-cap and fungiwood logs.
    IndoorWood,
    /// Allow use of outdoor wood types, such as mangrove and oak.
    OutdoorWood,
    /// Arguments: shape
    ///
    /// Precious gems cut by this civilization's jewelers can be of this shape.
    GemShape,
    /// Arguments: shape
    ///
    /// Ordinary non-gem stones cut by this civilization's jewelers can be of this shape.
    StoneShape,
    /// Allows use of materials with `[DIVINE]` for clothing. Used for generated divine entities.
    DivineMatClothing,
    /// Allows use of materials with `[DIVINE]` for crafts. Used for generated divine entities.
    DivineMatCrafts,
    /// Allows use of metals with `[DIVINE]` for weapons. Used for generated divine entities.
    DivineMatWeapons,
    /// Allows use of metals with `[DIVINE]` for armor. Used for generated divine entities.
    DivineMatArmor,

    // # Animal Definitions #
    /// Start an animal definition.
    Animal,
    /// Arguments: creature token
    ///
    /// Select specific creature.
    AnimalToken,
    /// Arguments: creature caste token
    ///
    /// Select specific creature caste (requires `ANIMAL_TOKEN`). Sites with animal populations will still include all castes,
    /// but only the selected ones will be used for specific roles.
    AnimalCasteToken,
    /// Arguments: creature class
    ///
    /// Select creature castes with this creature class (multiple uses allowed).
    AnimalClass,
    /// Arguments: creature class
    ///
    /// Forbid creature castes with this creature class (multiple uses allowed).
    AnimalForbiddenClass,
    /// Animal will be present even if it does not naturally occur in the entity's terrain.
    /// All creatures, including demons, night trolls and other generated ones will be used if no specific creature or class is selected.
    AnimalAlwaysPresent,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalNeverMount,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalAlwaysMount,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalNeverWagonPuller,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalAlwaysWagonPuller,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalNeverSiege,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalAlwaysSiege,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalNeverPet,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalAlwaysPet,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalNeverPackAnimal,
    /// Override creature usage tokens, ALWAYS overrides NEVER if a caste is matched by more than one animal definition
    AnimalAlwaysPackAnimal,
    /// Arguments: tissue style unit ID
    ///
    /// Select a tissue layer which has the ID attached using `TISSUE_STYLE_UNIT` token in unit raws.
    /// This allows setting further cultural style parameters for the selected tissue layer.
    TissueStyle,
    /// Arguments: min : max
    ///
    /// Presumably sets culturally preferred tissue length for selected tissue. Needs testing.
    /// Dwarves have their beards set to 100:NONE by default.
    TissueStyleMaintainLength,
    /// Arguments: styling token
    ///
    /// Valid tokens are `NEATLY_COMBED`, `BRAIDED`, `DOUBLE_BRAIDS`, `PONY_TAILS`, `CLEAN_SHAVEN` and `STANDARD_HAIR/BEARD/MOUSTACHE/SIDEBURNS_SHAPINGS`.
    /// Presumably sets culturally preferred tissue shapings for selected tissue. Needs testing.
    TissueStylePreferredShaping,
    /// An unknown token
    #[default]
    Unknown,

    // not yet tags, but they are used in some mods
    /// Prefers wood
    WoodPref,
    /// An undead candidate
    UndeadCandidate,

    // this needs separate handling.. later !Todo
    /// Cut an existing entity
    CutEntity,
    /// Select an entity to modify
    SelectEntity,
}
