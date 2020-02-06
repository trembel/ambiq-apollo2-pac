#[doc = "Reader of register XTALGENCTRL"]
pub type R = crate::R<u32, super::XTALGENCTRL>;
#[doc = "Writer for register XTALGENCTRL"]
pub type W = crate::W<u32, super::XTALGENCTRL>;
#[doc = "Register XTALGENCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::XTALGENCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTALKSBIASTRIM`"]
pub type XTALKSBIASTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTALKSBIASTRIM`"]
pub struct XTALKSBIASTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALKSBIASTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `XTALBIASTRIM`"]
pub type XTALBIASTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XTALBIASTRIM`"]
pub struct XTALBIASTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALBIASTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Auto-calibration delay control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACWARMUP_A {
    #[doc = "0: Warmup period of 1-2 seconds"]
    _1SEC = 0,
    #[doc = "1: Warmup period of 2-4 seconds"]
    _2SEC = 1,
    #[doc = "2: Warmup period of 4-8 seconds"]
    _4SEC = 2,
    #[doc = "3: Warmup period of 8-16 seconds"]
    _8SEC = 3,
}
impl From<ACWARMUP_A> for u8 {
    #[inline(always)]
    fn from(variant: ACWARMUP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACWARMUP`"]
pub type ACWARMUP_R = crate::R<u8, ACWARMUP_A>;
impl ACWARMUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACWARMUP_A {
        match self.bits {
            0 => ACWARMUP_A::_1SEC,
            1 => ACWARMUP_A::_2SEC,
            2 => ACWARMUP_A::_4SEC,
            3 => ACWARMUP_A::_8SEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1SEC`"]
    #[inline(always)]
    pub fn is_1sec(&self) -> bool {
        *self == ACWARMUP_A::_1SEC
    }
    #[doc = "Checks if the value of the field is `_2SEC`"]
    #[inline(always)]
    pub fn is_2sec(&self) -> bool {
        *self == ACWARMUP_A::_2SEC
    }
    #[doc = "Checks if the value of the field is `_4SEC`"]
    #[inline(always)]
    pub fn is_4sec(&self) -> bool {
        *self == ACWARMUP_A::_4SEC
    }
    #[doc = "Checks if the value of the field is `_8SEC`"]
    #[inline(always)]
    pub fn is_8sec(&self) -> bool {
        *self == ACWARMUP_A::_8SEC
    }
}
#[doc = "Write proxy for field `ACWARMUP`"]
pub struct ACWARMUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACWARMUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACWARMUP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Warmup period of 1-2 seconds"]
    #[inline(always)]
    pub fn _1sec(self) -> &'a mut W {
        self.variant(ACWARMUP_A::_1SEC)
    }
    #[doc = "Warmup period of 2-4 seconds"]
    #[inline(always)]
    pub fn _2sec(self) -> &'a mut W {
        self.variant(ACWARMUP_A::_2SEC)
    }
    #[doc = "Warmup period of 4-8 seconds"]
    #[inline(always)]
    pub fn _4sec(self) -> &'a mut W {
        self.variant(ACWARMUP_A::_4SEC)
    }
    #[doc = "Warmup period of 8-16 seconds"]
    #[inline(always)]
    pub fn _8sec(self) -> &'a mut W {
        self.variant(ACWARMUP_A::_8SEC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim . This trim value is used during the startup process to enable a faster lock and is applied when the kickstart signal is active."]
    #[inline(always)]
    pub fn xtalksbiastrim(&self) -> XTALKSBIASTRIM_R {
        XTALKSBIASTRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 2:7 - XTAL IBIAS trim"]
    #[inline(always)]
    pub fn xtalbiastrim(&self) -> XTALBIASTRIM_R {
        XTALBIASTRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline(always)]
    pub fn acwarmup(&self) -> ACWARMUP_R {
        ACWARMUP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - XTAL IBIAS Kick start trim . This trim value is used during the startup process to enable a faster lock and is applied when the kickstart signal is active."]
    #[inline(always)]
    pub fn xtalksbiastrim(&mut self) -> XTALKSBIASTRIM_W {
        XTALKSBIASTRIM_W { w: self }
    }
    #[doc = "Bits 2:7 - XTAL IBIAS trim"]
    #[inline(always)]
    pub fn xtalbiastrim(&mut self) -> XTALBIASTRIM_W {
        XTALBIASTRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - Auto-calibration delay control"]
    #[inline(always)]
    pub fn acwarmup(&mut self) -> ACWARMUP_W {
        ACWARMUP_W { w: self }
    }
}
