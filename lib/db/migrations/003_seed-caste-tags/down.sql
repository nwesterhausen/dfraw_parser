-- Remove all complex tag values
DELETE
FROM complex_tag_2_value
WHERE complex_tag_id IN (SELECT id
                         FROM raw_tag_complex_value
                         WHERE raw_tag_id IN (SELECT id
                                              FROM raw_tag
                                              WHERE raw_tag_type_id = (SELECT id
                                                                       FROM raw_tag_type
                                                                       WHERE name = 'CASTE')));

-- Remove all raw tag complex values
DELETE
FROM raw_tag_complex_value
WHERE raw_tag_id IN (SELECT id
                     FROM raw_tag
                     WHERE raw_tag_type_id = (SELECT id
                                              FROM raw_tag_type
                                              WHERE name = 'CASTE'));

-- Remove all raw tags for the CASTE type
DELETE
FROM raw_tag
WHERE raw_tag_type_id = (SELECT id
                         FROM raw_tag_type
                         WHERE name = 'CASTE');

-- Remove the CASTE tag type
DELETE
FROM raw_tag_type
WHERE name = 'CASTE';