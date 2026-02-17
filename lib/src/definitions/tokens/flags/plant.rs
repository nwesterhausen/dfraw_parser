use crate::tokens::PlantToken;

impl PlantToken {
    pub const FLAG_TOKENS: [&PlantToken; 5] = [
        &PlantToken::Dry,
        &PlantToken::Evil,
        &PlantToken::Good,
        &PlantToken::Savage,
        &PlantToken::Wet,
    ];
}
