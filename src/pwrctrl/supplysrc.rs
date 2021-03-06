#[doc = "Reader of register SUPPLYSRC"]
pub type R = crate::R<u32, super::SUPPLYSRC>;
#[doc = "Writer for register SUPPLYSRC"]
pub type W = crate::W<u32, super::SUPPLYSRC>;
#[doc = "Register SUPPLYSRC `reset()`'s with value 0x04"]
impl crate::ResetValue for super::SUPPLYSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Switches the CORE DOMAIN from BUCK mode (if enabled) to LDO when CPU is in DEEP SLEEP. If all the devices are off then this does not matter and LDO (low power mode) is used\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWITCH_LDO_IN_SLEEP_A {
    #[doc = "1: Automatically switch from CORE BUCK to CORE LDO when CPU is in DEEP SLEEP"]
    EN = 1,
}
impl From<SWITCH_LDO_IN_SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SWITCH_LDO_IN_SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWITCH_LDO_IN_SLEEP`"]
pub type SWITCH_LDO_IN_SLEEP_R = crate::R<bool, SWITCH_LDO_IN_SLEEP_A>;
impl SWITCH_LDO_IN_SLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SWITCH_LDO_IN_SLEEP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SWITCH_LDO_IN_SLEEP_A::EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SWITCH_LDO_IN_SLEEP_A::EN
    }
}
#[doc = "Write proxy for field `SWITCH_LDO_IN_SLEEP`"]
pub struct SWITCH_LDO_IN_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_LDO_IN_SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWITCH_LDO_IN_SLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatically switch from CORE BUCK to CORE LDO when CPU is in DEEP SLEEP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SWITCH_LDO_IN_SLEEP_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enables and Selects the Core Buck as the supply for the low-voltage power domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COREBUCKEN_A {
    #[doc = "1: Enable the Core Buck for the low-voltage power domain."]
    EN = 1,
}
impl From<COREBUCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: COREBUCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COREBUCKEN`"]
pub type COREBUCKEN_R = crate::R<bool, COREBUCKEN_A>;
impl COREBUCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COREBUCKEN_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COREBUCKEN_A::EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == COREBUCKEN_A::EN
    }
}
#[doc = "Write proxy for field `COREBUCKEN`"]
pub struct COREBUCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COREBUCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Core Buck for the low-voltage power domain."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(COREBUCKEN_A::EN)
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
#[doc = "Enables and select the Memory Buck as the supply for the Flash and SRAM power domain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMBUCKEN_A {
    #[doc = "1: Enable the Memory Buck as the supply for flash and SRAM."]
    EN = 1,
}
impl From<MEMBUCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: MEMBUCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMBUCKEN`"]
pub type MEMBUCKEN_R = crate::R<bool, MEMBUCKEN_A>;
impl MEMBUCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MEMBUCKEN_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(MEMBUCKEN_A::EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MEMBUCKEN_A::EN
    }
}
#[doc = "Write proxy for field `MEMBUCKEN`"]
pub struct MEMBUCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMBUCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Memory Buck as the supply for flash and SRAM."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMBUCKEN_A::EN)
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
    #[doc = "Bit 2 - Switches the CORE DOMAIN from BUCK mode (if enabled) to LDO when CPU is in DEEP SLEEP. If all the devices are off then this does not matter and LDO (low power mode) is used"]
    #[inline(always)]
    pub fn switch_ldo_in_sleep(&self) -> SWITCH_LDO_IN_SLEEP_R {
        SWITCH_LDO_IN_SLEEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables and Selects the Core Buck as the supply for the low-voltage power domain."]
    #[inline(always)]
    pub fn corebucken(&self) -> COREBUCKEN_R {
        COREBUCKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables and select the Memory Buck as the supply for the Flash and SRAM power domain."]
    #[inline(always)]
    pub fn membucken(&self) -> MEMBUCKEN_R {
        MEMBUCKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Switches the CORE DOMAIN from BUCK mode (if enabled) to LDO when CPU is in DEEP SLEEP. If all the devices are off then this does not matter and LDO (low power mode) is used"]
    #[inline(always)]
    pub fn switch_ldo_in_sleep(&mut self) -> SWITCH_LDO_IN_SLEEP_W {
        SWITCH_LDO_IN_SLEEP_W { w: self }
    }
    #[doc = "Bit 1 - Enables and Selects the Core Buck as the supply for the low-voltage power domain."]
    #[inline(always)]
    pub fn corebucken(&mut self) -> COREBUCKEN_W {
        COREBUCKEN_W { w: self }
    }
    #[doc = "Bit 0 - Enables and select the Memory Buck as the supply for the Flash and SRAM power domain."]
    #[inline(always)]
    pub fn membucken(&mut self) -> MEMBUCKEN_W {
        MEMBUCKEN_W { w: self }
    }
}
