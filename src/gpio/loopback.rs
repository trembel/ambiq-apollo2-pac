#[doc = "Reader of register LOOPBACK"]
pub type R = crate::R<u32, super::LOOPBACK>;
#[doc = "Writer for register LOOPBACK"]
pub type W = crate::W<u32, super::LOOPBACK>;
#[doc = "Register LOOPBACK `reset()`'s with value 0"]
impl crate::ResetValue for super::LOOPBACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "IOM to IOS loopback control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOOPBACK_A {
    #[doc = "0: Loop IOM0 to IOS"]
    LOOP0 = 0,
    #[doc = "1: Loop IOM1 to IOS"]
    LOOP1 = 1,
    #[doc = "2: Loop IOM2 to IOS"]
    LOOP2 = 2,
    #[doc = "3: Loop IOM3 to IOS"]
    LOOP3 = 3,
    #[doc = "4: Loop IOM4 to IOS"]
    LOOP4 = 4,
    #[doc = "5: Loop IOM5 to IOS"]
    LOOP5 = 5,
    #[doc = "6: No loopback connections"]
    LOOPNONE = 6,
}
impl From<LOOPBACK_A> for u8 {
    #[inline(always)]
    fn from(variant: LOOPBACK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOOPBACK`"]
pub type LOOPBACK_R = crate::R<u8, LOOPBACK_A>;
impl LOOPBACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOOPBACK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOOPBACK_A::LOOP0),
            1 => Val(LOOPBACK_A::LOOP1),
            2 => Val(LOOPBACK_A::LOOP2),
            3 => Val(LOOPBACK_A::LOOP3),
            4 => Val(LOOPBACK_A::LOOP4),
            5 => Val(LOOPBACK_A::LOOP5),
            6 => Val(LOOPBACK_A::LOOPNONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOOP0`"]
    #[inline(always)]
    pub fn is_loop0(&self) -> bool {
        *self == LOOPBACK_A::LOOP0
    }
    #[doc = "Checks if the value of the field is `LOOP1`"]
    #[inline(always)]
    pub fn is_loop1(&self) -> bool {
        *self == LOOPBACK_A::LOOP1
    }
    #[doc = "Checks if the value of the field is `LOOP2`"]
    #[inline(always)]
    pub fn is_loop2(&self) -> bool {
        *self == LOOPBACK_A::LOOP2
    }
    #[doc = "Checks if the value of the field is `LOOP3`"]
    #[inline(always)]
    pub fn is_loop3(&self) -> bool {
        *self == LOOPBACK_A::LOOP3
    }
    #[doc = "Checks if the value of the field is `LOOP4`"]
    #[inline(always)]
    pub fn is_loop4(&self) -> bool {
        *self == LOOPBACK_A::LOOP4
    }
    #[doc = "Checks if the value of the field is `LOOP5`"]
    #[inline(always)]
    pub fn is_loop5(&self) -> bool {
        *self == LOOPBACK_A::LOOP5
    }
    #[doc = "Checks if the value of the field is `LOOPNONE`"]
    #[inline(always)]
    pub fn is_loopnone(&self) -> bool {
        *self == LOOPBACK_A::LOOPNONE
    }
}
#[doc = "Write proxy for field `LOOPBACK`"]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPBACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Loop IOM0 to IOS"]
    #[inline(always)]
    pub fn loop0(self) -> &'a mut W {
        self.variant(LOOPBACK_A::LOOP0)
    }
    #[doc = "Loop IOM1 to IOS"]
    #[inline(always)]
    pub fn loop1(self) -> &'a mut W {
        self.variant(LOOPBACK_A::LOOP1)
    }
    #[doc = "Loop IOM2 to IOS"]
    #[inline(always)]
    pub fn loop2(self) -> &'a mut W {
        self.variant(LOOPBACK_A::LOOP2)
    }
    #[doc = "Loop IOM3 to IOS"]
    #[inline(always)]
    pub fn loop3(self) -> &'a mut W {
        self.variant(LOOPBACK_A::LOOP3)
    }
    #[doc = "Loop IOM4 to IOS"]
    #[inline(always)]
    pub fn loop4(self) -> &'a mut W {
        self.variant(LOOPBACK_A::LOOP4)
    }
    #[doc = "Loop IOM5 to IOS"]
    #[inline(always)]
    pub fn loop5(self) -> &'a mut W {
        self.variant(LOOPBACK_A::LOOP5)
    }
    #[doc = "No loopback connections"]
    #[inline(always)]
    pub fn loopnone(self) -> &'a mut W {
        self.variant(LOOPBACK_A::LOOPNONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - IOM to IOS loopback control."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - IOM to IOS loopback control."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
}
