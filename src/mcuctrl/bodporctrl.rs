#[doc = "Reader of register BODPORCTRL"]
pub type R = crate::R<u32, super::BODPORCTRL>;
#[doc = "Writer for register BODPORCTRL"]
pub type W = crate::W<u32, super::BODPORCTRL>;
#[doc = "Register BODPORCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BODPORCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BOD External Reference Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODEXTREFSEL_A {
    #[doc = "1: BOD external reference select."]
    SELECT = 1,
}
impl From<BODEXTREFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: BODEXTREFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BODEXTREFSEL`"]
pub type BODEXTREFSEL_R = crate::R<bool, BODEXTREFSEL_A>;
impl BODEXTREFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BODEXTREFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BODEXTREFSEL_A::SELECT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT`"]
    #[inline(always)]
    pub fn is_select(&self) -> bool {
        *self == BODEXTREFSEL_A::SELECT
    }
}
#[doc = "Write proxy for field `BODEXTREFSEL`"]
pub struct BODEXTREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODEXTREFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODEXTREFSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BOD external reference select."]
    #[inline(always)]
    pub fn select(self) -> &'a mut W {
        self.variant(BODEXTREFSEL_A::SELECT)
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
#[doc = "PDR External Reference Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDREXTREFSEL_A {
    #[doc = "1: PDR external reference select."]
    SELECT = 1,
}
impl From<PDREXTREFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PDREXTREFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDREXTREFSEL`"]
pub type PDREXTREFSEL_R = crate::R<bool, PDREXTREFSEL_A>;
impl PDREXTREFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PDREXTREFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PDREXTREFSEL_A::SELECT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT`"]
    #[inline(always)]
    pub fn is_select(&self) -> bool {
        *self == PDREXTREFSEL_A::SELECT
    }
}
#[doc = "Write proxy for field `PDREXTREFSEL`"]
pub struct PDREXTREFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDREXTREFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDREXTREFSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDR external reference select."]
    #[inline(always)]
    pub fn select(self) -> &'a mut W {
        self.variant(PDREXTREFSEL_A::SELECT)
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
#[doc = "BOD Power Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDBOD_A {
    #[doc = "1: BOD power down."]
    PWR_DN = 1,
}
impl From<PWDBOD_A> for bool {
    #[inline(always)]
    fn from(variant: PWDBOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWDBOD`"]
pub type PWDBOD_R = crate::R<bool, PWDBOD_A>;
impl PWDBOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PWDBOD_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PWDBOD_A::PWR_DN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWR_DN`"]
    #[inline(always)]
    pub fn is_pwr_dn(&self) -> bool {
        *self == PWDBOD_A::PWR_DN
    }
}
#[doc = "Write proxy for field `PWDBOD`"]
pub struct PWDBOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDBOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDBOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BOD power down."]
    #[inline(always)]
    pub fn pwr_dn(self) -> &'a mut W {
        self.variant(PWDBOD_A::PWR_DN)
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
#[doc = "PDR Power Down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDPDR_A {
    #[doc = "1: PDR power down"]
    PWR_DN = 1,
}
impl From<PWDPDR_A> for bool {
    #[inline(always)]
    fn from(variant: PWDPDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWDPDR`"]
pub type PWDPDR_R = crate::R<bool, PWDPDR_A>;
impl PWDPDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PWDPDR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PWDPDR_A::PWR_DN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWR_DN`"]
    #[inline(always)]
    pub fn is_pwr_dn(&self) -> bool {
        *self == PWDPDR_A::PWR_DN
    }
}
#[doc = "Write proxy for field `PWDPDR`"]
pub struct PWDPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDPDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDPDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDR power down"]
    #[inline(always)]
    pub fn pwr_dn(self) -> &'a mut W {
        self.variant(PWDPDR_A::PWR_DN)
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
    #[doc = "Bit 3 - BOD External Reference Select."]
    #[inline(always)]
    pub fn bodextrefsel(&self) -> BODEXTREFSEL_R {
        BODEXTREFSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PDR External Reference Select."]
    #[inline(always)]
    pub fn pdrextrefsel(&self) -> PDREXTREFSEL_R {
        PDREXTREFSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - BOD Power Down."]
    #[inline(always)]
    pub fn pwdbod(&self) -> PWDBOD_R {
        PWDBOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PDR Power Down."]
    #[inline(always)]
    pub fn pwdpdr(&self) -> PWDPDR_R {
        PWDPDR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - BOD External Reference Select."]
    #[inline(always)]
    pub fn bodextrefsel(&mut self) -> BODEXTREFSEL_W {
        BODEXTREFSEL_W { w: self }
    }
    #[doc = "Bit 2 - PDR External Reference Select."]
    #[inline(always)]
    pub fn pdrextrefsel(&mut self) -> PDREXTREFSEL_W {
        PDREXTREFSEL_W { w: self }
    }
    #[doc = "Bit 1 - BOD Power Down."]
    #[inline(always)]
    pub fn pwdbod(&mut self) -> PWDBOD_W {
        PWDBOD_W { w: self }
    }
    #[doc = "Bit 0 - PDR Power Down."]
    #[inline(always)]
    pub fn pwdpdr(&mut self) -> PWDPDR_W {
        PWDPDR_W { w: self }
    }
}
