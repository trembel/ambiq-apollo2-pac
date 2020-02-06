#[doc = "Reader of register SHADOWVALID"]
pub type R = crate::R<u32, super::SHADOWVALID>;
#[doc = "Writer for register SHADOWVALID"]
pub type W = crate::W<u32, super::SHADOWVALID>;
#[doc = "Register SHADOWVALID `reset()`'s with value 0x03"]
impl crate::ResetValue for super::SHADOWVALID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Indicates whether the bootloader should sleep or deep sleep if no image loaded.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BL_DSLEEP_A {
    #[doc = "1: Bootloader will go to deep sleep if no flash image loaded"]
    DEEPSLEEP = 1,
}
impl From<BL_DSLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: BL_DSLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BL_DSLEEP`"]
pub type BL_DSLEEP_R = crate::R<bool, BL_DSLEEP_A>;
impl BL_DSLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BL_DSLEEP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BL_DSLEEP_A::DEEPSLEEP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == BL_DSLEEP_A::DEEPSLEEP
    }
}
#[doc = "Write proxy for field `BL_DSLEEP`"]
pub struct BL_DSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_DSLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BL_DSLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bootloader will go to deep sleep if no flash image loaded"]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(BL_DSLEEP_A::DEEPSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Indicates whether the shadow registers contain valid data from the Flash Information Space.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_A {
    #[doc = "1: Flash information space contains valid data."]
    VALID = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, VALID_A>;
impl VALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, VALID_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(VALID_A::VALID),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VALID_A::VALID
    }
}
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash information space contains valid data."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VALID_A::VALID)
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
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    pub fn bl_dsleep(&self) -> BL_DSLEEP_R {
        BL_DSLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Indicates whether the bootloader should sleep or deep sleep if no image loaded."]
    #[inline(always)]
    pub fn bl_dsleep(&mut self) -> BL_DSLEEP_W {
        BL_DSLEEP_W { w: self }
    }
    #[doc = "Bit 0 - Indicates whether the shadow registers contain valid data from the Flash Information Space."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
}
