pub const APPLIED_CREATURE_VARIATIONS_TABLE: &str = r"
CREATE TABLE applied_creature_variations (
    id INTEGER PRIMARY KEY,

    -- The variation identifier (e.g., 'ANIMAL_PERSON', 'GIANT')
    identifier TEXT NOT NULL,

    -- Parent Linkage (Nullable FKs)
    creature_id INTEGER,
    caste_id INTEGER,

    FOREIGN KEY (creature_id) REFERENCES creatures(id),
    FOREIGN KEY (caste_id) REFERENCES castes(id),

    -- SQL constraint: Ensure it attaches to exactly one parent type
    CONSTRAINT check_one_parent CHECK (
        (creature_id IS NOT NULL AND caste_id IS NULL) OR
        (creature_id IS NULL AND caste_id IS NOT NULL)
    )
);";

pub const APPLIED_CREATURE_VARIATION_ARGUMENTS_TABLE: &str = r"
CREATE TABLE caste_applied_variation_args (
    variation_application_id INTEGER NOT NULL,
    argument_value TEXT,
    argument_order INTEGER NOT NULL, -- To reconstruct the list in order

    PRIMARY KEY (variation_application_id, argument_order),
    FOREIGN KEY (variation_application_id) REFERENCES applied_creature_variations(id)
);";
