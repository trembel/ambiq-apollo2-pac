#[doc = "Reader of register NCR1END"]
pub type R = crate::R<u32, super::NCR1END>;
#[doc = "Writer for register NCR1END"]
pub type W = crate::W<u32, super::NCR1END>;
#[doc = "Register NCR1END `reset()`'s with value 0"]
impl crate::ResetValue for super::NCR1END {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 4)) | (((value as u32) & 0xffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:19 - End address for non-cacheable region 1. The physical address of the end of this region should be programmed to this register and must be aligned to a 16-byte boundary (thus the lower 4 address bits are unused)."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:19 - End address for non-cacheable region 1. The physical address of the end of this region should be programmed to this register and must be aligned to a 16-byte boundary (thus the lower 4 address bits are unused)."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
