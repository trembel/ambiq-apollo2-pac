#[doc = "Reader of register LDOREG3"]
pub type R = crate::R<u32, super::LDOREG3>;
#[doc = "Writer for register LDOREG3"]
pub type W = crate::W<u32, super::LDOREG3>;
#[doc = "Register LDOREG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::LDOREG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIMMEMLDOR1`"]
pub type TRIMMEMLDOR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMMEMLDOR1`"]
pub struct TRIMMEMLDOR1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMMEMLDOR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `MEMLDOLPALTTRIM`"]
pub type MEMLDOLPALTTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMLDOLPALTTRIM`"]
pub struct MEMLDOLPALTTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMLDOLPALTTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `MEMLDOLPTRIM`"]
pub type MEMLDOLPTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEMLDOLPTRIM`"]
pub struct MEMLDOLPTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMLDOLPTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:17 - MEM LDO active mode trim (R1)."]
    #[inline(always)]
    pub fn trimmemldor1(&self) -> TRIMMEMLDOR1_R {
        TRIMMEMLDOR1_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - MEM LDO TRIM for low power mode with ADC active"]
    #[inline(always)]
    pub fn memldolpalttrim(&self) -> MEMLDOLPALTTRIM_R {
        MEMLDOLPALTTRIM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - MEM LDO TRIM for low power mode with ADC inactive"]
    #[inline(always)]
    pub fn memldolptrim(&self) -> MEMLDOLPTRIM_R {
        MEMLDOLPTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:17 - MEM LDO active mode trim (R1)."]
    #[inline(always)]
    pub fn trimmemldor1(&mut self) -> TRIMMEMLDOR1_W {
        TRIMMEMLDOR1_W { w: self }
    }
    #[doc = "Bits 6:11 - MEM LDO TRIM for low power mode with ADC active"]
    #[inline(always)]
    pub fn memldolpalttrim(&mut self) -> MEMLDOLPALTTRIM_W {
        MEMLDOLPALTTRIM_W { w: self }
    }
    #[doc = "Bits 0:5 - MEM LDO TRIM for low power mode with ADC inactive"]
    #[inline(always)]
    pub fn memldolptrim(&mut self) -> MEMLDOLPTRIM_W {
        MEMLDOLPTRIM_W { w: self }
    }
}
