use tracing::{debug, trace};

use crate::{
    Creature, CreatureVariation, ParserError,
    metadata::RawMetadata,
    tags::{ModificationTag, ObjectType},
    traits::RawObject,
    utilities::singularly_apply_creature_variation,
};

/// An unprocessed raw object
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UnprocessedRaw {
    /// The raw type of the object. This is to tell us what to parse it into.
    raw_type: ObjectType,
    /// The modifications to apply to the object.
    ///
    /// Allows us to handle parsing advanced things like
    ///
    /// * `COPY_TAGS_FROM` tag
    /// * `APPLY_CREATURE_VARIATION` tag
    /// * `GO_TO_TAG:tag` raw instruction
    /// * `GO_TO_END` and `GO_TO_START` raw instructions
    /// * (and includes the raws themselves under `MainRawBody`)
    ///
    /// So when the raws are parsed from this into the actual object, we can apply these modifications
    /// in order to get the final object.
    modifications: Vec<ModificationTag>,
    /// Metadata to be passed on to the final object
    metadata: RawMetadata,
    /// Identifier of the object
    identifier: String,
}

impl UnprocessedRaw {
    /// Creates a new unprocessed raw object
    ///
    /// # Arguments
    ///
    /// * `raw_type` - The raw type of the object
    /// * `metadata` - Metadata to be passed on to the final object
    ///
    /// # Returns
    ///
    /// A new unprocessed raw object
    #[must_use]
    #[allow(dead_code)]
    pub fn new(raw_type: ObjectType, metadata: &RawMetadata, identifier: &str) -> Self {
        Self {
            raw_type,
            metadata: metadata.clone(),
            identifier: identifier.to_string(),
            ..Default::default()
        }
    }

    /// Gets the raw type of the object. This is to tell us what to parse it into.
    ///
    /// # Returns
    ///
    /// The raw type of the object
    #[must_use]
    #[allow(dead_code)]
    pub fn raw_type(&self) -> ObjectType {
        self.raw_type
    }

    /// Check if there is nothing to parse
    ///
    /// # Returns
    ///
    /// * `true` if there is nothing to parse
    #[must_use]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.identifier.is_empty() && self.modifications.is_empty()
    }

    /// Gets the identifier of the object
    #[must_use]
    #[allow(dead_code)]
    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    /// Checks if the only modifications are
    ///
    /// * `MainRawBody`
    /// * `AddToBeginning`
    /// * `AddToEnding`
    /// * `AddBeforeTag`
    /// * `ApplyCreatureVariation`
    ///
    /// This is used to determine if we can parse the raws into the object without having to do any
    /// parsing against other creatures (which may be the result of `resolve`ing the raws)
    ///
    /// # Returns
    ///
    /// * `true` if the raws are simple
    #[must_use]
    #[allow(dead_code)]
    pub fn is_simple(&self) -> bool {
        self.modifications.iter().all(|m| {
            matches!(
                m,
                ModificationTag::MainRawBody { .. }
                    | ModificationTag::AddToBeginning { .. }
                    | ModificationTag::AddToEnding { .. }
                    | ModificationTag::AddBeforeTag { .. }
                    | ModificationTag::ApplyCreatureVariation { .. }
            )
        })
    }

    /// Gets the modifications to apply to the object.
    ///
    /// Allows us to handle parsing advanced things like
    ///
    /// * `COPY_TAGS_FROM` tag
    /// * `APPLY_CREATURE_VARIATION` tag
    /// * `GO_TO_TAG:tag` raw instruction
    /// * `GO_TO_END` and `GO_TO_START` raw instructions
    ///
    /// So when the raws are parsed from this into the actual object, we can apply these modifications
    /// in order to get the final object.
    ///
    /// # Returns
    ///
    /// The modifications to apply to the object
    #[must_use]
    #[allow(dead_code)]
    pub fn modifications(&self) -> &[ModificationTag] {
        &self.modifications
    }

    /// Adds a modification to the object
    #[allow(clippy::collapsible_match)]
    #[allow(dead_code)]
    pub fn add_modification(&mut self, modification: ModificationTag) {
        // We want to "squish" the modifications together if possible
        // So we want to compare the type of the last modification to the type of the new modification
        // If they are the same, we want to combine them, unless we can't
        // (like with `COPY_TAGS_FROM` and `APPLY_CREATURE_VARIATION`)
        match modification.clone() {
            ModificationTag::AddBeforeTag { tag, raws } => {
                // Check if last modification is also an `AddBeforeTag`
                if let Some(last_modification) = self.modifications.last_mut()
                    && let ModificationTag::AddBeforeTag {
                        tag: last_tag,
                        raws: last_raws,
                    } = last_modification
                {
                    // Check if the tags are the same
                    if &tag == last_tag {
                        // They are the same, so we can combine them
                        last_raws.extend(raws);
                        return;
                    }
                }
            }
            ModificationTag::AddToBeginning { raws } => {
                // Check if last modification is also an `AddToBeginning`
                if let Some(last_modification) = self.modifications.last_mut()
                    && let ModificationTag::AddToBeginning { raws: last_raws } = last_modification
                {
                    // They are the same, so we can combine them
                    last_raws.extend(raws);
                    return;
                }
            }
            ModificationTag::AddToEnding { raws } => {
                // Check if last modification is also an `AddToEnding`
                if let Some(last_modification) = self.modifications.last_mut()
                    && let ModificationTag::AddToEnding { raws: last_raws } = last_modification
                {
                    // They are the same, so we can combine them
                    last_raws.extend(raws);
                    return;
                }
            }
            ModificationTag::MainRawBody { raws } => {
                // Check if last modification is also an `MainRawBody`
                if let Some(last_modification) = self.modifications.last_mut()
                    && let ModificationTag::MainRawBody { raws: last_raws } = last_modification
                {
                    // They are the same, so we can combine them
                    last_raws.extend(raws);
                    return;
                }
            }
            _ => {}
        }

        // If we get here, we can't combine the modifications, so we just add it
        trace!("Adding modification: {:?}", modification);
        self.modifications.push(modification);
    }

    /// Resolves the raws into the final object.
    ///
    /// # Arguments
    ///
    /// * `creature_variations` - all possible creature variations to apply (this should be able to be
    ///   reused between all `resolve` calls, so we pass a reference here instead of re-creating it each time)
    /// * `all_raws` - All the raws to use when resolving the raws
    ///
    /// # Returns
    ///
    /// The resolved raw object or an error if there was a problem
    ///
    /// # Errors
    ///
    /// * `ParserError::NotYetImplemented` - If the raw type is not yet implemented
    pub fn resolve(
        &mut self,
        creature_variations: &[CreatureVariation],
        all_raws: &[Box<dyn RawObject>],
    ) -> Result<Box<dyn RawObject>, ParserError> {
        if self.raw_type != ObjectType::Creature {
            return Err(ParserError::NotYetImplemented);
        }

        // We need to pre-process the modifications to collapse `AddBeforeTag`, `AddToBeginning`,
        // `AddToEnding`, and `MainRawBody` into one modification.
        self.collapse_modifications();

        let mut creature = Creature::new(&self.identifier, &self.metadata);

        for modification in &self.modifications {
            match modification {
                ModificationTag::CopyTagsFrom { identifier } => {
                    // Get the creature we are copying from. If we find more than one, we just use the newest one. (But we log a warning)
                    let mut source_creature_options = all_raws
                        .iter()
                        .filter_map(|raw| {
                            if raw.get_type() == ObjectType::Creature
                                && raw.get_identifier().to_lowercase() == identifier.to_lowercase()
                            {
                                Some(
                                    raw.as_any()
                                        .downcast_ref::<Creature>()
                                        .unwrap_or(&Creature::empty())
                                        .clone(),
                                )
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<Creature>>();

                    if source_creature_options.len() > 1 {
                        debug!(
                            "Found {} creatures with identifier `{}` to copy tags from. Using the newest one.",
                            source_creature_options.len(),
                            identifier
                        );
                        source_creature_options.sort_by(|a, b| {
                            a.get_metadata()
                                .get_module_version()
                                .cmp(b.get_metadata().get_module_version())
                        });
                    }

                    if let Some(source_creature) = source_creature_options.first() {
                        // We found a creature to copy tags from, so we can copy the tags
                        creature = Creature::copy_tags_from(&creature, source_creature);
                    } else {
                        debug!(
                            "Unable to find creature with identifier `{}` to copy tags from",
                            identifier
                        );
                    }
                }
                ModificationTag::ApplyCreatureVariation { identifier } => {
                    if let Some(updated_creature) = singularly_apply_creature_variation(
                        &creature,
                        identifier,
                        creature_variations,
                    ) {
                        creature = updated_creature;
                    }
                }
                ModificationTag::MainRawBody { raws } => {
                    for raw_string in raws {
                        // Split the raw into the key and value (rest of the string)
                        let mut split = raw_string.split(':');
                        let key = split.next().unwrap_or("");
                        let value = split.collect::<Vec<&str>>().join(":");

                        match key {
                            "CASTE" | "SELECT_CASTE" => creature.select_caste(&value),
                            _ => creature.parse_tag(key, &value),
                        }
                    }
                }
                _ => {
                    // Since we've already collapsed the modifications, we don't need to do anything with the
                    // other modification types. If we end up here, its a bug.
                    debug!("Unexpectedly found {:?} modification", modification);
                }
            }
        }

        Ok(Box::new(creature))
    }

    fn collapse_modifications(&mut self) {
        // Grab the base raws first
        let mut collapsed_raws: Vec<String> = Vec::new();
        for modification in &self.modifications {
            if let ModificationTag::MainRawBody { raws } = modification {
                collapsed_raws.extend(raws.clone());
                trace!("collapsed {} base raws", raws.len());
            }
        }

        // Now we can remove all the `MainRawBody` modifications
        self.modifications
            .retain(|m| !matches!(m, ModificationTag::MainRawBody { .. }));

        // Next process the `AddToEnding` modifications
        let mut add_to_ending: Vec<String> = Vec::new();
        for modification in &self.modifications {
            if let ModificationTag::AddToEnding { raws } = modification {
                add_to_ending.extend(raws.clone());
                trace!("collapsed {} add to ending raws", raws.len());
            }
        }

        // Now we can remove all the `AddToEnding` modifications
        self.modifications
            .retain(|m| !matches!(m, ModificationTag::AddToEnding { .. }));

        // Next process the `AddToBeginning` modifications
        let mut add_to_beginning: Vec<String> = Vec::new();
        for modification in &self.modifications {
            if let ModificationTag::AddToBeginning { raws } = modification {
                add_to_beginning.extend(raws.clone());
                trace!("collapsed {} add to beginning raws", raws.len());
            }
        }

        // Now we can remove all the `AddToBeginning` modifications
        self.modifications
            .retain(|m| !matches!(m, ModificationTag::AddToBeginning { .. }));

        // Combine the raws into [add_to_beginning, raws, add_to_ending] (order matters)
        debug!(
            "collapsed {} total raws ({} base, {} add to beginning, {} add to ending)",
            collapsed_raws.len() + add_to_beginning.len() + add_to_ending.len(),
            collapsed_raws.len(),
            add_to_beginning.len(),
            add_to_ending.len()
        );

        collapsed_raws.splice(0..0, add_to_beginning);
        collapsed_raws.extend(add_to_ending);

        // Finally process the `AddBeforeTag` modifications
        // These have to get inserted before the tag, so we need to find where to insert first
        for modification in &self.modifications {
            if let ModificationTag::AddBeforeTag { tag, raws } = modification {
                // Find the index of the tag
                let index = collapsed_raws.iter().position(|r| r.starts_with(tag));

                // If we found the index, insert the raws before the tag (without replacing)
                if let Some(index) = index {
                    collapsed_raws.splice(index..index, raws.clone());
                    trace!(
                        "collapsed {} add before tag raws, before tag {}",
                        raws.len(),
                        tag
                    );
                } else {
                    // If we didn't find the index, just add the raws to the end
                    collapsed_raws.extend(raws.clone());
                    debug!(
                        "resolve: Unable to find tag `{}` to add raws before. Adding raws to end instead.",
                        tag
                    );
                }
            }
        }

        // Add the collapsed raws back as a `MainRawBody` modification
        self.modifications.push(ModificationTag::MainRawBody {
            raws: collapsed_raws,
        });
    }
}
