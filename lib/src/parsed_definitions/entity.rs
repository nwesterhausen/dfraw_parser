//! Contains the Entity struct and implementations.

use dfraw_parser_proc_macros::{Cleanable, IsEmpty};
use tracing::warn;
use uuid::Uuid;

use crate::{
    Color, Position,
    metadata::RawMetadata,
    raw_definitions::{ENTITY_TOKENS, POSITION_TOKENS},
    tags::{EntityTag, ObjectType},
    traits::{RawObject, Searchable},
    utilities::{clean_search_vec, generate_object_id_using_raw_metadata},
};

/// A struct representing an Entity object.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    specta::Type,
    PartialEq,
    IsEmpty,
    Cleanable,
)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    metadata: Option<RawMetadata>,
    identifier: String,
    object_id: Uuid,

    tags: Vec<EntityTag>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    creature: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    translation: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    exclusive_start_biome: Option<String>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    biome_support: Option<Vec<(String, u32)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    settlement_biome: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    start_biome: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    likes_sites: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tolerates_sites: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    world_constructions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[is_empty(value = 500)]
    max_pop_number: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[is_empty(value = 50)]
    max_site_pop_number: Option<u32>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    #[is_empty(value = 3)]
    max_starting_civ_number: Option<u32>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    permitted_buildings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    permitted_jobs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    permitted_reactions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    currency: Option<Vec<(String, u32)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    art_facet_modifier: Option<Vec<(String, u32)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    art_image_element_modifier: Option<Vec<(String, u32)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    item_improvement_modifier: Option<Vec<(String, u32)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    select_symbols: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    subselect_symbols: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    cull_symbols: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    friendly_color: Option<Color>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    religion: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    religion_spheres: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    sphere_alignments: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    positions: Option<Vec<Position>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    land_holder_trigger: Option<String>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    site_variable_positions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    variable_positions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    ethics: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    values: Option<Vec<(String, u32)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    variable_values: Option<Vec<(String, u32, u32)>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    active_season: Option<String>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    banditry: Option<f32>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    progress_trigger_population: Option<u8>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    progress_trigger_production: Option<u8>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    progress_trigger_trade: Option<u8>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    progress_trigger_population_siege: Option<u8>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    progress_trigger_production_siege: Option<u8>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    progress_trigger_trade_siege: Option<u8>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    scholars: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    ammo: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    armors: Option<Vec<(String, u16)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    diggers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    gloves: Option<Vec<(String, u16)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    helms: Option<Vec<(String, u16)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    instrument: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    pants: Option<Vec<(String, u16)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    shields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    shoes: Option<Vec<(String, u16)>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    siege_ammo: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    tool: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    toys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    trap_components: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    weapons: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    gem_shape: Option<Vec<String>>,
    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    stone_shape: Option<Vec<String>>,

    #[serde(skip_serializing_if = "crate::traits::IsEmpty::is_empty")]
    source_hfid: Option<u32>,
}

impl Entity {
    /// Function to create a new empty Entity.
    ///
    /// # Returns
    ///
    /// * `Entity` - The new empty Entity.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            // Default values which aren't rust defaults
            max_pop_number: Some(500),
            max_site_pop_number: Some(50),
            max_starting_civ_number: Some(3),

            ..Default::default()
        }
    }
    /// Function to create a new Entity.
    ///
    /// # Parameters
    ///
    /// * `identifier` - The identifier for the Entity.
    /// * `metadata` - The metadata for the Entity.
    ///
    /// # Returns
    ///
    /// * `Entity` - The new Entity.
    #[must_use]
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: Some(metadata.clone()),
            object_id: generate_object_id_using_raw_metadata(
                identifier,
                ObjectType::Entity,
                metadata,
            ),
            // Default values which aren't rust defaults
            max_pop_number: Some(500),
            max_site_pop_number: Some(50),
            max_starting_civ_number: Some(3),
            ..Default::default()
        }
    }
}

#[typetag::serde]
impl RawObject for Entity {
    fn get_searchable_tokens(&self) -> Vec<&str> {
        Vec::new()
    }
    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.as_ref().map_or_else(
            || {
                warn!("Entity::get_metadata: no metadata for {}", self.identifier);
                RawMetadata::default()
                    .with_object_type(ObjectType::Entity)
                    .with_hidden(true)
            },
            std::clone::Clone::clone,
        )
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
        &self.identifier
    }
    fn get_type(&self) -> ObjectType {
        ObjectType::Entity
    }
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        if let Some(position_token) = POSITION_TOKENS.get(key) {
            if self.positions.is_none() {
                self.positions = Some(Vec::new());
            }

            if let Some(positions) = self.positions.as_mut() {
                // Tags should be attached to the last Position in the list
                if let Some(position) = positions.last_mut() {
                    position.parse_tag(position_token, value);
                    return;
                }
                // If there is no position, create one with unknown name..
                let mut position = Position::new("unknown".into());
                position.parse_tag(position_token, value);
                positions.push(position);
                return;
            }
        }

        let Some(tag) = ENTITY_TOKENS.get(key) else {
            warn!(
                "Entity::parse_tag: called `Option::unwrap()` on a `None` value for presumed Entity tag: {}",
                key
            );
            return;
        };

        match tag {
            EntityTag::ActiveSeason => {
                self.active_season = Some(value.to_string());
            }
            EntityTag::Banditry => {
                self.banditry = Some(value.parse().unwrap_or_default());
            }
            EntityTag::Creature => {
                self.creature = Some(value.to_string());
            }
            EntityTag::ProgressTriggerPopulation => {
                self.progress_trigger_population = Some(value.parse().unwrap_or_default());
            }
            EntityTag::ProgressTriggerProduction => {
                self.progress_trigger_production = Some(value.parse().unwrap_or_default());
            }
            EntityTag::ProgressTriggerTrade => {
                self.progress_trigger_trade = Some(value.parse().unwrap_or_default());
            }
            EntityTag::ProgressTriggerPopulationSiege => {
                self.progress_trigger_population_siege = Some(value.parse().unwrap_or_default());
            }
            EntityTag::ProgressTriggerProductionSiege => {
                self.progress_trigger_production_siege = Some(value.parse().unwrap_or_default());
            }
            EntityTag::ProgressTriggerTradeSiege => {
                self.progress_trigger_trade_siege = Some(value.parse().unwrap_or_default());
            }
            EntityTag::Scholar => {
                if let Some(scholars) = &mut self.scholars {
                    scholars.push(value.to_string());
                } else {
                    self.scholars = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Ammo => {
                if let Some(ammo) = &mut self.ammo {
                    ammo.push(value.to_string());
                } else {
                    self.ammo = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Armor => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(armors) = &mut self.armors {
                    armors.push((armor, chance));
                } else {
                    self.armors = Some(vec![(armor, chance)]);
                }
            }
            EntityTag::Digger => {
                if let Some(diggers) = &mut self.diggers {
                    diggers.push(value.to_string());
                } else {
                    self.diggers = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Gloves => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(gloves) = &mut self.gloves {
                    gloves.push((armor, chance));
                } else {
                    self.gloves = Some(vec![(armor, chance)]);
                }
            }
            EntityTag::Helm => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(helms) = &mut self.helms {
                    helms.push((armor, chance));
                } else {
                    self.helms = Some(vec![(armor, chance)]);
                }
            }
            EntityTag::Instrument => {
                if let Some(instrument) = &mut self.instrument {
                    instrument.push(value.to_string());
                } else {
                    self.instrument = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Pants => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(pants) = &mut self.pants {
                    pants.push((armor, chance));
                } else {
                    self.pants = Some(vec![(armor, chance)]);
                }
            }
            EntityTag::Shield => {
                if let Some(shields) = &mut self.shields {
                    shields.push(value.to_string());
                } else {
                    self.shields = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Shoes => {
                let mut split = value.split(':');
                let armor = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(shoes) = &mut self.shoes {
                    shoes.push((armor, chance));
                } else {
                    self.shoes = Some(vec![(armor, chance)]);
                }
            }
            EntityTag::SiegeAmmo => {
                if let Some(siege_ammo) = &mut self.siege_ammo {
                    siege_ammo.push(value.to_string());
                } else {
                    self.siege_ammo = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Tool => {
                if let Some(tool) = &mut self.tool {
                    tool.push(value.to_string());
                } else {
                    self.tool = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Toy => {
                if let Some(toys) = &mut self.toys {
                    toys.push(value.to_string());
                } else {
                    self.toys = Some(vec![value.to_string()]);
                }
            }
            EntityTag::TrapComponent => {
                if let Some(trap_components) = &mut self.trap_components {
                    trap_components.push(value.to_string());
                } else {
                    self.trap_components = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Weapon => {
                if let Some(weapons) = &mut self.weapons {
                    weapons.push(value.to_string());
                } else {
                    self.weapons = Some(vec![value.to_string()]);
                }
            }
            EntityTag::GemShape => {
                if let Some(gem_shape) = &mut self.gem_shape {
                    gem_shape.push(value.to_string());
                } else {
                    self.gem_shape = Some(vec![value.to_string()]);
                }
            }
            EntityTag::StoneShape => {
                if let Some(stone_shape) = &mut self.stone_shape {
                    stone_shape.push(value.to_string());
                } else {
                    self.stone_shape = Some(vec![value.to_string()]);
                }
            }
            EntityTag::BiomeSupport => {
                let mut split = value.split(':');
                let biome = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(biome_support) = &mut self.biome_support {
                    biome_support.push((biome, chance));
                } else {
                    self.biome_support = Some(vec![(biome, chance)]);
                }
            }
            EntityTag::SettlementBiome => {
                if let Some(settlement_biome) = &mut self.settlement_biome {
                    settlement_biome.push(value.to_string());
                } else {
                    self.settlement_biome = Some(vec![value.to_string()]);
                }
            }
            EntityTag::StartBiome => {
                if let Some(start_biome) = &mut self.start_biome {
                    start_biome.push(value.to_string());
                } else {
                    self.start_biome = Some(vec![value.to_string()]);
                }
            }
            EntityTag::LikesSite => {
                if let Some(likes_sites) = &mut self.likes_sites {
                    likes_sites.push(value.to_string());
                } else {
                    self.likes_sites = Some(vec![value.to_string()]);
                }
            }
            EntityTag::ToleratesSite => {
                if let Some(tolerates_sites) = &mut self.tolerates_sites {
                    tolerates_sites.push(value.to_string());
                } else {
                    self.tolerates_sites = Some(vec![value.to_string()]);
                }
            }
            EntityTag::WorldConstruction => {
                if let Some(world_constructions) = &mut self.world_constructions {
                    world_constructions.push(value.to_string());
                } else {
                    self.world_constructions = Some(vec![value.to_string()]);
                }
            }
            EntityTag::PermittedBuilding => {
                if let Some(permitted_buildings) = &mut self.permitted_buildings {
                    permitted_buildings.push(value.to_string());
                } else {
                    self.permitted_buildings = Some(vec![value.to_string()]);
                }
            }
            EntityTag::PermittedJob => {
                if let Some(permitted_jobs) = &mut self.permitted_jobs {
                    permitted_jobs.push(value.to_string());
                } else {
                    self.permitted_jobs = Some(vec![value.to_string()]);
                }
            }
            EntityTag::PermittedReaction => {
                if let Some(permitted_reactions) = &mut self.permitted_reactions {
                    permitted_reactions.push(value.to_string());
                } else {
                    self.permitted_reactions = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Currency => {
                let mut split = value.split(':');
                let currency = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(self_currency) = &mut self.currency {
                    self_currency.push((currency, chance));
                } else {
                    self.currency = Some(vec![(currency, chance)]);
                }
            }
            EntityTag::ArtFacetModifier => {
                let mut split = value.split(':');
                let facet = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(art_facet_modifier) = &mut self.art_facet_modifier {
                    art_facet_modifier.push((facet, chance));
                } else {
                    self.art_facet_modifier = Some(vec![(facet, chance)]);
                }
            }
            EntityTag::ArtImageElementModifier => {
                let mut split = value.split(':');
                let element = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(art_image_element_modifier) = &mut self.art_image_element_modifier {
                    art_image_element_modifier.push((element, chance));
                } else {
                    self.art_image_element_modifier = Some(vec![(element, chance)]);
                }
            }
            EntityTag::ItemImprovementModifier => {
                let mut split = value.split(':');
                let improvement = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(item_improvement_modifier) = &mut self.item_improvement_modifier {
                    item_improvement_modifier.push((improvement, chance));
                } else {
                    self.item_improvement_modifier = Some(vec![(improvement, chance)]);
                }
            }
            EntityTag::SelectSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();

                if let Some(select_symbols) = &mut self.select_symbols {
                    select_symbols.push((symbol, chance));
                } else {
                    self.select_symbols = Some(vec![(symbol, chance)]);
                }
            }
            EntityTag::SubselectSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();

                if let Some(subselect_symbols) = &mut self.subselect_symbols {
                    subselect_symbols.push((symbol, chance));
                } else {
                    self.subselect_symbols = Some(vec![(symbol, chance)]);
                }
            }
            EntityTag::CullSymbol => {
                let mut split = value.split(':');
                let symbol = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();

                if let Some(cull_symbols) = &mut self.cull_symbols {
                    cull_symbols.push((symbol, chance));
                } else {
                    self.cull_symbols = Some(vec![(symbol, chance)]);
                }
            }
            EntityTag::FriendlyColor => {
                self.friendly_color = Some(Color::from_value(value));
            }
            EntityTag::Religion => {
                self.religion = Some(value.to_string());
            }
            EntityTag::ReligionSphere => {
                if let Some(religion_spheres) = &mut self.religion_spheres {
                    religion_spheres.push(value.to_string());
                } else {
                    self.religion_spheres = Some(vec![value.to_string()]);
                }
            }
            EntityTag::SphereAlignment => {
                if let Some(sphere_alignments) = &mut self.sphere_alignments {
                    sphere_alignments.push(value.to_string());
                } else {
                    self.sphere_alignments = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Position => {
                let position = Position::new(value.to_string());
                if let Some(positions) = &mut self.positions {
                    positions.push(position);
                } else {
                    self.positions = Some(vec![position]);
                }
            }
            EntityTag::LandHolderTrigger => {
                self.land_holder_trigger = Some(value.to_string());
            }
            EntityTag::SiteVariablePositions => {
                if let Some(site_variable_positions) = &mut self.site_variable_positions {
                    site_variable_positions.push(value.to_string());
                } else {
                    self.site_variable_positions = Some(vec![value.to_string()]);
                }
            }
            EntityTag::VariablePositions => {
                if let Some(variable_positions) = &mut self.variable_positions {
                    variable_positions.push(value.to_string());
                } else {
                    self.variable_positions = Some(vec![value.to_string()]);
                }
            }
            EntityTag::Ethic => {
                let mut split = value.split(':');
                let ethic = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().to_string();

                if let Some(ethics) = &mut self.ethics {
                    ethics.push((ethic, chance));
                } else {
                    self.ethics = Some(vec![(ethic, chance)]);
                }
            }
            EntityTag::Value => {
                let mut split = value.split(':');
                let value = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(values) = &mut self.values {
                    values.push((value, chance));
                } else {
                    self.values = Some(vec![(value, chance)]);
                }
            }
            EntityTag::VariableValue => {
                let mut split = value.split(':');
                let value = split.next().unwrap_or_default().to_string();
                let chance = split.next().unwrap_or_default().parse().unwrap_or_default();
                let max = split.next().unwrap_or_default().parse().unwrap_or_default();

                if let Some(variable_values) = &mut self.variable_values {
                    variable_values.push((value, chance, max));
                } else {
                    self.variable_values = Some(vec![(value, chance, max)]);
                }
            }
            EntityTag::ExclusiveStartBiome => {
                self.exclusive_start_biome = Some(value.to_string());
            }
            EntityTag::MaxPopNumber => {
                self.max_pop_number = Some(value.parse().unwrap_or_default());
            }
            EntityTag::MaxSitePopNumber => {
                self.max_site_pop_number = Some(value.parse().unwrap_or_default());
            }
            EntityTag::MaxStartingCivNumber => {
                self.max_starting_civ_number = Some(value.parse().unwrap_or_default());
            }
            EntityTag::SourceHfid => {
                self.source_hfid = Some(value.parse().unwrap_or_default());
            }
            EntityTag::Translation => {
                self.translation = Some(value.to_string());
            }

            _ => {
                self.tags.push(*tag);
            }
        }
    }
}

impl Searchable for Entity {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.identifier.clone());
        vec.extend(self.tags.iter().map(|tag| format!("{tag:?}")));
        if let Some(scholars) = &self.scholars {
            vec.extend(scholars.iter().cloned());
        }
        // vec.extend(self.ammo.iter().cloned());
        if let Some(ammo) = &self.ammo {
            vec.extend(ammo.iter().cloned());
        }
        // vec.extend(self.armors.iter().map(|(armor, _)| format!("{armor:?}")));
        if let Some(armors) = &self.armors {
            vec.extend(armors.iter().map(|(armor, _)| format!("{armor:?}")));
        }
        // vec.extend(self.diggers.iter().cloned());
        if let Some(diggers) = &self.diggers {
            vec.extend(diggers.iter().cloned());
        }
        // vec.extend(self.gloves.iter().map(|(glove, _)| format!("{glove:?}")));
        if let Some(gloves) = &self.gloves {
            vec.extend(gloves.iter().map(|(glove, _)| format!("{glove:?}")));
        }
        // vec.extend(self.helms.iter().map(|(helm, _)| format!("{helm:?}")));
        if let Some(helms) = &self.helms {
            vec.extend(helms.iter().map(|(helm, _)| format!("{helm:?}")));
        }
        // vec.extend(self.instrument.iter().cloned());
        if let Some(instrument) = &self.instrument {
            vec.extend(instrument.iter().cloned());
        }
        // vec.extend(self.pants.iter().map(|(pants, _)| format!("{pants:?}")));
        if let Some(pants) = &self.pants {
            vec.extend(pants.iter().map(|(pants, _)| format!("{pants:?}")));
        }
        // vec.extend(self.shields.iter().cloned());
        if let Some(shields) = &self.shields {
            vec.extend(shields.iter().cloned());
        }
        // vec.extend(self.shoes.iter().map(|(shoe, _)| format!("{shoe:?}")));
        if let Some(shoes) = &self.shoes {
            vec.extend(shoes.iter().map(|(shoe, _)| format!("{shoe:?}")));
        }
        // vec.extend(self.siege_ammo.iter().cloned());
        if let Some(siege_ammo) = &self.siege_ammo {
            vec.extend(siege_ammo.iter().cloned());
        }
        // vec.extend(self.tool.iter().cloned());
        if let Some(tool) = &self.tool {
            vec.extend(tool.iter().cloned());
        }
        // vec.extend(self.toys.iter().cloned());
        if let Some(toys) = &self.toys {
            vec.extend(toys.iter().cloned());
        }
        // vec.extend(self.trap_components.iter().cloned());
        if let Some(trap_components) = &self.trap_components {
            vec.extend(trap_components.iter().cloned());
        }
        // vec.extend(self.weapons.iter().cloned());
        if let Some(weapons) = &self.weapons {
            vec.extend(weapons.iter().cloned());
        }
        // vec.extend(self.gem_shape.iter().cloned());
        if let Some(gem_shape) = &self.gem_shape {
            vec.extend(gem_shape.iter().cloned());
        }
        // vec.extend(self.stone_shape.iter().cloned());
        if let Some(stone_shape) = &self.stone_shape {
            vec.extend(stone_shape.iter().cloned());
        }

        clean_search_vec(vec.as_slice())
    }
}
