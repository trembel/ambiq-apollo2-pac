#[doc = "Reader of register FR"]
pub type R = crate::R<u32, super::FR>;
#[doc = "Writer for register FR"]
pub type W = crate::W<u32, super::FR>;
#[doc = "Register FR `reset()`'s with value 0"]
impl crate::ResetValue for super::FR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOCNT`"]
pub type FIFOCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FIFOCNT`"]
pub struct FIFOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    pub fn fifocnt(&self) -> FIFOCNT_R {
        FIFOCNT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    pub fn fifocnt(&mut self) -> FIFOCNT_W {
        FIFOCNT_W { w: self }
    }
}
