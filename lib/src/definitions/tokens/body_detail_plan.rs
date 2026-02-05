use crate::custom_types::{BodyPartPosition, BodyPartSpecifier};

pub enum BodyDetailPlanToken {
    /// Create a new body detail plan with the specified identifier. The rest of the tokens in this table are then used to define this body detail plan's properties.
    PlanMarker { identifier: String },
    /// Adds a new material to the creature based on the specified template and assigned to the specified identifier.
    AddMaterial {
        identifier: String,
        material_template: String,
    },
    /// Adds a new tissue to the creature based on the specified template and assigned to the specified identifier.
    AddTissue {
        identifier: String,
        tissue_template: String,
    },
    /// Defines a series of tissue layers. Alternatively to specifying a tissue, variable arguments can be entered (numbered arbitrarily to a max of 5) to be
    /// filled with tissues when the plan is called in the creature entry. The `SELECT_TISSUE` creature token with `TL_RELATIVE_THICKNESS` can change tissue thickness,
    /// but tissuen layering is hard to do without a new detail plan.
    ///
    /// Arguments are defined as:
    /// - tissue name (or tissue ARG# for innermost tissue)
    /// - tissue thickness
    Layers {
        specifier: BodyPartSpecifier,
        body_part: String,
        arguments: Vec<(String, u32)>,
    },
    /// Works like BP_LAYERS, but defines layers over existing layers.
    LayersOver {
        specifier: BodyPartSpecifier,
        body_part: String,
        arguments: Vec<(String, u32)>,
    },
    /// Works like BP_LAYERS, but defines layers under existing layers.
    LayersUnder {
        specifier: BodyPartSpecifier,
        body_part: String,
        arguments: Vec<(String, u32)>,
    },
    /// Defines a position for the specified body part relative to its parent part (the nose is assigned the position FRONT, as it's on the front of the face).
    /// This has some effects on combat, attacks and the like. Valid position tokens are . The position token SIDES is
    /// of unverified validity.
    Position {
        specifier: BodyPartSpecifier,
        target: String,
        position: BodyPartPosition,
    },
    /// Defines a positional relationship between one body part and another (the right eyelid is `AROUND` the right eye with coverage 50, as it only partially
    /// covers the eye). This has some effects on combat, attacks and the like. Valid relation tokens are `AROUND`, `SURROUNDED_BY`, `ABOVE`, `BELOW`, `IN_FRONT`,
    /// `BEHIND`, `CLEANS`, and `CLEANED_BY`. The lattermost two tokens are used when specifying parts that clean each other (such as eyelids to eyes).
    Relation {
        specified_by: BodyPartSpecifier,
        target: String,
        relation: String,
        parent_specifier: BodyPartSpecifier,
        parent: String,
        coverage: String,
    },
    /// Defines a relsize for the selected body part for the current body detail plan
    RelativeSize {
        specifier: BodyPartSpecifier,
        target: String,
        relative_size: u32,
    },
}
