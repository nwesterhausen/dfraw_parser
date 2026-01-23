use serde::{Deserialize, Serialize};

/// A constraint on a query made with a [`NumericFilter`]
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "mode", content = "value")]
pub enum NumericConstraint {
    /// Matches values greater than or equal to X (val >= x)
    /// Example: "At least 50 pet value"
    Min(i64),

    /// Matches values less than or equal to X (val <= x)
    /// Example: "Difficulty less than 10"
    Max(i64),

    /// Matches values exactly equal to X (val == x)
    Exact(i64),

    /// Matches values between X and Y inclusive (x <= val <= y)
    Range(i64, i64),
}

/// A filter to apply to numeric values associated with a raw.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NumericFilter {
    /// The database key (e.g. `PET_VALUE`, `CLUTCH_SIZE_MAX`)
    pub key: String,
    /// The constraint to apply
    pub constraint: NumericConstraint,
}

impl NumericFilter {
    /// Add the numeric filter to the conditions and params vectors.
    ///
    /// * `alias` the name of the table joining this filter
    /// * `conditions` the conditions vector
    /// * `params_vec` the parameters vector
    ///
    /// For each [`NumericFilter`] that is searched, a join has to be added to the
    /// search query. The alias used in that join is provided here as `alias`. Then
    /// this adds the appropriate "WHERE" clauses to the conditions and parameters.
    pub fn add_sql_to_params(
        &self,
        alias: &str,
        conditions: &mut Vec<String>,
        params_vec: &mut Vec<Box<dyn rusqlite::ToSql>>,
    ) {
        // 1. Match the Key
        conditions.push(format!("{alias}.token_name = ?{}", params_vec.len() + 1));
        params_vec.push(Box::new(self.key.clone()));

        match self.constraint {
            NumericConstraint::Min(val) => {
                conditions.push(format!("{alias}.value >= ?{}", params_vec.len() + 1));
                params_vec.push(Box::new(val));
            }
            NumericConstraint::Max(val) => {
                conditions.push(format!("{alias}.value <= ?{}", params_vec.len() + 1));
                params_vec.push(Box::new(val));
            }
            NumericConstraint::Exact(val) => {
                conditions.push(format!("{alias}.value = ?{}", params_vec.len() + 1));
                params_vec.push(Box::new(val));
            }
            NumericConstraint::Range(min, max) => {
                conditions.push(format!("{alias}.value >= ?{}", params_vec.len() + 1));
                params_vec.push(Box::new(min));
                conditions.push(format!("{alias}.value <= ?{}", params_vec.len() + 1));
                params_vec.push(Box::new(max));
            }
        }
    }
}
