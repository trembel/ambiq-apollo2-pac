#[doc = "Reader of register BUCKTRIM"]
pub type R = crate::R<u32, super::BUCKTRIM>;
#[doc = "Writer for register BUCKTRIM"]
pub type W = crate::W<u32, super::BUCKTRIM>;
#[doc = "Register BUCKTRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::BUCKTRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSVD2`"]
pub struct RSVD2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `COREBUCKR1_HI`"]
pub type COREBUCKR1_HI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COREBUCKR1_HI`"]
pub struct COREBUCKR1_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKR1_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `COREBUCKR1_LO`"]
pub type COREBUCKR1_LO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COREBUCKR1_LO`"]
pub struct COREBUCKR1_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKR1_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEMBUCKR1`"]
pub type MEMBUCKR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMBUCKR1`"]
pub struct MEMBUCKR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - RESERVED."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Core Buck voltage output trim bits\\[9:6\\]. Concatenate with field COREBUCKR1_LO for the full trim value."]
    #[inline(always)]
    pub fn corebuckr1_hi(&self) -> COREBUCKR1_HI_R {
        COREBUCKR1_HI_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Core Buck voltage output trim bits\\[5:0\\], Concatenate with field COREBUCKR1_HI for the full trim value."]
    #[inline(always)]
    pub fn corebuckr1_lo(&self) -> COREBUCKR1_LO_R {
        COREBUCKR1_LO_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Trim values for BUCK regulator."]
    #[inline(always)]
    pub fn membuckr1(&self) -> MEMBUCKR1_R {
        MEMBUCKR1_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - RESERVED."]
    #[inline(always)]
    pub fn rsvd2(&mut self) -> RSVD2_W {
        RSVD2_W { w: self }
    }
    #[doc = "Bits 16:19 - Core Buck voltage output trim bits\\[9:6\\]. Concatenate with field COREBUCKR1_LO for the full trim value."]
    #[inline(always)]
    pub fn corebuckr1_hi(&mut self) -> COREBUCKR1_HI_W {
        COREBUCKR1_HI_W { w: self }
    }
    #[doc = "Bits 8:13 - Core Buck voltage output trim bits\\[5:0\\], Concatenate with field COREBUCKR1_HI for the full trim value."]
    #[inline(always)]
    pub fn corebuckr1_lo(&mut self) -> COREBUCKR1_LO_W {
        COREBUCKR1_LO_W { w: self }
    }
    #[doc = "Bits 0:5 - Trim values for BUCK regulator."]
    #[inline(always)]
    pub fn membuckr1(&mut self) -> MEMBUCKR1_W {
        MEMBUCKR1_W { w: self }
    }
}
