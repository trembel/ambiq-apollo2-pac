#[doc = "Reader of register CACHEMODE"]
pub type R = crate::R<u32, super::CACHEMODE>;
#[doc = "Writer for register CACHEMODE"]
pub type W = crate::W<u32, super::CACHEMODE>;
#[doc = "Register CACHEMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::CACHEMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THROTTLE6`"]
pub type THROTTLE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THROTTLE6`"]
pub struct THROTTLE6_W<'a> {
    w: &'a mut W,
}
impl<'a> THROTTLE6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `THROTTLE5`"]
pub type THROTTLE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THROTTLE5`"]
pub struct THROTTLE5_W<'a> {
    w: &'a mut W,
}
impl<'a> THROTTLE5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `THROTTLE4`"]
pub type THROTTLE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THROTTLE4`"]
pub struct THROTTLE4_W<'a> {
    w: &'a mut W,
}
impl<'a> THROTTLE4_W<'a> {
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
#[doc = "Reader of field `THROTTLE3`"]
pub type THROTTLE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THROTTLE3`"]
pub struct THROTTLE3_W<'a> {
    w: &'a mut W,
}
impl<'a> THROTTLE3_W<'a> {
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
#[doc = "Reader of field `THROTTLE2`"]
pub type THROTTLE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THROTTLE2`"]
pub struct THROTTLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> THROTTLE2_W<'a> {
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
#[doc = "Reader of field `THROTTLE1`"]
pub type THROTTLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `THROTTLE1`"]
pub struct THROTTLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> THROTTLE1_W<'a> {
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
    #[doc = "Bit 5 - Disallow Simultaneous Data RAM reads (from 2 line hits on each bus). Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle6(&self) -> THROTTLE6_R {
        THROTTLE6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Disallow Data RAM reads (from line hits) during lookup read ops. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle5(&self) -> THROTTLE5_R {
        THROTTLE5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disallow Data RAM reads (from line hits) on tag RAM fill cycles. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle4(&self) -> THROTTLE4_R {
        THROTTLE4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disallow cache data RAM writes on data RAM read cycles. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle3(&self) -> THROTTLE3_R {
        THROTTLE3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disallow cache data RAM writes on tag RAM read cycles. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle2(&self) -> THROTTLE2_R {
        THROTTLE2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Disallow cache data RAM writes on tag RAM fill cycles. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle1(&self) -> THROTTLE1_R {
        THROTTLE1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Disallow Simultaneous Data RAM reads (from 2 line hits on each bus). Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle6(&mut self) -> THROTTLE6_W {
        THROTTLE6_W { w: self }
    }
    #[doc = "Bit 4 - Disallow Data RAM reads (from line hits) during lookup read ops. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle5(&mut self) -> THROTTLE5_W {
        THROTTLE5_W { w: self }
    }
    #[doc = "Bit 3 - Disallow Data RAM reads (from line hits) on tag RAM fill cycles. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle4(&mut self) -> THROTTLE4_W {
        THROTTLE4_W { w: self }
    }
    #[doc = "Bit 2 - Disallow cache data RAM writes on data RAM read cycles. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle3(&mut self) -> THROTTLE3_W {
        THROTTLE3_W { w: self }
    }
    #[doc = "Bit 1 - Disallow cache data RAM writes on tag RAM read cycles. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle2(&mut self) -> THROTTLE2_W {
        THROTTLE2_W { w: self }
    }
    #[doc = "Bit 0 - Disallow cache data RAM writes on tag RAM fill cycles. Value should be left at zero for optimal performance."]
    #[inline(always)]
    pub fn throttle1(&mut self) -> THROTTLE1_W {
        THROTTLE1_W { w: self }
    }
}
