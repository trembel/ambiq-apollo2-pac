#[doc = "Reader of register CAPTURE_CONTROL"]
pub type R = crate::R<u32, super::CAPTURE_CONTROL>;
#[doc = "Writer for register CAPTURE_CONTROL"]
pub type W = crate::W<u32, super::CAPTURE_CONTROL>;
#[doc = "Register CAPTURE_CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CAPTURE_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects whether capture is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE_D_A {
    #[doc = "0: Capture function disabled."]
    DISABLE = 0,
    #[doc = "1: Capture function enabled."]
    ENABLE = 1,
}
impl From<CAPTURE_D_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_D_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTURE_D`"]
pub type CAPTURE_D_R = crate::R<bool, CAPTURE_D_A>;
impl CAPTURE_D_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_D_A {
        match self.bits {
            false => CAPTURE_D_A::DISABLE,
            true => CAPTURE_D_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURE_D_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAPTURE_D_A::ENABLE
    }
}
#[doc = "Write proxy for field `CAPTURE_D`"]
pub struct CAPTURE_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_D_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_D_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE_D_A::DISABLE)
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE_D_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Selects whether capture is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE_C_A {
    #[doc = "0: Capture function disabled."]
    DISABLE = 0,
    #[doc = "1: Capture function enabled."]
    ENABLE = 1,
}
impl From<CAPTURE_C_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_C_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTURE_C`"]
pub type CAPTURE_C_R = crate::R<bool, CAPTURE_C_A>;
impl CAPTURE_C_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_C_A {
        match self.bits {
            false => CAPTURE_C_A::DISABLE,
            true => CAPTURE_C_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURE_C_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAPTURE_C_A::ENABLE
    }
}
#[doc = "Write proxy for field `CAPTURE_C`"]
pub struct CAPTURE_C_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_C_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE_C_A::DISABLE)
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE_C_A::ENABLE)
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
#[doc = "Selects whether capture is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE_B_A {
    #[doc = "0: Capture function disabled."]
    DISABLE = 0,
    #[doc = "1: Capture function enabled."]
    ENABLE = 1,
}
impl From<CAPTURE_B_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTURE_B`"]
pub type CAPTURE_B_R = crate::R<bool, CAPTURE_B_A>;
impl CAPTURE_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_B_A {
        match self.bits {
            false => CAPTURE_B_A::DISABLE,
            true => CAPTURE_B_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURE_B_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAPTURE_B_A::ENABLE
    }
}
#[doc = "Write proxy for field `CAPTURE_B`"]
pub struct CAPTURE_B_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE_B_A::DISABLE)
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE_B_A::ENABLE)
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
#[doc = "Selects whether capture is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE_A_A {
    #[doc = "0: Capture function disabled."]
    DISABLE = 0,
    #[doc = "1: Capture function enabled."]
    ENABLE = 1,
}
impl From<CAPTURE_A_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_A_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTURE_A`"]
pub type CAPTURE_A_R = crate::R<bool, CAPTURE_A_A>;
impl CAPTURE_A_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_A_A {
        match self.bits {
            false => CAPTURE_A_A::DISABLE,
            true => CAPTURE_A_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAPTURE_A_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAPTURE_A_A::ENABLE
    }
}
#[doc = "Write proxy for field `CAPTURE_A`"]
pub struct CAPTURE_A_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_A_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_A_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture function disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE_A_A::DISABLE)
    }
    #[doc = "Capture function enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE_A_A::ENABLE)
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
    #[doc = "Bit 3 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture_d(&self) -> CAPTURE_D_R {
        CAPTURE_D_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture_c(&self) -> CAPTURE_C_R {
        CAPTURE_C_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture_b(&self) -> CAPTURE_B_R {
        CAPTURE_B_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture_a(&self) -> CAPTURE_A_R {
        CAPTURE_A_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture_d(&mut self) -> CAPTURE_D_W {
        CAPTURE_D_W { w: self }
    }
    #[doc = "Bit 2 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture_c(&mut self) -> CAPTURE_C_W {
        CAPTURE_C_W { w: self }
    }
    #[doc = "Bit 1 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture_b(&mut self) -> CAPTURE_B_W {
        CAPTURE_B_W { w: self }
    }
    #[doc = "Bit 0 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture_a(&mut self) -> CAPTURE_A_W {
        CAPTURE_A_W { w: self }
    }
}
