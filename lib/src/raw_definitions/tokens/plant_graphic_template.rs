//! String token to parsed tag map for plant graphic template tokens.

use crate::tokens::PlantGraphicTemplateToken;

/// Map of plant graphic templates to their string representation.
pub static PLANT_GRAPHIC_TEMPLATE_TOKENS: phf::Map<&'static str, PlantGraphicTemplateToken> = phf::phf_map! {
  "STANDARD_LEAVES" => PlantGraphicTemplateToken::StandardLeaves,
  "STANDARD_FLOWERS_1" => PlantGraphicTemplateToken::StandardFlowers1,
  "STANDARD_FRUIT_1" => PlantGraphicTemplateToken::StandardFruit1,
  "STANDARD_FLOWERS_2" => PlantGraphicTemplateToken::StandardFlowers2,
  "STANDARD_FRUIT_2" => PlantGraphicTemplateToken::StandardFruit2,
  "STANDARD_FLOWERS_3" => PlantGraphicTemplateToken::StandardFlowers3,
  "STANDARD_FRUIT_3" => PlantGraphicTemplateToken::StandardFruit3,
  "STANDARD_FLOWERS_4" => PlantGraphicTemplateToken::StandardFlowers4,
  "STANDARD_FRUIT_4" => PlantGraphicTemplateToken::StandardFruit4,
};
