#[doc = "Reader of register DEVICEEN"]
pub type R = crate::R<u32, super::DEVICEEN>;
#[doc = "Writer for register DEVICEEN"]
pub type W = crate::W<u32, super::DEVICEEN>;
#[doc = "Register DEVICEEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVICEEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable PDM Digital Block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRPDM_A {
    #[doc = "1: Enable PDM"]
    EN = 1,
    #[doc = "0: Disables PDM"]
    DIS = 0,
}
impl From<PWRPDM_A> for bool {
    #[inline(always)]
    fn from(variant: PWRPDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRPDM`"]
pub type PWRPDM_R = crate::R<bool, PWRPDM_A>;
impl PWRPDM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRPDM_A {
        match self.bits {
            true => PWRPDM_A::EN,
            false => PWRPDM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRPDM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRPDM_A::DIS
    }
}
#[doc = "Write proxy for field `PWRPDM`"]
pub struct PWRPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRPDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRPDM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable PDM"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRPDM_A::EN)
    }
    #[doc = "Disables PDM"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRPDM_A::DIS)
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
#[doc = "Enable ADC Digital Block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRADC_A {
    #[doc = "1: Enable ADC"]
    EN = 1,
    #[doc = "0: Disables ADC"]
    DIS = 0,
}
impl From<PWRADC_A> for bool {
    #[inline(always)]
    fn from(variant: PWRADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRADC`"]
pub type PWRADC_R = crate::R<bool, PWRADC_A>;
impl PWRADC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRADC_A {
        match self.bits {
            true => PWRADC_A::EN,
            false => PWRADC_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRADC_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRADC_A::DIS
    }
}
#[doc = "Write proxy for field `PWRADC`"]
pub struct PWRADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRADC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable ADC"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRADC_A::EN)
    }
    #[doc = "Disables ADC"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRADC_A::DIS)
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
#[doc = "Enable UART 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUART1_A {
    #[doc = "1: Enable UART 1"]
    EN = 1,
    #[doc = "0: Disables UART 1"]
    DIS = 0,
}
impl From<PWRUART1_A> for bool {
    #[inline(always)]
    fn from(variant: PWRUART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRUART1`"]
pub type PWRUART1_R = crate::R<bool, PWRUART1_A>;
impl PWRUART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRUART1_A {
        match self.bits {
            true => PWRUART1_A::EN,
            false => PWRUART1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRUART1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRUART1_A::DIS
    }
}
#[doc = "Write proxy for field `PWRUART1`"]
pub struct PWRUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRUART1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable UART 1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRUART1_A::EN)
    }
    #[doc = "Disables UART 1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRUART1_A::DIS)
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
#[doc = "Enable UART 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUART0_A {
    #[doc = "1: Enable UART 0"]
    EN = 1,
    #[doc = "0: Disables UART 0"]
    DIS = 0,
}
impl From<PWRUART0_A> for bool {
    #[inline(always)]
    fn from(variant: PWRUART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRUART0`"]
pub type PWRUART0_R = crate::R<bool, PWRUART0_A>;
impl PWRUART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRUART0_A {
        match self.bits {
            true => PWRUART0_A::EN,
            false => PWRUART0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PWRUART0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PWRUART0_A::DIS
    }
}
#[doc = "Write proxy for field `PWRUART0`"]
pub struct PWRUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRUART0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable UART 0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRUART0_A::EN)
    }
    #[doc = "Disables UART 0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRUART0_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Enable IO MASTER 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_MASTER5_A {
    #[doc = "1: Enable IO MASTER 5"]
    EN = 1,
    #[doc = "0: Disables IO MASTER 5"]
    DIS = 0,
}
impl From<IO_MASTER5_A> for bool {
    #[inline(always)]
    fn from(variant: IO_MASTER5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IO_MASTER5`"]
pub type IO_MASTER5_R = crate::R<bool, IO_MASTER5_A>;
impl IO_MASTER5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_MASTER5_A {
        match self.bits {
            true => IO_MASTER5_A::EN,
            false => IO_MASTER5_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IO_MASTER5_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IO_MASTER5_A::DIS
    }
}
#[doc = "Write proxy for field `IO_MASTER5`"]
pub struct IO_MASTER5_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MASTER5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_MASTER5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IO MASTER 5"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IO_MASTER5_A::EN)
    }
    #[doc = "Disables IO MASTER 5"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IO_MASTER5_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable IO MASTER 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_MASTER4_A {
    #[doc = "1: Enable IO MASTER 4"]
    EN = 1,
    #[doc = "0: Disables IO MASTER 4"]
    DIS = 0,
}
impl From<IO_MASTER4_A> for bool {
    #[inline(always)]
    fn from(variant: IO_MASTER4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IO_MASTER4`"]
pub type IO_MASTER4_R = crate::R<bool, IO_MASTER4_A>;
impl IO_MASTER4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_MASTER4_A {
        match self.bits {
            true => IO_MASTER4_A::EN,
            false => IO_MASTER4_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IO_MASTER4_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IO_MASTER4_A::DIS
    }
}
#[doc = "Write proxy for field `IO_MASTER4`"]
pub struct IO_MASTER4_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MASTER4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_MASTER4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IO MASTER 4"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IO_MASTER4_A::EN)
    }
    #[doc = "Disables IO MASTER 4"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IO_MASTER4_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Enable IO MASTER 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_MASTER3_A {
    #[doc = "1: Enable IO MASTER 3"]
    EN = 1,
    #[doc = "0: Disables IO MASTER 3"]
    DIS = 0,
}
impl From<IO_MASTER3_A> for bool {
    #[inline(always)]
    fn from(variant: IO_MASTER3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IO_MASTER3`"]
pub type IO_MASTER3_R = crate::R<bool, IO_MASTER3_A>;
impl IO_MASTER3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_MASTER3_A {
        match self.bits {
            true => IO_MASTER3_A::EN,
            false => IO_MASTER3_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IO_MASTER3_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IO_MASTER3_A::DIS
    }
}
#[doc = "Write proxy for field `IO_MASTER3`"]
pub struct IO_MASTER3_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MASTER3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_MASTER3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IO MASTER 3"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IO_MASTER3_A::EN)
    }
    #[doc = "Disables IO MASTER 3"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IO_MASTER3_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Enable IO MASTER 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_MASTER2_A {
    #[doc = "1: Enable IO MASTER 2"]
    EN = 1,
    #[doc = "0: Disables IO MASTER 2"]
    DIS = 0,
}
impl From<IO_MASTER2_A> for bool {
    #[inline(always)]
    fn from(variant: IO_MASTER2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IO_MASTER2`"]
pub type IO_MASTER2_R = crate::R<bool, IO_MASTER2_A>;
impl IO_MASTER2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_MASTER2_A {
        match self.bits {
            true => IO_MASTER2_A::EN,
            false => IO_MASTER2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IO_MASTER2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IO_MASTER2_A::DIS
    }
}
#[doc = "Write proxy for field `IO_MASTER2`"]
pub struct IO_MASTER2_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MASTER2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_MASTER2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IO MASTER 2"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IO_MASTER2_A::EN)
    }
    #[doc = "Disables IO MASTER 2"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IO_MASTER2_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable IO MASTER 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_MASTER1_A {
    #[doc = "1: Enable IO MASTER 1"]
    EN = 1,
    #[doc = "0: Disables IO MASTER 1"]
    DIS = 0,
}
impl From<IO_MASTER1_A> for bool {
    #[inline(always)]
    fn from(variant: IO_MASTER1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IO_MASTER1`"]
pub type IO_MASTER1_R = crate::R<bool, IO_MASTER1_A>;
impl IO_MASTER1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_MASTER1_A {
        match self.bits {
            true => IO_MASTER1_A::EN,
            false => IO_MASTER1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IO_MASTER1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IO_MASTER1_A::DIS
    }
}
#[doc = "Write proxy for field `IO_MASTER1`"]
pub struct IO_MASTER1_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MASTER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_MASTER1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IO MASTER 1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IO_MASTER1_A::EN)
    }
    #[doc = "Disables IO MASTER 1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IO_MASTER1_A::DIS)
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
#[doc = "Enable IO MASTER 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_MASTER0_A {
    #[doc = "1: Enable IO MASTER 0"]
    EN = 1,
    #[doc = "0: Disables IO MASTER 0"]
    DIS = 0,
}
impl From<IO_MASTER0_A> for bool {
    #[inline(always)]
    fn from(variant: IO_MASTER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IO_MASTER0`"]
pub type IO_MASTER0_R = crate::R<bool, IO_MASTER0_A>;
impl IO_MASTER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_MASTER0_A {
        match self.bits {
            true => IO_MASTER0_A::EN,
            false => IO_MASTER0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IO_MASTER0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IO_MASTER0_A::DIS
    }
}
#[doc = "Write proxy for field `IO_MASTER0`"]
pub struct IO_MASTER0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MASTER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_MASTER0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IO MASTER 0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IO_MASTER0_A::EN)
    }
    #[doc = "Disables IO MASTER 0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IO_MASTER0_A::DIS)
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
#[doc = "Enable IO SLAVE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SLAVE_A {
    #[doc = "1: Enable IO SLAVE"]
    EN = 1,
    #[doc = "0: Disables IO SLAVE"]
    DIS = 0,
}
impl From<IO_SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IO_SLAVE`"]
pub type IO_SLAVE_R = crate::R<bool, IO_SLAVE_A>;
impl IO_SLAVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SLAVE_A {
        match self.bits {
            true => IO_SLAVE_A::EN,
            false => IO_SLAVE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IO_SLAVE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IO_SLAVE_A::DIS
    }
}
#[doc = "Write proxy for field `IO_SLAVE`"]
pub struct IO_SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SLAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SLAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable IO SLAVE"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IO_SLAVE_A::EN)
    }
    #[doc = "Disables IO SLAVE"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IO_SLAVE_A::DIS)
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
    #[doc = "Bit 10 - Enable PDM Digital Block"]
    #[inline(always)]
    pub fn pwrpdm(&self) -> PWRPDM_R {
        PWRPDM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable ADC Digital Block"]
    #[inline(always)]
    pub fn pwradc(&self) -> PWRADC_R {
        PWRADC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable UART 1"]
    #[inline(always)]
    pub fn pwruart1(&self) -> PWRUART1_R {
        PWRUART1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable UART 0"]
    #[inline(always)]
    pub fn pwruart0(&self) -> PWRUART0_R {
        PWRUART0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable IO MASTER 5"]
    #[inline(always)]
    pub fn io_master5(&self) -> IO_MASTER5_R {
        IO_MASTER5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable IO MASTER 4"]
    #[inline(always)]
    pub fn io_master4(&self) -> IO_MASTER4_R {
        IO_MASTER4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable IO MASTER 3"]
    #[inline(always)]
    pub fn io_master3(&self) -> IO_MASTER3_R {
        IO_MASTER3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable IO MASTER 2"]
    #[inline(always)]
    pub fn io_master2(&self) -> IO_MASTER2_R {
        IO_MASTER2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable IO MASTER 1"]
    #[inline(always)]
    pub fn io_master1(&self) -> IO_MASTER1_R {
        IO_MASTER1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable IO MASTER 0"]
    #[inline(always)]
    pub fn io_master0(&self) -> IO_MASTER0_R {
        IO_MASTER0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable IO SLAVE"]
    #[inline(always)]
    pub fn io_slave(&self) -> IO_SLAVE_R {
        IO_SLAVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Enable PDM Digital Block"]
    #[inline(always)]
    pub fn pwrpdm(&mut self) -> PWRPDM_W {
        PWRPDM_W { w: self }
    }
    #[doc = "Bit 9 - Enable ADC Digital Block"]
    #[inline(always)]
    pub fn pwradc(&mut self) -> PWRADC_W {
        PWRADC_W { w: self }
    }
    #[doc = "Bit 8 - Enable UART 1"]
    #[inline(always)]
    pub fn pwruart1(&mut self) -> PWRUART1_W {
        PWRUART1_W { w: self }
    }
    #[doc = "Bit 7 - Enable UART 0"]
    #[inline(always)]
    pub fn pwruart0(&mut self) -> PWRUART0_W {
        PWRUART0_W { w: self }
    }
    #[doc = "Bit 6 - Enable IO MASTER 5"]
    #[inline(always)]
    pub fn io_master5(&mut self) -> IO_MASTER5_W {
        IO_MASTER5_W { w: self }
    }
    #[doc = "Bit 5 - Enable IO MASTER 4"]
    #[inline(always)]
    pub fn io_master4(&mut self) -> IO_MASTER4_W {
        IO_MASTER4_W { w: self }
    }
    #[doc = "Bit 4 - Enable IO MASTER 3"]
    #[inline(always)]
    pub fn io_master3(&mut self) -> IO_MASTER3_W {
        IO_MASTER3_W { w: self }
    }
    #[doc = "Bit 3 - Enable IO MASTER 2"]
    #[inline(always)]
    pub fn io_master2(&mut self) -> IO_MASTER2_W {
        IO_MASTER2_W { w: self }
    }
    #[doc = "Bit 2 - Enable IO MASTER 1"]
    #[inline(always)]
    pub fn io_master1(&mut self) -> IO_MASTER1_W {
        IO_MASTER1_W { w: self }
    }
    #[doc = "Bit 1 - Enable IO MASTER 0"]
    #[inline(always)]
    pub fn io_master0(&mut self) -> IO_MASTER0_W {
        IO_MASTER0_W { w: self }
    }
    #[doc = "Bit 0 - Enable IO SLAVE"]
    #[inline(always)]
    pub fn io_slave(&mut self) -> IO_SLAVE_W {
        IO_SLAVE_W { w: self }
    }
}
