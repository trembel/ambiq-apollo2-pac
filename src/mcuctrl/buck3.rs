#[doc = "Reader of register BUCK3"]
pub type R = crate::R<u32, super::BUCK3>;
#[doc = "Writer for register BUCK3"]
pub type W = crate::W<u32, super::BUCK3>;
#[doc = "Register BUCK3 `reset()`'s with value 0"]
impl crate::ResetValue for super::BUCK3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEMBUCKLOTON`"]
pub type MEMBUCKLOTON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMBUCKLOTON`"]
pub struct MEMBUCKLOTON_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKLOTON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `MEMBUCKBURSTEN`"]
pub type MEMBUCKBURSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEMBUCKBURSTEN`"]
pub struct MEMBUCKBURSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKBURSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MEMBUCKZXTRIM`"]
pub type MEMBUCKZXTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMBUCKZXTRIM`"]
pub struct MEMBUCKZXTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKZXTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
#[doc = "Reader of field `MEMBUCKHYSTTRIM`"]
pub type MEMBUCKHYSTTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMBUCKHYSTTRIM`"]
pub struct MEMBUCKHYSTTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBUCKHYSTTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `COREBUCKLOTON`"]
pub type COREBUCKLOTON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COREBUCKLOTON`"]
pub struct COREBUCKLOTON_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKLOTON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `COREBUCKBURSTEN`"]
pub type COREBUCKBURSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COREBUCKBURSTEN`"]
pub struct COREBUCKBURSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKBURSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `COREBUCKZXTRIM`"]
pub type COREBUCKZXTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COREBUCKZXTRIM`"]
pub struct COREBUCKZXTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKZXTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `COREBUCKHYSTTRIM`"]
pub type COREBUCKHYSTTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COREBUCKHYSTTRIM`"]
pub struct COREBUCKHYSTTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> COREBUCKHYSTTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:21 - MEM Buck low TON trim value"]
    #[inline(always)]
    pub fn membuckloton(&self) -> MEMBUCKLOTON_R {
        MEMBUCKLOTON_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - MEM Buck burst enable 0=disable, 0=disabled, 1=enable."]
    #[inline(always)]
    pub fn membuckbursten(&self) -> MEMBUCKBURSTEN_R {
        MEMBUCKBURSTEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - Memory buck zero crossing trim value"]
    #[inline(always)]
    pub fn membuckzxtrim(&self) -> MEMBUCKZXTRIM_R {
        MEMBUCKZXTRIM_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 11:12 - Hysterisis trim for mem buck"]
    #[inline(always)]
    pub fn membuckhysttrim(&self) -> MEMBUCKHYSTTRIM_R {
        MEMBUCKHYSTTRIM_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 7:10 - Core Buck low TON trim value"]
    #[inline(always)]
    pub fn corebuckloton(&self) -> COREBUCKLOTON_R {
        COREBUCKLOTON_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Core Buck burst enable. 0=disabled, 1=enabled"]
    #[inline(always)]
    pub fn corebuckbursten(&self) -> COREBUCKBURSTEN_R {
        COREBUCKBURSTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - Core buck zero crossing trim value"]
    #[inline(always)]
    pub fn corebuckzxtrim(&self) -> COREBUCKZXTRIM_R {
        COREBUCKZXTRIM_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Hysterisis trim for core buck"]
    #[inline(always)]
    pub fn corebuckhysttrim(&self) -> COREBUCKHYSTTRIM_R {
        COREBUCKHYSTTRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:21 - MEM Buck low TON trim value"]
    #[inline(always)]
    pub fn membuckloton(&mut self) -> MEMBUCKLOTON_W {
        MEMBUCKLOTON_W { w: self }
    }
    #[doc = "Bit 17 - MEM Buck burst enable 0=disable, 0=disabled, 1=enable."]
    #[inline(always)]
    pub fn membuckbursten(&mut self) -> MEMBUCKBURSTEN_W {
        MEMBUCKBURSTEN_W { w: self }
    }
    #[doc = "Bits 13:16 - Memory buck zero crossing trim value"]
    #[inline(always)]
    pub fn membuckzxtrim(&mut self) -> MEMBUCKZXTRIM_W {
        MEMBUCKZXTRIM_W { w: self }
    }
    #[doc = "Bits 11:12 - Hysterisis trim for mem buck"]
    #[inline(always)]
    pub fn membuckhysttrim(&mut self) -> MEMBUCKHYSTTRIM_W {
        MEMBUCKHYSTTRIM_W { w: self }
    }
    #[doc = "Bits 7:10 - Core Buck low TON trim value"]
    #[inline(always)]
    pub fn corebuckloton(&mut self) -> COREBUCKLOTON_W {
        COREBUCKLOTON_W { w: self }
    }
    #[doc = "Bit 6 - Core Buck burst enable. 0=disabled, 1=enabled"]
    #[inline(always)]
    pub fn corebuckbursten(&mut self) -> COREBUCKBURSTEN_W {
        COREBUCKBURSTEN_W { w: self }
    }
    #[doc = "Bits 2:5 - Core buck zero crossing trim value"]
    #[inline(always)]
    pub fn corebuckzxtrim(&mut self) -> COREBUCKZXTRIM_W {
        COREBUCKZXTRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - Hysterisis trim for core buck"]
    #[inline(always)]
    pub fn corebuckhysttrim(&mut self) -> COREBUCKHYSTTRIM_W {
        COREBUCKHYSTTRIM_W { w: self }
    }
}
