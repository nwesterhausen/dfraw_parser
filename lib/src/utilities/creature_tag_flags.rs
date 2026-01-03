use crate::tags::CreatureTag;

impl CreatureTag {
    pub const FLAG_TOKENS: [&CreatureTag; 27] = [
        &CreatureTag::AllCastesAlive,
        &CreatureTag::ArtificialHiveable,
        &CreatureTag::DoesNotExist,
        &CreatureTag::Equipment,
        &CreatureTag::EquipmentWagon,
        &CreatureTag::Evil,
        &CreatureTag::Fanciful,
        &CreatureTag::Generated,
        &CreatureTag::Good,
        &CreatureTag::LargeRoaming,
        &CreatureTag::LocalPopsControllable,
        &CreatureTag::LocalPopsProduceHeroes,
        &CreatureTag::LooseClusters,
        &CreatureTag::MatesToBreed,
        &CreatureTag::Mundane,
        &CreatureTag::OccursAsEntityRace,
        &CreatureTag::Savage,
        &CreatureTag::SmallRace,
        &CreatureTag::TwoGenders,
        &CreatureTag::Ubiquitous,
        &CreatureTag::Utterances,
        &CreatureTag::VerminEater,
        &CreatureTag::VerminFish,
        &CreatureTag::VerminGrounder,
        &CreatureTag::VerminRotter,
        &CreatureTag::VerminSoil,
        &CreatureTag::VerminSoilColony,
    ];
}
