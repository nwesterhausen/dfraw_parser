use uuid::Uuid;

/// Helper to check if a type is effectively "default", "empty", or "zeroed".
///
/// This is used by the `Cleanable` macro to determine if an `Option<T>` can be
/// safely set to `None`.
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for bool {
    #[inline]
    fn is_empty(&self) -> bool {
        *self
    }
}

impl IsEmpty for u8 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for u16 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for u32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for u64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for u128 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for f32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0.0
    }
}

impl IsEmpty for f64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0.0
    }
}

impl IsEmpty for i8 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for i16 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for i32 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for i64 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for i128 {
    #[inline]
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl IsEmpty for [u32; 2] {
    #[inline]
    fn is_empty(&self) -> bool {
        self[0] == 0 && self[1] == 0
    }
}

impl IsEmpty for [i32; 2] {
    #[inline]
    fn is_empty(&self) -> bool {
        self[0] == 0 && self[1] == 0
    }
}

impl IsEmpty for String {
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsEmpty for Vec<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: IsEmpty> IsEmpty for Option<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_ref().is_none_or(IsEmpty::is_empty)
    }
}

impl IsEmpty for Uuid {
    #[inline]
    fn is_empty(&self) -> bool {
        Uuid::is_nil(self)
    }
}
