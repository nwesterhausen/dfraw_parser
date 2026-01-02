//! Material usage tags.

/// A material usage that can be set in a material definition.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    specta::Type,
    Copy,
    strum_macros::EnumIter,
)]
pub enum MaterialUsageTag {
    /// Lets the game know that an animal was likely killed in the production of this item.
    /// Entities opposed to killing animals (Elves in vanilla) will refuse to accept these items in trade.
    ImpliesAnimalKill,
    /// Classifies the material as plant-based alcohol, allowing its storage in food stockpiles under "Drink (Plant)".
    AlcoholPlant,
    /// Classifies the material as animal-based alcohol, allowing its storage in food stockpiles under "Drink (Animal)".
    AlcoholCreature,
    /// Classifies the material as generic alcohol. Implied by both `ALCOHOL_PLANT` and `ALCOHOL_CREATURE`. Exact behavior unknown, possibly vestigial.
    Alcohol,
    /// Classifies the material as plant-based cheese, allowing its storage in food stockpiles under "Cheese (Plant)".
    CheesePlant,
    /// Classifies the material as animal-based cheese, allowing its storage in food stockpiles under "Cheese (Animal)".
    CheeseCreature,
    /// Classifies the material as generic cheese. Implied by both `CHEESE_PLANT` and `CHEESE_CREATURE`. Exact behavior unknown, possibly vestigial.
    Cheese,
    /// Classifies the material as plant powder, allowing its storage in food stockpiles under "Milled Plant".
    PowderMiscPlant,
    /// Classifies the material as creature powder, allowing its storage in food stockpiles under "Bone Meal".
    /// Unlike milled plants, such as sugar and flour, "Bone Meal" barrels or pots may not contain bags.
    /// Custom reactions using this product better use buckets or jugs instead.
    PowderMiscCreature,
    /// Classifies the material as generic powder. Implied by both `POWDER_MISC_PLANT` and `POWDER_MISC_CREATURE`.
    /// Exact behavior unknown, possibly vestigial.
    PowderMisc,
    /// Permits globs of the material in solid form to be stored in food stockpiles under "Fat" - without it,
    /// dwarves will come by and "clean" the items, destroying them (unless `DO_NOT_CLEAN_GLOB` is also included).
    StockpileGlobOrStockpileGlobSolid,
    /// Classifies the material as milled paste, allowing its storage in food stockpiles under "Paste".
    StockpileGlobPaste,
    /// Classifies the material as pressed goods, allowing its storage in food stockpiles under "Pressed Material".
    StockpileGlobPressed,
    /// Classifies the material as a plant growth (e.g. fruits, leaves), allowing its storage in food stockpiles under Plant Growth/Fruit.
    StockpilePlantGrowth,
    /// Classifies the material as a plant extract, allowing its storage in food stockpiles under "Extract (Plant)".
    LiquidMiscPlant,
    /// Classifies the material as a creature extract, allowing its storage in food stockpiles under "Extract (Animal)".
    LiquidMiscCreature,
    /// Classifies the material as a miscellaneous liquid, allowing its storage in food stockpiles under "Misc. Liquid" along with lye.
    LiquidMiscOther,
    /// Classifies the material as a generic liquid. Implied by `LIQUID_MISC_PLANT`, `LIQUID_MISC_CREATURE`, and `LIQUID_MISC_OTHER`. Exact behavior unknown, possibly vestigial.
    LiquidMisc,
    /// Classifies the material as a plant, allowing its storage in food stockpiles under "Plants".
    StructuralPlantMat,
    /// Classifies the material as a plant seed, allowing its storage in food stockpiles under "Seeds".
    SeedMat,
    /// Classifies the material as bone, allowing its use for bone carvers and restriction from stockpiles by material.
    Bone,
    /// Classifies the material as wood, allowing its use for carpenters and storage in wood stockpiles.
    /// Entities opposed to killing plants (i.e. Elves) will refuse to accept these items in trade.
    Wood,
    /// Classifies the material as plant fiber, allowing its use for clothiers and storage in cloth stockpiles under "Thread (Plant)" and "Cloth (Plant)".
    ThreadPlant,
    /// Classifies the material as tooth, allowing its use for bone carvers and restriction from stockpiles by material.
    Tooth,
    /// Classifies the material as horn, allowing its use for bone carvers and restriction from stockpiles by material.
    Horn,
    /// Classifies the material as hair, allowing for its use for spinners and restriction from refuse stockpiles by material.
    Hair,
    /// Classifies the material as pearl, allowing its use for bone carvers and restriction from stockpiles by material.
    Pearl,
    /// Classifies the material as shell, allowing its use for bone carvers and restriction from stockpiles by material.
    Shell,
    /// Classifies the material as leather, allowing its use for leatherworkers and storage in leather stockpiles.
    Leather,
    /// Classifies the material as silk, allowing its use for clothiers and storage in cloth stockpiles under "Thread (Silk)" and "Cloth (Silk)".
    Silk,
    /// Classifies the material as soap, allowing it to be used as a bath detergent and stored in bar/block stockpiles under "Bars: Other Materials".
    Soap,
    /// Material generates miasma when it rots.
    GeneratesMiasma,
    /// Classifies the material as edible meat.
    Meat,
    /// Material will rot if not stockpiled appropriately. Currently only affects food and refuse, other items made of this material will not rot.
    Rots,
    /// In most living creatures, it controls many bodily functions and movements by sending signals around the body. See: Nervous tissue
    NervousTissue,
    /// Tells the game to classify contaminants of this material as being "blood" in Adventurer mode tile descriptions ("Here we have a Dwarf in a slurry of blood.").
    BloodMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "ichor".
    IchorMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "goo".
    GooMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "slime".
    SlimeMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "pus".
    PusMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "sweat".
    SweatMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "tears".
    TearsMapDescriptor,
    /// Tells the game to classify contaminants of this material as being "spit".
    SpitMapDescriptor,
    /// Contaminants composed of this material evaporate over time, slowly disappearing from the map. Used internally by water.
    Evaporates,
    /// Used for materials which cause syndromes, causes it to enter the creature's blood instead of simply spattering on the surface.
    EntersBlood,
    /// Can be eaten by vermin.
    EdibleVermin,
    /// Can be eaten raw.
    EdibleRaw,
    /// Can be cooked and then eaten.
    EdibleCooked,
    /// Prevents globs made of this material from being cleaned up and destroyed.
    DoNotCleanGlob,
    /// Prevents the material from showing up in Stone stockpile settings.
    NoStoneStockpile,
    /// The material can be made into minecarts, wheelbarrows, and stepladders at the metalsmith's forge.
    ItemsMetal,
    /// Equivalent to `ITEMS_HARD`. Given to bone.
    ItemsBarred,
    /// Equivalent to `ITEMS_HARD`. Given to shell.
    ItemsScaled,
    /// Equivalent to `ITEMS_SOFT`. Given to leather.
    ItemsLeather,
    /// The material can be made into clothing, amulets, bracelets, earrings, backpacks, and quivers, contingent
    /// on which workshops accept the material. Given to plant fiber, silk and wool.
    ItemsSoft,
    /// The material can be made into furniture, crafts, mechanisms, and blocks, contingent on which workshops accept the material.
    /// Random crafts made from this material include all seven items. Given to stone, wood, bone, shell, chitin, claws, teeth,
    /// horns, hooves and beeswax. Hair, pearls and eggshells also have the tag.
    ItemsHard,
    /// Used to define that the material is a stone. Allows its usage in masonry and stonecrafting and storage in stone stockpiles, among other effects.
    IsStone,
    /// Defines the material is a ceramic.
    IsCeramic,
    /// Used for a stone that cannot be dug into.
    Undiggable,
    /// Causes containers made of this material to be prefixed with "unglazed" if they have not yet been glazed.
    DisplayUnglazed,
    /// Classifies the material as yarn, allowing its use for clothiers and its storage in cloth stockpiles under "Thread (Yarn)" and "Cloth (Yarn)".
    Yarn,
    /// Classifies the material as metal thread, permitting thread and cloth to be stored in cloth stockpiles under "Thread (Metal)" and "Cloth (Metal)".
    StockpileThreadMetal,
    /// Defines the material as being metal, allowing it to be used at forges.
    IsMetal,
    /// Used internally by green glass, clear glass, and crystal glass. Appears to only affect the `GLASS_MATERIAL` reaction token. Does not cause the game
    /// to treat the material like glass, i.e being referred to as "raw" instead of "rough" in its raw form or being displayed in the "glass" trade/embark category.
    IsGlass,
    /// Can be used in the production of crystal glass.
    CrystalGlassable,
    /// Melee weapons can be made out of this material.
    ItemsWeapon,
    /// Ranged weapons can be made out of this material.
    ItemsWeaponRanged,
    /// Anvils can be made out of this material.
    ItemsAnvil,
    /// Ammunition can be made out of this material.
    ItemsAmmo,
    /// Picks can be made out of this material.
    ItemsDigger,
    /// Armor can be made out of this material.
    ItemsArmor,
    /// Used internally by amber and coral. Functionally equivalent to `ITEMS_HARD`.
    ItemsDelicate,
    /// Siege engine parts can be made out of this material. Does not appear to work.
    ItemsSiegeEngine,
    /// Querns and millstones can be made out of this material. Does not appear to work.
    ItemsQuern,
    /// An unknown token
    #[default]
    Unknown,
}

impl std::fmt::Display for MaterialUsageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ImpliesAnimalKill => write!(f, "Implies Animal Kill"),
            Self::AlcoholPlant => write!(f, "Alcohol from Plant"),
            Self::AlcoholCreature => write!(f, "Alcohol from Creature"),
            Self::Alcohol => write!(f, "Alcohol"),
            Self::CheesePlant => write!(f, "Cheese from Plant"),
            Self::CheeseCreature => write!(f, "Cheese from Creature"),
            Self::Cheese => write!(f, "Cheese"),
            Self::PowderMiscPlant => write!(f, "Misc Plant Powder"),
            Self::PowderMiscCreature => write!(f, "Misc Creature Powder"),
            Self::PowderMisc => write!(f, "Misc Powder"),
            Self::StockpileGlobOrStockpileGlobSolid => {
                write!(f, "Stockpile as Glob Or Stockpile as Glob Solid")
            }
            Self::StockpileGlobPaste => write!(f, "Stockpile as Glob Paste"),
            Self::StockpileGlobPressed => write!(f, "Stockpile as Glob Pressed"),
            Self::StockpilePlantGrowth => write!(f, "Stockpile as Plant Growth"),
            Self::LiquidMiscPlant => write!(f, "Misc Plant Liquid"),
            Self::LiquidMiscCreature => write!(f, "Misc Creature Liquid"),
            Self::LiquidMiscOther => write!(f, "Misc Other Liquid"),
            Self::LiquidMisc => write!(f, "Misc Liquid"),
            Self::StructuralPlantMat => write!(f, "Structural Plant Material"),
            Self::SeedMat => write!(f, "Seed Material"),
            Self::Bone => write!(f, "Bone"),
            Self::Wood => write!(f, "Wood"),
            Self::ThreadPlant => write!(f, "Thread from Plant"),
            Self::Tooth => write!(f, "Tooth"),
            Self::Horn => write!(f, "Horn"),
            Self::Hair => write!(f, "Hair"),
            Self::Pearl => write!(f, "Pearl"),
            Self::Shell => write!(f, "Shell"),
            Self::Leather => write!(f, "Leather"),
            Self::Silk => write!(f, "Silk"),
            Self::Soap => write!(f, "Soap"),
            Self::GeneratesMiasma => write!(f, "Generates Miasma"),
            Self::Meat => write!(f, "Meat"),
            Self::Rots => write!(f, "Rots"),
            Self::NervousTissue => write!(f, "Nervous Tissue"),
            Self::BloodMapDescriptor => write!(f, "BloodMapDescriptor"),
            Self::IchorMapDescriptor => write!(f, "IchorMapDescriptor"),
            Self::GooMapDescriptor => write!(f, "GooMapDescriptor"),
            Self::SlimeMapDescriptor => write!(f, "SlimeMapDescriptor"),
            Self::PusMapDescriptor => write!(f, "PusMapDescriptor"),
            Self::SweatMapDescriptor => write!(f, "SweatMapDescriptor"),
            Self::TearsMapDescriptor => write!(f, "TearsMapDescriptor"),
            Self::SpitMapDescriptor => write!(f, "SpitMapDescriptor"),
            Self::Evaporates => write!(f, "Evaporates"),
            Self::EntersBlood => write!(f, "Enters Blood"),
            Self::EdibleVermin => write!(f, "Edible Vermin"),
            Self::EdibleRaw => write!(f, "Edible Raw"),
            Self::EdibleCooked => write!(f, "Edible Cooked"),
            Self::DoNotCleanGlob => write!(f, "Do Not Clean Glob"),
            Self::NoStoneStockpile => write!(f, "No Stone Stockpile"),
            Self::ItemsMetal => write!(f, "Metal Item"),
            Self::ItemsBarred => write!(f, "Item created from bars"),
            Self::ItemsScaled => write!(f, "Item created from scales"),
            Self::ItemsLeather => write!(f, "Leather Item"),
            Self::ItemsSoft => write!(f, "Soft Item"),
            Self::ItemsHard => write!(f, "Hard Item"),
            Self::IsStone => write!(f, "Is made of Stone"),
            Self::IsCeramic => write!(f, "Is made of Ceramic"),
            Self::Undiggable => write!(f, "Undiggable"),
            Self::DisplayUnglazed => write!(f, "Display Unglazed"),
            Self::Yarn => write!(f, "Yarn"),
            Self::StockpileThreadMetal => write!(f, "Stockpile as Thread Metal"),
            Self::IsMetal => write!(f, "Is made of Metal"),
            Self::IsGlass => write!(f, "Is made of Glass"),
            Self::CrystalGlassable => write!(f, "Usable for Crystal Glass"),
            Self::ItemsWeapon => write!(f, "Weapon Item"),
            Self::ItemsWeaponRanged => write!(f, "Ranged Weapon Item"),
            Self::ItemsAnvil => write!(f, "Anvil Item"),
            Self::ItemsAmmo => write!(f, "Ammo Item"),
            Self::ItemsDigger => write!(f, "Digging implement Item"),
            Self::ItemsArmor => write!(f, "Armor Item"),
            Self::ItemsDelicate => write!(f, "Delicate Item"),
            Self::ItemsSiegeEngine => write!(f, "Siege Engine Item"),
            Self::ItemsQuern => write!(f, "Quern Item"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
