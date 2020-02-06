#[doc = "Reader of register CLOCKEN2"]
pub type R = crate::R<u32, super::CLOCKEN2>;
#[doc = "Writer for register CLOCKEN2"]
pub type W = crate::W<u32, super::CLOCKEN2>;
#[doc = "Register CLOCKEN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCKEN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock enable status 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLOCKEN2_A {
    #[doc = "1: Clock enable for the UART1_HCLK."]
    UART1_HCLK_CLKEN = 1,
    #[doc = "2: Clock enable for the UART1HF."]
    UART1HF_CLKEN = 2,
    #[doc = "4: Clock enable for the WDT."]
    WDT_CLKEN = 4,
    #[doc = "1073741824: Clock enable for the XT_32KHz."]
    XT_32KHZ_EN = 1073741824,
    #[doc = "2147483648: Force HFRC On Status."]
    FRCHFRC = 2147483648,
}
impl From<CLOCKEN2_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKEN2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLOCKEN2`"]
pub type CLOCKEN2_R = crate::R<u32, CLOCKEN2_A>;
impl CLOCKEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLOCKEN2_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CLOCKEN2_A::UART1_HCLK_CLKEN),
            2 => Val(CLOCKEN2_A::UART1HF_CLKEN),
            4 => Val(CLOCKEN2_A::WDT_CLKEN),
            1073741824 => Val(CLOCKEN2_A::XT_32KHZ_EN),
            2147483648 => Val(CLOCKEN2_A::FRCHFRC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART1_HCLK_CLKEN`"]
    #[inline(always)]
    pub fn is_uart1_hclk_clken(&self) -> bool {
        *self == CLOCKEN2_A::UART1_HCLK_CLKEN
    }
    #[doc = "Checks if the value of the field is `UART1HF_CLKEN`"]
    #[inline(always)]
    pub fn is_uart1hf_clken(&self) -> bool {
        *self == CLOCKEN2_A::UART1HF_CLKEN
    }
    #[doc = "Checks if the value of the field is `WDT_CLKEN`"]
    #[inline(always)]
    pub fn is_wdt_clken(&self) -> bool {
        *self == CLOCKEN2_A::WDT_CLKEN
    }
    #[doc = "Checks if the value of the field is `XT_32KHZ_EN`"]
    #[inline(always)]
    pub fn is_xt_32khz_en(&self) -> bool {
        *self == CLOCKEN2_A::XT_32KHZ_EN
    }
    #[doc = "Checks if the value of the field is `FRCHFRC`"]
    #[inline(always)]
    pub fn is_frchfrc(&self) -> bool {
        *self == CLOCKEN2_A::FRCHFRC
    }
}
#[doc = "Write proxy for field `CLOCKEN2`"]
pub struct CLOCKEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKEN2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock enable for the UART1_HCLK."]
    #[inline(always)]
    pub fn uart1_hclk_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2_A::UART1_HCLK_CLKEN)
    }
    #[doc = "Clock enable for the UART1HF."]
    #[inline(always)]
    pub fn uart1hf_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2_A::UART1HF_CLKEN)
    }
    #[doc = "Clock enable for the WDT."]
    #[inline(always)]
    pub fn wdt_clken(self) -> &'a mut W {
        self.variant(CLOCKEN2_A::WDT_CLKEN)
    }
    #[doc = "Clock enable for the XT_32KHz."]
    #[inline(always)]
    pub fn xt_32khz_en(self) -> &'a mut W {
        self.variant(CLOCKEN2_A::XT_32KHZ_EN)
    }
    #[doc = "Force HFRC On Status."]
    #[inline(always)]
    pub fn frchfrc(self) -> &'a mut W {
        self.variant(CLOCKEN2_A::FRCHFRC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status 2"]
    #[inline(always)]
    pub fn clocken2(&self) -> CLOCKEN2_R {
        CLOCKEN2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status 2"]
    #[inline(always)]
    pub fn clocken2(&mut self) -> CLOCKEN2_W {
        CLOCKEN2_W { w: self }
    }
}
