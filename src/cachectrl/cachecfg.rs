#[doc = "Reader of register CACHECFG"]
pub type R = crate::R<u32, super::CACHECFG>;
#[doc = "Writer for register CACHECFG"]
pub type W = crate::W<u32, super::CACHECFG>;
#[doc = "Register CACHECFG `reset()`'s with value 0x0016_6c50"]
impl crate::ResetValue for super::CACHECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0016_6c50
    }
}
#[doc = "Reader of field `ENABLE_MONITOR`"]
pub type ENABLE_MONITOR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_MONITOR`"]
pub struct ENABLE_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_MONITOR_W<'a> {
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
#[doc = "Reader of field `DATA_CLKGATE`"]
pub type DATA_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_CLKGATE`"]
pub struct DATA_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CLKGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SMDLY`"]
pub type SMDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMDLY`"]
pub struct SMDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DLY`"]
pub type DLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DLY`"]
pub struct DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CACHE_LS`"]
pub type CACHE_LS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_LS`"]
pub struct CACHE_LS_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_LS_W<'a> {
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
#[doc = "Reader of field `CACHE_CLKGATE`"]
pub type CACHE_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHE_CLKGATE`"]
pub struct CACHE_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_CLKGATE_W<'a> {
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
#[doc = "Reader of field `DCACHE_ENABLE`"]
pub type DCACHE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCACHE_ENABLE`"]
pub struct DCACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCACHE_ENABLE_W<'a> {
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
#[doc = "Reader of field `ICACHE_ENABLE`"]
pub type ICACHE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHE_ENABLE`"]
pub struct ICACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_ENABLE_W<'a> {
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
#[doc = "Reader of field `SERIAL`"]
pub type SERIAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERIAL`"]
pub struct SERIAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SERIAL_W<'a> {
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
#[doc = "Sets the cache configuration. Only a single configuration of 0x5 is valid.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CONFIG_A {
    #[doc = "5: Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active)"]
    W2_128B_512E = 5,
}
impl From<CONFIG_A> for u8 {
    #[inline(always)]
    fn from(variant: CONFIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CONFIG`"]
pub type CONFIG_R = crate::R<u8, CONFIG_A>;
impl CONFIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CONFIG_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(CONFIG_A::W2_128B_512E),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `W2_128B_512E`"]
    #[inline(always)]
    pub fn is_w2_128b_512e(&self) -> bool {
        *self == CONFIG_A::W2_128B_512E
    }
}
#[doc = "Write proxy for field `CONFIG`"]
pub struct CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONFIG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Two-way set associative, 128-bit linesize, 512 entries (8 SRAMs active)"]
    #[inline(always)]
    pub fn w2_128b_512e(self) -> &'a mut W {
        self.variant(CONFIG_A::W2_128B_512E)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_NC1`"]
pub type ENABLE_NC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_NC1`"]
pub struct ENABLE_NC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_NC1_W<'a> {
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
#[doc = "Reader of field `ENABLE_NC0`"]
pub type ENABLE_NC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_NC0`"]
pub struct ENABLE_NC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_NC0_W<'a> {
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
#[doc = "Reader of field `LRU`"]
pub type LRU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LRU`"]
pub struct LRU_W<'a> {
    w: &'a mut W,
}
impl<'a> LRU_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Only enable this for debug/performance analysis since it will consume additional power. See IMON/DMON registers for data."]
    #[inline(always)]
    pub fn enable_monitor(&self) -> ENABLE_MONITOR_R {
        ENABLE_MONITOR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable clock gating of entire cache data array subsystem. This should be enabled for normal operation."]
    #[inline(always)]
    pub fn data_clkgate(&self) -> DATA_CLKGATE_R {
        DATA_CLKGATE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Unused. Should be left at default value."]
    #[inline(always)]
    pub fn smdly(&self) -> SMDLY_R {
        SMDLY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Unused. Should be left at default value."]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. This should not be enabled for normal operation. When this bit is set, the cache's RAMS will be put into light sleep mode while inactive. NOTE: if the cache is actively used, this may have an adverse affect on power since entering/exiting LS mode may consume more power than would be saved."]
    #[inline(always)]
    pub fn cache_ls(&self) -> CACHE_LS_R {
        CACHE_LS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable clock gating of individual cache RAMs. This bit should be enabled for normal operation for lowest power consumption."]
    #[inline(always)]
    pub fn cache_clkgate(&self) -> CACHE_CLKGATE_R {
        CACHE_CLKGATE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Flash Data Caching. When set to 1, all instruction accesses to flash will be cached."]
    #[inline(always)]
    pub fn dcache_enable(&self) -> DCACHE_ENABLE_R {
        DCACHE_ENABLE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Flash Instruction Caching. When set to 1, all instruction accesses to flash will be cached."]
    #[inline(always)]
    pub fn icache_enable(&self) -> ICACHE_ENABLE_R {
        ICACHE_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bitfield should always be programmed to 0."]
    #[inline(always)]
    pub fn serial(&self) -> SERIAL_R {
        SERIAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Sets the cache configuration. Only a single configuration of 0x5 is valid."]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See the NCR1 registers to set the region boundaries and size."]
    #[inline(always)]
    pub fn enable_nc1(&self) -> ENABLE_NC1_R {
        ENABLE_NC1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See the NCR0 registers to set the region boundaries and size."]
    #[inline(always)]
    pub fn enable_nc0(&self) -> ENABLE_NC0_R {
        ENABLE_NC0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sets the cache replacement policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM and is recommended."]
    #[inline(always)]
    pub fn lru(&self) -> LRU_R {
        LRU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables the main flash cache controller logic and enables power to the cache RAMs. Instruction and Data caching need to be enabled independently using the ICACHE_ENABLE and DCACHE_ENABLE bits."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Enable Cache Monitoring Stats. Only enable this for debug/performance analysis since it will consume additional power. See IMON/DMON registers for data."]
    #[inline(always)]
    pub fn enable_monitor(&mut self) -> ENABLE_MONITOR_W {
        ENABLE_MONITOR_W { w: self }
    }
    #[doc = "Bit 20 - Enable clock gating of entire cache data array subsystem. This should be enabled for normal operation."]
    #[inline(always)]
    pub fn data_clkgate(&mut self) -> DATA_CLKGATE_W {
        DATA_CLKGATE_W { w: self }
    }
    #[doc = "Bits 16:19 - Unused. Should be left at default value."]
    #[inline(always)]
    pub fn smdly(&mut self) -> SMDLY_W {
        SMDLY_W { w: self }
    }
    #[doc = "Bits 12:15 - Unused. Should be left at default value."]
    #[inline(always)]
    pub fn dly(&mut self) -> DLY_W {
        DLY_W { w: self }
    }
    #[doc = "Bit 11 - Enable LS (light sleep) of cache RAMs. This should not be enabled for normal operation. When this bit is set, the cache's RAMS will be put into light sleep mode while inactive. NOTE: if the cache is actively used, this may have an adverse affect on power since entering/exiting LS mode may consume more power than would be saved."]
    #[inline(always)]
    pub fn cache_ls(&mut self) -> CACHE_LS_W {
        CACHE_LS_W { w: self }
    }
    #[doc = "Bit 10 - Enable clock gating of individual cache RAMs. This bit should be enabled for normal operation for lowest power consumption."]
    #[inline(always)]
    pub fn cache_clkgate(&mut self) -> CACHE_CLKGATE_W {
        CACHE_CLKGATE_W { w: self }
    }
    #[doc = "Bit 9 - Enable Flash Data Caching. When set to 1, all instruction accesses to flash will be cached."]
    #[inline(always)]
    pub fn dcache_enable(&mut self) -> DCACHE_ENABLE_W {
        DCACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Enable Flash Instruction Caching. When set to 1, all instruction accesses to flash will be cached."]
    #[inline(always)]
    pub fn icache_enable(&mut self) -> ICACHE_ENABLE_W {
        ICACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 7 - Bitfield should always be programmed to 0."]
    #[inline(always)]
    pub fn serial(&mut self) -> SERIAL_W {
        SERIAL_W { w: self }
    }
    #[doc = "Bits 4:6 - Sets the cache configuration. Only a single configuration of 0x5 is valid."]
    #[inline(always)]
    pub fn config(&mut self) -> CONFIG_W {
        CONFIG_W { w: self }
    }
    #[doc = "Bit 3 - Enable Non-cacheable region 1. See the NCR1 registers to set the region boundaries and size."]
    #[inline(always)]
    pub fn enable_nc1(&mut self) -> ENABLE_NC1_W {
        ENABLE_NC1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Non-cacheable region 0. See the NCR0 registers to set the region boundaries and size."]
    #[inline(always)]
    pub fn enable_nc0(&mut self) -> ENABLE_NC0_W {
        ENABLE_NC0_W { w: self }
    }
    #[doc = "Bit 1 - Sets the cache replacement policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM and is recommended."]
    #[inline(always)]
    pub fn lru(&mut self) -> LRU_W {
        LRU_W { w: self }
    }
    #[doc = "Bit 0 - Enables the main flash cache controller logic and enables power to the cache RAMs. Instruction and Data caching need to be enabled independently using the ICACHE_ENABLE and DCACHE_ENABLE bits."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
