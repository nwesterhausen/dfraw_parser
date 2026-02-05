//! String token to parsed tag map for fuel type tokens.

use crate::{
    custom_types::{BodyPartPosition, BodyPartSpecifier},
    tokens::BodyDetailPlanToken,
};

/// Mapping of fuel type tokens to strings
pub static BODY_DETAIL_PLAN_TOKENS: phf::Map<&'static str, BodyDetailPlanToken> = phf::phf_map! {
    "BODY_DETAIL_PLAN" => BodyDetailPlanToken::PlanMarker { identifier: String::new() },
    "ADD_MATERIAL" => BodyDetailPlanToken::AddMaterial { identifier: String::new(), material_template: String::new()},
    "ADD_TISSUE" => BodyDetailPlanToken::AddTissue { identifier: String::new(), tissue_template: String::new()},
    "BP_LAYERS" => BodyDetailPlanToken::Layers { arguments: Vec::new(), body_part: String::new(), specifier: BodyPartSpecifier::Category},
    "BP_LAYERS_OVER" => BodyDetailPlanToken::LayersOver { arguments: Vec::new(), body_part: String::new(), specifier: BodyPartSpecifier::Category},
    "BP_LAYERS_UNDER" => BodyDetailPlanToken::LayersUnder { arguments: Vec::new(), body_part: String::new(), specifier: BodyPartSpecifier::Category},
    "BP_POSITION" => BodyDetailPlanToken::Position { position: BodyPartPosition::Front, specifier: BodyPartSpecifier::Category, target: String::new()},
    "BP_RELATION" => BodyDetailPlanToken::Relation {
        coverage: String::new(), parent: String::new(), parent_specifier: BodyPartSpecifier::Category,
        relation: String::new(), specified_by: BodyPartSpecifier::Category, target: String::new()},
    "BP_RELSIZE" => BodyDetailPlanToken::RelativeSize { relative_size: 0, specifier: BodyPartSpecifier::Category, target: String::new()},
};
