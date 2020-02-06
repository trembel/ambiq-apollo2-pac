#[doc = "Reader of register CLOCKEN"]
pub type R = crate::R<u32, super::CLOCKEN>;
#[doc = "Writer for register CLOCKEN"]
pub type W = crate::W<u32, super::CLOCKEN>;
#[doc = "Register CLOCKEN `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCKEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock enable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLOCKEN_A {
    #[doc = "1: Clock enable for the ADC."]
    ADC_CLKEN = 1,
    #[doc = "2: Clock enable for the CTIMER."]
    CTIMER_CLKEN = 2,
    #[doc = "4: Clock enable for the CTIMER0A."]
    CTIMER0A_CLKEN = 4,
    #[doc = "8: Clock enable for the CTIMER0B."]
    CTIMER0B_CLKEN = 8,
    #[doc = "16: Clock enable for the CTIMER1A."]
    CTIMER1A_CLKEN = 16,
    #[doc = "32: Clock enable for the CTIMER1B."]
    CTIMER1B_CLKEN = 32,
    #[doc = "64: Clock enable for the CTIMER2A."]
    CTIMER2A_CLKEN = 64,
    #[doc = "128: Clock enable for the CTIMER2B."]
    CTIMER2B_CLKEN = 128,
    #[doc = "256: Clock enable for the CTIMER3A."]
    CTIMER3A_CLKEN = 256,
    #[doc = "512: Clock enable for the CTIMER3B."]
    CTIMER3B_CLKEN = 512,
    #[doc = "1024: Clock enable for the IO Master 0."]
    IOMSTR0_CLKEN = 1024,
    #[doc = "2048: Clock enable for the IO Master 1."]
    IOMSTR1_CLKEN = 2048,
    #[doc = "4096: Clock enable for the IO Master 2."]
    IOMSTR2_CLKEN = 4096,
    #[doc = "8192: Clock enable for the IO Master 3."]
    IOMSTR3_CLKEN = 8192,
    #[doc = "16384: Clock enable for the IO Master 4."]
    IOMSTR4_CLKEN = 16384,
    #[doc = "32768: Clock enable for the IO Master 5."]
    IOMSTR5_CLKEN = 32768,
    #[doc = "65536: Clock enable for the IO Master IFC0."]
    IOMSTRIFC0_CLKEN = 65536,
    #[doc = "131072: Clock enable for the IO Master IFC1."]
    IOMSTRIFC1_CLKEN = 131072,
    #[doc = "262144: Clock enable for the IO Master IFC2."]
    IOMSTRIFC2_CLKEN = 262144,
    #[doc = "524288: Clock enable for the IO Master IFC3."]
    IOMSTRIFC3_CLKEN = 524288,
    #[doc = "1048576: Clock enable for the IO Master IFC4."]
    IOMSTRIFC4_CLKEN = 1048576,
    #[doc = "2097152: Clock enable for the IO Master IFC5."]
    IOMSTRIFC5_CLKEN = 2097152,
    #[doc = "4194304: Clock enable for the IO Slave."]
    IOSLAVE_CLKEN = 4194304,
    #[doc = "8388608: Clock enable for the PDM."]
    PDM_CLKEN = 8388608,
    #[doc = "16777216: Clock enable for the PDM IFC."]
    PDMIFC_CLKEN = 16777216,
    #[doc = "33554432: Clock enable for the RSTGEN."]
    RSTGEN_CLKEN = 33554432,
    #[doc = "67108864: Clock enable for the SRAM_WIPE."]
    SRAM_WIPE_CLKEN = 67108864,
    #[doc = "134217728: Clock enable for the STIMER."]
    STIMER_CLKEN = 134217728,
    #[doc = "268435456: Clock enable for the STIMER_CNT."]
    STIMER_CNT_CLKEN = 268435456,
    #[doc = "536870912: Clock enable for the TPIU."]
    TPIU_CLKEN = 536870912,
    #[doc = "1073741824: Clock enable for the UART0_HCLK."]
    UART0_HCLK_CLKEN = 1073741824,
    #[doc = "2147483648: Clock enable for the UART0HF."]
    UART0HF_CLKEN = 2147483648,
}
impl From<CLOCKEN_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLOCKEN`"]
pub type CLOCKEN_R = crate::R<u32, CLOCKEN_A>;
impl CLOCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLOCKEN_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CLOCKEN_A::ADC_CLKEN),
            2 => Val(CLOCKEN_A::CTIMER_CLKEN),
            4 => Val(CLOCKEN_A::CTIMER0A_CLKEN),
            8 => Val(CLOCKEN_A::CTIMER0B_CLKEN),
            16 => Val(CLOCKEN_A::CTIMER1A_CLKEN),
            32 => Val(CLOCKEN_A::CTIMER1B_CLKEN),
            64 => Val(CLOCKEN_A::CTIMER2A_CLKEN),
            128 => Val(CLOCKEN_A::CTIMER2B_CLKEN),
            256 => Val(CLOCKEN_A::CTIMER3A_CLKEN),
            512 => Val(CLOCKEN_A::CTIMER3B_CLKEN),
            1024 => Val(CLOCKEN_A::IOMSTR0_CLKEN),
            2048 => Val(CLOCKEN_A::IOMSTR1_CLKEN),
            4096 => Val(CLOCKEN_A::IOMSTR2_CLKEN),
            8192 => Val(CLOCKEN_A::IOMSTR3_CLKEN),
            16384 => Val(CLOCKEN_A::IOMSTR4_CLKEN),
            32768 => Val(CLOCKEN_A::IOMSTR5_CLKEN),
            65536 => Val(CLOCKEN_A::IOMSTRIFC0_CLKEN),
            131072 => Val(CLOCKEN_A::IOMSTRIFC1_CLKEN),
            262144 => Val(CLOCKEN_A::IOMSTRIFC2_CLKEN),
            524288 => Val(CLOCKEN_A::IOMSTRIFC3_CLKEN),
            1048576 => Val(CLOCKEN_A::IOMSTRIFC4_CLKEN),
            2097152 => Val(CLOCKEN_A::IOMSTRIFC5_CLKEN),
            4194304 => Val(CLOCKEN_A::IOSLAVE_CLKEN),
            8388608 => Val(CLOCKEN_A::PDM_CLKEN),
            16777216 => Val(CLOCKEN_A::PDMIFC_CLKEN),
            33554432 => Val(CLOCKEN_A::RSTGEN_CLKEN),
            67108864 => Val(CLOCKEN_A::SRAM_WIPE_CLKEN),
            134217728 => Val(CLOCKEN_A::STIMER_CLKEN),
            268435456 => Val(CLOCKEN_A::STIMER_CNT_CLKEN),
            536870912 => Val(CLOCKEN_A::TPIU_CLKEN),
            1073741824 => Val(CLOCKEN_A::UART0_HCLK_CLKEN),
            2147483648 => Val(CLOCKEN_A::UART0HF_CLKEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_CLKEN`"]
    #[inline(always)]
    pub fn is_adc_clken(&self) -> bool {
        *self == CLOCKEN_A::ADC_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER0A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer0a_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER0A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER0B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer0b_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER0B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER1A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer1a_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER1A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER1B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer1b_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER1B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER2A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer2a_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER2A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER2B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer2b_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER2B_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER3A_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer3a_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER3A_CLKEN
    }
    #[doc = "Checks if the value of the field is `CTIMER3B_CLKEN`"]
    #[inline(always)]
    pub fn is_ctimer3b_clken(&self) -> bool {
        *self == CLOCKEN_A::CTIMER3B_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTR0_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstr0_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTR0_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTR1_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstr1_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTR1_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTR2_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstr2_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTR2_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTR3_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstr3_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTR3_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTR4_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstr4_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTR4_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTR5_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstr5_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTR5_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC0_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc0_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTRIFC0_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC1_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc1_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTRIFC1_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC2_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc2_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTRIFC2_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC3_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc3_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTRIFC3_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC4_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc4_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTRIFC4_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOMSTRIFC5_CLKEN`"]
    #[inline(always)]
    pub fn is_iomstrifc5_clken(&self) -> bool {
        *self == CLOCKEN_A::IOMSTRIFC5_CLKEN
    }
    #[doc = "Checks if the value of the field is `IOSLAVE_CLKEN`"]
    #[inline(always)]
    pub fn is_ioslave_clken(&self) -> bool {
        *self == CLOCKEN_A::IOSLAVE_CLKEN
    }
    #[doc = "Checks if the value of the field is `PDM_CLKEN`"]
    #[inline(always)]
    pub fn is_pdm_clken(&self) -> bool {
        *self == CLOCKEN_A::PDM_CLKEN
    }
    #[doc = "Checks if the value of the field is `PDMIFC_CLKEN`"]
    #[inline(always)]
    pub fn is_pdmifc_clken(&self) -> bool {
        *self == CLOCKEN_A::PDMIFC_CLKEN
    }
    #[doc = "Checks if the value of the field is `RSTGEN_CLKEN`"]
    #[inline(always)]
    pub fn is_rstgen_clken(&self) -> bool {
        *self == CLOCKEN_A::RSTGEN_CLKEN
    }
    #[doc = "Checks if the value of the field is `SRAM_WIPE_CLKEN`"]
    #[inline(always)]
    pub fn is_sram_wipe_clken(&self) -> bool {
        *self == CLOCKEN_A::SRAM_WIPE_CLKEN
    }
    #[doc = "Checks if the value of the field is `STIMER_CLKEN`"]
    #[inline(always)]
    pub fn is_stimer_clken(&self) -> bool {
        *self == CLOCKEN_A::STIMER_CLKEN
    }
    #[doc = "Checks if the value of the field is `STIMER_CNT_CLKEN`"]
    #[inline(always)]
    pub fn is_stimer_cnt_clken(&self) -> bool {
        *self == CLOCKEN_A::STIMER_CNT_CLKEN
    }
    #[doc = "Checks if the value of the field is `TPIU_CLKEN`"]
    #[inline(always)]
    pub fn is_tpiu_clken(&self) -> bool {
        *self == CLOCKEN_A::TPIU_CLKEN
    }
    #[doc = "Checks if the value of the field is `UART0_HCLK_CLKEN`"]
    #[inline(always)]
    pub fn is_uart0_hclk_clken(&self) -> bool {
        *self == CLOCKEN_A::UART0_HCLK_CLKEN
    }
    #[doc = "Checks if the value of the field is `UART0HF_CLKEN`"]
    #[inline(always)]
    pub fn is_uart0hf_clken(&self) -> bool {
        *self == CLOCKEN_A::UART0HF_CLKEN
    }
}
#[doc = "Write proxy for field `CLOCKEN`"]
pub struct CLOCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock enable for the ADC."]
    #[inline(always)]
    pub fn adc_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::ADC_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER."]
    #[inline(always)]
    pub fn ctimer_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER0A."]
    #[inline(always)]
    pub fn ctimer0a_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER0A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER0B."]
    #[inline(always)]
    pub fn ctimer0b_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER0B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER1A."]
    #[inline(always)]
    pub fn ctimer1a_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER1A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER1B."]
    #[inline(always)]
    pub fn ctimer1b_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER1B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER2A."]
    #[inline(always)]
    pub fn ctimer2a_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER2A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER2B."]
    #[inline(always)]
    pub fn ctimer2b_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER2B_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER3A."]
    #[inline(always)]
    pub fn ctimer3a_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER3A_CLKEN)
    }
    #[doc = "Clock enable for the CTIMER3B."]
    #[inline(always)]
    pub fn ctimer3b_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::CTIMER3B_CLKEN)
    }
    #[doc = "Clock enable for the IO Master 0."]
    #[inline(always)]
    pub fn iomstr0_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTR0_CLKEN)
    }
    #[doc = "Clock enable for the IO Master 1."]
    #[inline(always)]
    pub fn iomstr1_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTR1_CLKEN)
    }
    #[doc = "Clock enable for the IO Master 2."]
    #[inline(always)]
    pub fn iomstr2_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTR2_CLKEN)
    }
    #[doc = "Clock enable for the IO Master 3."]
    #[inline(always)]
    pub fn iomstr3_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTR3_CLKEN)
    }
    #[doc = "Clock enable for the IO Master 4."]
    #[inline(always)]
    pub fn iomstr4_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTR4_CLKEN)
    }
    #[doc = "Clock enable for the IO Master 5."]
    #[inline(always)]
    pub fn iomstr5_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTR5_CLKEN)
    }
    #[doc = "Clock enable for the IO Master IFC0."]
    #[inline(always)]
    pub fn iomstrifc0_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTRIFC0_CLKEN)
    }
    #[doc = "Clock enable for the IO Master IFC1."]
    #[inline(always)]
    pub fn iomstrifc1_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTRIFC1_CLKEN)
    }
    #[doc = "Clock enable for the IO Master IFC2."]
    #[inline(always)]
    pub fn iomstrifc2_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTRIFC2_CLKEN)
    }
    #[doc = "Clock enable for the IO Master IFC3."]
    #[inline(always)]
    pub fn iomstrifc3_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTRIFC3_CLKEN)
    }
    #[doc = "Clock enable for the IO Master IFC4."]
    #[inline(always)]
    pub fn iomstrifc4_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTRIFC4_CLKEN)
    }
    #[doc = "Clock enable for the IO Master IFC5."]
    #[inline(always)]
    pub fn iomstrifc5_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOMSTRIFC5_CLKEN)
    }
    #[doc = "Clock enable for the IO Slave."]
    #[inline(always)]
    pub fn ioslave_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::IOSLAVE_CLKEN)
    }
    #[doc = "Clock enable for the PDM."]
    #[inline(always)]
    pub fn pdm_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::PDM_CLKEN)
    }
    #[doc = "Clock enable for the PDM IFC."]
    #[inline(always)]
    pub fn pdmifc_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::PDMIFC_CLKEN)
    }
    #[doc = "Clock enable for the RSTGEN."]
    #[inline(always)]
    pub fn rstgen_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::RSTGEN_CLKEN)
    }
    #[doc = "Clock enable for the SRAM_WIPE."]
    #[inline(always)]
    pub fn sram_wipe_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::SRAM_WIPE_CLKEN)
    }
    #[doc = "Clock enable for the STIMER."]
    #[inline(always)]
    pub fn stimer_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::STIMER_CLKEN)
    }
    #[doc = "Clock enable for the STIMER_CNT."]
    #[inline(always)]
    pub fn stimer_cnt_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::STIMER_CNT_CLKEN)
    }
    #[doc = "Clock enable for the TPIU."]
    #[inline(always)]
    pub fn tpiu_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::TPIU_CLKEN)
    }
    #[doc = "Clock enable for the UART0_HCLK."]
    #[inline(always)]
    pub fn uart0_hclk_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::UART0_HCLK_CLKEN)
    }
    #[doc = "Clock enable for the UART0HF."]
    #[inline(always)]
    pub fn uart0hf_clken(self) -> &'a mut W {
        self.variant(CLOCKEN_A::UART0HF_CLKEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status"]
    #[inline(always)]
    pub fn clocken(&self) -> CLOCKEN_R {
        CLOCKEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status"]
    #[inline(always)]
    pub fn clocken(&mut self) -> CLOCKEN_W {
        CLOCKEN_W { w: self }
    }
}
