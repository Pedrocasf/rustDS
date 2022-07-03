#![allow(unused)]

/// Sets up a constant new constructor for a zeroed value.
macro_rules! const_new {
  () => {
    pub const fn new() -> Self {
      Self(0)
    }
  };
}
pub(crate) use const_new;

/// Sets up a bitfield integer
macro_rules! bitfield_int {
  ($inner:ty; $low:literal ..= $high:literal : $nt:ident, $get:ident, $with:ident, $set:ident) => {
    #[inline]
    pub const fn $get(self) -> $nt {
      const MASK: $inner = ((1 << ($high - $low + 1)) - 1) << $low;
      ((self.0 & MASK) >> $low) as $nt
    }
    #[inline]
    pub const fn $with(self, $get: $nt) -> Self {
      const MASK: $inner = ((1 << ($high - $low + 1)) - 1) << $low;
      Self(self.0 ^ ((self.0 ^ (($get as $inner) << $low)) & MASK))
    }
    #[inline]
    pub fn $set(&mut self, $get: $nt) {
      *self = self.$with($get);
    }
  };
}
pub(crate) use bitfield_int;

/// Sets up a bitfield int wrapped newtype
macro_rules! bitfield_newtype {
  ($inner:ty; $low:literal ..= $high:literal : $nt:ident, $get:ident, $with:ident, $set:ident) => {
    #[inline]
    pub const fn $get(self) -> $nt {
      const MASK: $inner = ((1 << ($high - $low + 1)) - 1) << $low;
      $nt(self.0 & MASK)
    }
    #[inline]
    pub const fn $with(self, $get: $nt) -> Self {
      const MASK: $inner = ((1 << ($high - $low + 1)) - 1) << $low;
      Self(self.0 ^ ((self.0 ^ $get.0) & MASK))
    }
    #[inline]
    pub fn $set(&mut self, $get: $nt) {
      *self = self.$with($get);
    }
  };
}
pub(crate) use bitfield_newtype;

/// Sets up a bitfield enum (CAUTION: misuse of this can cause UB!)
macro_rules! bitfield_enum {
  ($inner:ty; $low:literal ..= $high:literal : $nt:ident, $get:ident, $with:ident, $set:ident) => {
    // TODO: make this const when we have const transmute
    #[inline]
    pub fn $get(self) -> $nt {
      const MASK: $inner = ((1 << $high) - 1) << $low;
      unsafe { core::mem::transmute(self.0 & MASK) }
    }
    #[inline]
    pub const fn $with(self, $get: $nt) -> Self {
      const MASK: $inner = ((1 << $high) - 1) << $low;
      Self(self.0 ^ ((self.0 ^ $get as $inner) & MASK))
    }
    #[inline]
    pub fn $set(&mut self, $get: $nt) {
      *self = self.$with($get);
    }
  };
}
pub(crate) use bitfield_enum;

/// Sets up a bitfield bool
macro_rules! bitfield_bool {
  ($inner:ty; $bit:literal, $get:ident, $with:ident, $set:ident) => {
    #[inline]
    pub const fn $get(self) -> bool {
      (self.0 & (1 << $bit)) != 0
    }
    #[inline]
    pub const fn $with(self, $get: bool) -> Self {
      Self(self.0 ^ ((($get as $inner).wrapping_neg() ^ self.0) & (1 << $bit)))
    }
    #[inline]
    pub fn $set(&mut self, $get: bool) {
      *self = self.$with($get);
    }
  };
}
pub(crate) use bitfield_bool;

/// Adds bitwise ops for this type
macro_rules! impl_bitwise_ops {
  ($outer:ty) => {
    impl core::ops::Not for $outer {
      type Output = Self;
      #[inline]
      fn not(self) -> Self {
        Self(!self.0)
      }
    }
    impl core::ops::BitAnd for $outer {
      type Output = $outer;
      #[inline]
      fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
      }
    }
    impl core::ops::BitOr for $outer {
      type Output = $outer;
      #[inline]
      fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
      }
    }
    impl core::ops::BitXor for $outer {
      type Output = $outer;
      #[inline]
      fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
      }
    }
    // // // // //
    impl core::ops::BitAndAssign for $outer {
      #[inline]
      fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
      }
    }
    impl core::ops::BitOrAssign for $outer {
      #[inline]
      fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
      }
    }
    impl core::ops::BitXorAssign for $outer {
      #[inline]
      fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
      }
    }
  };
}
pub(crate) use impl_bitwise_ops;