#![allow(unused_macros)]
#![allow(unused_imports)]
macro_rules! def_mmio {
  ($addr:literal = $name:ident : $t:ty $(; $comment:expr )?) => {
    // redirect a call **without** an alias list to just pass an empty alias list
    def_mmio!($addr = $name/[]: $t $(; $comment)? );
  };
  ($addr:literal = $name:ident / [ $( $alias:literal ),* ]: $t:ty $(; $comment:expr )?) => {
    $(#[doc = $comment])?
    $(#[doc(alias = $alias)])*
    #[allow(missing_docs)]
    pub const $name: $t = unsafe { <$t>::new($addr) };
  };
}
pub(crate) use def_mmio;

macro_rules! pub_const_fn_new_zeroed {
  () => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn new() -> Self {
      Self(0)
    }
  };
}
pub(crate) use pub_const_fn_new_zeroed;

macro_rules! u32_bool_field {
  ($bit:expr, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> bool {
      bitfrob::u32_get_bit($bit, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, b: bool) -> Self {
      Self(bitfrob::u32_with_bit($bit, self.0, b))
    }
  };
  (inverted $bit:expr, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> bool {
      !bitfrob::u32_get_bit($bit, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, b: bool) -> Self {
      Self(bitfrob::u32_with_bit($bit, self.0, !b))
    }
  };
}
pub(crate) use u32_bool_field;

macro_rules! u32_enum_field {
  ($low:literal - $high:literal : $t:ty, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> $t {
      unsafe {
        core::mem::transmute::<u32, $t>(bitfrob::u32_get_region(
          $low, $high, self.0,
        ))
      }
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: $t) -> Self {
      Self(bitfrob::u32_with_region($low, $high, self.0, val as u32))
    }
  };
}
pub(crate) use u32_enum_field;

macro_rules! u32_int_field {
  ($low:literal - $high:literal, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> u32 {
      bitfrob::u32_get_value($low, $high, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: u32) -> Self {
      Self(bitfrob::u32_with_value($low, $high, self.0, val))
    }
  };
}
pub(crate) use u32_int_field;

macro_rules! u16_bool_field {
  ($bit:expr, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> bool {
      bitfrob::u16_get_bit($bit, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, b: bool) -> Self {
      Self(bitfrob::u16_with_bit($bit, self.0, b))
    }
  };
  (inverted $bit:expr, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> bool {
      !bitfrob::u16_get_bit($bit, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, b: bool) -> Self {
      Self(bitfrob::u16_with_bit($bit, self.0, !b))
    }
  };
}
pub(crate) use u16_bool_field;

macro_rules! u16_enum_field {
  ($low:literal - $high:literal : $t:ty, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> $t {
      unsafe {
        core::mem::transmute::<u16, $t>(bitfrob::u16_get_region(
          $low, $high, self.0,
        ))
      }
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: $t) -> Self {
      Self(bitfrob::u16_with_region($low, $high, self.0, val as u16))
    }
  };
}
pub(crate) use u16_enum_field;

macro_rules! u16_int_field {
  ($low:literal - $high:literal, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> u16 {
      bitfrob::u16_get_value($low, $high, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: u16) -> Self {
      Self(bitfrob::u16_with_value($low, $high, self.0, val))
    }
  };
}
pub(crate) use u16_int_field;

macro_rules! u8_bool_field {
  ($bit:expr, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> bool {
      bitfrob::u8_get_bit($bit, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, b: bool) -> Self {
      Self(bitfrob::u8_with_bit($bit, self.0, b))
    }
  };
}
pub(crate) use u8_bool_field;

macro_rules! u8_enum_field {
  ($low:literal - $high:literal : $t:ty, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> $t {
      unsafe {
        core::mem::transmute::<u8, $t>(bitfrob::u8_get_region(
          $low, $high, self.0,
        ))
      }
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: $t) -> Self {
      Self(bitfrob::u8_with_region($low, $high, self.0, val as u8))
    }
  };
}
pub(crate) use u8_enum_field;

macro_rules! u8_int_field {
  ($low:literal - $high:literal, $get:ident, $with:ident) => {
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $get(self) -> u8 {
      bitfrob::u8_get_value($low, $high, self.0)
    }
    #[inline]
    #[must_use]
    #[allow(missing_docs)]
    pub const fn $with(self, val: u8) -> Self {
      Self(bitfrob::u8_with_value($low, $high, self.0, val))
    }
  };
}
pub(crate) use u8_int_field;