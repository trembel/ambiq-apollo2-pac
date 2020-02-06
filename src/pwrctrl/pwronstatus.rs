#[doc = "Reader of register PWRONSTATUS"]
pub type R = crate::R<u32, super::PWRONSTATUS>;
#[doc = "Writer for register PWRONSTATUS"]
pub type W = crate::W<u32, super::PWRONSTATUS>;
#[doc = "Register PWRONSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRONSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PD_CACHEB2`"]
pub type PD_CACHEB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_CACHEB2`"]
pub struct PD_CACHEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_CACHEB2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PD_CACHEB0`"]
pub type PD_CACHEB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_CACHEB0`"]
pub struct PD_CACHEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_CACHEB0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP7_SRAM`"]
pub type PD_GRP7_SRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP7_SRAM`"]
pub struct PD_GRP7_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP7_SRAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP6_SRAM`"]
pub type PD_GRP6_SRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP6_SRAM`"]
pub struct PD_GRP6_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP6_SRAM_W<'a> {
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
#[doc = "Reader of field `PD_GRP5_SRAM`"]
pub type PD_GRP5_SRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP5_SRAM`"]
pub struct PD_GRP5_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP5_SRAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP4_SRAM`"]
pub type PD_GRP4_SRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP4_SRAM`"]
pub struct PD_GRP4_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP4_SRAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP3_SRAM`"]
pub type PD_GRP3_SRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP3_SRAM`"]
pub struct PD_GRP3_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP3_SRAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP2_SRAM`"]
pub type PD_GRP2_SRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP2_SRAM`"]
pub struct PD_GRP2_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP2_SRAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP1_SRAM`"]
pub type PD_GRP1_SRAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP1_SRAM`"]
pub struct PD_GRP1_SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP1_SRAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP0_SRAM3`"]
pub type PD_GRP0_SRAM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP0_SRAM3`"]
pub struct PD_GRP0_SRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP0_SRAM3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP0_SRAM2`"]
pub type PD_GRP0_SRAM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP0_SRAM2`"]
pub struct PD_GRP0_SRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP0_SRAM2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP0_SRAM1`"]
pub type PD_GRP0_SRAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP0_SRAM1`"]
pub struct PD_GRP0_SRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP0_SRAM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PD_GRP0_SRAM0`"]
pub type PD_GRP0_SRAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_GRP0_SRAM0`"]
pub struct PD_GRP0_SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_GRP0_SRAM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PDADC`"]
pub type PDADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDADC`"]
pub struct PDADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PD_FLAM1`"]
pub type PD_FLAM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_FLAM1`"]
pub struct PD_FLAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_FLAM1_W<'a> {
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
#[doc = "Reader of field `PD_FLAM0`"]
pub type PD_FLAM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_FLAM0`"]
pub struct PD_FLAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_FLAM0_W<'a> {
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
#[doc = "Reader of field `PD_PDM`"]
pub type PD_PDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD_PDM`"]
pub struct PD_PDM_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_PDM_W<'a> {
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
#[doc = "Reader of field `PDC`"]
pub type PDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDC`"]
pub struct PDC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDC_W<'a> {
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
#[doc = "Reader of field `PDB`"]
pub type PDB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDB`"]
pub struct PDB_W<'a> {
    w: &'a mut W,
}
impl<'a> PDB_W<'a> {
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
#[doc = "Reader of field `PDA`"]
pub type PDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDA`"]
pub struct PDA_W<'a> {
    w: &'a mut W,
}
impl<'a> PDA_W<'a> {
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
impl R {
    #[doc = "Bit 21 - This bit is 1 if power is supplied to CACHE BANK 2"]
    #[inline(always)]
    pub fn pd_cacheb2(&self) -> PD_CACHEB2_R {
        PD_CACHEB2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 19 - This bit is 1 if power is supplied to CACHE BANK 0"]
    #[inline(always)]
    pub fn pd_cacheb0(&self) -> PD_CACHEB0_R {
        PD_CACHEB0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This bit is 1 if power is supplied to SRAM domain PD_GRP7"]
    #[inline(always)]
    pub fn pd_grp7_sram(&self) -> PD_GRP7_SRAM_R {
        PD_GRP7_SRAM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This bit is 1 if power is supplied to SRAM domain PD_GRP6"]
    #[inline(always)]
    pub fn pd_grp6_sram(&self) -> PD_GRP6_SRAM_R {
        PD_GRP6_SRAM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit is 1 if power is supplied to SRAM domain PD_GRP5"]
    #[inline(always)]
    pub fn pd_grp5_sram(&self) -> PD_GRP5_SRAM_R {
        PD_GRP5_SRAM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit is 1 if power is supplied to SRAM domain PD_GRP4"]
    #[inline(always)]
    pub fn pd_grp4_sram(&self) -> PD_GRP4_SRAM_R {
        PD_GRP4_SRAM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is 1 if power is supplied to SRAM domain PD_GRP3"]
    #[inline(always)]
    pub fn pd_grp3_sram(&self) -> PD_GRP3_SRAM_R {
        PD_GRP3_SRAM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is 1 if power is supplied to SRAM domain PD_GRP2"]
    #[inline(always)]
    pub fn pd_grp2_sram(&self) -> PD_GRP2_SRAM_R {
        PD_GRP2_SRAM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is 1 if power is supplied to SRAM domain PD_GRP1"]
    #[inline(always)]
    pub fn pd_grp1_sram(&self) -> PD_GRP1_SRAM_R {
        PD_GRP1_SRAM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit is 1 if power is supplied to SRAM domain PD_SRAM0_3"]
    #[inline(always)]
    pub fn pd_grp0_sram3(&self) -> PD_GRP0_SRAM3_R {
        PD_GRP0_SRAM3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit is 1 if power is supplied to SRAM domain PD_SRAM0_2"]
    #[inline(always)]
    pub fn pd_grp0_sram2(&self) -> PD_GRP0_SRAM2_R {
        PD_GRP0_SRAM2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit is 1 if power is supplied to SRAM domain SRAM0_1"]
    #[inline(always)]
    pub fn pd_grp0_sram1(&self) -> PD_GRP0_SRAM1_R {
        PD_GRP0_SRAM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to SRAM domain SRAM0_0"]
    #[inline(always)]
    pub fn pd_grp0_sram0(&self) -> PD_GRP0_SRAM0_R {
        PD_GRP0_SRAM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to domain PD_ADC"]
    #[inline(always)]
    pub fn pdadc(&self) -> PDADC_R {
        PDADC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to domain PD_FLAM1"]
    #[inline(always)]
    pub fn pd_flam1(&self) -> PD_FLAM1_R {
        PD_FLAM1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to domain PD_FLAM0"]
    #[inline(always)]
    pub fn pd_flam0(&self) -> PD_FLAM0_R {
        PD_FLAM0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to domain PD_PDM"]
    #[inline(always)]
    pub fn pd_pdm(&self) -> PD_PDM_R {
        PD_PDM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to power domain C, which supplies IOM3-5."]
    #[inline(always)]
    pub fn pdc(&self) -> PDC_R {
        PDC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to power domain B, which supplies IOM0-2."]
    #[inline(always)]
    pub fn pdb(&self) -> PDB_R {
        PDB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to power domain A, which supplies IOS and UART0,1."]
    #[inline(always)]
    pub fn pda(&self) -> PDA_R {
        PDA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - This bit is 1 if power is supplied to CACHE BANK 2"]
    #[inline(always)]
    pub fn pd_cacheb2(&mut self) -> PD_CACHEB2_W {
        PD_CACHEB2_W { w: self }
    }
    #[doc = "Bit 19 - This bit is 1 if power is supplied to CACHE BANK 0"]
    #[inline(always)]
    pub fn pd_cacheb0(&mut self) -> PD_CACHEB0_W {
        PD_CACHEB0_W { w: self }
    }
    #[doc = "Bit 18 - This bit is 1 if power is supplied to SRAM domain PD_GRP7"]
    #[inline(always)]
    pub fn pd_grp7_sram(&mut self) -> PD_GRP7_SRAM_W {
        PD_GRP7_SRAM_W { w: self }
    }
    #[doc = "Bit 17 - This bit is 1 if power is supplied to SRAM domain PD_GRP6"]
    #[inline(always)]
    pub fn pd_grp6_sram(&mut self) -> PD_GRP6_SRAM_W {
        PD_GRP6_SRAM_W { w: self }
    }
    #[doc = "Bit 16 - This bit is 1 if power is supplied to SRAM domain PD_GRP5"]
    #[inline(always)]
    pub fn pd_grp5_sram(&mut self) -> PD_GRP5_SRAM_W {
        PD_GRP5_SRAM_W { w: self }
    }
    #[doc = "Bit 15 - This bit is 1 if power is supplied to SRAM domain PD_GRP4"]
    #[inline(always)]
    pub fn pd_grp4_sram(&mut self) -> PD_GRP4_SRAM_W {
        PD_GRP4_SRAM_W { w: self }
    }
    #[doc = "Bit 14 - This bit is 1 if power is supplied to SRAM domain PD_GRP3"]
    #[inline(always)]
    pub fn pd_grp3_sram(&mut self) -> PD_GRP3_SRAM_W {
        PD_GRP3_SRAM_W { w: self }
    }
    #[doc = "Bit 13 - This bit is 1 if power is supplied to SRAM domain PD_GRP2"]
    #[inline(always)]
    pub fn pd_grp2_sram(&mut self) -> PD_GRP2_SRAM_W {
        PD_GRP2_SRAM_W { w: self }
    }
    #[doc = "Bit 12 - This bit is 1 if power is supplied to SRAM domain PD_GRP1"]
    #[inline(always)]
    pub fn pd_grp1_sram(&mut self) -> PD_GRP1_SRAM_W {
        PD_GRP1_SRAM_W { w: self }
    }
    #[doc = "Bit 11 - This bit is 1 if power is supplied to SRAM domain PD_SRAM0_3"]
    #[inline(always)]
    pub fn pd_grp0_sram3(&mut self) -> PD_GRP0_SRAM3_W {
        PD_GRP0_SRAM3_W { w: self }
    }
    #[doc = "Bit 10 - This bit is 1 if power is supplied to SRAM domain PD_SRAM0_2"]
    #[inline(always)]
    pub fn pd_grp0_sram2(&mut self) -> PD_GRP0_SRAM2_W {
        PD_GRP0_SRAM2_W { w: self }
    }
    #[doc = "Bit 9 - This bit is 1 if power is supplied to SRAM domain SRAM0_1"]
    #[inline(always)]
    pub fn pd_grp0_sram1(&mut self) -> PD_GRP0_SRAM1_W {
        PD_GRP0_SRAM1_W { w: self }
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to SRAM domain SRAM0_0"]
    #[inline(always)]
    pub fn pd_grp0_sram0(&mut self) -> PD_GRP0_SRAM0_W {
        PD_GRP0_SRAM0_W { w: self }
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to domain PD_ADC"]
    #[inline(always)]
    pub fn pdadc(&mut self) -> PDADC_W {
        PDADC_W { w: self }
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to domain PD_FLAM1"]
    #[inline(always)]
    pub fn pd_flam1(&mut self) -> PD_FLAM1_W {
        PD_FLAM1_W { w: self }
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to domain PD_FLAM0"]
    #[inline(always)]
    pub fn pd_flam0(&mut self) -> PD_FLAM0_W {
        PD_FLAM0_W { w: self }
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to domain PD_PDM"]
    #[inline(always)]
    pub fn pd_pdm(&mut self) -> PD_PDM_W {
        PD_PDM_W { w: self }
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to power domain C, which supplies IOM3-5."]
    #[inline(always)]
    pub fn pdc(&mut self) -> PDC_W {
        PDC_W { w: self }
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to power domain B, which supplies IOM0-2."]
    #[inline(always)]
    pub fn pdb(&mut self) -> PDB_W {
        PDB_W { w: self }
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to power domain A, which supplies IOS and UART0,1."]
    #[inline(always)]
    pub fn pda(&mut self) -> PDA_W {
        PDA_W { w: self }
    }
}
