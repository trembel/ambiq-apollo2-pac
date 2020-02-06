#[doc = "Reader of register SRAMPWDINSLEEP"]
pub type R = crate::R<u32, super::SRAMPWDINSLEEP>;
#[doc = "Writer for register SRAMPWDINSLEEP"]
pub type W = crate::W<u32, super::SRAMPWDINSLEEP>;
#[doc = "Register SRAMPWDINSLEEP `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAMPWDINSLEEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable CACHE BANKS to power down in deep sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_PWD_SLP_A {
    #[doc = "1: CACHE BANKS POWER DOWN in CORE SLEEP"]
    EN = 1,
    #[doc = "0: CACHE BANKS STAYS in Retention in CORE SLEEP"]
    DIS = 0,
}
impl From<CACHE_PWD_SLP_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_PWD_SLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHE_PWD_SLP`"]
pub type CACHE_PWD_SLP_R = crate::R<bool, CACHE_PWD_SLP_A>;
impl CACHE_PWD_SLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_PWD_SLP_A {
        match self.bits {
            true => CACHE_PWD_SLP_A::EN,
            false => CACHE_PWD_SLP_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHE_PWD_SLP_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHE_PWD_SLP_A::DIS
    }
}
#[doc = "Write proxy for field `CACHE_PWD_SLP`"]
pub struct CACHE_PWD_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_PWD_SLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHE_PWD_SLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CACHE BANKS POWER DOWN in CORE SLEEP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHE_PWD_SLP_A::EN)
    }
    #[doc = "CACHE BANKS STAYS in Retention in CORE SLEEP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHE_PWD_SLP_A::DIS)
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
#[doc = "Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SRAMSLEEPPOWERDOWN_A {
    #[doc = "0: All banks retained"]
    NONE = 0,
    #[doc = "1: 0KB-8KB SRAM"]
    GROUP0_SRAM0 = 1,
    #[doc = "2: 8KB-16KB SRAM"]
    GROUP0_SRAM1 = 2,
    #[doc = "4: 16KB-24KB SRAM"]
    GROUP0_SRAM2 = 4,
    #[doc = "8: 24KB-32KB SRAM"]
    GROUP0_SRAM3 = 8,
    #[doc = "16: 32KB-64KB SRAMs"]
    GROUP1 = 16,
    #[doc = "32: 64KB-96KB SRAMs"]
    GROUP2 = 32,
    #[doc = "64: 96KB-128KB SRAMs"]
    GROUP3 = 64,
    #[doc = "128: 128KB-160KB SRAMs"]
    GROUP4 = 128,
    #[doc = "256: 160KB-192KB SRAMs"]
    GROUP5 = 256,
    #[doc = "512: 192KB-224KB SRAMs"]
    GROUP6 = 512,
    #[doc = "1024: 224KB-256KB SRAMs"]
    GROUP7 = 1024,
    #[doc = "3: Do not Retain lower 16KB"]
    SRAM16K = 3,
    #[doc = "15: Do not Retain lower 32KB"]
    SRAM32K = 15,
    #[doc = "31: Do not Retain lower 64KB"]
    SRAM64K = 31,
    #[doc = "127: Do not Retain lower 128KB"]
    SRAM128K = 127,
    #[doc = "2046: All banks but lower 8k powered down."]
    ALLBUTLOWER8K = 2046,
    #[doc = "2044: All banks but lower 16k powered down."]
    ALLBUTLOWER16K = 2044,
    #[doc = "2040: All banks but lower 24k powered down."]
    ALLBUTLOWER24K = 2040,
    #[doc = "2032: All banks but lower 32k powered down."]
    ALLBUTLOWER32K = 2032,
    #[doc = "2016: All banks but lower 64k powered down."]
    ALLBUTLOWER64K = 2016,
    #[doc = "1920: All banks but lower 128k powered down."]
    ALLBUTLOWER128K = 1920,
    #[doc = "2047: All banks powered down."]
    ALL = 2047,
}
impl From<SRAMSLEEPPOWERDOWN_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAMSLEEPPOWERDOWN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMSLEEPPOWERDOWN`"]
pub type SRAMSLEEPPOWERDOWN_R = crate::R<u16, SRAMSLEEPPOWERDOWN_A>;
impl SRAMSLEEPPOWERDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SRAMSLEEPPOWERDOWN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRAMSLEEPPOWERDOWN_A::NONE),
            1 => Val(SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM0),
            2 => Val(SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM1),
            4 => Val(SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM2),
            8 => Val(SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM3),
            16 => Val(SRAMSLEEPPOWERDOWN_A::GROUP1),
            32 => Val(SRAMSLEEPPOWERDOWN_A::GROUP2),
            64 => Val(SRAMSLEEPPOWERDOWN_A::GROUP3),
            128 => Val(SRAMSLEEPPOWERDOWN_A::GROUP4),
            256 => Val(SRAMSLEEPPOWERDOWN_A::GROUP5),
            512 => Val(SRAMSLEEPPOWERDOWN_A::GROUP6),
            1024 => Val(SRAMSLEEPPOWERDOWN_A::GROUP7),
            3 => Val(SRAMSLEEPPOWERDOWN_A::SRAM16K),
            15 => Val(SRAMSLEEPPOWERDOWN_A::SRAM32K),
            31 => Val(SRAMSLEEPPOWERDOWN_A::SRAM64K),
            127 => Val(SRAMSLEEPPOWERDOWN_A::SRAM128K),
            2046 => Val(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER8K),
            2044 => Val(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER16K),
            2040 => Val(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER24K),
            2032 => Val(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER32K),
            2016 => Val(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER64K),
            1920 => Val(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER128K),
            2047 => Val(SRAMSLEEPPOWERDOWN_A::ALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0_SRAM0`"]
    #[inline(always)]
    pub fn is_group0_sram0(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM0
    }
    #[doc = "Checks if the value of the field is `GROUP0_SRAM1`"]
    #[inline(always)]
    pub fn is_group0_sram1(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM1
    }
    #[doc = "Checks if the value of the field is `GROUP0_SRAM2`"]
    #[inline(always)]
    pub fn is_group0_sram2(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM2
    }
    #[doc = "Checks if the value of the field is `GROUP0_SRAM3`"]
    #[inline(always)]
    pub fn is_group0_sram3(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM3
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP1
    }
    #[doc = "Checks if the value of the field is `GROUP2`"]
    #[inline(always)]
    pub fn is_group2(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP2
    }
    #[doc = "Checks if the value of the field is `GROUP3`"]
    #[inline(always)]
    pub fn is_group3(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP3
    }
    #[doc = "Checks if the value of the field is `GROUP4`"]
    #[inline(always)]
    pub fn is_group4(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP4
    }
    #[doc = "Checks if the value of the field is `GROUP5`"]
    #[inline(always)]
    pub fn is_group5(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP5
    }
    #[doc = "Checks if the value of the field is `GROUP6`"]
    #[inline(always)]
    pub fn is_group6(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP6
    }
    #[doc = "Checks if the value of the field is `GROUP7`"]
    #[inline(always)]
    pub fn is_group7(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::GROUP7
    }
    #[doc = "Checks if the value of the field is `SRAM16K`"]
    #[inline(always)]
    pub fn is_sram16k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::SRAM16K
    }
    #[doc = "Checks if the value of the field is `SRAM32K`"]
    #[inline(always)]
    pub fn is_sram32k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::SRAM32K
    }
    #[doc = "Checks if the value of the field is `SRAM64K`"]
    #[inline(always)]
    pub fn is_sram64k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::SRAM64K
    }
    #[doc = "Checks if the value of the field is `SRAM128K`"]
    #[inline(always)]
    pub fn is_sram128k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::SRAM128K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER8K`"]
    #[inline(always)]
    pub fn is_allbutlower8k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER8K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER16K`"]
    #[inline(always)]
    pub fn is_allbutlower16k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER16K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER24K`"]
    #[inline(always)]
    pub fn is_allbutlower24k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER24K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER32K`"]
    #[inline(always)]
    pub fn is_allbutlower32k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER32K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER64K`"]
    #[inline(always)]
    pub fn is_allbutlower64k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER64K
    }
    #[doc = "Checks if the value of the field is `ALLBUTLOWER128K`"]
    #[inline(always)]
    pub fn is_allbutlower128k(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER128K
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == SRAMSLEEPPOWERDOWN_A::ALL
    }
}
#[doc = "Write proxy for field `SRAMSLEEPPOWERDOWN`"]
pub struct SRAMSLEEPPOWERDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMSLEEPPOWERDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMSLEEPPOWERDOWN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All banks retained"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::NONE)
    }
    #[doc = "0KB-8KB SRAM"]
    #[inline(always)]
    pub fn group0_sram0(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM0)
    }
    #[doc = "8KB-16KB SRAM"]
    #[inline(always)]
    pub fn group0_sram1(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM1)
    }
    #[doc = "16KB-24KB SRAM"]
    #[inline(always)]
    pub fn group0_sram2(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM2)
    }
    #[doc = "24KB-32KB SRAM"]
    #[inline(always)]
    pub fn group0_sram3(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP0_SRAM3)
    }
    #[doc = "32KB-64KB SRAMs"]
    #[inline(always)]
    pub fn group1(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP1)
    }
    #[doc = "64KB-96KB SRAMs"]
    #[inline(always)]
    pub fn group2(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP2)
    }
    #[doc = "96KB-128KB SRAMs"]
    #[inline(always)]
    pub fn group3(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP3)
    }
    #[doc = "128KB-160KB SRAMs"]
    #[inline(always)]
    pub fn group4(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP4)
    }
    #[doc = "160KB-192KB SRAMs"]
    #[inline(always)]
    pub fn group5(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP5)
    }
    #[doc = "192KB-224KB SRAMs"]
    #[inline(always)]
    pub fn group6(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP6)
    }
    #[doc = "224KB-256KB SRAMs"]
    #[inline(always)]
    pub fn group7(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::GROUP7)
    }
    #[doc = "Do not Retain lower 16KB"]
    #[inline(always)]
    pub fn sram16k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::SRAM16K)
    }
    #[doc = "Do not Retain lower 32KB"]
    #[inline(always)]
    pub fn sram32k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::SRAM32K)
    }
    #[doc = "Do not Retain lower 64KB"]
    #[inline(always)]
    pub fn sram64k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::SRAM64K)
    }
    #[doc = "Do not Retain lower 128KB"]
    #[inline(always)]
    pub fn sram128k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::SRAM128K)
    }
    #[doc = "All banks but lower 8k powered down."]
    #[inline(always)]
    pub fn allbutlower8k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER8K)
    }
    #[doc = "All banks but lower 16k powered down."]
    #[inline(always)]
    pub fn allbutlower16k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER16K)
    }
    #[doc = "All banks but lower 24k powered down."]
    #[inline(always)]
    pub fn allbutlower24k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER24K)
    }
    #[doc = "All banks but lower 32k powered down."]
    #[inline(always)]
    pub fn allbutlower32k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER32K)
    }
    #[doc = "All banks but lower 64k powered down."]
    #[inline(always)]
    pub fn allbutlower64k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER64K)
    }
    #[doc = "All banks but lower 128k powered down."]
    #[inline(always)]
    pub fn allbutlower128k(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::ALLBUTLOWER128K)
    }
    #[doc = "All banks powered down."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(SRAMSLEEPPOWERDOWN_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enable CACHE BANKS to power down in deep sleep"]
    #[inline(always)]
    pub fn cache_pwd_slp(&self) -> CACHE_PWD_SLP_R {
        CACHE_PWD_SLP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn sramsleeppowerdown(&self) -> SRAMSLEEPPOWERDOWN_R {
        SRAMSLEEPPOWERDOWN_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Enable CACHE BANKS to power down in deep sleep"]
    #[inline(always)]
    pub fn cache_pwd_slp(&mut self) -> CACHE_PWD_SLP_W {
        CACHE_PWD_SLP_W { w: self }
    }
    #[doc = "Bits 0:10 - Selects which SRAM banks are powered down in deep sleep mode, causing the contents of the bank to be lost."]
    #[inline(always)]
    pub fn sramsleeppowerdown(&mut self) -> SRAMSLEEPPOWERDOWN_W {
        SRAMSLEEPPOWERDOWN_W { w: self }
    }
}
