#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDM Configuration Register"]
    pub pcfg: PCFG,
    #[doc = "0x04 - Voice Configuration Register"]
    pub vcfg: VCFG,
    #[doc = "0x08 - Voice Status Register"]
    pub fr: FR,
    #[doc = "0x0c - FIFO Read"]
    pub frd: FRD,
    #[doc = "0x10 - FIFO Flush"]
    pub flush: FLUSH,
    #[doc = "0x14 - FIFO Threshold"]
    pub fthr: FTHR,
    _reserved6: [u8; 488usize],
    #[doc = "0x200 - IO Master Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - IO Master Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - IO Master Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - IO Master Interrupts: Set"]
    pub intset: INTSET,
}
#[doc = "PDM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg](pcfg) module"]
pub type PCFG = crate::Reg<u32, _PCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCFG;
#[doc = "`read()` method returns [pcfg::R](pcfg::R) reader structure"]
impl crate::Readable for PCFG {}
#[doc = "`write(|w| ..)` method takes [pcfg::W](pcfg::W) writer structure"]
impl crate::Writable for PCFG {}
#[doc = "PDM Configuration Register"]
pub mod pcfg;
#[doc = "Voice Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcfg](vcfg) module"]
pub type VCFG = crate::Reg<u32, _VCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCFG;
#[doc = "`read()` method returns [vcfg::R](vcfg::R) reader structure"]
impl crate::Readable for VCFG {}
#[doc = "`write(|w| ..)` method takes [vcfg::W](vcfg::W) writer structure"]
impl crate::Writable for VCFG {}
#[doc = "Voice Configuration Register"]
pub mod vcfg;
#[doc = "Voice Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr](fr) module"]
pub type FR = crate::Reg<u32, _FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR;
#[doc = "`read()` method returns [fr::R](fr::R) reader structure"]
impl crate::Readable for FR {}
#[doc = "`write(|w| ..)` method takes [fr::W](fr::W) writer structure"]
impl crate::Writable for FR {}
#[doc = "Voice Status Register"]
pub mod fr;
#[doc = "FIFO Read\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frd](frd) module"]
pub type FRD = crate::Reg<u32, _FRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRD;
#[doc = "`read()` method returns [frd::R](frd::R) reader structure"]
impl crate::Readable for FRD {}
#[doc = "`write(|w| ..)` method takes [frd::W](frd::W) writer structure"]
impl crate::Writable for FRD {}
#[doc = "FIFO Read"]
pub mod frd;
#[doc = "FIFO Flush\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flush](flush) module"]
pub type FLUSH = crate::Reg<u32, _FLUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLUSH;
#[doc = "`read()` method returns [flush::R](flush::R) reader structure"]
impl crate::Readable for FLUSH {}
#[doc = "`write(|w| ..)` method takes [flush::W](flush::W) writer structure"]
impl crate::Writable for FLUSH {}
#[doc = "FIFO Flush"]
pub mod flush;
#[doc = "FIFO Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fthr](fthr) module"]
pub type FTHR = crate::Reg<u32, _FTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTHR;
#[doc = "`read()` method returns [fthr::R](fthr::R) reader structure"]
impl crate::Readable for FTHR {}
#[doc = "`write(|w| ..)` method takes [fthr::W](fthr::W) writer structure"]
impl crate::Writable for FTHR {}
#[doc = "FIFO Threshold"]
pub mod fthr;
#[doc = "IO Master Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "IO Master Interrupts: Enable"]
pub mod inten;
#[doc = "IO Master Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "IO Master Interrupts: Status"]
pub mod intstat;
#[doc = "IO Master Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "IO Master Interrupts: Clear"]
pub mod intclr;
#[doc = "IO Master Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "IO Master Interrupts: Set"]
pub mod intset;
