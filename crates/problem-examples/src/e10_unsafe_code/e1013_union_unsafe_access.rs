/// E1013: Union with unsafe field access
/// Severity: HIGH
/// LLM confusion: 4 (HIGH)
///
/// Description: Unions allow storing different types in the same memory location, but accessing
/// the wrong field causes undefined behavior. Unlike enums, unions don't track which variant is
/// active - you must remember it yourself. Reading from the wrong union field is like interpreting
/// bytes as the wrong type. This is extremely unsafe and confusing because there's no compiler
/// help. Fix by using enums instead, or being extremely careful with manual tracking.
///
/// Mitigation: Prefer enums over unions - they're safer and the compiler tracks the active variant.
/// If you must use unions, extensively document which field is valid when. Use wrapper types that
/// track the active field. Never access union fields without unsafe blocks.

// PROBLEM E1013: Union with different types - accessing wrong field is UB
#[repr(C)]
pub union Value {
    int: i32,
    float: f32,
    bytes: [u8; 4],
}

pub enum ValueTag {
    Int,
    Float,
    Bytes,
}

// PROBLEM E1013: Manual tracking of union variant - error-prone
pub struct TaggedValue {
    tag: ValueTag,
    value: Value,
}

impl TaggedValue {
    pub fn e1013_union_unsafe_access_int(val: i32) -> Self {
        TaggedValue {
            tag: ValueTag::Int,
            value: Value { int: val },
        }
    }

    pub fn e1013_union_unsafe_access_float(val: f32) -> Self {
        TaggedValue {
            tag: ValueTag::Float,
            value: Value { float: val },
        }
    }

    // PROBLEM E1013: Easy to call get_int when value is actually float
    pub unsafe fn get_int(&self) -> i32 {
        self.value.int
    }

    pub unsafe fn get_float(&self) -> f32 {
        self.value.float
    }
}

// PROBLEM E1013: Transmuting between union fields
pub fn e1013_union_transmute() {
    let val = Value {
        int: 0x3f80_0000_u32 as i32,
    };

    // PROBLEM E1003: Direct use of unsafe code
    unsafe {
        // PROBLEM E1004: No safety documentation
        // PROBLEM E1013: Reading float interpretation of int bits
        let as_float = val.float;
        let _ = as_float; // Should be 1.0
    }
}

pub fn e1013_entry() -> Result<(), Box<dyn std::error::Error>> {
    e1013_union_transmute();
    Ok(())
}
