-- Insert simple tags (no values)
WITH caste_type AS (SELECT id
                    FROM object_type
                    WHERE identifier = 'CREATURE_CASTE')
INSERT
INTO raw_tag (identifier, name, description, has_value, has_complex_value, object_type)
VALUES ('ADOPTS_OWNER', 'Adopts Owner',
        'Prevents tamed creature from being made available for adoption, instead allowing it to automatically adopt whoever it wants.',
        0, 0, (SELECT id FROM caste_type)),
       ('ALCOHOL_DEPENDENT', 'Alcohol Dependent',
        'Makes the creature need alcohol to get through the working day; it will choose to drink booze instead of water if possible.',
        0, 0, (SELECT id FROM caste_type)),
       ('ALL_ACTIVE', 'All Active',
        'Sets the creature to be active during the day, night, and twilight in Adventurer Mode.', 0, 0, (SELECT id FROM caste_type)),
       ('AMBUSHPREDATOR', 'Ambush Predator',
        'Found on [LargePredator]s who ambush their prey. Instead of charging relentlessly at prey, the predator will wait till the prey is within a few squares before charging.',
        0, 0, (SELECT id FROM caste_type)),
       ('AMPHIBIOUS', 'Amphibious',
        'Allows a creature to breathe both in and out of water - does not prevent drowning in magma.', 0, 0, (SELECT id FROM caste_type)),
       ('AQUATIC', 'Aquatic', 'Enables the creature to breathe in water, but causes it to air-drown on dry land.', 0,
        0, (SELECT id FROM caste_type)),
       ('ARENA_RESTRICTED', 'Arena Restricted',
        'Causes the creature to be excluded from the object testing arena''s creature spawning list.', 0, 0, (SELECT id FROM caste_type)),
       ('AT_PEACE_WITH_WILDLIFE', 'At Peace With Wildlife',
        'Prevents the creature from attacking or frightening creatures with the [Natural] tag.', 0, 0, (SELECT id FROM caste_type)),
       ('APPLY_CURRENT_CREATURE_VARIATION', 'Apply Current Creature Variation',
        'Applies the effects of all pending [CV_ADD_TAG] and [CV_REMOVE_TAG] tokens.', 0, 0, (SELECT id FROM caste_type)),
       ('NOSTUN', 'No Stun', 'If this creature needs to sleep while playing, it will die.', 0, 0, (SELECT id FROM caste_type)),
       ('NOT_BUTCHERABLE', 'Not Butcherable', 'Cannot be butchered.', 0, 0, (SELECT id FROM caste_type)),
       ('NOT_LIVING', 'Not Living', 'Cannot be raised from the dead by necromancers or evil clouds.', 0, 0, (SELECT id FROM caste_type)),
       ('NOTHOUGHT', 'No Thought', 'Creature doesn''t require a [THOUGHT] body part to survive.', 0, 0, (SELECT id FROM caste_type)),
       ('OPPOSED_TO_LIFE', 'Opposed To Life', 'Is hostile to all creatures except undead and other non-living ones.', 0,
        0, (SELECT id FROM caste_type)),
       ('OUTSIDER_CONTROLLABLE', 'Outsider Controllable',
        'Lets you play as an outsider of this species in adventure mode.', 0, 0, (SELECT id FROM caste_type)),
       ('PACK_ANIMAL', 'Pack Animal', 'Allows the creature to be used as a pack animal.', 0, 0, (SELECT id FROM caste_type)),
       ('PARALYZEIMMUNE', 'Paralyze Immune', 'The creature is immune to all paralyzing special attacks.', 0, 0, (SELECT id FROM caste_type)),
       ('PET', 'Pet', 'Allows the creature to be tamed in Fortress mode.', 0, 0, (SELECT id FROM caste_type)),
       ('PET_EXOTIC', 'Pet Exotic',
        'Allows the creature to be tamed in Fortress mode but civilizations cannot domesticate it in worldgen.', 0, 0,
        (SELECT id FROM caste_type)),
       ('POWER', 'Power', 'Allows the being to represent itself as a deity.', 0, 0, (SELECT id FROM caste_type)),
       ('REMAINS_ON_VERMIN_BITE_DEATH', 'Remains On Vermin Bite Death',
        'The vermin creature will leave remains on death when biting.', 0, 0, (SELECT id FROM caste_type)),
       ('RETURNS_VERMIN_KILLS_TO_OWNER', 'Returns Vermin Kills To Owner',
        'If it kills a vermin creature and has an owner, it carries the remains to their feet.', 0, 0, (SELECT id FROM caste_type)),
       ('SEMIMEGABEAST', 'Semi Megabeast', 'Similar to [MEGABEAST], but more are created during worldgen.', 0, 0, (SELECT id FROM caste_type)),
       ('SLOW_LEARNER', 'Slow Learner', 'Shorthand for [CAN_LEARN] + [SKILL_LEARN_RATES:50].', 0, 0, (SELECT id FROM caste_type)),
       ('SMALL_REMAINS', 'Small Remains', 'Creature leaves "remains" instead of a corpse. Used by vermin.', 0, 0, (SELECT id FROM caste_type)),
       ('STANCE_CLIMBER', 'Stance Climber', 'Caste does not require [GRASP] body parts to climb.', 0, 0, (SELECT id FROM caste_type)),
       ('STANDARD_GRAZER', 'Standard Grazer', 'Acts as [GRAZER] but with a calculated value.', 0, 0, (SELECT id FROM caste_type)),
       ('STRANGE_MOODS', 'Strange Moods',
        'The creature will get strange moods in fortress mode and can produce artifacts.', 0, 0, (SELECT id FROM caste_type));

-- Insert tags with simple values
WITH caste_type AS (SELECT id
                    FROM object_type
                    WHERE identifier = 'CREATURE_CASTE')
INSERT
INTO raw_tag (identifier, name, description, has_value, has_complex_value, object_type)
VALUES ('CASTE_ALTTILE', 'Caste Alt Tile', 'Caste-specific version of [Creature::AltTile]. Requires [Tile].', 1, 0, (SELECT id FROM caste_type)),
       ('BABY', 'Baby', 'Age at which creature is considered a child, the default is zero.', 1, 0, (SELECT id FROM caste_type)),
       ('ODOR_LEVEL', 'Odor Level',
        'How easy the creature is to smell. The higher the number, the easier the creature is to sniff out.', 1, 0, (SELECT id FROM caste_type)),
       ('ODOR_STRING', 'Odor String', 'What the creature smells like.', 1, 0, (SELECT id FROM caste_type)),
       ('PETVALUE', 'Pet Value', 'How valuable a tamed animal is.', 1, 0, (SELECT id FROM caste_type)),
       ('POP_RATIO', 'Population Ratio', 'Weighted population of caste; Lower is rarer.', 1, 0, (SELECT id FROM caste_type)),
       ('PRONE_TO_RAGE', 'Prone To Rage',
        'Creature has a percentage chance to flip out at visible non-friendly creatures.', 1, 0, (SELECT id FROM caste_type)),
       ('VIEWRANGE', 'View Range', 'Sets how close you have to get to a creature before it attacks.', 1, 0, (SELECT id FROM caste_type)),
       ('WEBBER', 'Webber', 'Allows the creature to create webs, and defines what the webs are made of.', 1, 0, (SELECT id FROM caste_type));

-- Insert tags with complex values
WITH caste_type AS (SELECT id
                    FROM object_type
                    WHERE identifier = 'CREATURE_CASTE')
INSERT
INTO raw_tag (identifier, name, description, has_value, has_complex_value, object_type)
VALUES ('APPLY_CREATURE_VARIATION', 'Apply Creature Variation', 'Applies a creature variation to the creature.', 1, 1,
        (SELECT id FROM caste_type)),
       ('ARBITRARY_TAGS', 'Arbitrary Tags', 'Adds arbitrary tags to the creature.', 1, 1, (SELECT id FROM caste_type)),
       ('ATTACK_FLAG', 'Attack Flag', 'Adds a flag to the attack.', 1, 1, (SELECT id FROM caste_type)),
       ('BODY_APPEARANCE_MODIFIER', 'Body Appearance Modifier', 'Allows customization of body appearance with ranges.',
        1, 1, (SELECT id FROM caste_type)),
       ('BODY_SIZE', 'Body Size', 'Specifies body size at different ages.', 1, 1, (SELECT id FROM caste_type)),
       ('CREATURE_CLASS', 'Creature Class', 'Assigns creature to a class for various game mechanics.', 1, 1, (SELECT id FROM caste_type)),
       ('DESCRIPTION', 'Description', 'Provides a description for the creature.', 1, 1, (SELECT id FROM caste_type)),
       ('EGG_MATERIAL', 'Egg Material', 'Specifies the materials of eggs.', 1, 1, (SELECT id FROM caste_type)),
       ('LITTERSIZE', 'Litter Size', 'Sets the range for offspring count per birth.', 1, 1, (SELECT id FROM caste_type)),
       ('LOW_LIGHT_VISION', 'Low Light Vision', 'Determines ability to see in darkness.', 1, 1, (SELECT id FROM caste_type)),
       ('MANNERISM_FINGERS', 'Mannerism Fingers', 'Adds finger-related mannerisms to profiles.', 1, 1, (SELECT id FROM caste_type)),
       ('MAX_AGE', 'Maximum Age', 'Sets the lifespan range of the creature.', 1, 1, (SELECT id FROM caste_type)),
       ('MENTAL_ATTRIBUTE_CAP_PERCENTAGE', 'Mental Attribute Cap', 'Sets maximum potential for a mental attribute.', 1,
        1, (SELECT id FROM caste_type)),
       ('MENTAL_ATTRIBUTE_RANGE', 'Mental Attribute Range', 'Sets the range of values for a mental attribute.', 1, 1,
        (SELECT id FROM caste_type)),
       ('MENTAL_ATTRIBUTE_RATE', 'Mental Attribute Rate', 'Sets gain and decay rates for mental attributes.', 1, 1, (SELECT id FROM caste_type)),
       ('MILKABLE', 'Milkable', 'Allows creature to be milked for a specific material.', 1, 1, (SELECT id FROM caste_type)),
       ('NAME', 'Name', 'Defines the singular, plural, and adjective forms of the caste name.', 1, 1, (SELECT id FROM caste_type)),
       ('NATURAL_SKILL', 'Natural Skill', 'Gives the creature an innate skill level.', 1, 1, (SELECT id FROM caste_type)),
       ('ORIENTATION', 'Orientation', 'Sets sexual attraction probabilities.', 1, 1, (SELECT id FROM caste_type)),
       ('PERSONALITY', 'Personality', 'Sets personality trait ranges.', 1, 1, (SELECT id FROM caste_type)),
       ('PHYSICAL_ATTRIBUTE_RANGE', 'Physical Attribute Range', 'Sets the range of values for a physical attribute.', 1,
        1, (SELECT id FROM caste_type)),
       ('PHYSICAL_ATTRIBUTE_RATE', 'Physical Attribute Rate', 'Sets gain and decay rates for physical attributes.', 1,
        1, (SELECT id FROM caste_type)),
       ('RETRACT_INTO_BP', 'Retract into Body Part', 'Allows creature to retract into a body part when threatened.', 1,
        1, (SELECT id FROM caste_type)),
       ('ROOT_AROUND', 'Root Around', 'Enables creature to root for food with specific behaviors.', 1, 1, (SELECT id FROM caste_type)),
       ('SECRETION', 'Secretion', 'Defines material secretion from body parts under conditions.', 1, 1, (SELECT id FROM caste_type)),
       ('SENSE_CREATURE_CLASS', 'Sense Creature Class', 'Allows sensing specific creature types.', 1, 1, (SELECT id FROM caste_type)),
       ('SKILL_RATE', 'Skill Rate', 'Controls skill improvement and decay rates.', 1, 1, (SELECT id FROM caste_type)),
       ('SKILL_RATES', 'Skill Rates', 'Sets overall skill learning and decay rates.', 1, 1, (SELECT id FROM caste_type)),
       ('SOUND', 'Sound', 'Defines sounds the creature makes and how they are heard.', 1, 1, (SELECT id FROM caste_type)),
       ('SPECIFIC_FOOD', 'Specific Food', 'Specifies required food types for the creature.', 1, 1, (SELECT id FROM caste_type)),
       ('TISSUE_LAYER', 'Tissue Layer', 'Adds tissue layers to body parts.', 1, 1, (SELECT id FROM caste_type)),
       ('VERMIN_BITE', 'Vermin Bite', 'Enables biting with material injection.', 1, 1, (SELECT id FROM caste_type));


-- For AltTile
INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'tile', 'Tile', 'The tile to use for this caste'
FROM raw_tag
WHERE identifier = 'ALT_TILE'
  AND has_complex_value = 1;

-- For ApplyCreatureVariation
INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'id', 'Variation ID', 'Creature variation ID to apply'
FROM raw_tag
WHERE identifier = 'APPLY_CREATURE_VARIATION'
  AND has_complex_value = 1;

INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'args', 'Arguments', 'Arguments to pass to the creature variation'
FROM raw_tag
WHERE identifier = 'APPLY_CREATURE_VARIATION'
  AND has_complex_value = 1;

-- For Attack
INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'name', 'Attack Name', 'The name of the attack'
FROM raw_tag
WHERE identifier = 'ATTACK'
  AND has_complex_value = 1;

INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'body_part', 'Body Part', 'The body part used for the attack'
FROM raw_tag
WHERE identifier = 'ATTACK'
  AND has_complex_value = 1;

-- For Orientation
INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'caste', 'Caste', 'The caste to set orientation to (MALE or FEMALE typically)'
FROM raw_tag
WHERE identifier = 'ORIENTATION'
  AND has_complex_value = 1;

INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'disinterested_chance', 'Disinterested Chance', 'Chance of being disinterested in this caste'
FROM raw_tag
WHERE identifier = 'ORIENTATION'
  AND has_complex_value = 1;

INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'casual_chance', 'Casual Chance', 'Chance of being casually interested in this caste'
FROM raw_tag
WHERE identifier = 'ORIENTATION'
  AND has_complex_value = 1;

INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'strong_chance', 'Strong Chance', 'Chance of being strongly interested in this caste'
FROM raw_tag
WHERE identifier = 'ORIENTATION'
  AND has_complex_value = 1;

-- For Personality
INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'personality_trait', 'Trait', 'The personality trait to modify'
FROM raw_tag
WHERE identifier = 'PERSONALITY'
  AND has_complex_value = 1;

INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'low', 'Minimum', 'The lowest chance of having this trait'
FROM raw_tag
WHERE identifier = 'PERSONALITY'
  AND has_complex_value = 1;

INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'median', 'Median', 'The median chance of having this trait'
FROM raw_tag
WHERE identifier = 'PERSONALITY'
  AND has_complex_value = 1;

INSERT INTO raw_tag_complex_value (raw_tag_id, identifier, name, description)
SELECT id, 'high', 'Maximum', 'The highest chance of having this trait'
FROM raw_tag
WHERE identifier = 'PERSONALITY'
  AND has_complex_value = 1;