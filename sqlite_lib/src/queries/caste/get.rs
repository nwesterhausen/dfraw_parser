use const_format::concatcp;

/// Query to get all castes
pub const GET_ALL_CASTES: &str = r"
SELECT
    -- 1. Identity & Scalars
    c.id,
    c.creature_id,
    c.identifier AS caste_identifier,
    c.description,
    c.baby_name_singular,
    c.caste_name_singular,
    c.pet_value,
    c.pop_ratio,

    -- 2. Visuals (1:1 Relationship from caste_tiles)
    t.character AS tile_char,
    t.alt_character AS tile_alt_char,
    t.color AS tile_color,
    t.glow_color,

    -- 3. Flags (Aggregated Lists of Strings)
    (SELECT json_group_array(ref.token)
     FROM caste_flags cf
     JOIN ref_caste_flag_types ref ON cf.flag_id = ref.id
     WHERE cf.caste_id = c.id
    ) AS flags,

    (SELECT json_group_array(class_name)
     FROM caste_creature_classes ccc
     WHERE ccc.caste_id = c.id
    ) AS classes,

    -- 4. (Aggregated Lists of Objects)
    (SELECT json_group_array(json_object(
        'years', years,
        'days', days,
        'size', size_cm3
     ))
     FROM caste_body_sizes bs
     WHERE bs.caste_id = c.id
    ) AS body_sizes,

    (SELECT json_group_array(json_object(
        'type', gait_type,
        'max_speed', max_speed,
        'start_speed', start_speed,
        'energy', energy_usage
     ))
     FROM caste_gaits cg
     WHERE cg.caste_id = c.id
    ) AS gaits,

    -- 7. (Aggregated Lists of Complex Objects)
    (SELECT json_group_array(json_object(
        'variation', av.identifier,
        'order', av.sort_order,
        'args', (
            SELECT json_group_array(arg.argument_value)
            FROM applied_variation_args arg
            WHERE arg.variation_id = av.id
            ORDER BY arg.argument_order
        )
     ))
     FROM applied_variations av
     WHERE av.caste_id = c.id
     ORDER BY av.sort_order
    ) AS variations

FROM castes c
LEFT JOIN caste_tiles t ON c.id = t.caste_id";

/// Puts the creature id paramter as '?1'
pub const GET_BY_CREATURE_ID: &str = concatcp!(GET_ALL_CASTES, " WHERE c.creature_id = ?1");

/// Puts the caste id parameter as '?1'
pub const GET_BY_ID: &str = concatcp!(GET_ALL_CASTES, " WHERE c.id = ?1");
