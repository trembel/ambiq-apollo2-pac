#[doc = "Reader of register SRAMCTRL"]
pub type R = crate::R<u32, super::SRAMCTRL>;
#[doc = "Writer for register SRAMCTRL"]
pub type W = crate::W<u32, super::SRAMCTRL>;
#[doc = "Register SRAMCTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SRAMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Enables top-level clock gating in the SRAM block. This bit should be enabled for lowest power operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_MASTER_CLKGATE_A {
    #[doc = "1: Enable Master SRAM Clock Gate"]
    EN = 1,
    #[doc = "0: Disables Master SRAM Clock Gating"]
    DIS = 0,
}
impl From<SRAM_MASTER_CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_MASTER_CLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM_MASTER_CLKGATE`"]
pub type SRAM_MASTER_CLKGATE_R = crate::R<bool, SRAM_MASTER_CLKGATE_A>;
impl SRAM_MASTER_CLKGATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_MASTER_CLKGATE_A {
        match self.bits {
            true => SRAM_MASTER_CLKGATE_A::EN,
            false => SRAM_MASTER_CLKGATE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SRAM_MASTER_CLKGATE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SRAM_MASTER_CLKGATE_A::DIS
    }
}
#[doc = "Write proxy for field `SRAM_MASTER_CLKGATE`"]
pub struct SRAM_MASTER_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_MASTER_CLKGATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_MASTER_CLKGATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Master SRAM Clock Gate"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SRAM_MASTER_CLKGATE_A::EN)
    }
    #[doc = "Disables Master SRAM Clock Gating"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAM_MASTER_CLKGATE_A::DIS)
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
#[doc = "Enables individual per-RAM clock gating in the SRAM block. This bit should be enabled for lowest power operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CLKGATE_A {
    #[doc = "1: Enable Individual SRAM Clock Gating"]
    EN = 1,
    #[doc = "0: Disables Individual SRAM Clock Gating"]
    DIS = 0,
}
impl From<SRAM_CLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM_CLKGATE`"]
pub type SRAM_CLKGATE_R = crate::R<bool, SRAM_CLKGATE_A>;
impl SRAM_CLKGATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CLKGATE_A {
        match self.bits {
            true => SRAM_CLKGATE_A::EN,
            false => SRAM_CLKGATE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SRAM_CLKGATE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SRAM_CLKGATE_A::DIS
    }
}
#[doc = "Write proxy for field `SRAM_CLKGATE`"]
pub struct SRAM_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CLKGATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CLKGATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Individual SRAM Clock Gating"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SRAM_CLKGATE_A::EN)
    }
    #[doc = "Disables Individual SRAM Clock Gating"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAM_CLKGATE_A::DIS)
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
#[doc = "Enable LS (light sleep) of cache RAMs. When this bit is set, the RAMS will be put into light sleep mode while inactive. NOTE: if the SRAM is actively used, this may have an adverse affect on power since entering/exiting LS mode may consume more power than would be saved.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_LIGHT_SLEEP_A {
    #[doc = "1: Enable LIGHT SLEEP for SRAMs"]
    EN = 1,
    #[doc = "0: Disables LIGHT SLEEP for SRAMs"]
    DIS = 0,
}
impl From<SRAM_LIGHT_SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_LIGHT_SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM_LIGHT_SLEEP`"]
pub type SRAM_LIGHT_SLEEP_R = crate::R<bool, SRAM_LIGHT_SLEEP_A>;
impl SRAM_LIGHT_SLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_LIGHT_SLEEP_A {
        match self.bits {
            true => SRAM_LIGHT_SLEEP_A::EN,
            false => SRAM_LIGHT_SLEEP_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SRAM_LIGHT_SLEEP_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SRAM_LIGHT_SLEEP_A::DIS
    }
}
#[doc = "Write proxy for field `SRAM_LIGHT_SLEEP`"]
pub struct SRAM_LIGHT_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_LIGHT_SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_LIGHT_SLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable LIGHT SLEEP for SRAMs"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SRAM_LIGHT_SLEEP_A::EN)
    }
    #[doc = "Disables LIGHT SLEEP for SRAMs"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAM_LIGHT_SLEEP_A::DIS)
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
    #[doc = "Bit 2 - Enables top-level clock gating in the SRAM block. This bit should be enabled for lowest power operation."]
    #[inline(always)]
    pub fn sram_master_clkgate(&self) -> SRAM_MASTER_CLKGATE_R {
        SRAM_MASTER_CLKGATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables individual per-RAM clock gating in the SRAM block. This bit should be enabled for lowest power operation."]
    #[inline(always)]
    pub fn sram_clkgate(&self) -> SRAM_CLKGATE_R {
        SRAM_CLKGATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable LS (light sleep) of cache RAMs. When this bit is set, the RAMS will be put into light sleep mode while inactive. NOTE: if the SRAM is actively used, this may have an adverse affect on power since entering/exiting LS mode may consume more power than would be saved."]
    #[inline(always)]
    pub fn sram_light_sleep(&self) -> SRAM_LIGHT_SLEEP_R {
        SRAM_LIGHT_SLEEP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enables top-level clock gating in the SRAM block. This bit should be enabled for lowest power operation."]
    #[inline(always)]
    pub fn sram_master_clkgate(&mut self) -> SRAM_MASTER_CLKGATE_W {
        SRAM_MASTER_CLKGATE_W { w: self }
    }
    #[doc = "Bit 1 - Enables individual per-RAM clock gating in the SRAM block. This bit should be enabled for lowest power operation."]
    #[inline(always)]
    pub fn sram_clkgate(&mut self) -> SRAM_CLKGATE_W {
        SRAM_CLKGATE_W { w: self }
    }
    #[doc = "Bit 0 - Enable LS (light sleep) of cache RAMs. When this bit is set, the RAMS will be put into light sleep mode while inactive. NOTE: if the SRAM is actively used, this may have an adverse affect on power since entering/exiting LS mode may consume more power than would be saved."]
    #[inline(always)]
    pub fn sram_light_sleep(&mut self) -> SRAM_LIGHT_SLEEP_W {
        SRAM_LIGHT_SLEEP_W { w: self }
    }
}
