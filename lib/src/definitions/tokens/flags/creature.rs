use crate::tokens::CreatureToken;

impl CreatureToken {
    pub const FLAG_TOKENS: [&CreatureToken; 27] = [
        &CreatureToken::AllCastesAlive,
        &CreatureToken::ArtificialHiveable,
        &CreatureToken::DoesNotExist,
        &CreatureToken::Equipment,
        &CreatureToken::EquipmentWagon,
        &CreatureToken::Evil,
        &CreatureToken::Fanciful,
        &CreatureToken::Generated,
        &CreatureToken::Good,
        &CreatureToken::LargeRoaming,
        &CreatureToken::LocalPopsControllable,
        &CreatureToken::LocalPopsProduceHeroes,
        &CreatureToken::LooseClusters,
        &CreatureToken::MatesToBreed,
        &CreatureToken::Mundane,
        &CreatureToken::OccursAsEntityRace,
        &CreatureToken::Savage,
        &CreatureToken::SmallRace,
        &CreatureToken::TwoGenders,
        &CreatureToken::Ubiquitous,
        &CreatureToken::Utterances,
        &CreatureToken::VerminEater,
        &CreatureToken::VerminFish,
        &CreatureToken::VerminGrounder,
        &CreatureToken::VerminRotter,
        &CreatureToken::VerminSoil,
        &CreatureToken::VerminSoilColony,
    ];
}
