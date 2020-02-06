#[doc = "Reader of register ADCSTATUS"]
pub type R = crate::R<u32, super::ADCSTATUS>;
#[doc = "Writer for register ADCSTATUS"]
pub type W = crate::W<u32, super::ADCSTATUS>;
#[doc = "Register ADCSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_REFBUF_PWD`"]
pub type ADC_REFBUF_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_REFBUF_PWD`"]
pub struct ADC_REFBUF_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REFBUF_PWD_W<'a> {
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
#[doc = "Reader of field `ADC_REFKEEP_PWD`"]
pub type ADC_REFKEEP_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_REFKEEP_PWD`"]
pub struct ADC_REFKEEP_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REFKEEP_PWD_W<'a> {
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
#[doc = "Reader of field `ADC_VBAT_PWD`"]
pub type ADC_VBAT_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_VBAT_PWD`"]
pub struct ADC_VBAT_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_VBAT_PWD_W<'a> {
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
#[doc = "Reader of field `ADC_VPTAT_PWD`"]
pub type ADC_VPTAT_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_VPTAT_PWD`"]
pub struct ADC_VPTAT_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_VPTAT_PWD_W<'a> {
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
#[doc = "Reader of field `ADC_BGT_PWD`"]
pub type ADC_BGT_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_BGT_PWD`"]
pub struct ADC_BGT_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_BGT_PWD_W<'a> {
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
#[doc = "Reader of field `ADC_PWD`"]
pub type ADC_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_PWD`"]
pub struct ADC_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PWD_W<'a> {
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
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    pub fn adc_refbuf_pwd(&self) -> ADC_REFBUF_PWD_R {
        ADC_REFBUF_PWD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    pub fn adc_refkeep_pwd(&self) -> ADC_REFKEEP_PWD_R {
        ADC_REFKEEP_PWD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    pub fn adc_vbat_pwd(&self) -> ADC_VBAT_PWD_R {
        ADC_VBAT_PWD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    pub fn adc_vptat_pwd(&self) -> ADC_VPTAT_PWD_R {
        ADC_VPTAT_PWD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    pub fn adc_bgt_pwd(&self) -> ADC_BGT_PWD_R {
        ADC_BGT_PWD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    pub fn adc_pwd(&self) -> ADC_PWD_R {
        ADC_PWD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - This bit indicates that the ADC REFBUF is powered down"]
    #[inline(always)]
    pub fn adc_refbuf_pwd(&mut self) -> ADC_REFBUF_PWD_W {
        ADC_REFBUF_PWD_W { w: self }
    }
    #[doc = "Bit 4 - This bit indicates that the ADC REFKEEP is powered down"]
    #[inline(always)]
    pub fn adc_refkeep_pwd(&mut self) -> ADC_REFKEEP_PWD_W {
        ADC_REFKEEP_PWD_W { w: self }
    }
    #[doc = "Bit 3 - This bit indicates that the ADC VBAT resistor divider is powered down"]
    #[inline(always)]
    pub fn adc_vbat_pwd(&mut self) -> ADC_VBAT_PWD_W {
        ADC_VBAT_PWD_W { w: self }
    }
    #[doc = "Bit 2 - This bit indicates that the ADC temperature sensor input buffer is powered down"]
    #[inline(always)]
    pub fn adc_vptat_pwd(&mut self) -> ADC_VPTAT_PWD_W {
        ADC_VPTAT_PWD_W { w: self }
    }
    #[doc = "Bit 1 - This bit indicates that the ADC Band Gap is powered down"]
    #[inline(always)]
    pub fn adc_bgt_pwd(&mut self) -> ADC_BGT_PWD_W {
        ADC_BGT_PWD_W { w: self }
    }
    #[doc = "Bit 0 - This bit indicates that the ADC is powered down"]
    #[inline(always)]
    pub fn adc_pwd(&mut self) -> ADC_PWD_W {
        ADC_PWD_W { w: self }
    }
}
