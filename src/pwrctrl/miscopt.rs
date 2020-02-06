#[doc = "Reader of register MISCOPT"]
pub type R = crate::R<u32, super::MISCOPT>;
#[doc = "Writer for register MISCOPT"]
pub type W = crate::W<u32, super::MISCOPT>;
#[doc = "Register MISCOPT `reset()`'s with value 0"]
impl crate::ResetValue for super::MISCOPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIS_LDOLPMODE_TIMERS`"]
pub type DIS_LDOLPMODE_TIMERS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_LDOLPMODE_TIMERS`"]
pub struct DIS_LDOLPMODE_TIMERS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_LDOLPMODE_TIMERS_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Setting this bit will enable the MEM LDO to be in LPMODE during deep sleep even when the ctimers or stimers are running"]
    #[inline(always)]
    pub fn dis_ldolpmode_timers(&self) -> DIS_LDOLPMODE_TIMERS_R {
        DIS_LDOLPMODE_TIMERS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Setting this bit will enable the MEM LDO to be in LPMODE during deep sleep even when the ctimers or stimers are running"]
    #[inline(always)]
    pub fn dis_ldolpmode_timers(&mut self) -> DIS_LDOLPMODE_TIMERS_W {
        DIS_LDOLPMODE_TIMERS_W { w: self }
    }
}
