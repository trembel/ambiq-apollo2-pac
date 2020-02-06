#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "This bit enables the IO Master.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCEN_A {
    #[doc = "0: Disable the IO Master."]
    DIS = 0,
    #[doc = "1: Enable the IO Master."]
    EN = 1,
}
impl From<IFCEN_A> for bool {
    #[inline(always)]
    fn from(variant: IFCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFCEN`"]
pub type IFCEN_R = crate::R<bool, IFCEN_A>;
impl IFCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFCEN_A {
        match self.bits {
            false => IFCEN_A::DIS,
            true => IFCEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IFCEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IFCEN_A::EN
    }
}
#[doc = "Write proxy for field `IFCEN`"]
pub struct IFCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the IO Master."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IFCEN_A::DIS)
    }
    #[doc = "Enable the IO Master."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IFCEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "This bit selects the read flow control signal polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFCPOL_A {
    #[doc = "0: Flow control signal high creates flow control."]
    HIGH = 0,
    #[doc = "1: Flow control signal low creates flow control."]
    LOW = 1,
}
impl From<RDFCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RDFCPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDFCPOL`"]
pub type RDFCPOL_R = crate::R<bool, RDFCPOL_A>;
impl RDFCPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDFCPOL_A {
        match self.bits {
            false => RDFCPOL_A::HIGH,
            true => RDFCPOL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RDFCPOL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RDFCPOL_A::LOW
    }
}
#[doc = "Write proxy for field `RDFCPOL`"]
pub struct RDFCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RDFCPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDFCPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flow control signal high creates flow control."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RDFCPOL_A::HIGH)
    }
    #[doc = "Flow control signal low creates flow control."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RDFCPOL_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "This bit selects the write flow control signal polarity.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTFCPOL_A {
    #[doc = "0: Flow control signal high creates flow control."]
    HIGH = 0,
    #[doc = "1: Flow control signal low creates flow control."]
    LOW = 1,
}
impl From<WTFCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WTFCPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTFCPOL`"]
pub type WTFCPOL_R = crate::R<bool, WTFCPOL_A>;
impl WTFCPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTFCPOL_A {
        match self.bits {
            false => WTFCPOL_A::HIGH,
            true => WTFCPOL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WTFCPOL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WTFCPOL_A::LOW
    }
}
#[doc = "Write proxy for field `WTFCPOL`"]
pub struct WTFCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WTFCPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTFCPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flow control signal high creates flow control."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WTFCPOL_A::HIGH)
    }
    #[doc = "Flow control signal low creates flow control."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WTFCPOL_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "This bit selects the write mode flow control signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTFCIRQ_A {
    #[doc = "0: MISO is used as the write mode flow control signal."]
    MISO = 0,
    #[doc = "1: IRQ is used as the write mode flow control signal."]
    IRQ = 1,
}
impl From<WTFCIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: WTFCIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTFCIRQ`"]
pub type WTFCIRQ_R = crate::R<bool, WTFCIRQ_A>;
impl WTFCIRQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTFCIRQ_A {
        match self.bits {
            false => WTFCIRQ_A::MISO,
            true => WTFCIRQ_A::IRQ,
        }
    }
    #[doc = "Checks if the value of the field is `MISO`"]
    #[inline(always)]
    pub fn is_miso(&self) -> bool {
        *self == WTFCIRQ_A::MISO
    }
    #[doc = "Checks if the value of the field is `IRQ`"]
    #[inline(always)]
    pub fn is_irq(&self) -> bool {
        *self == WTFCIRQ_A::IRQ
    }
}
#[doc = "Write proxy for field `WTFCIRQ`"]
pub struct WTFCIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WTFCIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTFCIRQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MISO is used as the write mode flow control signal."]
    #[inline(always)]
    pub fn miso(self) -> &'a mut W {
        self.variant(WTFCIRQ_A::MISO)
    }
    #[doc = "IRQ is used as the write mode flow control signal."]
    #[inline(always)]
    pub fn irq(self) -> &'a mut W {
        self.variant(WTFCIRQ_A::IRQ)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FCDEL`"]
pub type FCDEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCDEL`"]
pub struct FCDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FCDEL_W<'a> {
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
#[doc = "This bit invewrts MOSI when flow control is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOSIINV_A {
    #[doc = "0: MOSI is set to 0 in read mode and 1 in write mode."]
    NORMAL = 0,
    #[doc = "1: MOSI is set to 1 in read mode and 0 in write mode."]
    INVERT = 1,
}
impl From<MOSIINV_A> for bool {
    #[inline(always)]
    fn from(variant: MOSIINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MOSIINV`"]
pub type MOSIINV_R = crate::R<bool, MOSIINV_A>;
impl MOSIINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOSIINV_A {
        match self.bits {
            false => MOSIINV_A::NORMAL,
            true => MOSIINV_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MOSIINV_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == MOSIINV_A::INVERT
    }
}
#[doc = "Write proxy for field `MOSIINV`"]
pub struct MOSIINV_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSIINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOSIINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MOSI is set to 0 in read mode and 1 in write mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MOSIINV_A::NORMAL)
    }
    #[doc = "MOSI is set to 1 in read mode and 0 in write mode."]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(MOSIINV_A::INVERT)
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
#[doc = "This bit enables read mode flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFC_A {
    #[doc = "0: Read mode flow control disabled."]
    DIS = 0,
    #[doc = "1: Read mode flow control enabled."]
    EN = 1,
}
impl From<RDFC_A> for bool {
    #[inline(always)]
    fn from(variant: RDFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDFC`"]
pub type RDFC_R = crate::R<bool, RDFC_A>;
impl RDFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDFC_A {
        match self.bits {
            false => RDFC_A::DIS,
            true => RDFC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RDFC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RDFC_A::EN
    }
}
#[doc = "Write proxy for field `RDFC`"]
pub struct RDFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read mode flow control disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RDFC_A::DIS)
    }
    #[doc = "Read mode flow control enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RDFC_A::EN)
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
#[doc = "This bit enables write mode flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTFC_A {
    #[doc = "0: Write mode flow control disabled."]
    DIS = 0,
    #[doc = "1: Write mode flow control enabled."]
    EN = 1,
}
impl From<WTFC_A> for bool {
    #[inline(always)]
    fn from(variant: WTFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTFC`"]
pub type WTFC_R = crate::R<bool, WTFC_A>;
impl WTFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTFC_A {
        match self.bits {
            false => WTFC_A::DIS,
            true => WTFC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WTFC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WTFC_A::EN
    }
}
#[doc = "Write proxy for field `WTFC`"]
pub struct WTFC_W<'a> {
    w: &'a mut W,
}
impl<'a> WTFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write mode flow control disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WTFC_A::DIS)
    }
    #[doc = "Write mode flow control enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WTFC_A::EN)
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
#[doc = "This bit selects the preread timing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTRD_A {
    #[doc = "0: 0 read delay cycles."]
    PRERD0 = 0,
    #[doc = "1: 1 read delay cycles."]
    PRERD1 = 1,
    #[doc = "2: 2 read delay cycles."]
    PRERD2 = 2,
    #[doc = "3: 3 read delay cycles."]
    PRERD3 = 3,
}
impl From<STARTRD_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTRD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STARTRD`"]
pub type STARTRD_R = crate::R<u8, STARTRD_A>;
impl STARTRD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTRD_A {
        match self.bits {
            0 => STARTRD_A::PRERD0,
            1 => STARTRD_A::PRERD1,
            2 => STARTRD_A::PRERD2,
            3 => STARTRD_A::PRERD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRERD0`"]
    #[inline(always)]
    pub fn is_prerd0(&self) -> bool {
        *self == STARTRD_A::PRERD0
    }
    #[doc = "Checks if the value of the field is `PRERD1`"]
    #[inline(always)]
    pub fn is_prerd1(&self) -> bool {
        *self == STARTRD_A::PRERD1
    }
    #[doc = "Checks if the value of the field is `PRERD2`"]
    #[inline(always)]
    pub fn is_prerd2(&self) -> bool {
        *self == STARTRD_A::PRERD2
    }
    #[doc = "Checks if the value of the field is `PRERD3`"]
    #[inline(always)]
    pub fn is_prerd3(&self) -> bool {
        *self == STARTRD_A::PRERD3
    }
}
#[doc = "Write proxy for field `STARTRD`"]
pub struct STARTRD_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTRD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0 read delay cycles."]
    #[inline(always)]
    pub fn prerd0(self) -> &'a mut W {
        self.variant(STARTRD_A::PRERD0)
    }
    #[doc = "1 read delay cycles."]
    #[inline(always)]
    pub fn prerd1(self) -> &'a mut W {
        self.variant(STARTRD_A::PRERD1)
    }
    #[doc = "2 read delay cycles."]
    #[inline(always)]
    pub fn prerd2(self) -> &'a mut W {
        self.variant(STARTRD_A::PRERD2)
    }
    #[doc = "3 read delay cycles."]
    #[inline(always)]
    pub fn prerd3(self) -> &'a mut W {
        self.variant(STARTRD_A::PRERD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "This bit selects full duplex mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULLDUP_A {
    #[doc = "0: 128 byte FIFO in half duplex mode."]
    NORMAL = 0,
    #[doc = "1: 64 byte FIFO in full duplex mode."]
    FULLDUP = 1,
}
impl From<FULLDUP_A> for bool {
    #[inline(always)]
    fn from(variant: FULLDUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FULLDUP`"]
pub type FULLDUP_R = crate::R<bool, FULLDUP_A>;
impl FULLDUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FULLDUP_A {
        match self.bits {
            false => FULLDUP_A::NORMAL,
            true => FULLDUP_A::FULLDUP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FULLDUP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FULLDUP`"]
    #[inline(always)]
    pub fn is_fulldup(&self) -> bool {
        *self == FULLDUP_A::FULLDUP
    }
}
#[doc = "Write proxy for field `FULLDUP`"]
pub struct FULLDUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FULLDUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "128 byte FIFO in half duplex mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FULLDUP_A::NORMAL)
    }
    #[doc = "64 byte FIFO in full duplex mode."]
    #[inline(always)]
    pub fn fulldup(self) -> &'a mut W {
        self.variant(FULLDUP_A::FULLDUP)
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
#[doc = "This bit selects SPI phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPHA_A {
    #[doc = "0: Sample on the leading (first) clock edge."]
    SAMPLE_LEADING_EDGE = 0,
    #[doc = "1: Sample on the trailing (second) clock edge."]
    SAMPLE_TRAILING_EDGE = 1,
}
impl From<SPHA_A> for bool {
    #[inline(always)]
    fn from(variant: SPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPHA`"]
pub type SPHA_R = crate::R<bool, SPHA_A>;
impl SPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPHA_A {
        match self.bits {
            false => SPHA_A::SAMPLE_LEADING_EDGE,
            true => SPHA_A::SAMPLE_TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_LEADING_EDGE`"]
    #[inline(always)]
    pub fn is_sample_leading_edge(&self) -> bool {
        *self == SPHA_A::SAMPLE_LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `SAMPLE_TRAILING_EDGE`"]
    #[inline(always)]
    pub fn is_sample_trailing_edge(&self) -> bool {
        *self == SPHA_A::SAMPLE_TRAILING_EDGE
    }
}
#[doc = "Write proxy for field `SPHA`"]
pub struct SPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sample on the leading (first) clock edge."]
    #[inline(always)]
    pub fn sample_leading_edge(self) -> &'a mut W {
        self.variant(SPHA_A::SAMPLE_LEADING_EDGE)
    }
    #[doc = "Sample on the trailing (second) clock edge."]
    #[inline(always)]
    pub fn sample_trailing_edge(self) -> &'a mut W {
        self.variant(SPHA_A::SAMPLE_TRAILING_EDGE)
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
#[doc = "This bit selects SPI polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL_A {
    #[doc = "0: The base value of the clock is 0."]
    CLK_BASE_0 = 0,
    #[doc = "1: The base value of the clock is 1."]
    CLK_BASE_1 = 1,
}
impl From<SPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPOL`"]
pub type SPOL_R = crate::R<bool, SPOL_A>;
impl SPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            false => SPOL_A::CLK_BASE_0,
            true => SPOL_A::CLK_BASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_BASE_0`"]
    #[inline(always)]
    pub fn is_clk_base_0(&self) -> bool {
        *self == SPOL_A::CLK_BASE_0
    }
    #[doc = "Checks if the value of the field is `CLK_BASE_1`"]
    #[inline(always)]
    pub fn is_clk_base_1(&self) -> bool {
        *self == SPOL_A::CLK_BASE_1
    }
}
#[doc = "Write proxy for field `SPOL`"]
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The base value of the clock is 0."]
    #[inline(always)]
    pub fn clk_base_0(self) -> &'a mut W {
        self.variant(SPOL_A::CLK_BASE_0)
    }
    #[doc = "The base value of the clock is 1."]
    #[inline(always)]
    pub fn clk_base_1(self) -> &'a mut W {
        self.variant(SPOL_A::CLK_BASE_1)
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
#[doc = "This bit selects the I/O interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCSEL_A {
    #[doc = "0: Selects I2C interface for the I/O Master."]
    I2C = 0,
    #[doc = "1: Selects SPI interface for the I/O Master."]
    SPI = 1,
}
impl From<IFCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IFCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IFCSEL`"]
pub type IFCSEL_R = crate::R<bool, IFCSEL_A>;
impl IFCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFCSEL_A {
        match self.bits {
            false => IFCSEL_A::I2C,
            true => IFCSEL_A::SPI,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == IFCSEL_A::I2C
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == IFCSEL_A::SPI
    }
}
#[doc = "Write proxy for field `IFCSEL`"]
pub struct IFCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects I2C interface for the I/O Master."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(IFCSEL_A::I2C)
    }
    #[doc = "Selects SPI interface for the I/O Master."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(IFCSEL_A::SPI)
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
    #[doc = "Bit 31 - This bit enables the IO Master."]
    #[inline(always)]
    pub fn ifcen(&self) -> IFCEN_R {
        IFCEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit selects the read flow control signal polarity."]
    #[inline(always)]
    pub fn rdfcpol(&self) -> RDFCPOL_R {
        RDFCPOL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit selects the write flow control signal polarity."]
    #[inline(always)]
    pub fn wtfcpol(&self) -> WTFCPOL_R {
        WTFCPOL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit selects the write mode flow control signal."]
    #[inline(always)]
    pub fn wtfcirq(&self) -> WTFCIRQ_R {
        WTFCIRQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit must be left at the default value of 0."]
    #[inline(always)]
    pub fn fcdel(&self) -> FCDEL_R {
        FCDEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit invewrts MOSI when flow control is enabled."]
    #[inline(always)]
    pub fn mosiinv(&self) -> MOSIINV_R {
        MOSIINV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit enables read mode flow control."]
    #[inline(always)]
    pub fn rdfc(&self) -> RDFC_R {
        RDFC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit enables write mode flow control."]
    #[inline(always)]
    pub fn wtfc(&self) -> WTFC_R {
        WTFC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - This bit selects the preread timing."]
    #[inline(always)]
    pub fn startrd(&self) -> STARTRD_R {
        STARTRD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - This bit selects full duplex mode."]
    #[inline(always)]
    pub fn fulldup(&self) -> FULLDUP_R {
        FULLDUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit selects SPI phase."]
    #[inline(always)]
    pub fn spha(&self) -> SPHA_R {
        SPHA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline(always)]
    pub fn ifcsel(&self) -> IFCSEL_R {
        IFCSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - This bit enables the IO Master."]
    #[inline(always)]
    pub fn ifcen(&mut self) -> IFCEN_W {
        IFCEN_W { w: self }
    }
    #[doc = "Bit 14 - This bit selects the read flow control signal polarity."]
    #[inline(always)]
    pub fn rdfcpol(&mut self) -> RDFCPOL_W {
        RDFCPOL_W { w: self }
    }
    #[doc = "Bit 13 - This bit selects the write flow control signal polarity."]
    #[inline(always)]
    pub fn wtfcpol(&mut self) -> WTFCPOL_W {
        WTFCPOL_W { w: self }
    }
    #[doc = "Bit 12 - This bit selects the write mode flow control signal."]
    #[inline(always)]
    pub fn wtfcirq(&mut self) -> WTFCIRQ_W {
        WTFCIRQ_W { w: self }
    }
    #[doc = "Bit 11 - This bit must be left at the default value of 0."]
    #[inline(always)]
    pub fn fcdel(&mut self) -> FCDEL_W {
        FCDEL_W { w: self }
    }
    #[doc = "Bit 10 - This bit invewrts MOSI when flow control is enabled."]
    #[inline(always)]
    pub fn mosiinv(&mut self) -> MOSIINV_W {
        MOSIINV_W { w: self }
    }
    #[doc = "Bit 9 - This bit enables read mode flow control."]
    #[inline(always)]
    pub fn rdfc(&mut self) -> RDFC_W {
        RDFC_W { w: self }
    }
    #[doc = "Bit 8 - This bit enables write mode flow control."]
    #[inline(always)]
    pub fn wtfc(&mut self) -> WTFC_W {
        WTFC_W { w: self }
    }
    #[doc = "Bits 4:5 - This bit selects the preread timing."]
    #[inline(always)]
    pub fn startrd(&mut self) -> STARTRD_W {
        STARTRD_W { w: self }
    }
    #[doc = "Bit 3 - This bit selects full duplex mode."]
    #[inline(always)]
    pub fn fulldup(&mut self) -> FULLDUP_W {
        FULLDUP_W { w: self }
    }
    #[doc = "Bit 2 - This bit selects SPI phase."]
    #[inline(always)]
    pub fn spha(&mut self) -> SPHA_W {
        SPHA_W { w: self }
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline(always)]
    pub fn ifcsel(&mut self) -> IFCSEL_W {
        IFCSEL_W { w: self }
    }
}
