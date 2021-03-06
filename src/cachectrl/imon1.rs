#[doc = "Reader of register IMON1"]
pub type R = crate::R<u32, super::IMON1>;
#[doc = "Writer for register IMON1"]
pub type W = crate::W<u32, super::IMON1>;
#[doc = "Register IMON1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMON1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ILOOKUP_COUNT`"]
pub type ILOOKUP_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ILOOKUP_COUNT`"]
pub struct ILOOKUP_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ILOOKUP_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Total tag lookups from Instruction cache"]
    #[inline(always)]
    pub fn ilookup_count(&self) -> ILOOKUP_COUNT_R {
        ILOOKUP_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total tag lookups from Instruction cache"]
    #[inline(always)]
    pub fn ilookup_count(&mut self) -> ILOOKUP_COUNT_W {
        ILOOKUP_COUNT_W { w: self }
    }
}
