#[doc = "Reader of register PCFG"]
pub type R = crate::R<u32, super::PCFG>;
#[doc = "Writer for register PCFG"]
pub type W = crate::W<u32, super::PCFG>;
#[doc = "Register PCFG `reset()`'s with value 0xc365"]
impl crate::ResetValue for super::PCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc365
    }
}
#[doc = "Left/right channel swap.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRSWAP_A {
    #[doc = "1: Swap left and right channels (FIFO Read RIGHT_LEFT)."]
    EN = 1,
    #[doc = "0: No channel swapping (IFO Read LEFT_RIGHT)."]
    NOSWAP = 0,
}
impl From<LRSWAP_A> for bool {
    #[inline(always)]
    fn from(variant: LRSWAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LRSWAP`"]
pub type LRSWAP_R = crate::R<bool, LRSWAP_A>;
impl LRSWAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRSWAP_A {
        match self.bits {
            true => LRSWAP_A::EN,
            false => LRSWAP_A::NOSWAP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == LRSWAP_A::EN
    }
    #[doc = "Checks if the value of the field is `NOSWAP`"]
    #[inline(always)]
    pub fn is_noswap(&self) -> bool {
        *self == LRSWAP_A::NOSWAP
    }
}
#[doc = "Write proxy for field `LRSWAP`"]
pub struct LRSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> LRSWAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRSWAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Swap left and right channels (FIFO Read RIGHT_LEFT)."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(LRSWAP_A::EN)
    }
    #[doc = "No channel swapping (IFO Read LEFT_RIGHT)."]
    #[inline(always)]
    pub fn noswap(self) -> &'a mut W {
        self.variant(LRSWAP_A::NOSWAP)
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
#[doc = "Right channel PGA gain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGARIGHT_A {
    #[doc = "15: -1.5 db gain."]
    M15DB = 15,
    #[doc = "14: -3.0 db gain."]
    M300DB = 14,
    #[doc = "13: -4.5 db gain."]
    M45DB = 13,
    #[doc = "12: -6.0 db gain."]
    M60DB = 12,
    #[doc = "11: -7.5 db gain."]
    M75DB = 11,
    #[doc = "10: -9.0 db gain."]
    M90DB = 10,
    #[doc = "9: -10.5 db gain."]
    M105DB = 9,
    #[doc = "8: -12.0 db gain."]
    M120DB = 8,
    #[doc = "7: 10.5 db gain."]
    P105DB = 7,
    #[doc = "6: 9.0 db gain."]
    P90DB = 6,
    #[doc = "5: 7.5 db gain."]
    P75DB = 5,
    #[doc = "4: 6.0 db gain."]
    P60DB = 4,
    #[doc = "3: 4.5 db gain."]
    P45DB = 3,
    #[doc = "2: 3.0 db gain."]
    P30DB = 2,
    #[doc = "1: 1.5 db gain."]
    P15DB = 1,
    #[doc = "0: 0.0 db gain."]
    _0DB = 0,
}
impl From<PGARIGHT_A> for u8 {
    #[inline(always)]
    fn from(variant: PGARIGHT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PGARIGHT`"]
pub type PGARIGHT_R = crate::R<u8, PGARIGHT_A>;
impl PGARIGHT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGARIGHT_A {
        match self.bits {
            15 => PGARIGHT_A::M15DB,
            14 => PGARIGHT_A::M300DB,
            13 => PGARIGHT_A::M45DB,
            12 => PGARIGHT_A::M60DB,
            11 => PGARIGHT_A::M75DB,
            10 => PGARIGHT_A::M90DB,
            9 => PGARIGHT_A::M105DB,
            8 => PGARIGHT_A::M120DB,
            7 => PGARIGHT_A::P105DB,
            6 => PGARIGHT_A::P90DB,
            5 => PGARIGHT_A::P75DB,
            4 => PGARIGHT_A::P60DB,
            3 => PGARIGHT_A::P45DB,
            2 => PGARIGHT_A::P30DB,
            1 => PGARIGHT_A::P15DB,
            0 => PGARIGHT_A::_0DB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M15DB`"]
    #[inline(always)]
    pub fn is_m15db(&self) -> bool {
        *self == PGARIGHT_A::M15DB
    }
    #[doc = "Checks if the value of the field is `M300DB`"]
    #[inline(always)]
    pub fn is_m300db(&self) -> bool {
        *self == PGARIGHT_A::M300DB
    }
    #[doc = "Checks if the value of the field is `M45DB`"]
    #[inline(always)]
    pub fn is_m45db(&self) -> bool {
        *self == PGARIGHT_A::M45DB
    }
    #[doc = "Checks if the value of the field is `M60DB`"]
    #[inline(always)]
    pub fn is_m60db(&self) -> bool {
        *self == PGARIGHT_A::M60DB
    }
    #[doc = "Checks if the value of the field is `M75DB`"]
    #[inline(always)]
    pub fn is_m75db(&self) -> bool {
        *self == PGARIGHT_A::M75DB
    }
    #[doc = "Checks if the value of the field is `M90DB`"]
    #[inline(always)]
    pub fn is_m90db(&self) -> bool {
        *self == PGARIGHT_A::M90DB
    }
    #[doc = "Checks if the value of the field is `M105DB`"]
    #[inline(always)]
    pub fn is_m105db(&self) -> bool {
        *self == PGARIGHT_A::M105DB
    }
    #[doc = "Checks if the value of the field is `M120DB`"]
    #[inline(always)]
    pub fn is_m120db(&self) -> bool {
        *self == PGARIGHT_A::M120DB
    }
    #[doc = "Checks if the value of the field is `P105DB`"]
    #[inline(always)]
    pub fn is_p105db(&self) -> bool {
        *self == PGARIGHT_A::P105DB
    }
    #[doc = "Checks if the value of the field is `P90DB`"]
    #[inline(always)]
    pub fn is_p90db(&self) -> bool {
        *self == PGARIGHT_A::P90DB
    }
    #[doc = "Checks if the value of the field is `P75DB`"]
    #[inline(always)]
    pub fn is_p75db(&self) -> bool {
        *self == PGARIGHT_A::P75DB
    }
    #[doc = "Checks if the value of the field is `P60DB`"]
    #[inline(always)]
    pub fn is_p60db(&self) -> bool {
        *self == PGARIGHT_A::P60DB
    }
    #[doc = "Checks if the value of the field is `P45DB`"]
    #[inline(always)]
    pub fn is_p45db(&self) -> bool {
        *self == PGARIGHT_A::P45DB
    }
    #[doc = "Checks if the value of the field is `P30DB`"]
    #[inline(always)]
    pub fn is_p30db(&self) -> bool {
        *self == PGARIGHT_A::P30DB
    }
    #[doc = "Checks if the value of the field is `P15DB`"]
    #[inline(always)]
    pub fn is_p15db(&self) -> bool {
        *self == PGARIGHT_A::P15DB
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        *self == PGARIGHT_A::_0DB
    }
}
#[doc = "Write proxy for field `PGARIGHT`"]
pub struct PGARIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> PGARIGHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGARIGHT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "-1.5 db gain."]
    #[inline(always)]
    pub fn m15db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M15DB)
    }
    #[doc = "-3.0 db gain."]
    #[inline(always)]
    pub fn m300db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M300DB)
    }
    #[doc = "-4.5 db gain."]
    #[inline(always)]
    pub fn m45db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M45DB)
    }
    #[doc = "-6.0 db gain."]
    #[inline(always)]
    pub fn m60db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M60DB)
    }
    #[doc = "-7.5 db gain."]
    #[inline(always)]
    pub fn m75db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M75DB)
    }
    #[doc = "-9.0 db gain."]
    #[inline(always)]
    pub fn m90db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M90DB)
    }
    #[doc = "-10.5 db gain."]
    #[inline(always)]
    pub fn m105db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M105DB)
    }
    #[doc = "-12.0 db gain."]
    #[inline(always)]
    pub fn m120db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M120DB)
    }
    #[doc = "10.5 db gain."]
    #[inline(always)]
    pub fn p105db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P105DB)
    }
    #[doc = "9.0 db gain."]
    #[inline(always)]
    pub fn p90db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P90DB)
    }
    #[doc = "7.5 db gain."]
    #[inline(always)]
    pub fn p75db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P75DB)
    }
    #[doc = "6.0 db gain."]
    #[inline(always)]
    pub fn p60db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P60DB)
    }
    #[doc = "4.5 db gain."]
    #[inline(always)]
    pub fn p45db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P45DB)
    }
    #[doc = "3.0 db gain."]
    #[inline(always)]
    pub fn p30db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P30DB)
    }
    #[doc = "1.5 db gain."]
    #[inline(always)]
    pub fn p15db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P15DB)
    }
    #[doc = "0.0 db gain."]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::_0DB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 27)) | (((value as u32) & 0x0f) << 27);
        self.w
    }
}
#[doc = "Left channel PGA gain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGALEFT_A {
    #[doc = "15: -1.5 db gain."]
    M15DB = 15,
    #[doc = "14: -3.0 db gain."]
    M300DB = 14,
    #[doc = "13: -4.5 db gain."]
    M45DB = 13,
    #[doc = "12: -6.0 db gain."]
    M60DB = 12,
    #[doc = "11: -7.5 db gain."]
    M75DB = 11,
    #[doc = "10: -9.0 db gain."]
    M90DB = 10,
    #[doc = "9: -10.5 db gain."]
    M105DB = 9,
    #[doc = "8: -12.0 db gain."]
    M120DB = 8,
    #[doc = "7: 10.5 db gain."]
    P105DB = 7,
    #[doc = "6: 9.0 db gain."]
    P90DB = 6,
    #[doc = "5: 7.5 db gain."]
    P75DB = 5,
    #[doc = "4: 6.0 db gain."]
    P60DB = 4,
    #[doc = "3: 4.5 db gain."]
    P45DB = 3,
    #[doc = "2: 3.0 db gain."]
    P30DB = 2,
    #[doc = "1: 1.5 db gain."]
    P15DB = 1,
    #[doc = "0: 0.0 db gain."]
    _0DB = 0,
}
impl From<PGALEFT_A> for u8 {
    #[inline(always)]
    fn from(variant: PGALEFT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PGALEFT`"]
pub type PGALEFT_R = crate::R<u8, PGALEFT_A>;
impl PGALEFT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGALEFT_A {
        match self.bits {
            15 => PGALEFT_A::M15DB,
            14 => PGALEFT_A::M300DB,
            13 => PGALEFT_A::M45DB,
            12 => PGALEFT_A::M60DB,
            11 => PGALEFT_A::M75DB,
            10 => PGALEFT_A::M90DB,
            9 => PGALEFT_A::M105DB,
            8 => PGALEFT_A::M120DB,
            7 => PGALEFT_A::P105DB,
            6 => PGALEFT_A::P90DB,
            5 => PGALEFT_A::P75DB,
            4 => PGALEFT_A::P60DB,
            3 => PGALEFT_A::P45DB,
            2 => PGALEFT_A::P30DB,
            1 => PGALEFT_A::P15DB,
            0 => PGALEFT_A::_0DB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M15DB`"]
    #[inline(always)]
    pub fn is_m15db(&self) -> bool {
        *self == PGALEFT_A::M15DB
    }
    #[doc = "Checks if the value of the field is `M300DB`"]
    #[inline(always)]
    pub fn is_m300db(&self) -> bool {
        *self == PGALEFT_A::M300DB
    }
    #[doc = "Checks if the value of the field is `M45DB`"]
    #[inline(always)]
    pub fn is_m45db(&self) -> bool {
        *self == PGALEFT_A::M45DB
    }
    #[doc = "Checks if the value of the field is `M60DB`"]
    #[inline(always)]
    pub fn is_m60db(&self) -> bool {
        *self == PGALEFT_A::M60DB
    }
    #[doc = "Checks if the value of the field is `M75DB`"]
    #[inline(always)]
    pub fn is_m75db(&self) -> bool {
        *self == PGALEFT_A::M75DB
    }
    #[doc = "Checks if the value of the field is `M90DB`"]
    #[inline(always)]
    pub fn is_m90db(&self) -> bool {
        *self == PGALEFT_A::M90DB
    }
    #[doc = "Checks if the value of the field is `M105DB`"]
    #[inline(always)]
    pub fn is_m105db(&self) -> bool {
        *self == PGALEFT_A::M105DB
    }
    #[doc = "Checks if the value of the field is `M120DB`"]
    #[inline(always)]
    pub fn is_m120db(&self) -> bool {
        *self == PGALEFT_A::M120DB
    }
    #[doc = "Checks if the value of the field is `P105DB`"]
    #[inline(always)]
    pub fn is_p105db(&self) -> bool {
        *self == PGALEFT_A::P105DB
    }
    #[doc = "Checks if the value of the field is `P90DB`"]
    #[inline(always)]
    pub fn is_p90db(&self) -> bool {
        *self == PGALEFT_A::P90DB
    }
    #[doc = "Checks if the value of the field is `P75DB`"]
    #[inline(always)]
    pub fn is_p75db(&self) -> bool {
        *self == PGALEFT_A::P75DB
    }
    #[doc = "Checks if the value of the field is `P60DB`"]
    #[inline(always)]
    pub fn is_p60db(&self) -> bool {
        *self == PGALEFT_A::P60DB
    }
    #[doc = "Checks if the value of the field is `P45DB`"]
    #[inline(always)]
    pub fn is_p45db(&self) -> bool {
        *self == PGALEFT_A::P45DB
    }
    #[doc = "Checks if the value of the field is `P30DB`"]
    #[inline(always)]
    pub fn is_p30db(&self) -> bool {
        *self == PGALEFT_A::P30DB
    }
    #[doc = "Checks if the value of the field is `P15DB`"]
    #[inline(always)]
    pub fn is_p15db(&self) -> bool {
        *self == PGALEFT_A::P15DB
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        *self == PGALEFT_A::_0DB
    }
}
#[doc = "Write proxy for field `PGALEFT`"]
pub struct PGALEFT_W<'a> {
    w: &'a mut W,
}
impl<'a> PGALEFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGALEFT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "-1.5 db gain."]
    #[inline(always)]
    pub fn m15db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M15DB)
    }
    #[doc = "-3.0 db gain."]
    #[inline(always)]
    pub fn m300db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M300DB)
    }
    #[doc = "-4.5 db gain."]
    #[inline(always)]
    pub fn m45db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M45DB)
    }
    #[doc = "-6.0 db gain."]
    #[inline(always)]
    pub fn m60db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M60DB)
    }
    #[doc = "-7.5 db gain."]
    #[inline(always)]
    pub fn m75db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M75DB)
    }
    #[doc = "-9.0 db gain."]
    #[inline(always)]
    pub fn m90db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M90DB)
    }
    #[doc = "-10.5 db gain."]
    #[inline(always)]
    pub fn m105db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M105DB)
    }
    #[doc = "-12.0 db gain."]
    #[inline(always)]
    pub fn m120db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M120DB)
    }
    #[doc = "10.5 db gain."]
    #[inline(always)]
    pub fn p105db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P105DB)
    }
    #[doc = "9.0 db gain."]
    #[inline(always)]
    pub fn p90db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P90DB)
    }
    #[doc = "7.5 db gain."]
    #[inline(always)]
    pub fn p75db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P75DB)
    }
    #[doc = "6.0 db gain."]
    #[inline(always)]
    pub fn p60db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P60DB)
    }
    #[doc = "4.5 db gain."]
    #[inline(always)]
    pub fn p45db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P45DB)
    }
    #[doc = "3.0 db gain."]
    #[inline(always)]
    pub fn p30db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P30DB)
    }
    #[doc = "1.5 db gain."]
    #[inline(always)]
    pub fn p15db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P15DB)
    }
    #[doc = "0.0 db gain."]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut W {
        self.variant(PGALEFT_A::_0DB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "PDM_CLK frequency divisor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCLKDIV_A {
    #[doc = "3: Divide input clock by 4"]
    MCKDIV4 = 3,
    #[doc = "2: Divide input clock by 3"]
    MCKDIV3 = 2,
    #[doc = "1: Divide input clock by 2"]
    MCKDIV2 = 1,
    #[doc = "0: Divide input clock by 1"]
    MCKDIV1 = 0,
}
impl From<MCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCLKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCLKDIV`"]
pub type MCLKDIV_R = crate::R<u8, MCLKDIV_A>;
impl MCLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKDIV_A {
        match self.bits {
            3 => MCLKDIV_A::MCKDIV4,
            2 => MCLKDIV_A::MCKDIV3,
            1 => MCLKDIV_A::MCKDIV2,
            0 => MCLKDIV_A::MCKDIV1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCKDIV4`"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        *self == MCLKDIV_A::MCKDIV4
    }
    #[doc = "Checks if the value of the field is `MCKDIV3`"]
    #[inline(always)]
    pub fn is_mckdiv3(&self) -> bool {
        *self == MCLKDIV_A::MCKDIV3
    }
    #[doc = "Checks if the value of the field is `MCKDIV2`"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        *self == MCLKDIV_A::MCKDIV2
    }
    #[doc = "Checks if the value of the field is `MCKDIV1`"]
    #[inline(always)]
    pub fn is_mckdiv1(&self) -> bool {
        *self == MCLKDIV_A::MCKDIV1
    }
}
#[doc = "Write proxy for field `MCLKDIV`"]
pub struct MCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide input clock by 4"]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut W {
        self.variant(MCLKDIV_A::MCKDIV4)
    }
    #[doc = "Divide input clock by 3"]
    #[inline(always)]
    pub fn mckdiv3(self) -> &'a mut W {
        self.variant(MCLKDIV_A::MCKDIV3)
    }
    #[doc = "Divide input clock by 2"]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut W {
        self.variant(MCLKDIV_A::MCKDIV2)
    }
    #[doc = "Divide input clock by 1"]
    #[inline(always)]
    pub fn mckdiv1(self) -> &'a mut W {
        self.variant(MCLKDIV_A::MCKDIV1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `SINCRATE`"]
pub type SINCRATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINCRATE`"]
pub struct SINCRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINCRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 10)) | (((value as u32) & 0x7f) << 10);
        self.w
    }
}
#[doc = "High pass filter disable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCHPD_A {
    #[doc = "0: Enable high pass filter."]
    EN = 0,
    #[doc = "1: Disable high pass filter."]
    DIS = 1,
}
impl From<ADCHPD_A> for bool {
    #[inline(always)]
    fn from(variant: ADCHPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCHPD`"]
pub type ADCHPD_R = crate::R<bool, ADCHPD_A>;
impl ADCHPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCHPD_A {
        match self.bits {
            false => ADCHPD_A::EN,
            true => ADCHPD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADCHPD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADCHPD_A::DIS
    }
}
#[doc = "Write proxy for field `ADCHPD`"]
pub struct ADCHPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCHPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCHPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable high pass filter."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCHPD_A::EN)
    }
    #[doc = "Disable high pass filter."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCHPD_A::DIS)
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
#[doc = "Reader of field `HPCUTOFF`"]
pub type HPCUTOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPCUTOFF`"]
pub struct HPCUTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> HPCUTOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Reader of field `CYCLES`"]
pub type CYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CYCLES`"]
pub struct CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Soft mute control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTMUTE_A {
    #[doc = "1: Enable Soft Mute."]
    EN = 1,
    #[doc = "0: Disable Soft Mute."]
    DIS = 0,
}
impl From<SOFTMUTE_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTMUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOFTMUTE`"]
pub type SOFTMUTE_R = crate::R<bool, SOFTMUTE_A>;
impl SOFTMUTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTMUTE_A {
        match self.bits {
            true => SOFTMUTE_A::EN,
            false => SOFTMUTE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SOFTMUTE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SOFTMUTE_A::DIS
    }
}
#[doc = "Write proxy for field `SOFTMUTE`"]
pub struct SOFTMUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTMUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFTMUTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Soft Mute."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SOFTMUTE_A::EN)
    }
    #[doc = "Disable Soft Mute."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SOFTMUTE_A::DIS)
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
#[doc = "Data Streaming Control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMCORE_A {
    #[doc = "1: Enable Data Streaming."]
    EN = 1,
    #[doc = "0: Disable Data Streaming."]
    DIS = 0,
}
impl From<PDMCORE_A> for bool {
    #[inline(always)]
    fn from(variant: PDMCORE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDMCORE`"]
pub type PDMCORE_R = crate::R<bool, PDMCORE_A>;
impl PDMCORE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMCORE_A {
        match self.bits {
            true => PDMCORE_A::EN,
            false => PDMCORE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PDMCORE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PDMCORE_A::DIS
    }
}
#[doc = "Write proxy for field `PDMCORE`"]
pub struct PDMCORE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMCORE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMCORE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable Data Streaming."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PDMCORE_A::EN)
    }
    #[doc = "Disable Data Streaming."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PDMCORE_A::DIS)
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
    #[doc = "Bit 31 - Left/right channel swap."]
    #[inline(always)]
    pub fn lrswap(&self) -> LRSWAP_R {
        LRSWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 27:30 - Right channel PGA gain."]
    #[inline(always)]
    pub fn pgaright(&self) -> PGARIGHT_R {
        PGARIGHT_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - Left channel PGA gain."]
    #[inline(always)]
    pub fn pgaleft(&self) -> PGALEFT_R {
        PGALEFT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - PDM_CLK frequency divisor."]
    #[inline(always)]
    pub fn mclkdiv(&self) -> MCLKDIV_R {
        MCLKDIV_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 10:16 - SINC decimation rate."]
    #[inline(always)]
    pub fn sincrate(&self) -> SINCRATE_R {
        SINCRATE_R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bit 9 - High pass filter disable."]
    #[inline(always)]
    pub fn adchpd(&self) -> ADCHPD_R {
        ADCHPD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - High pass filter coefficients."]
    #[inline(always)]
    pub fn hpcutoff(&self) -> HPCUTOFF_R {
        HPCUTOFF_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 2:4 - Number of clocks during gain-setting changes."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1 - Soft mute control."]
    #[inline(always)]
    pub fn softmute(&self) -> SOFTMUTE_R {
        SOFTMUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Data Streaming Control."]
    #[inline(always)]
    pub fn pdmcore(&self) -> PDMCORE_R {
        PDMCORE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Left/right channel swap."]
    #[inline(always)]
    pub fn lrswap(&mut self) -> LRSWAP_W {
        LRSWAP_W { w: self }
    }
    #[doc = "Bits 27:30 - Right channel PGA gain."]
    #[inline(always)]
    pub fn pgaright(&mut self) -> PGARIGHT_W {
        PGARIGHT_W { w: self }
    }
    #[doc = "Bits 23:26 - Left channel PGA gain."]
    #[inline(always)]
    pub fn pgaleft(&mut self) -> PGALEFT_W {
        PGALEFT_W { w: self }
    }
    #[doc = "Bits 17:18 - PDM_CLK frequency divisor."]
    #[inline(always)]
    pub fn mclkdiv(&mut self) -> MCLKDIV_W {
        MCLKDIV_W { w: self }
    }
    #[doc = "Bits 10:16 - SINC decimation rate."]
    #[inline(always)]
    pub fn sincrate(&mut self) -> SINCRATE_W {
        SINCRATE_W { w: self }
    }
    #[doc = "Bit 9 - High pass filter disable."]
    #[inline(always)]
    pub fn adchpd(&mut self) -> ADCHPD_W {
        ADCHPD_W { w: self }
    }
    #[doc = "Bits 5:8 - High pass filter coefficients."]
    #[inline(always)]
    pub fn hpcutoff(&mut self) -> HPCUTOFF_W {
        HPCUTOFF_W { w: self }
    }
    #[doc = "Bits 2:4 - Number of clocks during gain-setting changes."]
    #[inline(always)]
    pub fn cycles(&mut self) -> CYCLES_W {
        CYCLES_W { w: self }
    }
    #[doc = "Bit 1 - Soft mute control."]
    #[inline(always)]
    pub fn softmute(&mut self) -> SOFTMUTE_W {
        SOFTMUTE_W { w: self }
    }
    #[doc = "Bit 0 - Data Streaming Control."]
    #[inline(always)]
    pub fn pdmcore(&mut self) -> PDMCORE_W {
        PDMCORE_W { w: self }
    }
}
