use turso::{Connection, params};

/// Reprsents a value on a caste tag
#[derive(Debug, Clone)]
pub enum TagValue {
    /// A boolean value
    Bool(bool),
    /// An integer value
    Int(i64),
    /// A string value
    String(String),
}

/// insert a caste value tag
///
/// # Errors
///
/// Passes along database errors
pub async fn insert_value_tag(
    conn: &Connection,
    caste_id: i64,
    flag_id: i64,
    position: i64,
    values: &[TagValue], // Accepts mixed slice: [Int(1), String("A"), Int(2)]
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bit_val: Option<bool> = None;
    let mut str_vals: Vec<Option<String>> = vec![None; 7];
    let mut int_vals: Vec<Option<i64>> = vec![None; 7];

    let mut str_idx = 0;
    let mut int_idx = 0;

    for v in values {
        match v {
            // right now only supports one boolean per tag
            TagValue::Bool(b) => bit_val = Some(*b),
            TagValue::String(s) => {
                if str_idx < 7 {
                    str_vals[str_idx] = Some(s.clone());
                    str_idx += 1;
                }
            }
            TagValue::Int(i) => {
                if int_idx < 7 {
                    int_vals[int_idx] = Some(*i);
                    int_idx += 1;
                }
            }
        }
    }

    // 4. Execute the Single Wide Insert
    // We unroll our vectors into the params! macro.
    conn.execute(
        super::INSERT_VALUE_TAG,
        params![
            caste_id,
            flag_id,
            position,
            // The boolean
            bit_val,
            // The 7 strings (Some("val") or None)
            str_vals[0].as_deref(),
            str_vals[1].as_deref(),
            str_vals[2].as_deref(),
            str_vals[3].as_deref(),
            str_vals[4].as_deref(),
            str_vals[5].as_deref(),
            str_vals[6].as_deref(),
            // The 7 integers (Some(1) or None)
            int_vals[0],
            int_vals[1],
            int_vals[2],
            int_vals[3],
            int_vals[4],
            int_vals[5],
            int_vals[6],
        ],
    )
    .await?;

    Ok(())
}
