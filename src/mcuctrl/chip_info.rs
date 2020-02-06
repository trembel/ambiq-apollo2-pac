#[doc = "Reader of register CHIP_INFO"]
pub type R = crate::R<u32, super::CHIP_INFO>;
#[doc = "Writer for register CHIP_INFO"]
pub type W = crate::W<u32, super::CHIP_INFO>;
#[doc = "Register CHIP_INFO `reset()`'s with value 0x0300_0000"]
impl crate::ResetValue for super::CHIP_INFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300_0000
    }
}
#[doc = "BCD part number.\n\nValue on reset: 50331648"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PARTNUM_A {
    #[doc = "50331648: Apollo2 part number is 0x03XXXXXX."]
    APOLLO2 = 50331648,
    #[doc = "16777216: Apollo part number is 0x01XXXXXX."]
    APOLLO = 16777216,
    #[doc = "4278190080: Mask for the PN field."]
    PN_M = 4278190080,
}
impl From<PARTNUM_A> for u32 {
    #[inline(always)]
    fn from(variant: PARTNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PARTNUM`"]
pub type PARTNUM_R = crate::R<u32, PARTNUM_A>;
impl PARTNUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PARTNUM_A> {
        use crate::Variant::*;
        match self.bits {
            50331648 => Val(PARTNUM_A::APOLLO2),
            16777216 => Val(PARTNUM_A::APOLLO),
            4278190080 => Val(PARTNUM_A::PN_M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO2`"]
    #[inline(always)]
    pub fn is_apollo2(&self) -> bool {
        *self == PARTNUM_A::APOLLO2
    }
    #[doc = "Checks if the value of the field is `APOLLO`"]
    #[inline(always)]
    pub fn is_apollo(&self) -> bool {
        *self == PARTNUM_A::APOLLO
    }
    #[doc = "Checks if the value of the field is `PN_M`"]
    #[inline(always)]
    pub fn is_pn_m(&self) -> bool {
        *self == PARTNUM_A::PN_M
    }
}
#[doc = "Write proxy for field `PARTNUM`"]
pub struct PARTNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PARTNUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARTNUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo2 part number is 0x03XXXXXX."]
    #[inline(always)]
    pub fn apollo2(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO2)
    }
    #[doc = "Apollo part number is 0x01XXXXXX."]
    #[inline(always)]
    pub fn apollo(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO)
    }
    #[doc = "Mask for the PN field."]
    #[inline(always)]
    pub fn pn_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::PN_M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BCD part number."]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BCD part number."]
    #[inline(always)]
    pub fn partnum(&mut self) -> PARTNUM_W {
        PARTNUM_W { w: self }
    }
}
