#[doc = "Reader of register CLOCKEN3"]
pub type R = crate::R<u32, super::CLOCKEN3>;
#[doc = "Writer for register CLOCKEN3"]
pub type W = crate::W<u32, super::CLOCKEN3>;
#[doc = "Register CLOCKEN3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCKEN3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock enable status 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLOCKEN3_A {
    #[doc = "16777216: At least 1 peripherial is requesting for XTAL Clock"]
    PERIPH_ALL_XTAL_EN = 16777216,
    #[doc = "33554432: At least 1 peripherial is requesting for HFRC Clock"]
    PERIPH_ALL_HFRC_EN = 33554432,
    #[doc = "67108864: HFRC Adjust Enable Status"]
    HFADJEN = 67108864,
    #[doc = "134217728: HFRC is enabled during adjustment status"]
    HFRC_EN_OUT = 134217728,
    #[doc = "268435456: Selects the RTC oscillator (0 => LFRC, 1 => XT)"]
    RTC_SOURCE = 268435456,
    #[doc = "536870912: XT is enabled Status"]
    XTAL_EN = 536870912,
    #[doc = "1073741824: HFRC is enabled Status"]
    HFRC_EN = 1073741824,
    #[doc = "2147483648: Flash Clock is enabled Status"]
    FLASHCLK_EN = 2147483648,
}
impl From<CLOCKEN3_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKEN3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLOCKEN3`"]
pub type CLOCKEN3_R = crate::R<u32, CLOCKEN3_A>;
impl CLOCKEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLOCKEN3_A> {
        use crate::Variant::*;
        match self.bits {
            16777216 => Val(CLOCKEN3_A::PERIPH_ALL_XTAL_EN),
            33554432 => Val(CLOCKEN3_A::PERIPH_ALL_HFRC_EN),
            67108864 => Val(CLOCKEN3_A::HFADJEN),
            134217728 => Val(CLOCKEN3_A::HFRC_EN_OUT),
            268435456 => Val(CLOCKEN3_A::RTC_SOURCE),
            536870912 => Val(CLOCKEN3_A::XTAL_EN),
            1073741824 => Val(CLOCKEN3_A::HFRC_EN),
            2147483648 => Val(CLOCKEN3_A::FLASHCLK_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_ALL_XTAL_EN`"]
    #[inline(always)]
    pub fn is_periph_all_xtal_en(&self) -> bool {
        *self == CLOCKEN3_A::PERIPH_ALL_XTAL_EN
    }
    #[doc = "Checks if the value of the field is `PERIPH_ALL_HFRC_EN`"]
    #[inline(always)]
    pub fn is_periph_all_hfrc_en(&self) -> bool {
        *self == CLOCKEN3_A::PERIPH_ALL_HFRC_EN
    }
    #[doc = "Checks if the value of the field is `HFADJEN`"]
    #[inline(always)]
    pub fn is_hfadjen(&self) -> bool {
        *self == CLOCKEN3_A::HFADJEN
    }
    #[doc = "Checks if the value of the field is `HFRC_EN_OUT`"]
    #[inline(always)]
    pub fn is_hfrc_en_out(&self) -> bool {
        *self == CLOCKEN3_A::HFRC_EN_OUT
    }
    #[doc = "Checks if the value of the field is `RTC_SOURCE`"]
    #[inline(always)]
    pub fn is_rtc_source(&self) -> bool {
        *self == CLOCKEN3_A::RTC_SOURCE
    }
    #[doc = "Checks if the value of the field is `XTAL_EN`"]
    #[inline(always)]
    pub fn is_xtal_en(&self) -> bool {
        *self == CLOCKEN3_A::XTAL_EN
    }
    #[doc = "Checks if the value of the field is `HFRC_EN`"]
    #[inline(always)]
    pub fn is_hfrc_en(&self) -> bool {
        *self == CLOCKEN3_A::HFRC_EN
    }
    #[doc = "Checks if the value of the field is `FLASHCLK_EN`"]
    #[inline(always)]
    pub fn is_flashclk_en(&self) -> bool {
        *self == CLOCKEN3_A::FLASHCLK_EN
    }
}
#[doc = "Write proxy for field `CLOCKEN3`"]
pub struct CLOCKEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKEN3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "At least 1 peripherial is requesting for XTAL Clock"]
    #[inline(always)]
    pub fn periph_all_xtal_en(self) -> &'a mut W {
        self.variant(CLOCKEN3_A::PERIPH_ALL_XTAL_EN)
    }
    #[doc = "At least 1 peripherial is requesting for HFRC Clock"]
    #[inline(always)]
    pub fn periph_all_hfrc_en(self) -> &'a mut W {
        self.variant(CLOCKEN3_A::PERIPH_ALL_HFRC_EN)
    }
    #[doc = "HFRC Adjust Enable Status"]
    #[inline(always)]
    pub fn hfadjen(self) -> &'a mut W {
        self.variant(CLOCKEN3_A::HFADJEN)
    }
    #[doc = "HFRC is enabled during adjustment status"]
    #[inline(always)]
    pub fn hfrc_en_out(self) -> &'a mut W {
        self.variant(CLOCKEN3_A::HFRC_EN_OUT)
    }
    #[doc = "Selects the RTC oscillator (0 => LFRC, 1 => XT)"]
    #[inline(always)]
    pub fn rtc_source(self) -> &'a mut W {
        self.variant(CLOCKEN3_A::RTC_SOURCE)
    }
    #[doc = "XT is enabled Status"]
    #[inline(always)]
    pub fn xtal_en(self) -> &'a mut W {
        self.variant(CLOCKEN3_A::XTAL_EN)
    }
    #[doc = "HFRC is enabled Status"]
    #[inline(always)]
    pub fn hfrc_en(self) -> &'a mut W {
        self.variant(CLOCKEN3_A::HFRC_EN)
    }
    #[doc = "Flash Clock is enabled Status"]
    #[inline(always)]
    pub fn flashclk_en(self) -> &'a mut W {
        self.variant(CLOCKEN3_A::FLASHCLK_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline(always)]
    pub fn clocken3(&self) -> CLOCKEN3_R {
        CLOCKEN3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline(always)]
    pub fn clocken3(&mut self) -> CLOCKEN3_W {
        CLOCKEN3_W { w: self }
    }
}
