#[doc = "Reader of register LDOREG1"]
pub type R = crate::R<u32, super::LDOREG1>;
#[doc = "Writer for register LDOREG1"]
pub type W = crate::W<u32, super::LDOREG1>;
#[doc = "Register LDOREG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LDOREG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CORELDOIBSTRM`"]
pub type CORELDOIBSTRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CORELDOIBSTRM`"]
pub struct CORELDOIBSTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> CORELDOIBSTRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CORELDOLPTRIM`"]
pub type CORELDOLPTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CORELDOLPTRIM`"]
pub struct CORELDOLPTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CORELDOLPTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `TRIMCORELDOR3`"]
pub type TRIMCORELDOR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMCORELDOR3`"]
pub struct TRIMCORELDOR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMCORELDOR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `TRIMCORELDOR1`"]
pub type TRIMCORELDOR1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TRIMCORELDOR1`"]
pub struct TRIMCORELDOR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMCORELDOR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - CORE LDO IBIAS Trim"]
    #[inline(always)]
    pub fn coreldoibstrm(&self) -> CORELDOIBSTRM_R {
        CORELDOIBSTRM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 14:19 - CORE LDO Low Power Trim"]
    #[inline(always)]
    pub fn coreldolptrim(&self) -> CORELDOLPTRIM_R {
        CORELDOLPTRIM_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 10:13 - CORE LDO tempco trim (R3)."]
    #[inline(always)]
    pub fn trimcoreldor3(&self) -> TRIMCORELDOR3_R {
        TRIMCORELDOR3_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9 - CORE LDO Active mode ouput trim (R1)."]
    #[inline(always)]
    pub fn trimcoreldor1(&self) -> TRIMCORELDOR1_R {
        TRIMCORELDOR1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 20 - CORE LDO IBIAS Trim"]
    #[inline(always)]
    pub fn coreldoibstrm(&mut self) -> CORELDOIBSTRM_W {
        CORELDOIBSTRM_W { w: self }
    }
    #[doc = "Bits 14:19 - CORE LDO Low Power Trim"]
    #[inline(always)]
    pub fn coreldolptrim(&mut self) -> CORELDOLPTRIM_W {
        CORELDOLPTRIM_W { w: self }
    }
    #[doc = "Bits 10:13 - CORE LDO tempco trim (R3)."]
    #[inline(always)]
    pub fn trimcoreldor3(&mut self) -> TRIMCORELDOR3_W {
        TRIMCORELDOR3_W { w: self }
    }
    #[doc = "Bits 0:9 - CORE LDO Active mode ouput trim (R1)."]
    #[inline(always)]
    pub fn trimcoreldor1(&mut self) -> TRIMCORELDOR1_W {
        TRIMCORELDOR1_W { w: self }
    }
}
