#[doc = "Reader of register CHIPREV"]
pub type R = crate::R<u32, super::CHIPREV>;
#[doc = "Writer for register CHIPREV"]
pub type W = crate::W<u32, super::CHIPREV>;
#[doc = "Register CHIPREV `reset()`'s with value 0x11"]
impl crate::ResetValue for super::CHIPREV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x11
    }
}
#[doc = "Major Revision ID.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVMAJ_A {
    #[doc = "2: Apollo2 revision B"]
    B = 2,
    #[doc = "1: Apollo2 revision A"]
    A = 1,
}
impl From<REVMAJ_A> for u8 {
    #[inline(always)]
    fn from(variant: REVMAJ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REVMAJ`"]
pub type REVMAJ_R = crate::R<u8, REVMAJ_A>;
impl REVMAJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVMAJ_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(REVMAJ_A::B),
            1 => Val(REVMAJ_A::A),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B`"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == REVMAJ_A::B
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == REVMAJ_A::A
    }
}
#[doc = "Write proxy for field `REVMAJ`"]
pub struct REVMAJ_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMAJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVMAJ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo2 revision B"]
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(REVMAJ_A::B)
    }
    #[doc = "Apollo2 revision A"]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(REVMAJ_A::A)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Minor Revision ID.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVMIN_A {
    #[doc = "0: Apollo2 minor revision value. Succeeding minor revisions will increment from this value."]
    REV0 = 0,
    #[doc = "2: Apollo2 minor revision value."]
    REV2 = 2,
}
impl From<REVMIN_A> for u8 {
    #[inline(always)]
    fn from(variant: REVMIN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REVMIN`"]
pub type REVMIN_R = crate::R<u8, REVMIN_A>;
impl REVMIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVMIN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REVMIN_A::REV0),
            2 => Val(REVMIN_A::REV2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REV0`"]
    #[inline(always)]
    pub fn is_rev0(&self) -> bool {
        *self == REVMIN_A::REV0
    }
    #[doc = "Checks if the value of the field is `REV2`"]
    #[inline(always)]
    pub fn is_rev2(&self) -> bool {
        *self == REVMIN_A::REV2
    }
}
#[doc = "Write proxy for field `REVMIN`"]
pub struct REVMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REVMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REVMIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo2 minor revision value. Succeeding minor revisions will increment from this value."]
    #[inline(always)]
    pub fn rev0(self) -> &'a mut W {
        self.variant(REVMIN_A::REV0)
    }
    #[doc = "Apollo2 minor revision value."]
    #[inline(always)]
    pub fn rev2(self) -> &'a mut W {
        self.variant(REVMIN_A::REV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline(always)]
    pub fn revmaj(&self) -> REVMAJ_R {
        REVMAJ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline(always)]
    pub fn revmin(&self) -> REVMIN_R {
        REVMIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Major Revision ID."]
    #[inline(always)]
    pub fn revmaj(&mut self) -> REVMAJ_W {
        REVMAJ_W { w: self }
    }
    #[doc = "Bits 0:3 - Minor Revision ID."]
    #[inline(always)]
    pub fn revmin(&mut self) -> REVMIN_W {
        REVMIN_W { w: self }
    }
}
