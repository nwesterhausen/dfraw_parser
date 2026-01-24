use std::io::Read;
use std::path::Path;

use quick_xml::{Reader, events::Event};
use tracing::{error, info};

use crate::{
    ParserError,
    legends_export::{ExportedCreature, ExportedEntity},
    metadata::ParserOptions,
    tokens::ObjectType,
    traits::RawObject,
    utilities::try_get_file,
};

use super::util::legends_metadata;

#[derive(Eq, PartialEq)]
enum Current {
    None,
    NameSingular,
    NamePlural,
    Identifier,
    CivId,
    Child,
    Type,
    Race,
}

#[derive(Eq, PartialEq)]
enum Parent {
    None,
    Creature,
    Entity,
}

#[allow(clippy::too_many_lines)]
/// Parses the legends export file at the specified input path and returns a vector of raw objects.
///
/// # Arguments
///
/// * `input_path` - The path to the legends export file.
///
/// # Returns
///
/// A vector of boxed dynamic `RawObject` trait objects.
///
/// # Errors
///
/// Returns a `ParserError` if there is an issue reading the file or parsing the file.
pub fn parse_legends_export<P: AsRef<Path>>(
    input_path: &P,
    options: &ParserOptions,
) -> Result<Vec<Box<dyn RawObject>>, ParserError> {
    let mut results: Vec<Box<dyn RawObject>> = Vec::new();
    let mut file = try_get_file(&input_path.as_ref())?;

    // Read the file into a str for parsing
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)
        .map_err(|e| ParserError::Io { source: e })?;

    info!(
        "parse_legends_export: Parsing file {}",
        input_path
            .as_ref()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );

    let mut reader = Reader::from_str(&file_str);
    reader.config_mut().trim_text(true);

    let mut current_tag = Current::None;
    let mut parent_tag = Parent::None;
    let mut creatures: Vec<ExportedCreature> = Vec::new();
    let mut temp_creature = ExportedCreature::default();
    let mut entities: Vec<ExportedEntity> = Vec::new();
    let mut temp_entity = ExportedEntity::default();

    let mut tag_txt = String::new();

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                error!(
                    "parse_legends_export: Error at position {}: {:?}",
                    reader.buffer_position(),
                    e
                );
                break;
            }
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"creature" => parent_tag = Parent::Creature,
                b"name_singular" => current_tag = Current::NameSingular,
                b"name_plural" => current_tag = Current::NamePlural,
                b"creature_id" | b"id" => current_tag = Current::Identifier,
                b"race" => current_tag = Current::Race,
                b"civ_id" => current_tag = Current::CivId,
                b"type" => current_tag = Current::Type,
                b"child" => current_tag = Current::Child,
                b"entity" => parent_tag = Parent::Entity,
                _ => (),
            },
            Ok(Event::Text(e)) => {
                if parent_tag != Parent::None {
                    tag_txt = e
                        .decode()
                        .map_or_else(|_| String::new(), std::borrow::Cow::into_owned);
                }
                match parent_tag {
                    Parent::Creature => match current_tag {
                        Current::NameSingular => temp_creature.set_name_singular(&tag_txt),
                        Current::NamePlural => temp_creature.set_name_plural(&tag_txt),
                        Current::Identifier => temp_creature.set_creature_id(&tag_txt),
                        _ => (),
                    },
                    Parent::Entity => match current_tag {
                        Current::Identifier => {
                            temp_entity.set_id(tag_txt.parse().unwrap_or_default());
                        }
                        Current::Child => {
                            temp_entity.set_child_id(tag_txt.parse().unwrap_or_default());
                        }
                        Current::Race => temp_entity.set_race(&tag_txt),
                        Current::Type => temp_entity.set_entity_type(&tag_txt),
                        _ => (),
                    },
                    Parent::None => (),
                }
            }

            Ok(Event::Empty(e)) => {
                let tag_name = reader
                    .decoder()
                    .decode(e.name().as_ref())
                    .map_or_else(|_| String::new(), std::borrow::Cow::into_owned);
                if parent_tag == Parent::Creature {
                    temp_creature.add_tag(&tag_name);
                }
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"creature" => {
                    parent_tag = Parent::None;
                    if !temp_creature.is_empty() {
                        creatures.push(temp_creature);
                        temp_creature = ExportedCreature::default();
                    }
                }
                b"entity" => {
                    parent_tag = Parent::None;
                    entities.push(temp_entity);
                    temp_entity = ExportedEntity::default();
                }
                _ => (),
            },
            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    let legend_metadata = legends_metadata(input_path.as_ref(), ObjectType::Creature, options);

    for creature in creatures {
        results.push(Box::new(creature.into_creature(&legend_metadata)));
    }

    let legend_metadata = legends_metadata(input_path.as_ref(), ObjectType::Entity, options);

    for entity in entities {
        results.push(Box::new(entity.into_entity(&legend_metadata)));
    }

    Ok(results)
}
