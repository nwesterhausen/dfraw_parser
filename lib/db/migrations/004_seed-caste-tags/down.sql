-- Remove all complex tag values
DELETE
FROM complex_tag_2_value
WHERE complex_tag_id IN (SELECT id
                         FROM raw_tag_complex_value
                         WHERE raw_tag_id IN (SELECT id
                                              FROM raw_tag
                                              WHERE object_type = (SELECT id
                                                                   FROM object_type
                                                                   WHERE identifier = 'CREATURE_CASTE')));

-- Remove all raw tag complex values
DELETE
FROM raw_tag_complex_value
WHERE raw_tag_id IN (SELECT id
                     FROM raw_tag
                     WHERE object_type = (SELECT id
                                          FROM object_type
                                          WHERE identifier = 'CREATURE_CASTE'));

-- Remove all raw tags for the CASTE type
DELETE
FROM raw_tag
WHERE object_type = (SELECT id
                     FROM object_type
                     WHERE identifier = 'CREATURE_CASTE');
