#[doc = "Reader of register MEMEN"]
pub type R = crate::R<u32, super::MEMEN>;
#[doc = "Writer for register MEMEN"]
pub type W = crate::W<u32, super::MEMEN>;
#[doc = "Register MEMEN `reset()`'s with value 0xa000_1fff"]
impl crate::ResetValue for super::MEMEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa000_1fff
    }
}
#[doc = "Enable CACHE BANK 2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB2_A {
    #[doc = "1: Enable CACHE BANK 2"]
    EN = 1,
    #[doc = "0: Disable CACHE BANK 2"]
    DIS = 0,
}
impl From<CACHEB2_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEB2`"]
pub type CACHEB2_R = crate::R<bool, CACHEB2_A>;
impl CACHEB2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB2_A {
        match self.bits {
            true => CACHEB2_A::EN,
            false => CACHEB2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHEB2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB2_A::DIS
    }
}
#[doc = "Write proxy for field `CACHEB2`"]
pub struct CACHEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable CACHE BANK 2"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB2_A::EN)
    }
    #[doc = "Disable CACHE BANK 2"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB2_A::DIS)
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
#[doc = "Enable CACHE BANK 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB0_A {
    #[doc = "1: Enable CACHE BANK 0"]
    EN = 1,
    #[doc = "0: Disable CACHE BANK 0"]
    DIS = 0,
}
impl From<CACHEB0_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEB0`"]
pub type CACHEB0_R = crate::R<bool, CACHEB0_A>;
impl CACHEB0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB0_A {
        match self.bits {
            true => CACHEB0_A::EN,
            false => CACHEB0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHEB0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB0_A::DIS
    }
}
#[doc = "Write proxy for field `CACHEB0`"]
pub struct CACHEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable CACHE BANK 0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB0_A::EN)
    }
    #[doc = "Disable CACHE BANK 0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB0_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Enable FLASH1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1_A {
    #[doc = "1: Enable FLASH1"]
    EN = 1,
    #[doc = "0: Disables FLASH1"]
    DIS = 0,
}
impl From<FLASH1_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH1`"]
pub type FLASH1_R = crate::R<bool, FLASH1_A>;
impl FLASH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH1_A {
        match self.bits {
            true => FLASH1_A::EN,
            false => FLASH1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLASH1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLASH1_A::DIS
    }
}
#[doc = "Write proxy for field `FLASH1`"]
pub struct FLASH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable FLASH1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1_A::EN)
    }
    #[doc = "Disables FLASH1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1_A::DIS)
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
#[doc = "Enable FLASH 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0_A {
    #[doc = "1: Enable FLASH 0"]
    EN = 1,
    #[doc = "0: Disables FLASH 0"]
    DIS = 0,
}
impl From<FLASH0_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH0`"]
pub type FLASH0_R = crate::R<bool, FLASH0_A>;
impl FLASH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH0_A {
        match self.bits {
            true => FLASH0_A::EN,
            false => FLASH0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLASH0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLASH0_A::DIS
    }
}
#[doc = "Write proxy for field `FLASH0`"]
pub struct FLASH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable FLASH 0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0_A::EN)
    }
    #[doc = "Disables FLASH 0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Enables power for selected SRAM banks (else an access to its address space to generate a Hard Fault).\n\nValue on reset: 2047"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SRAMEN_A {
    #[doc = "0: All banks disabled"]
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
    #[doc = "3: ENABLE lower 16KB"]
    SRAM16K = 3,
    #[doc = "15: ENABLE lower 32KB"]
    SRAM32K = 15,
    #[doc = "31: ENABLE lower 64KB"]
    SRAM64K = 31,
    #[doc = "127: ENABLE lower 128KB"]
    SRAM128K = 127,
    #[doc = "2047: ENABLE lower 256KB"]
    SRAM256K = 2047,
}
impl From<SRAMEN_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAMEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMEN`"]
pub type SRAMEN_R = crate::R<u16, SRAMEN_A>;
impl SRAMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SRAMEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRAMEN_A::NONE),
            1 => Val(SRAMEN_A::GROUP0_SRAM0),
            2 => Val(SRAMEN_A::GROUP0_SRAM1),
            4 => Val(SRAMEN_A::GROUP0_SRAM2),
            8 => Val(SRAMEN_A::GROUP0_SRAM3),
            16 => Val(SRAMEN_A::GROUP1),
            32 => Val(SRAMEN_A::GROUP2),
            64 => Val(SRAMEN_A::GROUP3),
            128 => Val(SRAMEN_A::GROUP4),
            256 => Val(SRAMEN_A::GROUP5),
            512 => Val(SRAMEN_A::GROUP6),
            1024 => Val(SRAMEN_A::GROUP7),
            3 => Val(SRAMEN_A::SRAM16K),
            15 => Val(SRAMEN_A::SRAM32K),
            31 => Val(SRAMEN_A::SRAM64K),
            127 => Val(SRAMEN_A::SRAM128K),
            2047 => Val(SRAMEN_A::SRAM256K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRAMEN_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0_SRAM0`"]
    #[inline(always)]
    pub fn is_group0_sram0(&self) -> bool {
        *self == SRAMEN_A::GROUP0_SRAM0
    }
    #[doc = "Checks if the value of the field is `GROUP0_SRAM1`"]
    #[inline(always)]
    pub fn is_group0_sram1(&self) -> bool {
        *self == SRAMEN_A::GROUP0_SRAM1
    }
    #[doc = "Checks if the value of the field is `GROUP0_SRAM2`"]
    #[inline(always)]
    pub fn is_group0_sram2(&self) -> bool {
        *self == SRAMEN_A::GROUP0_SRAM2
    }
    #[doc = "Checks if the value of the field is `GROUP0_SRAM3`"]
    #[inline(always)]
    pub fn is_group0_sram3(&self) -> bool {
        *self == SRAMEN_A::GROUP0_SRAM3
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == SRAMEN_A::GROUP1
    }
    #[doc = "Checks if the value of the field is `GROUP2`"]
    #[inline(always)]
    pub fn is_group2(&self) -> bool {
        *self == SRAMEN_A::GROUP2
    }
    #[doc = "Checks if the value of the field is `GROUP3`"]
    #[inline(always)]
    pub fn is_group3(&self) -> bool {
        *self == SRAMEN_A::GROUP3
    }
    #[doc = "Checks if the value of the field is `GROUP4`"]
    #[inline(always)]
    pub fn is_group4(&self) -> bool {
        *self == SRAMEN_A::GROUP4
    }
    #[doc = "Checks if the value of the field is `GROUP5`"]
    #[inline(always)]
    pub fn is_group5(&self) -> bool {
        *self == SRAMEN_A::GROUP5
    }
    #[doc = "Checks if the value of the field is `GROUP6`"]
    #[inline(always)]
    pub fn is_group6(&self) -> bool {
        *self == SRAMEN_A::GROUP6
    }
    #[doc = "Checks if the value of the field is `GROUP7`"]
    #[inline(always)]
    pub fn is_group7(&self) -> bool {
        *self == SRAMEN_A::GROUP7
    }
    #[doc = "Checks if the value of the field is `SRAM16K`"]
    #[inline(always)]
    pub fn is_sram16k(&self) -> bool {
        *self == SRAMEN_A::SRAM16K
    }
    #[doc = "Checks if the value of the field is `SRAM32K`"]
    #[inline(always)]
    pub fn is_sram32k(&self) -> bool {
        *self == SRAMEN_A::SRAM32K
    }
    #[doc = "Checks if the value of the field is `SRAM64K`"]
    #[inline(always)]
    pub fn is_sram64k(&self) -> bool {
        *self == SRAMEN_A::SRAM64K
    }
    #[doc = "Checks if the value of the field is `SRAM128K`"]
    #[inline(always)]
    pub fn is_sram128k(&self) -> bool {
        *self == SRAMEN_A::SRAM128K
    }
    #[doc = "Checks if the value of the field is `SRAM256K`"]
    #[inline(always)]
    pub fn is_sram256k(&self) -> bool {
        *self == SRAMEN_A::SRAM256K
    }
}
#[doc = "Write proxy for field `SRAMEN`"]
pub struct SRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All banks disabled"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAMEN_A::NONE)
    }
    #[doc = "0KB-8KB SRAM"]
    #[inline(always)]
    pub fn group0_sram0(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP0_SRAM0)
    }
    #[doc = "8KB-16KB SRAM"]
    #[inline(always)]
    pub fn group0_sram1(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP0_SRAM1)
    }
    #[doc = "16KB-24KB SRAM"]
    #[inline(always)]
    pub fn group0_sram2(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP0_SRAM2)
    }
    #[doc = "24KB-32KB SRAM"]
    #[inline(always)]
    pub fn group0_sram3(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP0_SRAM3)
    }
    #[doc = "32KB-64KB SRAMs"]
    #[inline(always)]
    pub fn group1(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP1)
    }
    #[doc = "64KB-96KB SRAMs"]
    #[inline(always)]
    pub fn group2(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP2)
    }
    #[doc = "96KB-128KB SRAMs"]
    #[inline(always)]
    pub fn group3(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP3)
    }
    #[doc = "128KB-160KB SRAMs"]
    #[inline(always)]
    pub fn group4(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP4)
    }
    #[doc = "160KB-192KB SRAMs"]
    #[inline(always)]
    pub fn group5(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP5)
    }
    #[doc = "192KB-224KB SRAMs"]
    #[inline(always)]
    pub fn group6(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP6)
    }
    #[doc = "224KB-256KB SRAMs"]
    #[inline(always)]
    pub fn group7(self) -> &'a mut W {
        self.variant(SRAMEN_A::GROUP7)
    }
    #[doc = "ENABLE lower 16KB"]
    #[inline(always)]
    pub fn sram16k(self) -> &'a mut W {
        self.variant(SRAMEN_A::SRAM16K)
    }
    #[doc = "ENABLE lower 32KB"]
    #[inline(always)]
    pub fn sram32k(self) -> &'a mut W {
        self.variant(SRAMEN_A::SRAM32K)
    }
    #[doc = "ENABLE lower 64KB"]
    #[inline(always)]
    pub fn sram64k(self) -> &'a mut W {
        self.variant(SRAMEN_A::SRAM64K)
    }
    #[doc = "ENABLE lower 128KB"]
    #[inline(always)]
    pub fn sram128k(self) -> &'a mut W {
        self.variant(SRAMEN_A::SRAM128K)
    }
    #[doc = "ENABLE lower 256KB"]
    #[inline(always)]
    pub fn sram256k(self) -> &'a mut W {
        self.variant(SRAMEN_A::SRAM256K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enable CACHE BANK 2"]
    #[inline(always)]
    pub fn cacheb2(&self) -> CACHEB2_R {
        CACHEB2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable CACHE BANK 0"]
    #[inline(always)]
    pub fn cacheb0(&self) -> CACHEB0_R {
        CACHEB0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable FLASH1"]
    #[inline(always)]
    pub fn flash1(&self) -> FLASH1_R {
        FLASH1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable FLASH 0"]
    #[inline(always)]
    pub fn flash0(&self) -> FLASH0_R {
        FLASH0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Enables power for selected SRAM banks (else an access to its address space to generate a Hard Fault)."]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Enable CACHE BANK 2"]
    #[inline(always)]
    pub fn cacheb2(&mut self) -> CACHEB2_W {
        CACHEB2_W { w: self }
    }
    #[doc = "Bit 29 - Enable CACHE BANK 0"]
    #[inline(always)]
    pub fn cacheb0(&mut self) -> CACHEB0_W {
        CACHEB0_W { w: self }
    }
    #[doc = "Bit 12 - Enable FLASH1"]
    #[inline(always)]
    pub fn flash1(&mut self) -> FLASH1_W {
        FLASH1_W { w: self }
    }
    #[doc = "Bit 11 - Enable FLASH 0"]
    #[inline(always)]
    pub fn flash0(&mut self) -> FLASH0_W {
        FLASH0_W { w: self }
    }
    #[doc = "Bits 0:10 - Enables power for selected SRAM banks (else an access to its address space to generate a Hard Fault)."]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W { w: self }
    }
}
