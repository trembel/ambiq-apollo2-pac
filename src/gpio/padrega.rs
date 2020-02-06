#[doc = "Reader of register PADREGA"]
pub type R = crate::R<u32, super::PADREGA>;
#[doc = "Writer for register PADREGA"]
pub type W = crate::W<u32, super::PADREGA>;
#[doc = "Register PADREGA `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 3 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD3FNCSEL_A {
    #[doc = "0: Configure as the UART0 RTS output"]
    UA0RTS = 0,
    #[doc = "1: Configure as the IOSLAVE SPI nCE signal"]
    SLNCE = 1,
    #[doc = "2: Configure as the SPI channel 4 nCE signal from IOMSTR1"]
    M1NCE4 = 2,
    #[doc = "3: Configure as GPIO3"]
    GPIO3 = 3,
    #[doc = "4: Configure as the IOSLAVE SPI nCE loopback signal from IOMSTRx"]
    MXNCELB = 4,
    #[doc = "5: Configure as the SPI channel 0 nCE signal from IOMSTR2"]
    M2NCE0 = 5,
    #[doc = "6: Configure as the ADC Trigger 1 signal"]
    TRIG1 = 6,
    #[doc = "7: Configure as the I2S Word Clock input"]
    I2S_WCLK = 7,
}
impl From<PAD3FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD3FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD3FNCSEL`"]
pub type PAD3FNCSEL_R = crate::R<u8, PAD3FNCSEL_A>;
impl PAD3FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD3FNCSEL_A {
        match self.bits {
            0 => PAD3FNCSEL_A::UA0RTS,
            1 => PAD3FNCSEL_A::SLNCE,
            2 => PAD3FNCSEL_A::M1NCE4,
            3 => PAD3FNCSEL_A::GPIO3,
            4 => PAD3FNCSEL_A::MXNCELB,
            5 => PAD3FNCSEL_A::M2NCE0,
            6 => PAD3FNCSEL_A::TRIG1,
            7 => PAD3FNCSEL_A::I2S_WCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD3FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `SLNCE`"]
    #[inline(always)]
    pub fn is_sln_ce(&self) -> bool {
        *self == PAD3FNCSEL_A::SLNCE
    }
    #[doc = "Checks if the value of the field is `M1NCE4`"]
    #[inline(always)]
    pub fn is_m1n_ce4(&self) -> bool {
        *self == PAD3FNCSEL_A::M1NCE4
    }
    #[doc = "Checks if the value of the field is `GPIO3`"]
    #[inline(always)]
    pub fn is_gpio3(&self) -> bool {
        *self == PAD3FNCSEL_A::GPIO3
    }
    #[doc = "Checks if the value of the field is `MXNCELB`"]
    #[inline(always)]
    pub fn is_mxn_celb(&self) -> bool {
        *self == PAD3FNCSEL_A::MXNCELB
    }
    #[doc = "Checks if the value of the field is `M2NCE0`"]
    #[inline(always)]
    pub fn is_m2n_ce0(&self) -> bool {
        *self == PAD3FNCSEL_A::M2NCE0
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == PAD3FNCSEL_A::TRIG1
    }
    #[doc = "Checks if the value of the field is `I2S_WCLK`"]
    #[inline(always)]
    pub fn is_i2s_wclk(&self) -> bool {
        *self == PAD3FNCSEL_A::I2S_WCLK
    }
}
#[doc = "Write proxy for field `PAD3FNCSEL`"]
pub struct PAD3FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the UART0 RTS output"]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as the IOSLAVE SPI nCE signal"]
    #[inline(always)]
    pub fn sln_ce(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::SLNCE)
    }
    #[doc = "Configure as the SPI channel 4 nCE signal from IOMSTR1"]
    #[inline(always)]
    pub fn m1n_ce4(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::M1NCE4)
    }
    #[doc = "Configure as GPIO3"]
    #[inline(always)]
    pub fn gpio3(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::GPIO3)
    }
    #[doc = "Configure as the IOSLAVE SPI nCE loopback signal from IOMSTRx"]
    #[inline(always)]
    pub fn mxn_celb(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::MXNCELB)
    }
    #[doc = "Configure as the SPI channel 0 nCE signal from IOMSTR2"]
    #[inline(always)]
    pub fn m2n_ce0(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::M2NCE0)
    }
    #[doc = "Configure as the ADC Trigger 1 signal"]
    #[inline(always)]
    pub fn trig1(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::TRIG1)
    }
    #[doc = "Configure as the I2S Word Clock input"]
    #[inline(always)]
    pub fn i2s_wclk(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::I2S_WCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 3 drive strength.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD3STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD3STRNG`"]
pub type PAD3STRNG_R = crate::R<bool, PAD3STRNG_A>;
impl PAD3STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD3STRNG_A {
        match self.bits {
            false => PAD3STRNG_A::LOW,
            true => PAD3STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD3STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD3STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD3STRNG`"]
pub struct PAD3STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD3STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD3STRNG_A::HIGH)
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
#[doc = "Pad 3 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD3INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD3INPEN`"]
pub type PAD3INPEN_R = crate::R<bool, PAD3INPEN_A>;
impl PAD3INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD3INPEN_A {
        match self.bits {
            false => PAD3INPEN_A::DIS,
            true => PAD3INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD3INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD3INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD3INPEN`"]
pub struct PAD3INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD3INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD3INPEN_A::EN)
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
#[doc = "Pad 3 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD3PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD3PULL`"]
pub type PAD3PULL_R = crate::R<bool, PAD3PULL_A>;
impl PAD3PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD3PULL_A {
        match self.bits {
            false => PAD3PULL_A::DIS,
            true => PAD3PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD3PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD3PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD3PULL`"]
pub struct PAD3PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD3PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD3PULL_A::EN)
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
#[doc = "Pad 2 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD2FNCSEL_A {
    #[doc = "0: Configure as the IOSLAVE SPI 3-wire MOSI/MISO signal"]
    SLWIR3 = 0,
    #[doc = "1: Configure as the IOSLAVE SPI MOSI signal"]
    SLMOSI = 1,
    #[doc = "2: Configure as the UART0 RX input"]
    UART0RX = 2,
    #[doc = "3: Configure as GPIO2"]
    GPIO2 = 3,
    #[doc = "4: Configure as the IOSLAVE SPI MOSI loopback signal from IOMSTRx"]
    MXMOSILB = 4,
    #[doc = "5: Configure as the IOMSTR2 SPI MOSI output signal"]
    M2MOSI = 5,
    #[doc = "6: Configure as the IOSLAVE SPI 3-wire MOSI/MISO loopback signal from IOMSTRx"]
    MXWIR3LB = 6,
    #[doc = "7: Configure as the IOMSTR2 SPI 3-wire MOSI/MISO signal"]
    M2WIR3 = 7,
}
impl From<PAD2FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD2FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD2FNCSEL`"]
pub type PAD2FNCSEL_R = crate::R<u8, PAD2FNCSEL_A>;
impl PAD2FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD2FNCSEL_A {
        match self.bits {
            0 => PAD2FNCSEL_A::SLWIR3,
            1 => PAD2FNCSEL_A::SLMOSI,
            2 => PAD2FNCSEL_A::UART0RX,
            3 => PAD2FNCSEL_A::GPIO2,
            4 => PAD2FNCSEL_A::MXMOSILB,
            5 => PAD2FNCSEL_A::M2MOSI,
            6 => PAD2FNCSEL_A::MXWIR3LB,
            7 => PAD2FNCSEL_A::M2WIR3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLWIR3`"]
    #[inline(always)]
    pub fn is_slwir3(&self) -> bool {
        *self == PAD2FNCSEL_A::SLWIR3
    }
    #[doc = "Checks if the value of the field is `SLMOSI`"]
    #[inline(always)]
    pub fn is_slmosi(&self) -> bool {
        *self == PAD2FNCSEL_A::SLMOSI
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD2FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `GPIO2`"]
    #[inline(always)]
    pub fn is_gpio2(&self) -> bool {
        *self == PAD2FNCSEL_A::GPIO2
    }
    #[doc = "Checks if the value of the field is `MXMOSILB`"]
    #[inline(always)]
    pub fn is_mx_mosilb(&self) -> bool {
        *self == PAD2FNCSEL_A::MXMOSILB
    }
    #[doc = "Checks if the value of the field is `M2MOSI`"]
    #[inline(always)]
    pub fn is_m2mosi(&self) -> bool {
        *self == PAD2FNCSEL_A::M2MOSI
    }
    #[doc = "Checks if the value of the field is `MXWIR3LB`"]
    #[inline(always)]
    pub fn is_mx_wir3lb(&self) -> bool {
        *self == PAD2FNCSEL_A::MXWIR3LB
    }
    #[doc = "Checks if the value of the field is `M2WIR3`"]
    #[inline(always)]
    pub fn is_m2wir3(&self) -> bool {
        *self == PAD2FNCSEL_A::M2WIR3
    }
}
#[doc = "Write proxy for field `PAD2FNCSEL`"]
pub struct PAD2FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the IOSLAVE SPI 3-wire MOSI/MISO signal"]
    #[inline(always)]
    pub fn slwir3(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::SLWIR3)
    }
    #[doc = "Configure as the IOSLAVE SPI MOSI signal"]
    #[inline(always)]
    pub fn slmosi(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::SLMOSI)
    }
    #[doc = "Configure as the UART0 RX input"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as GPIO2"]
    #[inline(always)]
    pub fn gpio2(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::GPIO2)
    }
    #[doc = "Configure as the IOSLAVE SPI MOSI loopback signal from IOMSTRx"]
    #[inline(always)]
    pub fn mx_mosilb(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::MXMOSILB)
    }
    #[doc = "Configure as the IOMSTR2 SPI MOSI output signal"]
    #[inline(always)]
    pub fn m2mosi(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::M2MOSI)
    }
    #[doc = "Configure as the IOSLAVE SPI 3-wire MOSI/MISO loopback signal from IOMSTRx"]
    #[inline(always)]
    pub fn mx_wir3lb(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::MXWIR3LB)
    }
    #[doc = "Configure as the IOMSTR2 SPI 3-wire MOSI/MISO signal"]
    #[inline(always)]
    pub fn m2wir3(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::M2WIR3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 2 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD2STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD2STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD2STRNG`"]
pub type PAD2STRNG_R = crate::R<bool, PAD2STRNG_A>;
impl PAD2STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD2STRNG_A {
        match self.bits {
            false => PAD2STRNG_A::LOW,
            true => PAD2STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD2STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD2STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD2STRNG`"]
pub struct PAD2STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD2STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD2STRNG_A::HIGH)
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
#[doc = "Pad 2 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD2INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD2INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD2INPEN`"]
pub type PAD2INPEN_R = crate::R<bool, PAD2INPEN_A>;
impl PAD2INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD2INPEN_A {
        match self.bits {
            false => PAD2INPEN_A::DIS,
            true => PAD2INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD2INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD2INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD2INPEN`"]
pub struct PAD2INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD2INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD2INPEN_A::EN)
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
#[doc = "Pad 2 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD2PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD2PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD2PULL`"]
pub type PAD2PULL_R = crate::R<bool, PAD2PULL_A>;
impl PAD2PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD2PULL_A {
        match self.bits {
            false => PAD2PULL_A::DIS,
            true => PAD2PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD2PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD2PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD2PULL`"]
pub struct PAD2PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD2PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD2PULL_A::EN)
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
#[doc = "Pad 1 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD1RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD1RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD1RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD1RSEL`"]
pub type PAD1RSEL_R = crate::R<u8, PAD1RSEL_A>;
impl PAD1RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1RSEL_A {
        match self.bits {
            0 => PAD1RSEL_A::PULL1_5K,
            1 => PAD1RSEL_A::PULL6K,
            2 => PAD1RSEL_A::PULL12K,
            3 => PAD1RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD1RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD1RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD1RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD1RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD1RSEL`"]
pub struct PAD1RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD1RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD1RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD1RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD1RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pad 1 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD1FNCSEL_A {
    #[doc = "0: Configure as the IOSLAVE I2C SDA signal"]
    SLSDA = 0,
    #[doc = "1: Configure as the IOSLAVE SPI MISO signal"]
    SLMISO = 1,
    #[doc = "2: Configure as the UART0 TX output signal"]
    UART0TX = 2,
    #[doc = "3: Configure as GPIO1"]
    GPIO1 = 3,
    #[doc = "4: Configure as the IOSLAVE SPI MISO loopback signal from IOMSTRx"]
    MXMISOLB = 4,
    #[doc = "5: Configure as the IOMSTR2 SPI MISO input signal"]
    M2MISO = 5,
    #[doc = "6: Configure as the IOSLAVE I2C SDA loopback signal from IOMSTRx"]
    MXSDALB = 6,
    #[doc = "7: Configure as the IOMSTR2 I2C Serial data I/O signal"]
    M2SDA = 7,
}
impl From<PAD1FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD1FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD1FNCSEL`"]
pub type PAD1FNCSEL_R = crate::R<u8, PAD1FNCSEL_A>;
impl PAD1FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1FNCSEL_A {
        match self.bits {
            0 => PAD1FNCSEL_A::SLSDA,
            1 => PAD1FNCSEL_A::SLMISO,
            2 => PAD1FNCSEL_A::UART0TX,
            3 => PAD1FNCSEL_A::GPIO1,
            4 => PAD1FNCSEL_A::MXMISOLB,
            5 => PAD1FNCSEL_A::M2MISO,
            6 => PAD1FNCSEL_A::MXSDALB,
            7 => PAD1FNCSEL_A::M2SDA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLSDA`"]
    #[inline(always)]
    pub fn is_slsda(&self) -> bool {
        *self == PAD1FNCSEL_A::SLSDA
    }
    #[doc = "Checks if the value of the field is `SLMISO`"]
    #[inline(always)]
    pub fn is_slmiso(&self) -> bool {
        *self == PAD1FNCSEL_A::SLMISO
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD1FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `GPIO1`"]
    #[inline(always)]
    pub fn is_gpio1(&self) -> bool {
        *self == PAD1FNCSEL_A::GPIO1
    }
    #[doc = "Checks if the value of the field is `MXMISOLB`"]
    #[inline(always)]
    pub fn is_mx_misolb(&self) -> bool {
        *self == PAD1FNCSEL_A::MXMISOLB
    }
    #[doc = "Checks if the value of the field is `M2MISO`"]
    #[inline(always)]
    pub fn is_m2miso(&self) -> bool {
        *self == PAD1FNCSEL_A::M2MISO
    }
    #[doc = "Checks if the value of the field is `MXSDALB`"]
    #[inline(always)]
    pub fn is_mx_sdalb(&self) -> bool {
        *self == PAD1FNCSEL_A::MXSDALB
    }
    #[doc = "Checks if the value of the field is `M2SDA`"]
    #[inline(always)]
    pub fn is_m2sda(&self) -> bool {
        *self == PAD1FNCSEL_A::M2SDA
    }
}
#[doc = "Write proxy for field `PAD1FNCSEL`"]
pub struct PAD1FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the IOSLAVE I2C SDA signal"]
    #[inline(always)]
    pub fn slsda(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::SLSDA)
    }
    #[doc = "Configure as the IOSLAVE SPI MISO signal"]
    #[inline(always)]
    pub fn slmiso(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::SLMISO)
    }
    #[doc = "Configure as the UART0 TX output signal"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as GPIO1"]
    #[inline(always)]
    pub fn gpio1(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::GPIO1)
    }
    #[doc = "Configure as the IOSLAVE SPI MISO loopback signal from IOMSTRx"]
    #[inline(always)]
    pub fn mx_misolb(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::MXMISOLB)
    }
    #[doc = "Configure as the IOMSTR2 SPI MISO input signal"]
    #[inline(always)]
    pub fn m2miso(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::M2MISO)
    }
    #[doc = "Configure as the IOSLAVE I2C SDA loopback signal from IOMSTRx"]
    #[inline(always)]
    pub fn mx_sdalb(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::MXSDALB)
    }
    #[doc = "Configure as the IOMSTR2 I2C Serial data I/O signal"]
    #[inline(always)]
    pub fn m2sda(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::M2SDA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 1 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD1STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD1STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD1STRNG`"]
pub type PAD1STRNG_R = crate::R<bool, PAD1STRNG_A>;
impl PAD1STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1STRNG_A {
        match self.bits {
            false => PAD1STRNG_A::LOW,
            true => PAD1STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD1STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD1STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD1STRNG`"]
pub struct PAD1STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD1STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD1STRNG_A::HIGH)
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
#[doc = "Pad 1 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD1INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD1INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD1INPEN`"]
pub type PAD1INPEN_R = crate::R<bool, PAD1INPEN_A>;
impl PAD1INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1INPEN_A {
        match self.bits {
            false => PAD1INPEN_A::DIS,
            true => PAD1INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD1INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD1INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD1INPEN`"]
pub struct PAD1INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD1INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD1INPEN_A::EN)
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
#[doc = "Pad 1 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD1PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD1PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD1PULL`"]
pub type PAD1PULL_R = crate::R<bool, PAD1PULL_A>;
impl PAD1PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1PULL_A {
        match self.bits {
            false => PAD1PULL_A::DIS,
            true => PAD1PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD1PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD1PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD1PULL`"]
pub struct PAD1PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD1PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD1PULL_A::EN)
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
#[doc = "Pad 0 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD0RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD0RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD0RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD0RSEL`"]
pub type PAD0RSEL_R = crate::R<u8, PAD0RSEL_A>;
impl PAD0RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0RSEL_A {
        match self.bits {
            0 => PAD0RSEL_A::PULL1_5K,
            1 => PAD0RSEL_A::PULL6K,
            2 => PAD0RSEL_A::PULL12K,
            3 => PAD0RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD0RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD0RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD0RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD0RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD0RSEL`"]
pub struct PAD0RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD0RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD0RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD0RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD0RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pad 0 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD0FNCSEL_A {
    #[doc = "0: Configure as the IOSLAVE I2C SCL signal"]
    SLSCL = 0,
    #[doc = "1: Configure as the IOSLAVE SPI SCK signal"]
    SLSCK = 1,
    #[doc = "2: Configure as the CLKOUT signal"]
    CLKOUT = 2,
    #[doc = "3: Configure as GPIO0"]
    GPIO0 = 3,
    #[doc = "4: Configure as the IOSLAVE SPI SCK loopback signal from IOMSTRx"]
    MXSCKLB = 4,
    #[doc = "5: Configure as the IOMSTR2 SPI SCK output"]
    M2SCK = 5,
    #[doc = "6: Configure as the IOSLAVE I2C SCL loopback signal from IOMSTRx"]
    MXSCLLB = 6,
    #[doc = "7: Configure as the IOMSTR2 I2C SCL clock I/O signal"]
    M2SCL = 7,
}
impl From<PAD0FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD0FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD0FNCSEL`"]
pub type PAD0FNCSEL_R = crate::R<u8, PAD0FNCSEL_A>;
impl PAD0FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0FNCSEL_A {
        match self.bits {
            0 => PAD0FNCSEL_A::SLSCL,
            1 => PAD0FNCSEL_A::SLSCK,
            2 => PAD0FNCSEL_A::CLKOUT,
            3 => PAD0FNCSEL_A::GPIO0,
            4 => PAD0FNCSEL_A::MXSCKLB,
            5 => PAD0FNCSEL_A::M2SCK,
            6 => PAD0FNCSEL_A::MXSCLLB,
            7 => PAD0FNCSEL_A::M2SCL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLSCL`"]
    #[inline(always)]
    pub fn is_slscl(&self) -> bool {
        *self == PAD0FNCSEL_A::SLSCL
    }
    #[doc = "Checks if the value of the field is `SLSCK`"]
    #[inline(always)]
    pub fn is_slsck(&self) -> bool {
        *self == PAD0FNCSEL_A::SLSCK
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == PAD0FNCSEL_A::CLKOUT
    }
    #[doc = "Checks if the value of the field is `GPIO0`"]
    #[inline(always)]
    pub fn is_gpio0(&self) -> bool {
        *self == PAD0FNCSEL_A::GPIO0
    }
    #[doc = "Checks if the value of the field is `MXSCKLB`"]
    #[inline(always)]
    pub fn is_mx_scklb(&self) -> bool {
        *self == PAD0FNCSEL_A::MXSCKLB
    }
    #[doc = "Checks if the value of the field is `M2SCK`"]
    #[inline(always)]
    pub fn is_m2sck(&self) -> bool {
        *self == PAD0FNCSEL_A::M2SCK
    }
    #[doc = "Checks if the value of the field is `MXSCLLB`"]
    #[inline(always)]
    pub fn is_mx_scllb(&self) -> bool {
        *self == PAD0FNCSEL_A::MXSCLLB
    }
    #[doc = "Checks if the value of the field is `M2SCL`"]
    #[inline(always)]
    pub fn is_m2scl(&self) -> bool {
        *self == PAD0FNCSEL_A::M2SCL
    }
}
#[doc = "Write proxy for field `PAD0FNCSEL`"]
pub struct PAD0FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the IOSLAVE I2C SCL signal"]
    #[inline(always)]
    pub fn slscl(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::SLSCL)
    }
    #[doc = "Configure as the IOSLAVE SPI SCK signal"]
    #[inline(always)]
    pub fn slsck(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::SLSCK)
    }
    #[doc = "Configure as the CLKOUT signal"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::CLKOUT)
    }
    #[doc = "Configure as GPIO0"]
    #[inline(always)]
    pub fn gpio0(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::GPIO0)
    }
    #[doc = "Configure as the IOSLAVE SPI SCK loopback signal from IOMSTRx"]
    #[inline(always)]
    pub fn mx_scklb(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::MXSCKLB)
    }
    #[doc = "Configure as the IOMSTR2 SPI SCK output"]
    #[inline(always)]
    pub fn m2sck(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::M2SCK)
    }
    #[doc = "Configure as the IOSLAVE I2C SCL loopback signal from IOMSTRx"]
    #[inline(always)]
    pub fn mx_scllb(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::MXSCLLB)
    }
    #[doc = "Configure as the IOMSTR2 I2C SCL clock I/O signal"]
    #[inline(always)]
    pub fn m2scl(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::M2SCL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 0 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD0STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD0STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD0STRNG`"]
pub type PAD0STRNG_R = crate::R<bool, PAD0STRNG_A>;
impl PAD0STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0STRNG_A {
        match self.bits {
            false => PAD0STRNG_A::LOW,
            true => PAD0STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD0STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD0STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD0STRNG`"]
pub struct PAD0STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD0STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD0STRNG_A::HIGH)
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
#[doc = "Pad 0 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD0INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD0INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD0INPEN`"]
pub type PAD0INPEN_R = crate::R<bool, PAD0INPEN_A>;
impl PAD0INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0INPEN_A {
        match self.bits {
            false => PAD0INPEN_A::DIS,
            true => PAD0INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD0INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD0INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD0INPEN`"]
pub struct PAD0INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD0INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD0INPEN_A::EN)
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
#[doc = "Pad 0 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD0PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD0PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD0PULL`"]
pub type PAD0PULL_R = crate::R<bool, PAD0PULL_A>;
impl PAD0PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0PULL_A {
        match self.bits {
            false => PAD0PULL_A::DIS,
            true => PAD0PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD0PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD0PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD0PULL`"]
pub struct PAD0PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD0PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD0PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 3 function select"]
    #[inline(always)]
    pub fn pad3fncsel(&self) -> PAD3FNCSEL_R {
        PAD3FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 3 drive strength."]
    #[inline(always)]
    pub fn pad3strng(&self) -> PAD3STRNG_R {
        PAD3STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 3 input enable."]
    #[inline(always)]
    pub fn pad3inpen(&self) -> PAD3INPEN_R {
        PAD3INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 3 pullup enable"]
    #[inline(always)]
    pub fn pad3pull(&self) -> PAD3PULL_R {
        PAD3PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 2 function select"]
    #[inline(always)]
    pub fn pad2fncsel(&self) -> PAD2FNCSEL_R {
        PAD2FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 2 drive strength"]
    #[inline(always)]
    pub fn pad2strng(&self) -> PAD2STRNG_R {
        PAD2STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 2 input enable"]
    #[inline(always)]
    pub fn pad2inpen(&self) -> PAD2INPEN_R {
        PAD2INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 2 pullup enable"]
    #[inline(always)]
    pub fn pad2pull(&self) -> PAD2PULL_R {
        PAD2PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pad 1 pullup resistor selection."]
    #[inline(always)]
    pub fn pad1rsel(&self) -> PAD1RSEL_R {
        PAD1RSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Pad 1 function select"]
    #[inline(always)]
    pub fn pad1fncsel(&self) -> PAD1FNCSEL_R {
        PAD1FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 1 drive strength"]
    #[inline(always)]
    pub fn pad1strng(&self) -> PAD1STRNG_R {
        PAD1STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 1 input enable"]
    #[inline(always)]
    pub fn pad1inpen(&self) -> PAD1INPEN_R {
        PAD1INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 1 pullup enable"]
    #[inline(always)]
    pub fn pad1pull(&self) -> PAD1PULL_R {
        PAD1PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pad 0 pullup resistor selection."]
    #[inline(always)]
    pub fn pad0rsel(&self) -> PAD0RSEL_R {
        PAD0RSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Pad 0 function select"]
    #[inline(always)]
    pub fn pad0fncsel(&self) -> PAD0FNCSEL_R {
        PAD0FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 0 drive strength"]
    #[inline(always)]
    pub fn pad0strng(&self) -> PAD0STRNG_R {
        PAD0STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 0 input enable"]
    #[inline(always)]
    pub fn pad0inpen(&self) -> PAD0INPEN_R {
        PAD0INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 0 pullup enable"]
    #[inline(always)]
    pub fn pad0pull(&self) -> PAD0PULL_R {
        PAD0PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 3 function select"]
    #[inline(always)]
    pub fn pad3fncsel(&mut self) -> PAD3FNCSEL_W {
        PAD3FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 3 drive strength."]
    #[inline(always)]
    pub fn pad3strng(&mut self) -> PAD3STRNG_W {
        PAD3STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 3 input enable."]
    #[inline(always)]
    pub fn pad3inpen(&mut self) -> PAD3INPEN_W {
        PAD3INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 3 pullup enable"]
    #[inline(always)]
    pub fn pad3pull(&mut self) -> PAD3PULL_W {
        PAD3PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 2 function select"]
    #[inline(always)]
    pub fn pad2fncsel(&mut self) -> PAD2FNCSEL_W {
        PAD2FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 2 drive strength"]
    #[inline(always)]
    pub fn pad2strng(&mut self) -> PAD2STRNG_W {
        PAD2STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 2 input enable"]
    #[inline(always)]
    pub fn pad2inpen(&mut self) -> PAD2INPEN_W {
        PAD2INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 2 pullup enable"]
    #[inline(always)]
    pub fn pad2pull(&mut self) -> PAD2PULL_W {
        PAD2PULL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pad 1 pullup resistor selection."]
    #[inline(always)]
    pub fn pad1rsel(&mut self) -> PAD1RSEL_W {
        PAD1RSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 1 function select"]
    #[inline(always)]
    pub fn pad1fncsel(&mut self) -> PAD1FNCSEL_W {
        PAD1FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 1 drive strength"]
    #[inline(always)]
    pub fn pad1strng(&mut self) -> PAD1STRNG_W {
        PAD1STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 1 input enable"]
    #[inline(always)]
    pub fn pad1inpen(&mut self) -> PAD1INPEN_W {
        PAD1INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 1 pullup enable"]
    #[inline(always)]
    pub fn pad1pull(&mut self) -> PAD1PULL_W {
        PAD1PULL_W { w: self }
    }
    #[doc = "Bits 6:7 - Pad 0 pullup resistor selection."]
    #[inline(always)]
    pub fn pad0rsel(&mut self) -> PAD0RSEL_W {
        PAD0RSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 0 function select"]
    #[inline(always)]
    pub fn pad0fncsel(&mut self) -> PAD0FNCSEL_W {
        PAD0FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 0 drive strength"]
    #[inline(always)]
    pub fn pad0strng(&mut self) -> PAD0STRNG_W {
        PAD0STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 0 input enable"]
    #[inline(always)]
    pub fn pad0inpen(&mut self) -> PAD0INPEN_W {
        PAD0INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 0 pullup enable"]
    #[inline(always)]
    pub fn pad0pull(&mut self) -> PAD0PULL_W {
        PAD0PULL_W { w: self }
    }
}
