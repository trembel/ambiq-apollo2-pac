#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH1_SLM_ENABLE`"]
pub type FLASH1_SLM_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH1_SLM_ENABLE`"]
pub struct FLASH1_SLM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_ENABLE_W<'a> {
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
#[doc = "Reader of field `FLASH1_SLM_DISABLE`"]
pub type FLASH1_SLM_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH1_SLM_DISABLE`"]
pub struct FLASH1_SLM_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_DISABLE_W<'a> {
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
#[doc = "Reader of field `FLASH1_SLM_STATUS`"]
pub type FLASH1_SLM_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH1_SLM_STATUS`"]
pub struct FLASH1_SLM_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_SLM_STATUS_W<'a> {
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
#[doc = "Reader of field `FLASH0_SLM_ENABLE`"]
pub type FLASH0_SLM_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH0_SLM_ENABLE`"]
pub struct FLASH0_SLM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_ENABLE_W<'a> {
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
#[doc = "Reader of field `FLASH0_SLM_DISABLE`"]
pub type FLASH0_SLM_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH0_SLM_DISABLE`"]
pub struct FLASH0_SLM_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_DISABLE_W<'a> {
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
#[doc = "Reader of field `FLASH0_SLM_STATUS`"]
pub type FLASH0_SLM_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH0_SLM_STATUS`"]
pub struct FLASH0_SLM_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_SLM_STATUS_W<'a> {
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
#[doc = "Reader of field `CACHE_READY`"]
pub type CACHE_READY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_READY`"]
pub struct CACHE_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_READY_W<'a> {
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
#[doc = "Writing a 1 to this bitfield will reset the cache monitor statistics (DMON0-3, IMON0-3). Statistic gathering can be paused/stopped by disabling the MONITOR_ENABLE bit in CACHECFG, which will maintain the count values until the stats are reset by writing this bitfield.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_STAT_A {
    #[doc = "1: Clear Cache Stats"]
    CLEAR = 1,
}
impl From<RESET_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESET_STAT`"]
pub type RESET_STAT_R = crate::R<bool, RESET_STAT_A>;
impl RESET_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, RESET_STAT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(RESET_STAT_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RESET_STAT_A::CLEAR
    }
}
#[doc = "Write proxy for field `RESET_STAT`"]
pub struct RESET_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_STAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_STAT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear Cache Stats"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RESET_STAT_A::CLEAR)
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
#[doc = "Writing a 1 to this bitfield invalidates the flash cache contents.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDATE_A {
    #[doc = "1: Initiate a programming operation to flash info."]
    GO = 1,
}
impl From<INVALIDATE_A> for bool {
    #[inline(always)]
    fn from(variant: INVALIDATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVALIDATE`"]
pub type INVALIDATE_R = crate::R<bool, INVALIDATE_A>;
impl INVALIDATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, INVALIDATE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(INVALIDATE_A::GO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GO`"]
    #[inline(always)]
    pub fn is_go(&self) -> bool {
        *self == INVALIDATE_A::GO
    }
}
#[doc = "Write proxy for field `INVALIDATE`"]
pub struct INVALIDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVALIDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVALIDATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Initiate a programming operation to flash info."]
    #[inline(always)]
    pub fn go(self) -> &'a mut W {
        self.variant(INVALIDATE_A::GO)
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
    #[doc = "Bit 10 - Enable Flash Sleep Mode. After writing this bit, the flash instance 1 will enter a low-power mode until the CPU writes the SLM_DISABLE bit or a flash access occurs. Wake from SLM requires ~5us, so this should only be set if the flash will not be accessed for reasonably long time."]
    #[inline(always)]
    pub fn flash1_slm_enable(&self) -> FLASH1_SLM_ENABLE_R {
        FLASH1_SLM_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable Flash Sleep Mode. Allows CPU to manually disable SLM mode. Performing a flash read will also wake the array."]
    #[inline(always)]
    pub fn flash1_slm_disable(&self) -> FLASH1_SLM_DISABLE_R {
        FLASH1_SLM_DISABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash Sleep Mode Status. When 1, flash instance 1 is asleep."]
    #[inline(always)]
    pub fn flash1_slm_status(&self) -> FLASH1_SLM_STATUS_R {
        FLASH1_SLM_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Flash Sleep Mode. After writing this bit, the flash instance 0 will enter a low-power mode until the CPU writes the SLM_DISABLE bit or a flash access occurs. Wake from SLM requires ~5us, so this should only be set if the flash will not be accessed for reasonably long time."]
    #[inline(always)]
    pub fn flash0_slm_enable(&self) -> FLASH0_SLM_ENABLE_R {
        FLASH0_SLM_ENABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable Flash Sleep Mode. Allows CPU to manually disable SLM mode. Performing a flash read will also wake the array."]
    #[inline(always)]
    pub fn flash0_slm_disable(&self) -> FLASH0_SLM_DISABLE_R {
        FLASH0_SLM_DISABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Sleep Mode Status. When 1, flash instance 0 is asleep."]
    #[inline(always)]
    pub fn flash0_slm_status(&self) -> FLASH0_SLM_STATUS_R {
        FLASH0_SLM_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cache Ready Status. A value of 1 indicates the cache is enabled and not processing an invalidate operation."]
    #[inline(always)]
    pub fn cache_ready(&self) -> CACHE_READY_R {
        CACHE_READY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Writing a 1 to this bitfield will reset the cache monitor statistics (DMON0-3, IMON0-3). Statistic gathering can be paused/stopped by disabling the MONITOR_ENABLE bit in CACHECFG, which will maintain the count values until the stats are reset by writing this bitfield."]
    #[inline(always)]
    pub fn reset_stat(&self) -> RESET_STAT_R {
        RESET_STAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline(always)]
    pub fn invalidate(&self) -> INVALIDATE_R {
        INVALIDATE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Enable Flash Sleep Mode. After writing this bit, the flash instance 1 will enter a low-power mode until the CPU writes the SLM_DISABLE bit or a flash access occurs. Wake from SLM requires ~5us, so this should only be set if the flash will not be accessed for reasonably long time."]
    #[inline(always)]
    pub fn flash1_slm_enable(&mut self) -> FLASH1_SLM_ENABLE_W {
        FLASH1_SLM_ENABLE_W { w: self }
    }
    #[doc = "Bit 9 - Disable Flash Sleep Mode. Allows CPU to manually disable SLM mode. Performing a flash read will also wake the array."]
    #[inline(always)]
    pub fn flash1_slm_disable(&mut self) -> FLASH1_SLM_DISABLE_W {
        FLASH1_SLM_DISABLE_W { w: self }
    }
    #[doc = "Bit 8 - Flash Sleep Mode Status. When 1, flash instance 1 is asleep."]
    #[inline(always)]
    pub fn flash1_slm_status(&mut self) -> FLASH1_SLM_STATUS_W {
        FLASH1_SLM_STATUS_W { w: self }
    }
    #[doc = "Bit 6 - Enable Flash Sleep Mode. After writing this bit, the flash instance 0 will enter a low-power mode until the CPU writes the SLM_DISABLE bit or a flash access occurs. Wake from SLM requires ~5us, so this should only be set if the flash will not be accessed for reasonably long time."]
    #[inline(always)]
    pub fn flash0_slm_enable(&mut self) -> FLASH0_SLM_ENABLE_W {
        FLASH0_SLM_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Disable Flash Sleep Mode. Allows CPU to manually disable SLM mode. Performing a flash read will also wake the array."]
    #[inline(always)]
    pub fn flash0_slm_disable(&mut self) -> FLASH0_SLM_DISABLE_W {
        FLASH0_SLM_DISABLE_W { w: self }
    }
    #[doc = "Bit 4 - Flash Sleep Mode Status. When 1, flash instance 0 is asleep."]
    #[inline(always)]
    pub fn flash0_slm_status(&mut self) -> FLASH0_SLM_STATUS_W {
        FLASH0_SLM_STATUS_W { w: self }
    }
    #[doc = "Bit 2 - Cache Ready Status. A value of 1 indicates the cache is enabled and not processing an invalidate operation."]
    #[inline(always)]
    pub fn cache_ready(&mut self) -> CACHE_READY_W {
        CACHE_READY_W { w: self }
    }
    #[doc = "Bit 1 - Writing a 1 to this bitfield will reset the cache monitor statistics (DMON0-3, IMON0-3). Statistic gathering can be paused/stopped by disabling the MONITOR_ENABLE bit in CACHECFG, which will maintain the count values until the stats are reset by writing this bitfield."]
    #[inline(always)]
    pub fn reset_stat(&mut self) -> RESET_STAT_W {
        RESET_STAT_W { w: self }
    }
    #[doc = "Bit 0 - Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline(always)]
    pub fn invalidate(&mut self) -> INVALIDATE_W {
        INVALIDATE_W { w: self }
    }
}
