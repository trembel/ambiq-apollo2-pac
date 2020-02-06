#[doc = "Reader of register BUCK"]
pub type R = crate::R<u32, super::BUCK>;
#[doc = "Writer for register BUCK"]
pub type W = crate::W<u32, super::BUCK>;
#[doc = "Register BUCK `reset()`'s with value 0"]
impl crate::ResetValue for super::BUCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEMBUCKRST`"]
pub type MEMBUCKRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEMBUCKRST`"]
pub struct MEMBUCKRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `COREBUCKRST`"]
pub type COREBUCKRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COREBUCKRST`"]
pub struct COREBUCKRST_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `BYPBUCKMEM`"]
pub type BYPBUCKMEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPBUCKMEM`"]
pub struct BYPBUCKMEM_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPBUCKMEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Memory buck power down override. 1=Powered Down; 0=Enabled; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMBUCKPWD_A {
    #[doc = "0: Memory Buck Enable."]
    EN = 0,
}
impl From<MEMBUCKPWD_A> for bool {
    #[inline(always)]
    fn from(variant: MEMBUCKPWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMBUCKPWD`"]
pub type MEMBUCKPWD_R = crate::R<bool, MEMBUCKPWD_A>;
impl MEMBUCKPWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, MEMBUCKPWD_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(MEMBUCKPWD_A::EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MEMBUCKPWD_A::EN
    }
}
#[doc = "Write proxy for field `MEMBUCKPWD`"]
pub struct MEMBUCKPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKPWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMBUCKPWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Memory Buck Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMBUCKPWD_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SLEEPBUCKANA`"]
pub type SLEEPBUCKANA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEEPBUCKANA`"]
pub struct SLEEPBUCKANA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPBUCKANA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Core buck power down override. 1=Powered Down; 0=Enabled; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COREBUCKPWD_A {
    #[doc = "0: Core Buck enable."]
    EN = 0,
}
impl From<COREBUCKPWD_A> for bool {
    #[inline(always)]
    fn from(variant: COREBUCKPWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COREBUCKPWD`"]
pub type COREBUCKPWD_R = crate::R<bool, COREBUCKPWD_A>;
impl COREBUCKPWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COREBUCKPWD_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(COREBUCKPWD_A::EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == COREBUCKPWD_A::EN
    }
}
#[doc = "Write proxy for field `COREBUCKPWD`"]
pub struct COREBUCKPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKPWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COREBUCKPWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Core Buck enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(COREBUCKPWD_A::EN)
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
#[doc = "Reader of field `BYPBUCKCORE`"]
pub type BYPBUCKCORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPBUCKCORE`"]
pub struct BYPBUCKCORE_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPBUCKCORE_W<'a> {
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
#[doc = "Buck Register Software Override Enable. This will enable the override values for MEMBUCKPWD, COREBUCKPWD, COREBUCKRST, MEMBUCKRST, all to be propagated to the control logic, instead of the normal power control module signal. Note - Must take care to have correct value for ALL the register bits when this SWE is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUCKSWE_A {
    #[doc = "0: BUCK Software Override Disable."]
    OVERRIDE_DIS = 0,
    #[doc = "1: BUCK Software Override Enable."]
    OVERRIDE_EN = 1,
}
impl From<BUCKSWE_A> for bool {
    #[inline(always)]
    fn from(variant: BUCKSWE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUCKSWE`"]
pub type BUCKSWE_R = crate::R<bool, BUCKSWE_A>;
impl BUCKSWE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUCKSWE_A {
        match self.bits {
            false => BUCKSWE_A::OVERRIDE_DIS,
            true => BUCKSWE_A::OVERRIDE_EN,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE_DIS`"]
    #[inline(always)]
    pub fn is_override_dis(&self) -> bool {
        *self == BUCKSWE_A::OVERRIDE_DIS
    }
    #[doc = "Checks if the value of the field is `OVERRIDE_EN`"]
    #[inline(always)]
    pub fn is_override_en(&self) -> bool {
        *self == BUCKSWE_A::OVERRIDE_EN
    }
}
#[doc = "Write proxy for field `BUCKSWE`"]
pub struct BUCKSWE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCKSWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUCKSWE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BUCK Software Override Disable."]
    #[inline(always)]
    pub fn override_dis(self) -> &'a mut W {
        self.variant(BUCKSWE_A::OVERRIDE_DIS)
    }
    #[doc = "BUCK Software Override Enable."]
    #[inline(always)]
    pub fn override_en(self) -> &'a mut W {
        self.variant(BUCKSWE_A::OVERRIDE_EN)
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
    #[doc = "Bit 7 - Reset control override for Mem Buck; 0=enabled, 1=reset; Value is propagated only when the BUCKSWE bit is active, otherwise contrl is from the power control module."]
    #[inline(always)]
    pub fn membuckrst(&self) -> MEMBUCKRST_R {
        MEMBUCKRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset control override for Core Buck; 0=enabled, 1=reset; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline(always)]
    pub fn corebuckrst(&self) -> COREBUCKRST_R {
        COREBUCKRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Not used. Additional control of buck is available in the power control module"]
    #[inline(always)]
    pub fn bypbuckmem(&self) -> BYPBUCKMEM_R {
        BYPBUCKMEM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Memory buck power down override. 1=Powered Down; 0=Enabled; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline(always)]
    pub fn membuckpwd(&self) -> MEMBUCKPWD_R {
        MEMBUCKPWD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HFRC clkgen bit 0 override. When set, this will override to 0 bit 0 of the hfrc_freq_clkgen internal bus (see internal Shelby-1473)"]
    #[inline(always)]
    pub fn sleepbuckana(&self) -> SLEEPBUCKANA_R {
        SLEEPBUCKANA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Core buck power down override. 1=Powered Down; 0=Enabled; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline(always)]
    pub fn corebuckpwd(&self) -> COREBUCKPWD_R {
        COREBUCKPWD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Not used. Additional control of buck is available in the power control module"]
    #[inline(always)]
    pub fn bypbuckcore(&self) -> BYPBUCKCORE_R {
        BYPBUCKCORE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Buck Register Software Override Enable. This will enable the override values for MEMBUCKPWD, COREBUCKPWD, COREBUCKRST, MEMBUCKRST, all to be propagated to the control logic, instead of the normal power control module signal. Note - Must take care to have correct value for ALL the register bits when this SWE is enabled."]
    #[inline(always)]
    pub fn buckswe(&self) -> BUCKSWE_R {
        BUCKSWE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Reset control override for Mem Buck; 0=enabled, 1=reset; Value is propagated only when the BUCKSWE bit is active, otherwise contrl is from the power control module."]
    #[inline(always)]
    pub fn membuckrst(&mut self) -> MEMBUCKRST_W {
        MEMBUCKRST_W { w: self }
    }
    #[doc = "Bit 6 - Reset control override for Core Buck; 0=enabled, 1=reset; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline(always)]
    pub fn corebuckrst(&mut self) -> COREBUCKRST_W {
        COREBUCKRST_W { w: self }
    }
    #[doc = "Bit 5 - Not used. Additional control of buck is available in the power control module"]
    #[inline(always)]
    pub fn bypbuckmem(&mut self) -> BYPBUCKMEM_W {
        BYPBUCKMEM_W { w: self }
    }
    #[doc = "Bit 4 - Memory buck power down override. 1=Powered Down; 0=Enabled; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline(always)]
    pub fn membuckpwd(&mut self) -> MEMBUCKPWD_W {
        MEMBUCKPWD_W { w: self }
    }
    #[doc = "Bit 3 - HFRC clkgen bit 0 override. When set, this will override to 0 bit 0 of the hfrc_freq_clkgen internal bus (see internal Shelby-1473)"]
    #[inline(always)]
    pub fn sleepbuckana(&mut self) -> SLEEPBUCKANA_W {
        SLEEPBUCKANA_W { w: self }
    }
    #[doc = "Bit 2 - Core buck power down override. 1=Powered Down; 0=Enabled; Value is propagated only when the BUCKSWE bit is active, otherwise control is from the power control module."]
    #[inline(always)]
    pub fn corebuckpwd(&mut self) -> COREBUCKPWD_W {
        COREBUCKPWD_W { w: self }
    }
    #[doc = "Bit 1 - Not used. Additional control of buck is available in the power control module"]
    #[inline(always)]
    pub fn bypbuckcore(&mut self) -> BYPBUCKCORE_W {
        BYPBUCKCORE_W { w: self }
    }
    #[doc = "Bit 0 - Buck Register Software Override Enable. This will enable the override values for MEMBUCKPWD, COREBUCKPWD, COREBUCKRST, MEMBUCKRST, all to be propagated to the control logic, instead of the normal power control module signal. Note - Must take care to have correct value for ALL the register bits when this SWE is enabled."]
    #[inline(always)]
    pub fn buckswe(&mut self) -> BUCKSWE_W {
        BUCKSWE_W { w: self }
    }
}
