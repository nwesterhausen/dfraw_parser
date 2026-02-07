//! String token to parsed tag map for fuel type tokens.

use crate::{
    custom_types::{BodyPartPosition, PartSpecifier},
    tokens::BodyDetailPlanToken,
};

/// Mapping of fuel type tokens to strings
pub static BODY_DETAIL_PLAN_TOKENS: phf::Map<&'static str, BodyDetailPlanToken> = phf::phf_map! {
    "BODY_DETAIL_PLAN" => BodyDetailPlanToken::PlanMarker { identifier: String::new() },
    "ADD_MATERIAL" => BodyDetailPlanToken::AddMaterial { identifier: String::new(), material_template: String::new()},
    "ADD_TISSUE" => BodyDetailPlanToken::AddTissue { identifier: String::new(), tissue_template: String::new()},
    "BP_LAYERS" => BodyDetailPlanToken::Layers { arguments: Vec::new(), selector: Vec::new(), specifier: PartSpecifier::Category},
    "BP_LAYERS_OVER" => BodyDetailPlanToken::LayersOver { arguments: Vec::new(), selector: Vec::new(), specifier: PartSpecifier::Category},
    "BP_LAYERS_UNDER" => BodyDetailPlanToken::LayersUnder { arguments: Vec::new(), selector: Vec::new(), specifier: PartSpecifier::Category},
    "BP_POSITION" => BodyDetailPlanToken::Position { position: BodyPartPosition::Front, specifier: PartSpecifier::Category, target: String::new()},
    "BP_RELATION" => BodyDetailPlanToken::Relation {
        coverage: String::new(), parent_selector: Vec::new(), parent_specifier: PartSpecifier::Category,
        relation: String::new(), specifier: PartSpecifier::Category, selector: Vec::new()},
    "BP_RELSIZE" => BodyDetailPlanToken::RelativeSize { relative_size: 0, specifier: PartSpecifier::Category, selector: Vec::new()},
};
