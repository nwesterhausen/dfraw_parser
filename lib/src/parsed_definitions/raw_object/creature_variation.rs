use tracing::warn;
use uuid::Uuid;

use crate::{
    CreatureVariation,
    metadata::RawMetadata,
    raw_definitions::CREATURE_VARIATION_TOKENS,
    tokens::{CreatureVariationRuleToken, CreatureVariationToken, ObjectType},
    traits::RawObject,
};

#[typetag::serde]
impl RawObject for CreatureVariation {
    fn get_metadata(&self) -> RawMetadata {
        self.metadata.clone()
    }
    fn get_identifier(&self) -> &str {
        self.identifier.as_str()
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::CreatureVariation
    }

    #[allow(clippy::too_many_lines)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(token) = CREATURE_VARIATION_TOKENS.get(key) else {
            warn!("Unknown tag in creature variation: {}", key);
            return;
        };

        // We need to split up the value string into it's parts.
        //
        // Add/new tags [CV_TAG:value] (value is optional)
        // Add/new with conditions [CV_TAG:argument_index:argument_value:value(s)] (value is optional)
        // Remove tags [CV_TAG] (value is optional)
        // Remove with conditions [CV_TAG:argument_index:argument_value:value(s)] (value is optional)
        // Convert tags
        //  [CV_CONVERT_TAG]
        //      [CVCT_MASTER:tag:value]
        //      [CVCT_TARGET:tag:value(s)]
        //      [CVCT_REPLACEMENT:tag:value(s)]
        // Convert with conditions
        //  [CV_CONVERT_CTAG:argument_index:argument_value]
        //      [CVCT_MASTER:tag:argument_index:argument_value]
        //      [CVCT_TARGET:tag:argument_index:argument_value(s)]
        //      [CVCT_REPLACEMENT:tag:argument_index:argument_value(s)]

        self.tags.push((*token, value.to_string()));

        let mut parts = value.split(':');

        match token {
            CreatureVariationToken::AddTag | CreatureVariationToken::NewTag => {
                // Parts can be any number of strings long, but the first part is always the tag
                let tag = parts.next().unwrap_or_default().to_string();
                // For Add and New we just want to squish all the remaining parts together for value
                let value = parts.collect::<Vec<&str>>().join(":");
                let value = if value.is_empty() { None } else { Some(value) };

                self.rules
                    .push(CreatureVariationRuleToken::AddTag { tag, value });
            }
            CreatureVariationToken::ConditionalAddTag
            | CreatureVariationToken::ConditionalNewTag => {
                // For conditional tags, the first part is the argument index, the second part is the
                // argument value, the third part is the tag, and the remaining parts are the value.
                let argument_index = parts.next().unwrap_or_default();
                let Ok(argument_index) = argument_index.parse::<usize>() else {
                    warn!(
                        "Invalid index argument '{}' for conditional tag: {}",
                        argument_index, key
                    );
                    return;
                };
                let argument_requirement = parts.next().unwrap_or_default().to_string();
                let tag = parts.next().unwrap_or_default().to_string();
                let value = parts.collect::<Vec<&str>>().join(":");
                let value = if value.is_empty() { None } else { Some(value) };

                self.rules
                    .push(CreatureVariationRuleToken::ConditionalAddTag {
                        argument_index,
                        tag,
                        value,
                        argument_requirement,
                    });
            }
            CreatureVariationToken::RemoveTag => {
                // Parts can be any number of strings long, but the first part is always the tag
                let tag = parts.next().unwrap_or_default().to_string();
                // For Add and New we just want to squish all the remaining parts together for value
                let value = parts.collect::<Vec<&str>>().join(":");
                let value = if value.is_empty() { None } else { Some(value) };

                self.rules
                    .push(CreatureVariationRuleToken::RemoveTag { tag, value });
            }
            CreatureVariationToken::ConditionalRemoveTag => {
                // For conditional tags, the first part is the argument index, the second part is the
                // argument value, the third part is the tag, and the remaining parts are the value.
                let argument_index = parts.next().unwrap_or_default();
                let Ok(argument_index) = argument_index.parse::<usize>() else {
                    warn!(
                        "Invalid index argument '{}' for conditional tag: {}",
                        argument_index, key
                    );
                    return;
                };
                let argument_requirement = parts.next().unwrap_or_default().to_string();
                let tag = parts.next().unwrap_or_default().to_string();
                let value = parts.collect::<Vec<&str>>().join(":");
                let value = if value.is_empty() { None } else { Some(value) };

                self.rules
                    .push(CreatureVariationRuleToken::ConditionalRemoveTag {
                        tag,
                        value,
                        argument_index,
                        argument_requirement,
                    });
            }
            CreatureVariationToken::ConvertTag => {
                // Convert tag actually just tells us that we're starting a convert tag rule.
                self.rules.push(CreatureVariationRuleToken::ConvertTag {
                    tag: String::new(),
                    replacement: None,
                    target: None,
                });
            }
            CreatureVariationToken::ConditionalConvertTag => {
                // For conditional tags, the first part is the argument index, the second part is the
                // argument value, the third part is the tag, and the remaining parts are the value.
                let argument_index = parts.next().unwrap_or_default();
                let Ok(argument_index) = argument_index.parse::<usize>() else {
                    warn!(
                        "Invalid index argument '{}' for conditional tag: {}",
                        argument_index, key
                    );
                    return;
                };
                let argument_requirement = parts.next().unwrap_or_default().to_string();

                self.rules
                    .push(CreatureVariationRuleToken::ConditionalConvertTag {
                        argument_index,
                        argument_requirement,
                        tag: String::new(),
                        replacement: None,
                        target: None,
                    });
            }
            CreatureVariationToken::ConvertTagMaster => {
                // Grab the last rule and set the master (i.e. the target tag)
                let Some(rule) = self.rules.last_mut() else {
                    warn!("No rule to add master tag to for tag: {}", key);
                    return;
                };

                let Some(new_tag) = parts.next() else {
                    warn!("No target tag for convert tag: {}", key);
                    return;
                };

                match rule {
                    CreatureVariationRuleToken::ConvertTag { tag, .. }
                    | CreatureVariationRuleToken::ConditionalConvertTag { tag, .. } => {
                        *tag = String::from(new_tag);
                    }
                    CreatureVariationRuleToken::Unknown => {
                        warn!("No rule to add master tag to for tag: {}", key);
                    }
                    _ => {
                        warn!("Invalid rule to add master tag to for tag: {}", key);
                    }
                }
            }
            CreatureVariationToken::ConvertTagTarget => {
                // Grab the last rule and set the target (i.e. the tag to convert)
                let Some(rule) = self.rules.last_mut() else {
                    warn!("No rule to add target tag to for tag: {}", key);
                    return;
                };

                let Some(new_target) = parts.next() else {
                    warn!("No target tag for convert tag: {}", key);
                    return;
                };

                match rule {
                    CreatureVariationRuleToken::ConvertTag { target, .. }
                    | CreatureVariationRuleToken::ConditionalConvertTag { target, .. } => {
                        *target = Some(String::from(new_target));
                    }
                    CreatureVariationRuleToken::Unknown => {
                        warn!("No rule to add target tag to for tag: {}", key);
                    }
                    _ => {
                        warn!("Invalid rule to add target tag to for tag: {}", key);
                    }
                }
            }
            CreatureVariationToken::ConvertTagReplacement => {
                // Grab the last rule and set the replacement (i.e. the tag to convert to)
                let Some(rule) = self.rules.last_mut() else {
                    warn!("No rule to add replacement tag to for tag: {}", key);
                    return;
                };

                let Some(new_replacement) = parts.next() else {
                    warn!("No replacement tag for convert tag: {}", key);
                    return;
                };

                match rule {
                    CreatureVariationRuleToken::ConvertTag { replacement, .. }
                    | CreatureVariationRuleToken::ConditionalConvertTag { replacement, .. } => {
                        *replacement = Some(String::from(new_replacement));
                    }
                    CreatureVariationRuleToken::Unknown => {
                        warn!("No rule to add replacement tag to for tag: {}", key);
                    }
                    _ => {
                        warn!("Invalid rule to add replacement tag to for tag: {}", key);
                    }
                }
            }
            CreatureVariationToken::Unknown => {
                warn!("Unknown tag in creature variation: {}", key);
            }
        }
    }
    fn get_object_id(&self) -> Uuid {
        self.object_id
    }
    fn get_name(&self) -> &str {
        self.identifier.as_str()
    }
}
