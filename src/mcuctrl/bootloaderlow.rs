#[doc = "Reader of register BOOTLOADERLOW"]
pub type R = crate::R<u32, super::BOOTLOADERLOW>;
#[doc = "Writer for register BOOTLOADERLOW"]
pub type W = crate::W<u32, super::BOOTLOADERLOW>;
#[doc = "Register BOOTLOADERLOW `reset()`'s with value 0x01"]
impl crate::ResetValue for super::BOOTLOADERLOW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Determines whether the bootloader code is visible at address 0x00000000 or not.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALUE_A {
    #[doc = "1: Bootloader code at 0x00000000."]
    ADDR0 = 1,
}
impl From<VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: VALUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<bool, VALUE_A>;
impl VALUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, VALUE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(VALUE_A::ADDR0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADDR0`"]
    #[inline(always)]
    pub fn is_addr0(&self) -> bool {
        *self == VALUE_A::ADDR0
    }
}
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALUE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bootloader code at 0x00000000."]
    #[inline(always)]
    pub fn addr0(self) -> &'a mut W {
        self.variant(VALUE_A::ADDR0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Determines whether the bootloader code is visible at address 0x00000000 or not."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether the bootloader code is visible at address 0x00000000 or not."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
