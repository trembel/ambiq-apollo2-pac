#[doc = "Reader of register UARTEN"]
pub type R = crate::R<u32, super::UARTEN>;
#[doc = "Writer for register UARTEN"]
pub type W = crate::W<u32, super::UARTEN>;
#[doc = "Register UARTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART1 system clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART1EN_A {
    #[doc = "0: Disable the UART1 system clock"]
    DIS = 0,
    #[doc = "1: Enable the UART1 system clock"]
    EN = 1,
    #[doc = "2: Run UART_Hclk at the same frequency as UART_hfclk"]
    REDUCE_FREQ = 2,
    #[doc = "3: Enable UART_hclk to reduce to UART_hfclk at low power mode"]
    EN_POWER_SAV = 3,
}
impl From<UART1EN_A> for u8 {
    #[inline(always)]
    fn from(variant: UART1EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART1EN`"]
pub type UART1EN_R = crate::R<u8, UART1EN_A>;
impl UART1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1EN_A {
        match self.bits {
            0 => UART1EN_A::DIS,
            1 => UART1EN_A::EN,
            2 => UART1EN_A::REDUCE_FREQ,
            3 => UART1EN_A::EN_POWER_SAV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART1EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART1EN_A::EN
    }
    #[doc = "Checks if the value of the field is `REDUCE_FREQ`"]
    #[inline(always)]
    pub fn is_reduce_freq(&self) -> bool {
        *self == UART1EN_A::REDUCE_FREQ
    }
    #[doc = "Checks if the value of the field is `EN_POWER_SAV`"]
    #[inline(always)]
    pub fn is_en_power_sav(&self) -> bool {
        *self == UART1EN_A::EN_POWER_SAV
    }
}
#[doc = "Write proxy for field `UART1EN`"]
pub struct UART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1EN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable the UART1 system clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART1EN_A::DIS)
    }
    #[doc = "Enable the UART1 system clock"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART1EN_A::EN)
    }
    #[doc = "Run UART_Hclk at the same frequency as UART_hfclk"]
    #[inline(always)]
    pub fn reduce_freq(self) -> &'a mut W {
        self.variant(UART1EN_A::REDUCE_FREQ)
    }
    #[doc = "Enable UART_hclk to reduce to UART_hfclk at low power mode"]
    #[inline(always)]
    pub fn en_power_sav(self) -> &'a mut W {
        self.variant(UART1EN_A::EN_POWER_SAV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "UART0 system clock control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART0EN_A {
    #[doc = "0: Disable the UART0 system clock"]
    DIS = 0,
    #[doc = "1: Enable the UART0 system clock"]
    EN = 1,
    #[doc = "2: Run UART_Hclk at the same frequency as UART_hfclk"]
    REDUCE_FREQ = 2,
    #[doc = "3: Enable UART_hclk to reduce to UART_hfclk at low power mode"]
    EN_POWER_SAV = 3,
}
impl From<UART0EN_A> for u8 {
    #[inline(always)]
    fn from(variant: UART0EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART0EN`"]
pub type UART0EN_R = crate::R<u8, UART0EN_A>;
impl UART0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0EN_A {
        match self.bits {
            0 => UART0EN_A::DIS,
            1 => UART0EN_A::EN,
            2 => UART0EN_A::REDUCE_FREQ,
            3 => UART0EN_A::EN_POWER_SAV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UART0EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UART0EN_A::EN
    }
    #[doc = "Checks if the value of the field is `REDUCE_FREQ`"]
    #[inline(always)]
    pub fn is_reduce_freq(&self) -> bool {
        *self == UART0EN_A::REDUCE_FREQ
    }
    #[doc = "Checks if the value of the field is `EN_POWER_SAV`"]
    #[inline(always)]
    pub fn is_en_power_sav(&self) -> bool {
        *self == UART0EN_A::EN_POWER_SAV
    }
}
#[doc = "Write proxy for field `UART0EN`"]
pub struct UART0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0EN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable the UART0 system clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(UART0EN_A::DIS)
    }
    #[doc = "Enable the UART0 system clock"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(UART0EN_A::EN)
    }
    #[doc = "Run UART_Hclk at the same frequency as UART_hfclk"]
    #[inline(always)]
    pub fn reduce_freq(self) -> &'a mut W {
        self.variant(UART0EN_A::REDUCE_FREQ)
    }
    #[doc = "Enable UART_hclk to reduce to UART_hfclk at low power mode"]
    #[inline(always)]
    pub fn en_power_sav(self) -> &'a mut W {
        self.variant(UART0EN_A::EN_POWER_SAV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - UART1 system clock control"]
    #[inline(always)]
    pub fn uart1en(&self) -> UART1EN_R {
        UART1EN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - UART0 system clock control"]
    #[inline(always)]
    pub fn uart0en(&self) -> UART0EN_R {
        UART0EN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - UART1 system clock control"]
    #[inline(always)]
    pub fn uart1en(&mut self) -> UART1EN_W {
        UART1EN_W { w: self }
    }
    #[doc = "Bits 0:1 - UART0 system clock control"]
    #[inline(always)]
    pub fn uart0en(&mut self) -> UART0EN_W {
        UART0EN_W { w: self }
    }
}
