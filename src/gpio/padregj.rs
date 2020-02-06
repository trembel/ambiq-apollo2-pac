#[doc = "Reader of register PADREGJ"]
pub type R = crate::R<u32, super::PADREGJ>;
#[doc = "Writer for register PADREGJ"]
pub type W = crate::W<u32, super::PADREGJ>;
#[doc = "Register PADREGJ `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGJ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 39 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD39RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD39RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD39RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD39RSEL`"]
pub type PAD39RSEL_R = crate::R<u8, PAD39RSEL_A>;
impl PAD39RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD39RSEL_A {
        match self.bits {
            0 => PAD39RSEL_A::PULL1_5K,
            1 => PAD39RSEL_A::PULL6K,
            2 => PAD39RSEL_A::PULL12K,
            3 => PAD39RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD39RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD39RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD39RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD39RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD39RSEL`"]
pub struct PAD39RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD39RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD39RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD39RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD39RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD39RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Pad 39 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD39FNCSEL_A {
    #[doc = "0: Configure as the UART0 TX Signal"]
    UART0TX = 0,
    #[doc = "1: Configure as the UART1 TX signal"]
    UART1TX = 1,
    #[doc = "2: Configure as the CLKOUT signal"]
    CLKOUT = 2,
    #[doc = "3: Configure as GPIO39"]
    GPIO39 = 3,
    #[doc = "4: Configure as the IOMSTR4 I2C SCL signal"]
    M4SCL = 4,
    #[doc = "5: Configure as the IOMSTR4 SPI SCK signal"]
    M4SCK = 5,
    #[doc = "6: Configure as the IOMSTR4 SPI SCK loopback signal from IOSLAVE"]
    M4SCKLB = 6,
    #[doc = "7: Configure as the IOMSTR4 I2C SCL loopback signal from IOSLAVE"]
    M4SCLLB = 7,
}
impl From<PAD39FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD39FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD39FNCSEL`"]
pub type PAD39FNCSEL_R = crate::R<u8, PAD39FNCSEL_A>;
impl PAD39FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD39FNCSEL_A {
        match self.bits {
            0 => PAD39FNCSEL_A::UART0TX,
            1 => PAD39FNCSEL_A::UART1TX,
            2 => PAD39FNCSEL_A::CLKOUT,
            3 => PAD39FNCSEL_A::GPIO39,
            4 => PAD39FNCSEL_A::M4SCL,
            5 => PAD39FNCSEL_A::M4SCK,
            6 => PAD39FNCSEL_A::M4SCKLB,
            7 => PAD39FNCSEL_A::M4SCLLB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD39FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD39FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == PAD39FNCSEL_A::CLKOUT
    }
    #[doc = "Checks if the value of the field is `GPIO39`"]
    #[inline(always)]
    pub fn is_gpio39(&self) -> bool {
        *self == PAD39FNCSEL_A::GPIO39
    }
    #[doc = "Checks if the value of the field is `M4SCL`"]
    #[inline(always)]
    pub fn is_m4scl(&self) -> bool {
        *self == PAD39FNCSEL_A::M4SCL
    }
    #[doc = "Checks if the value of the field is `M4SCK`"]
    #[inline(always)]
    pub fn is_m4sck(&self) -> bool {
        *self == PAD39FNCSEL_A::M4SCK
    }
    #[doc = "Checks if the value of the field is `M4SCKLB`"]
    #[inline(always)]
    pub fn is_m4scklb(&self) -> bool {
        *self == PAD39FNCSEL_A::M4SCKLB
    }
    #[doc = "Checks if the value of the field is `M4SCLLB`"]
    #[inline(always)]
    pub fn is_m4scllb(&self) -> bool {
        *self == PAD39FNCSEL_A::M4SCLLB
    }
}
#[doc = "Write proxy for field `PAD39FNCSEL`"]
pub struct PAD39FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD39FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the UART0 TX Signal"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD39FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART1 TX signal"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD39FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the CLKOUT signal"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD39FNCSEL_A::CLKOUT)
    }
    #[doc = "Configure as GPIO39"]
    #[inline(always)]
    pub fn gpio39(self) -> &'a mut W {
        self.variant(PAD39FNCSEL_A::GPIO39)
    }
    #[doc = "Configure as the IOMSTR4 I2C SCL signal"]
    #[inline(always)]
    pub fn m4scl(self) -> &'a mut W {
        self.variant(PAD39FNCSEL_A::M4SCL)
    }
    #[doc = "Configure as the IOMSTR4 SPI SCK signal"]
    #[inline(always)]
    pub fn m4sck(self) -> &'a mut W {
        self.variant(PAD39FNCSEL_A::M4SCK)
    }
    #[doc = "Configure as the IOMSTR4 SPI SCK loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn m4scklb(self) -> &'a mut W {
        self.variant(PAD39FNCSEL_A::M4SCKLB)
    }
    #[doc = "Configure as the IOMSTR4 I2C SCL loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn m4scllb(self) -> &'a mut W {
        self.variant(PAD39FNCSEL_A::M4SCLLB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 39 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD39STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD39STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD39STRNG`"]
pub type PAD39STRNG_R = crate::R<bool, PAD39STRNG_A>;
impl PAD39STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD39STRNG_A {
        match self.bits {
            false => PAD39STRNG_A::LOW,
            true => PAD39STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD39STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD39STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD39STRNG`"]
pub struct PAD39STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD39STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD39STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD39STRNG_A::HIGH)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Pad 39 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD39INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD39INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD39INPEN`"]
pub type PAD39INPEN_R = crate::R<bool, PAD39INPEN_A>;
impl PAD39INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD39INPEN_A {
        match self.bits {
            false => PAD39INPEN_A::DIS,
            true => PAD39INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD39INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD39INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD39INPEN`"]
pub struct PAD39INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD39INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD39INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD39INPEN_A::EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Pad 39 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD39PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD39PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD39PULL`"]
pub type PAD39PULL_R = crate::R<bool, PAD39PULL_A>;
impl PAD39PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD39PULL_A {
        match self.bits {
            false => PAD39PULL_A::DIS,
            true => PAD39PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD39PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD39PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD39PULL`"]
pub struct PAD39PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD39PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD39PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD39PULL_A::EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Pad 38 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD38FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 3 signal"]
    TRIG3 = 0,
    #[doc = "1: Configure as the SPI channel 3 nCE signal from IOMSTR1"]
    M1NCE3 = 1,
    #[doc = "2: Configure as the UART0 CTS signal"]
    UA0CTS = 2,
    #[doc = "3: Configure as GPIO38"]
    GPIO38 = 3,
    #[doc = "4: Configure as the IOSLAVE SPI 3-wire MOSI/MISO signal"]
    M3WIR3 = 4,
    #[doc = "5: Configure as the IOMSTR3 SPI MOSI output signal"]
    M3MOSI = 5,
    #[doc = "6: Configure as the SPI channel 7 nCE signal from IOMSTR4"]
    M4NCE7 = 6,
    #[doc = "7: Configure as the IOMSTR3 SPI 3-wire MOSI/MISO loopback signal from IOSLAVE"]
    SLWIR3LB = 7,
}
impl From<PAD38FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD38FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD38FNCSEL`"]
pub type PAD38FNCSEL_R = crate::R<u8, PAD38FNCSEL_A>;
impl PAD38FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD38FNCSEL_A {
        match self.bits {
            0 => PAD38FNCSEL_A::TRIG3,
            1 => PAD38FNCSEL_A::M1NCE3,
            2 => PAD38FNCSEL_A::UA0CTS,
            3 => PAD38FNCSEL_A::GPIO38,
            4 => PAD38FNCSEL_A::M3WIR3,
            5 => PAD38FNCSEL_A::M3MOSI,
            6 => PAD38FNCSEL_A::M4NCE7,
            7 => PAD38FNCSEL_A::SLWIR3LB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG3`"]
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == PAD38FNCSEL_A::TRIG3
    }
    #[doc = "Checks if the value of the field is `M1NCE3`"]
    #[inline(always)]
    pub fn is_m1n_ce3(&self) -> bool {
        *self == PAD38FNCSEL_A::M1NCE3
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD38FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `GPIO38`"]
    #[inline(always)]
    pub fn is_gpio38(&self) -> bool {
        *self == PAD38FNCSEL_A::GPIO38
    }
    #[doc = "Checks if the value of the field is `M3WIR3`"]
    #[inline(always)]
    pub fn is_m3wir3(&self) -> bool {
        *self == PAD38FNCSEL_A::M3WIR3
    }
    #[doc = "Checks if the value of the field is `M3MOSI`"]
    #[inline(always)]
    pub fn is_m3mosi(&self) -> bool {
        *self == PAD38FNCSEL_A::M3MOSI
    }
    #[doc = "Checks if the value of the field is `M4NCE7`"]
    #[inline(always)]
    pub fn is_m4n_ce7(&self) -> bool {
        *self == PAD38FNCSEL_A::M4NCE7
    }
    #[doc = "Checks if the value of the field is `SLWIR3LB`"]
    #[inline(always)]
    pub fn is_slwir3lb(&self) -> bool {
        *self == PAD38FNCSEL_A::SLWIR3LB
    }
}
#[doc = "Write proxy for field `PAD38FNCSEL`"]
pub struct PAD38FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD38FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD38FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 3 signal"]
    #[inline(always)]
    pub fn trig3(self) -> &'a mut W {
        self.variant(PAD38FNCSEL_A::TRIG3)
    }
    #[doc = "Configure as the SPI channel 3 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce3(self) -> &'a mut W {
        self.variant(PAD38FNCSEL_A::M1NCE3)
    }
    #[doc = "Configure as the UART0 CTS signal"]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD38FNCSEL_A::UA0CTS)
    }
    #[doc = "Configure as GPIO38"]
    #[inline(always)]
    pub fn gpio38(self) -> &'a mut W {
        self.variant(PAD38FNCSEL_A::GPIO38)
    }
    #[doc = "Configure as the IOSLAVE SPI 3-wire MOSI/MISO signal"]
    #[inline(always)]
    pub fn m3wir3(self) -> &'a mut W {
        self.variant(PAD38FNCSEL_A::M3WIR3)
    }
    #[doc = "Configure as the IOMSTR3 SPI MOSI output signal"]
    #[inline(always)]
    pub fn m3mosi(self) -> &'a mut W {
        self.variant(PAD38FNCSEL_A::M3MOSI)
    }
    #[doc = "Configure as the SPI channel 7 nCE signal from IOMSTR4"]
    #[inline(always)]
    pub fn m4n_ce7(self) -> &'a mut W {
        self.variant(PAD38FNCSEL_A::M4NCE7)
    }
    #[doc = "Configure as the IOMSTR3 SPI 3-wire MOSI/MISO loopback signal from IOSLAVE"]
    #[inline(always)]
    pub fn slwir3lb(self) -> &'a mut W {
        self.variant(PAD38FNCSEL_A::SLWIR3LB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 38 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD38STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD38STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD38STRNG`"]
pub type PAD38STRNG_R = crate::R<bool, PAD38STRNG_A>;
impl PAD38STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD38STRNG_A {
        match self.bits {
            false => PAD38STRNG_A::LOW,
            true => PAD38STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD38STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD38STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD38STRNG`"]
pub struct PAD38STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD38STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD38STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD38STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD38STRNG_A::HIGH)
    }
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
#[doc = "Pad 38 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD38INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD38INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD38INPEN`"]
pub type PAD38INPEN_R = crate::R<bool, PAD38INPEN_A>;
impl PAD38INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD38INPEN_A {
        match self.bits {
            false => PAD38INPEN_A::DIS,
            true => PAD38INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD38INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD38INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD38INPEN`"]
pub struct PAD38INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD38INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD38INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD38INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD38INPEN_A::EN)
    }
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
#[doc = "Pad 38 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD38PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD38PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD38PULL`"]
pub type PAD38PULL_R = crate::R<bool, PAD38PULL_A>;
impl PAD38PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD38PULL_A {
        match self.bits {
            false => PAD38PULL_A::DIS,
            true => PAD38PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD38PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD38PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD38PULL`"]
pub struct PAD38PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD38PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD38PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD38PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD38PULL_A::EN)
    }
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
#[doc = "Pad 37 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD37FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 2 signal"]
    TRIG2 = 0,
    #[doc = "1: Configure as the SPI channel 2 nCE signal from IOMSTR1"]
    M1NCE2 = 1,
    #[doc = "2: Configure as the UART0 RTS signal"]
    UA0RTS = 2,
    #[doc = "3: Configure as GPIO37"]
    GPIO37 = 3,
    #[doc = "4: Configure as the SPI channel 4 nCE signal from IOMSTR3"]
    M3NCE4 = 4,
    #[doc = "5: Configure as the SPI channel 1 nCE signal from IOMSTR4"]
    M4NCE1 = 5,
    #[doc = "6: Configure as the PDM CLK output signal"]
    PDM_CLK = 6,
    #[doc = "7: Configure as the input/output signal from CTIMER A1"]
    TCTA1 = 7,
}
impl From<PAD37FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD37FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD37FNCSEL`"]
pub type PAD37FNCSEL_R = crate::R<u8, PAD37FNCSEL_A>;
impl PAD37FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD37FNCSEL_A {
        match self.bits {
            0 => PAD37FNCSEL_A::TRIG2,
            1 => PAD37FNCSEL_A::M1NCE2,
            2 => PAD37FNCSEL_A::UA0RTS,
            3 => PAD37FNCSEL_A::GPIO37,
            4 => PAD37FNCSEL_A::M3NCE4,
            5 => PAD37FNCSEL_A::M4NCE1,
            6 => PAD37FNCSEL_A::PDM_CLK,
            7 => PAD37FNCSEL_A::TCTA1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2`"]
    #[inline(always)]
    pub fn is_trig2(&self) -> bool {
        *self == PAD37FNCSEL_A::TRIG2
    }
    #[doc = "Checks if the value of the field is `M1NCE2`"]
    #[inline(always)]
    pub fn is_m1n_ce2(&self) -> bool {
        *self == PAD37FNCSEL_A::M1NCE2
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD37FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `GPIO37`"]
    #[inline(always)]
    pub fn is_gpio37(&self) -> bool {
        *self == PAD37FNCSEL_A::GPIO37
    }
    #[doc = "Checks if the value of the field is `M3NCE4`"]
    #[inline(always)]
    pub fn is_m3n_ce4(&self) -> bool {
        *self == PAD37FNCSEL_A::M3NCE4
    }
    #[doc = "Checks if the value of the field is `M4NCE1`"]
    #[inline(always)]
    pub fn is_m4n_ce1(&self) -> bool {
        *self == PAD37FNCSEL_A::M4NCE1
    }
    #[doc = "Checks if the value of the field is `PDM_CLK`"]
    #[inline(always)]
    pub fn is_pdm_clk(&self) -> bool {
        *self == PAD37FNCSEL_A::PDM_CLK
    }
    #[doc = "Checks if the value of the field is `TCTA1`"]
    #[inline(always)]
    pub fn is_tcta1(&self) -> bool {
        *self == PAD37FNCSEL_A::TCTA1
    }
}
#[doc = "Write proxy for field `PAD37FNCSEL`"]
pub struct PAD37FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD37FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD37FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 2 signal"]
    #[inline(always)]
    pub fn trig2(self) -> &'a mut W {
        self.variant(PAD37FNCSEL_A::TRIG2)
    }
    #[doc = "Configure as the SPI channel 2 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce2(self) -> &'a mut W {
        self.variant(PAD37FNCSEL_A::M1NCE2)
    }
    #[doc = "Configure as the UART0 RTS signal"]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD37FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as GPIO37"]
    #[inline(always)]
    pub fn gpio37(self) -> &'a mut W {
        self.variant(PAD37FNCSEL_A::GPIO37)
    }
    #[doc = "Configure as the SPI channel 4 nCE signal from IOMSTR3"]
    #[inline(always)]
    pub fn m3n_ce4(self) -> &'a mut W {
        self.variant(PAD37FNCSEL_A::M3NCE4)
    }
    #[doc = "Configure as the SPI channel 1 nCE signal from IOMSTR4"]
    #[inline(always)]
    pub fn m4n_ce1(self) -> &'a mut W {
        self.variant(PAD37FNCSEL_A::M4NCE1)
    }
    #[doc = "Configure as the PDM CLK output signal"]
    #[inline(always)]
    pub fn pdm_clk(self) -> &'a mut W {
        self.variant(PAD37FNCSEL_A::PDM_CLK)
    }
    #[doc = "Configure as the input/output signal from CTIMER A1"]
    #[inline(always)]
    pub fn tcta1(self) -> &'a mut W {
        self.variant(PAD37FNCSEL_A::TCTA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 37 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD37STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD37STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD37STRNG`"]
pub type PAD37STRNG_R = crate::R<bool, PAD37STRNG_A>;
impl PAD37STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD37STRNG_A {
        match self.bits {
            false => PAD37STRNG_A::LOW,
            true => PAD37STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD37STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD37STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD37STRNG`"]
pub struct PAD37STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD37STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD37STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD37STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD37STRNG_A::HIGH)
    }
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
#[doc = "Pad 37 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD37INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD37INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD37INPEN`"]
pub type PAD37INPEN_R = crate::R<bool, PAD37INPEN_A>;
impl PAD37INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD37INPEN_A {
        match self.bits {
            false => PAD37INPEN_A::DIS,
            true => PAD37INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD37INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD37INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD37INPEN`"]
pub struct PAD37INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD37INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD37INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD37INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD37INPEN_A::EN)
    }
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
#[doc = "Pad 37 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD37PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD37PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD37PULL`"]
pub type PAD37PULL_R = crate::R<bool, PAD37PULL_A>;
impl PAD37PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD37PULL_A {
        match self.bits {
            false => PAD37PULL_A::DIS,
            true => PAD37PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD37PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD37PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD37PULL`"]
pub struct PAD37PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD37PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD37PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD37PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD37PULL_A::EN)
    }
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
#[doc = "Pad 36 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD36FNCSEL_A {
    #[doc = "0: Configure as the ADC Trigger 1 signal"]
    TRIG1 = 0,
    #[doc = "1: Configure as the SPI channel 1 nCE signal from IOMSTR1"]
    M1NCE1 = 1,
    #[doc = "2: Configure as the UART1 RX signal"]
    UART1RX = 2,
    #[doc = "3: Configure as GPIO36"]
    GPIO36 = 3,
    #[doc = "4: Configure as the 32kHz output clock from the crystal"]
    _32KHZ_XT = 4,
    #[doc = "5: Configure as the SPI channel 0 nCE signal from IOMSTR2"]
    M2NCE0 = 5,
    #[doc = "6: Configure as the UART0 CTS signal"]
    UA0CTS = 6,
    #[doc = "7: Configure as the SPI channel 3 nCE signal from IOMSTR3"]
    M3NCE3 = 7,
}
impl From<PAD36FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD36FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD36FNCSEL`"]
pub type PAD36FNCSEL_R = crate::R<u8, PAD36FNCSEL_A>;
impl PAD36FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD36FNCSEL_A {
        match self.bits {
            0 => PAD36FNCSEL_A::TRIG1,
            1 => PAD36FNCSEL_A::M1NCE1,
            2 => PAD36FNCSEL_A::UART1RX,
            3 => PAD36FNCSEL_A::GPIO36,
            4 => PAD36FNCSEL_A::_32KHZ_XT,
            5 => PAD36FNCSEL_A::M2NCE0,
            6 => PAD36FNCSEL_A::UA0CTS,
            7 => PAD36FNCSEL_A::M3NCE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == PAD36FNCSEL_A::TRIG1
    }
    #[doc = "Checks if the value of the field is `M1NCE1`"]
    #[inline(always)]
    pub fn is_m1n_ce1(&self) -> bool {
        *self == PAD36FNCSEL_A::M1NCE1
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD36FNCSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `GPIO36`"]
    #[inline(always)]
    pub fn is_gpio36(&self) -> bool {
        *self == PAD36FNCSEL_A::GPIO36
    }
    #[doc = "Checks if the value of the field is `_32KHZ_XT`"]
    #[inline(always)]
    pub fn is_32khz_xt(&self) -> bool {
        *self == PAD36FNCSEL_A::_32KHZ_XT
    }
    #[doc = "Checks if the value of the field is `M2NCE0`"]
    #[inline(always)]
    pub fn is_m2n_ce0(&self) -> bool {
        *self == PAD36FNCSEL_A::M2NCE0
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD36FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `M3NCE3`"]
    #[inline(always)]
    pub fn is_m3n_ce3(&self) -> bool {
        *self == PAD36FNCSEL_A::M3NCE3
    }
}
#[doc = "Write proxy for field `PAD36FNCSEL`"]
pub struct PAD36FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD36FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD36FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the ADC Trigger 1 signal"]
    #[inline(always)]
    pub fn trig1(self) -> &'a mut W {
        self.variant(PAD36FNCSEL_A::TRIG1)
    }
    #[doc = "Configure as the SPI channel 1 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce1(self) -> &'a mut W {
        self.variant(PAD36FNCSEL_A::M1NCE1)
    }
    #[doc = "Configure as the UART1 RX signal"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD36FNCSEL_A::UART1RX)
    }
    #[doc = "Configure as GPIO36"]
    #[inline(always)]
    pub fn gpio36(self) -> &'a mut W {
        self.variant(PAD36FNCSEL_A::GPIO36)
    }
    #[doc = "Configure as the 32kHz output clock from the crystal"]
    #[inline(always)]
    pub fn _32khz_xt(self) -> &'a mut W {
        self.variant(PAD36FNCSEL_A::_32KHZ_XT)
    }
    #[doc = "Configure as the SPI channel 0 nCE signal from IOMSTR2"]
    #[inline(always)]
    pub fn m2n_ce0(self) -> &'a mut W {
        self.variant(PAD36FNCSEL_A::M2NCE0)
    }
    #[doc = "Configure as the UART0 CTS signal"]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD36FNCSEL_A::UA0CTS)
    }
    #[doc = "Configure as the SPI channel 3 nCE signal from IOMSTR3"]
    #[inline(always)]
    pub fn m3n_ce3(self) -> &'a mut W {
        self.variant(PAD36FNCSEL_A::M3NCE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 36 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD36STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD36STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD36STRNG`"]
pub type PAD36STRNG_R = crate::R<bool, PAD36STRNG_A>;
impl PAD36STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD36STRNG_A {
        match self.bits {
            false => PAD36STRNG_A::LOW,
            true => PAD36STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD36STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD36STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD36STRNG`"]
pub struct PAD36STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD36STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD36STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD36STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD36STRNG_A::HIGH)
    }
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
#[doc = "Pad 36 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD36INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD36INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD36INPEN`"]
pub type PAD36INPEN_R = crate::R<bool, PAD36INPEN_A>;
impl PAD36INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD36INPEN_A {
        match self.bits {
            false => PAD36INPEN_A::DIS,
            true => PAD36INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD36INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD36INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD36INPEN`"]
pub struct PAD36INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD36INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD36INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD36INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD36INPEN_A::EN)
    }
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
#[doc = "Pad 36 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD36PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD36PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD36PULL`"]
pub type PAD36PULL_R = crate::R<bool, PAD36PULL_A>;
impl PAD36PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD36PULL_A {
        match self.bits {
            false => PAD36PULL_A::DIS,
            true => PAD36PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD36PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD36PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD36PULL`"]
pub struct PAD36PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD36PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD36PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD36PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD36PULL_A::EN)
    }
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
    #[doc = "Bits 30:31 - Pad 39 pullup resistor selection."]
    #[inline(always)]
    pub fn pad39rsel(&self) -> PAD39RSEL_R {
        PAD39RSEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:29 - Pad 39 function select"]
    #[inline(always)]
    pub fn pad39fncsel(&self) -> PAD39FNCSEL_R {
        PAD39FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 39 drive strength"]
    #[inline(always)]
    pub fn pad39strng(&self) -> PAD39STRNG_R {
        PAD39STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 39 input enable"]
    #[inline(always)]
    pub fn pad39inpen(&self) -> PAD39INPEN_R {
        PAD39INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 39 pullup enable"]
    #[inline(always)]
    pub fn pad39pull(&self) -> PAD39PULL_R {
        PAD39PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 38 function select"]
    #[inline(always)]
    pub fn pad38fncsel(&self) -> PAD38FNCSEL_R {
        PAD38FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 38 drive strength"]
    #[inline(always)]
    pub fn pad38strng(&self) -> PAD38STRNG_R {
        PAD38STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 38 input enable"]
    #[inline(always)]
    pub fn pad38inpen(&self) -> PAD38INPEN_R {
        PAD38INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 38 pullup enable"]
    #[inline(always)]
    pub fn pad38pull(&self) -> PAD38PULL_R {
        PAD38PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 37 function select"]
    #[inline(always)]
    pub fn pad37fncsel(&self) -> PAD37FNCSEL_R {
        PAD37FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 37 drive strength"]
    #[inline(always)]
    pub fn pad37strng(&self) -> PAD37STRNG_R {
        PAD37STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 37 input enable"]
    #[inline(always)]
    pub fn pad37inpen(&self) -> PAD37INPEN_R {
        PAD37INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 37 pullup enable"]
    #[inline(always)]
    pub fn pad37pull(&self) -> PAD37PULL_R {
        PAD37PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 36 function select"]
    #[inline(always)]
    pub fn pad36fncsel(&self) -> PAD36FNCSEL_R {
        PAD36FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 36 drive strength"]
    #[inline(always)]
    pub fn pad36strng(&self) -> PAD36STRNG_R {
        PAD36STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 36 input enable"]
    #[inline(always)]
    pub fn pad36inpen(&self) -> PAD36INPEN_R {
        PAD36INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 36 pullup enable"]
    #[inline(always)]
    pub fn pad36pull(&self) -> PAD36PULL_R {
        PAD36PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - Pad 39 pullup resistor selection."]
    #[inline(always)]
    pub fn pad39rsel(&mut self) -> PAD39RSEL_W {
        PAD39RSEL_W { w: self }
    }
    #[doc = "Bits 27:29 - Pad 39 function select"]
    #[inline(always)]
    pub fn pad39fncsel(&mut self) -> PAD39FNCSEL_W {
        PAD39FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 39 drive strength"]
    #[inline(always)]
    pub fn pad39strng(&mut self) -> PAD39STRNG_W {
        PAD39STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 39 input enable"]
    #[inline(always)]
    pub fn pad39inpen(&mut self) -> PAD39INPEN_W {
        PAD39INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 39 pullup enable"]
    #[inline(always)]
    pub fn pad39pull(&mut self) -> PAD39PULL_W {
        PAD39PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 38 function select"]
    #[inline(always)]
    pub fn pad38fncsel(&mut self) -> PAD38FNCSEL_W {
        PAD38FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 38 drive strength"]
    #[inline(always)]
    pub fn pad38strng(&mut self) -> PAD38STRNG_W {
        PAD38STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 38 input enable"]
    #[inline(always)]
    pub fn pad38inpen(&mut self) -> PAD38INPEN_W {
        PAD38INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 38 pullup enable"]
    #[inline(always)]
    pub fn pad38pull(&mut self) -> PAD38PULL_W {
        PAD38PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 37 function select"]
    #[inline(always)]
    pub fn pad37fncsel(&mut self) -> PAD37FNCSEL_W {
        PAD37FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 37 drive strength"]
    #[inline(always)]
    pub fn pad37strng(&mut self) -> PAD37STRNG_W {
        PAD37STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 37 input enable"]
    #[inline(always)]
    pub fn pad37inpen(&mut self) -> PAD37INPEN_W {
        PAD37INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 37 pullup enable"]
    #[inline(always)]
    pub fn pad37pull(&mut self) -> PAD37PULL_W {
        PAD37PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 36 function select"]
    #[inline(always)]
    pub fn pad36fncsel(&mut self) -> PAD36FNCSEL_W {
        PAD36FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 36 drive strength"]
    #[inline(always)]
    pub fn pad36strng(&mut self) -> PAD36STRNG_W {
        PAD36STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 36 input enable"]
    #[inline(always)]
    pub fn pad36inpen(&mut self) -> PAD36INPEN_W {
        PAD36INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 36 pullup enable"]
    #[inline(always)]
    pub fn pad36pull(&mut self) -> PAD36PULL_W {
        PAD36PULL_W { w: self }
    }
}
