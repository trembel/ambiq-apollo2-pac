#[doc = "Reader of register FLASHCFG"]
pub type R = crate::R<u32, super::FLASHCFG>;
#[doc = "Writer for register FLASHCFG"]
pub type W = crate::W<u32, super::FLASHCFG>;
#[doc = "Register FLASHCFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FLASHCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RD_WAIT`"]
pub type RD_WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_WAIT`"]
pub struct RD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sets read waitstates for flash accesses (in clock cycles). This should be left at the default value for normal flash operation."]
    #[inline(always)]
    pub fn rd_wait(&self) -> RD_WAIT_R {
        RD_WAIT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets read waitstates for flash accesses (in clock cycles). This should be left at the default value for normal flash operation."]
    #[inline(always)]
    pub fn rd_wait(&mut self) -> RD_WAIT_W {
        RD_WAIT_W { w: self }
    }
}
