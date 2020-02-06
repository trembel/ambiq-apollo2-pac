#[doc = "Reader of register DMON3"]
pub type R = crate::R<u32, super::DMON3>;
#[doc = "Writer for register DMON3"]
pub type W = crate::W<u32, super::DMON3>;
#[doc = "Register DMON3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMON3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLINE_COUNT`"]
pub type DLINE_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DLINE_COUNT`"]
pub struct DLINE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLINE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cache hits from line cache"]
    #[inline(always)]
    pub fn dline_count(&self) -> DLINE_COUNT_R {
        DLINE_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache hits from line cache"]
    #[inline(always)]
    pub fn dline_count(&mut self) -> DLINE_COUNT_W {
        DLINE_COUNT_W { w: self }
    }
}
