#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Counter/Timer Register"]
    pub tmr0: TMR0,
    #[doc = "0x04 - Counter/Timer A0 Compare Registers"]
    pub cmpra0: CMPRA0,
    #[doc = "0x08 - Counter/Timer B0 Compare Registers"]
    pub cmprb0: CMPRB0,
    #[doc = "0x0c - Counter/Timer Control"]
    pub ctrl0: CTRL0,
    #[doc = "0x10 - Counter/Timer Register"]
    pub tmr1: TMR1,
    #[doc = "0x14 - Counter/Timer A1 Compare Registers"]
    pub cmpra1: CMPRA1,
    #[doc = "0x18 - Counter/Timer B1 Compare Registers"]
    pub cmprb1: CMPRB1,
    #[doc = "0x1c - Counter/Timer Control"]
    pub ctrl1: CTRL1,
    #[doc = "0x20 - Counter/Timer Register"]
    pub tmr2: TMR2,
    #[doc = "0x24 - Counter/Timer A2 Compare Registers"]
    pub cmpra2: CMPRA2,
    #[doc = "0x28 - Counter/Timer B2 Compare Registers"]
    pub cmprb2: CMPRB2,
    #[doc = "0x2c - Counter/Timer Control"]
    pub ctrl2: CTRL2,
    #[doc = "0x30 - Counter/Timer Register"]
    pub tmr3: TMR3,
    #[doc = "0x34 - Counter/Timer A3 Compare Registers"]
    pub cmpra3: CMPRA3,
    #[doc = "0x38 - Counter/Timer B3 Compare Registers"]
    pub cmprb3: CMPRB3,
    #[doc = "0x3c - Counter/Timer Control"]
    pub ctrl3: CTRL3,
    _reserved16: [u8; 192usize],
    #[doc = "0x100 - Configuration Register"]
    pub stcfg: STCFG,
    #[doc = "0x104 - System Timer Count Register (Real Time Counter)"]
    pub sttmr: STTMR,
    #[doc = "0x108 - Capture Control Register"]
    pub capture_control: CAPTURE_CONTROL,
    _reserved19: [u8; 4usize],
    #[doc = "0x110 - Compare Register A"]
    pub scmpr0: SCMPR0,
    #[doc = "0x114 - Compare Register B"]
    pub scmpr1: SCMPR1,
    #[doc = "0x118 - Compare Register C"]
    pub scmpr2: SCMPR2,
    #[doc = "0x11c - Compare Register D"]
    pub scmpr3: SCMPR3,
    #[doc = "0x120 - Compare Register E"]
    pub scmpr4: SCMPR4,
    #[doc = "0x124 - Compare Register F"]
    pub scmpr5: SCMPR5,
    #[doc = "0x128 - Compare Register G"]
    pub scmpr6: SCMPR6,
    #[doc = "0x12c - Compare Register H"]
    pub scmpr7: SCMPR7,
    _reserved27: [u8; 176usize],
    #[doc = "0x1e0 - Capture Register A"]
    pub scapt0: SCAPT0,
    #[doc = "0x1e4 - Capture Register B"]
    pub scapt1: SCAPT1,
    #[doc = "0x1e8 - Capture Register C"]
    pub scapt2: SCAPT2,
    #[doc = "0x1ec - Capture Register D"]
    pub scapt3: SCAPT3,
    #[doc = "0x1f0 - System Timer NVRAM_A Register"]
    pub snvr0: SNVR0,
    #[doc = "0x1f4 - System Timer NVRAM_B Register"]
    pub snvr1: SNVR1,
    #[doc = "0x1f8 - System Timer NVRAM_C Register"]
    pub snvr2: SNVR2,
    _reserved34: [u8; 4usize],
    #[doc = "0x200 - Counter/Timer Interrupts: Enable"]
    pub inten: INTEN,
    #[doc = "0x204 - Counter/Timer Interrupts: Status"]
    pub intstat: INTSTAT,
    #[doc = "0x208 - Counter/Timer Interrupts: Clear"]
    pub intclr: INTCLR,
    #[doc = "0x20c - Counter/Timer Interrupts: Set"]
    pub intset: INTSET,
    _reserved38: [u8; 240usize],
    #[doc = "0x300 - STIMER Interrupt registers: Enable"]
    pub stminten: STMINTEN,
    #[doc = "0x304 - STIMER Interrupt registers: Status"]
    pub stmintstat: STMINTSTAT,
    #[doc = "0x308 - STIMER Interrupt registers: Clear"]
    pub stmintclr: STMINTCLR,
    #[doc = "0x30c - STIMER Interrupt registers: Set"]
    pub stmintset: STMINTSET,
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr0](tmr0) module"]
pub type TMR0 = crate::Reg<u32, _TMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR0;
#[doc = "`read()` method returns [tmr0::R](tmr0::R) reader structure"]
impl crate::Readable for TMR0 {}
#[doc = "`write(|w| ..)` method takes [tmr0::W](tmr0::W) writer structure"]
impl crate::Writable for TMR0 {}
#[doc = "Counter/Timer Register"]
pub mod tmr0;
#[doc = "Counter/Timer A0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra0](cmpra0) module"]
pub type CMPRA0 = crate::Reg<u32, _CMPRA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA0;
#[doc = "`read()` method returns [cmpra0::R](cmpra0::R) reader structure"]
impl crate::Readable for CMPRA0 {}
#[doc = "`write(|w| ..)` method takes [cmpra0::W](cmpra0::W) writer structure"]
impl crate::Writable for CMPRA0 {}
#[doc = "Counter/Timer A0 Compare Registers"]
pub mod cmpra0;
#[doc = "Counter/Timer B0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb0](cmprb0) module"]
pub type CMPRB0 = crate::Reg<u32, _CMPRB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB0;
#[doc = "`read()` method returns [cmprb0::R](cmprb0::R) reader structure"]
impl crate::Readable for CMPRB0 {}
#[doc = "`write(|w| ..)` method takes [cmprb0::W](cmprb0::W) writer structure"]
impl crate::Writable for CMPRB0 {}
#[doc = "Counter/Timer B0 Compare Registers"]
pub mod cmprb0;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](ctrl0) module"]
pub type CTRL0 = crate::Reg<u32, _CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL0;
#[doc = "`read()` method returns [ctrl0::R](ctrl0::R) reader structure"]
impl crate::Readable for CTRL0 {}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](ctrl0::W) writer structure"]
impl crate::Writable for CTRL0 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl0;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr1](tmr1) module"]
pub type TMR1 = crate::Reg<u32, _TMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR1;
#[doc = "`read()` method returns [tmr1::R](tmr1::R) reader structure"]
impl crate::Readable for TMR1 {}
#[doc = "`write(|w| ..)` method takes [tmr1::W](tmr1::W) writer structure"]
impl crate::Writable for TMR1 {}
#[doc = "Counter/Timer Register"]
pub mod tmr1;
#[doc = "Counter/Timer A1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra1](cmpra1) module"]
pub type CMPRA1 = crate::Reg<u32, _CMPRA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA1;
#[doc = "`read()` method returns [cmpra1::R](cmpra1::R) reader structure"]
impl crate::Readable for CMPRA1 {}
#[doc = "`write(|w| ..)` method takes [cmpra1::W](cmpra1::W) writer structure"]
impl crate::Writable for CMPRA1 {}
#[doc = "Counter/Timer A1 Compare Registers"]
pub mod cmpra1;
#[doc = "Counter/Timer B1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb1](cmprb1) module"]
pub type CMPRB1 = crate::Reg<u32, _CMPRB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB1;
#[doc = "`read()` method returns [cmprb1::R](cmprb1::R) reader structure"]
impl crate::Readable for CMPRB1 {}
#[doc = "`write(|w| ..)` method takes [cmprb1::W](cmprb1::W) writer structure"]
impl crate::Writable for CMPRB1 {}
#[doc = "Counter/Timer B1 Compare Registers"]
pub mod cmprb1;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](ctrl1) module"]
pub type CTRL1 = crate::Reg<u32, _CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL1;
#[doc = "`read()` method returns [ctrl1::R](ctrl1::R) reader structure"]
impl crate::Readable for CTRL1 {}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](ctrl1::W) writer structure"]
impl crate::Writable for CTRL1 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl1;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2](tmr2) module"]
pub type TMR2 = crate::Reg<u32, _TMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2;
#[doc = "`read()` method returns [tmr2::R](tmr2::R) reader structure"]
impl crate::Readable for TMR2 {}
#[doc = "`write(|w| ..)` method takes [tmr2::W](tmr2::W) writer structure"]
impl crate::Writable for TMR2 {}
#[doc = "Counter/Timer Register"]
pub mod tmr2;
#[doc = "Counter/Timer A2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra2](cmpra2) module"]
pub type CMPRA2 = crate::Reg<u32, _CMPRA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA2;
#[doc = "`read()` method returns [cmpra2::R](cmpra2::R) reader structure"]
impl crate::Readable for CMPRA2 {}
#[doc = "`write(|w| ..)` method takes [cmpra2::W](cmpra2::W) writer structure"]
impl crate::Writable for CMPRA2 {}
#[doc = "Counter/Timer A2 Compare Registers"]
pub mod cmpra2;
#[doc = "Counter/Timer B2 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb2](cmprb2) module"]
pub type CMPRB2 = crate::Reg<u32, _CMPRB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB2;
#[doc = "`read()` method returns [cmprb2::R](cmprb2::R) reader structure"]
impl crate::Readable for CMPRB2 {}
#[doc = "`write(|w| ..)` method takes [cmprb2::W](cmprb2::W) writer structure"]
impl crate::Writable for CMPRB2 {}
#[doc = "Counter/Timer B2 Compare Registers"]
pub mod cmprb2;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](ctrl2) module"]
pub type CTRL2 = crate::Reg<u32, _CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL2;
#[doc = "`read()` method returns [ctrl2::R](ctrl2::R) reader structure"]
impl crate::Readable for CTRL2 {}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](ctrl2::W) writer structure"]
impl crate::Writable for CTRL2 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl2;
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3](tmr3) module"]
pub type TMR3 = crate::Reg<u32, _TMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3;
#[doc = "`read()` method returns [tmr3::R](tmr3::R) reader structure"]
impl crate::Readable for TMR3 {}
#[doc = "`write(|w| ..)` method takes [tmr3::W](tmr3::W) writer structure"]
impl crate::Writable for TMR3 {}
#[doc = "Counter/Timer Register"]
pub mod tmr3;
#[doc = "Counter/Timer A3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra3](cmpra3) module"]
pub type CMPRA3 = crate::Reg<u32, _CMPRA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRA3;
#[doc = "`read()` method returns [cmpra3::R](cmpra3::R) reader structure"]
impl crate::Readable for CMPRA3 {}
#[doc = "`write(|w| ..)` method takes [cmpra3::W](cmpra3::W) writer structure"]
impl crate::Writable for CMPRA3 {}
#[doc = "Counter/Timer A3 Compare Registers"]
pub mod cmpra3;
#[doc = "Counter/Timer B3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb3](cmprb3) module"]
pub type CMPRB3 = crate::Reg<u32, _CMPRB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRB3;
#[doc = "`read()` method returns [cmprb3::R](cmprb3::R) reader structure"]
impl crate::Readable for CMPRB3 {}
#[doc = "`write(|w| ..)` method takes [cmprb3::W](cmprb3::W) writer structure"]
impl crate::Writable for CMPRB3 {}
#[doc = "Counter/Timer B3 Compare Registers"]
pub mod cmprb3;
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl3](ctrl3) module"]
pub type CTRL3 = crate::Reg<u32, _CTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL3;
#[doc = "`read()` method returns [ctrl3::R](ctrl3::R) reader structure"]
impl crate::Readable for CTRL3 {}
#[doc = "`write(|w| ..)` method takes [ctrl3::W](ctrl3::W) writer structure"]
impl crate::Writable for CTRL3 {}
#[doc = "Counter/Timer Control"]
pub mod ctrl3;
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcfg](stcfg) module"]
pub type STCFG = crate::Reg<u32, _STCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCFG;
#[doc = "`read()` method returns [stcfg::R](stcfg::R) reader structure"]
impl crate::Readable for STCFG {}
#[doc = "`write(|w| ..)` method takes [stcfg::W](stcfg::W) writer structure"]
impl crate::Writable for STCFG {}
#[doc = "Configuration Register"]
pub mod stcfg;
#[doc = "System Timer Count Register (Real Time Counter)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sttmr](sttmr) module"]
pub type STTMR = crate::Reg<u32, _STTMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STTMR;
#[doc = "`read()` method returns [sttmr::R](sttmr::R) reader structure"]
impl crate::Readable for STTMR {}
#[doc = "`write(|w| ..)` method takes [sttmr::W](sttmr::W) writer structure"]
impl crate::Writable for STTMR {}
#[doc = "System Timer Count Register (Real Time Counter)"]
pub mod sttmr;
#[doc = "Capture Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capture_control](capture_control) module"]
pub type CAPTURE_CONTROL = crate::Reg<u32, _CAPTURE_CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTURE_CONTROL;
#[doc = "`read()` method returns [capture_control::R](capture_control::R) reader structure"]
impl crate::Readable for CAPTURE_CONTROL {}
#[doc = "`write(|w| ..)` method takes [capture_control::W](capture_control::W) writer structure"]
impl crate::Writable for CAPTURE_CONTROL {}
#[doc = "Capture Control Register"]
pub mod capture_control;
#[doc = "Compare Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr0](scmpr0) module"]
pub type SCMPR0 = crate::Reg<u32, _SCMPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR0;
#[doc = "`read()` method returns [scmpr0::R](scmpr0::R) reader structure"]
impl crate::Readable for SCMPR0 {}
#[doc = "`write(|w| ..)` method takes [scmpr0::W](scmpr0::W) writer structure"]
impl crate::Writable for SCMPR0 {}
#[doc = "Compare Register A"]
pub mod scmpr0;
#[doc = "Compare Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr1](scmpr1) module"]
pub type SCMPR1 = crate::Reg<u32, _SCMPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR1;
#[doc = "`read()` method returns [scmpr1::R](scmpr1::R) reader structure"]
impl crate::Readable for SCMPR1 {}
#[doc = "`write(|w| ..)` method takes [scmpr1::W](scmpr1::W) writer structure"]
impl crate::Writable for SCMPR1 {}
#[doc = "Compare Register B"]
pub mod scmpr1;
#[doc = "Compare Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr2](scmpr2) module"]
pub type SCMPR2 = crate::Reg<u32, _SCMPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR2;
#[doc = "`read()` method returns [scmpr2::R](scmpr2::R) reader structure"]
impl crate::Readable for SCMPR2 {}
#[doc = "`write(|w| ..)` method takes [scmpr2::W](scmpr2::W) writer structure"]
impl crate::Writable for SCMPR2 {}
#[doc = "Compare Register C"]
pub mod scmpr2;
#[doc = "Compare Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr3](scmpr3) module"]
pub type SCMPR3 = crate::Reg<u32, _SCMPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR3;
#[doc = "`read()` method returns [scmpr3::R](scmpr3::R) reader structure"]
impl crate::Readable for SCMPR3 {}
#[doc = "`write(|w| ..)` method takes [scmpr3::W](scmpr3::W) writer structure"]
impl crate::Writable for SCMPR3 {}
#[doc = "Compare Register D"]
pub mod scmpr3;
#[doc = "Compare Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr4](scmpr4) module"]
pub type SCMPR4 = crate::Reg<u32, _SCMPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR4;
#[doc = "`read()` method returns [scmpr4::R](scmpr4::R) reader structure"]
impl crate::Readable for SCMPR4 {}
#[doc = "`write(|w| ..)` method takes [scmpr4::W](scmpr4::W) writer structure"]
impl crate::Writable for SCMPR4 {}
#[doc = "Compare Register E"]
pub mod scmpr4;
#[doc = "Compare Register F\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr5](scmpr5) module"]
pub type SCMPR5 = crate::Reg<u32, _SCMPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR5;
#[doc = "`read()` method returns [scmpr5::R](scmpr5::R) reader structure"]
impl crate::Readable for SCMPR5 {}
#[doc = "`write(|w| ..)` method takes [scmpr5::W](scmpr5::W) writer structure"]
impl crate::Writable for SCMPR5 {}
#[doc = "Compare Register F"]
pub mod scmpr5;
#[doc = "Compare Register G\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr6](scmpr6) module"]
pub type SCMPR6 = crate::Reg<u32, _SCMPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR6;
#[doc = "`read()` method returns [scmpr6::R](scmpr6::R) reader structure"]
impl crate::Readable for SCMPR6 {}
#[doc = "`write(|w| ..)` method takes [scmpr6::W](scmpr6::W) writer structure"]
impl crate::Writable for SCMPR6 {}
#[doc = "Compare Register G"]
pub mod scmpr6;
#[doc = "Compare Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr7](scmpr7) module"]
pub type SCMPR7 = crate::Reg<u32, _SCMPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCMPR7;
#[doc = "`read()` method returns [scmpr7::R](scmpr7::R) reader structure"]
impl crate::Readable for SCMPR7 {}
#[doc = "`write(|w| ..)` method takes [scmpr7::W](scmpr7::W) writer structure"]
impl crate::Writable for SCMPR7 {}
#[doc = "Compare Register H"]
pub mod scmpr7;
#[doc = "Capture Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt0](scapt0) module"]
pub type SCAPT0 = crate::Reg<u32, _SCAPT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAPT0;
#[doc = "`read()` method returns [scapt0::R](scapt0::R) reader structure"]
impl crate::Readable for SCAPT0 {}
#[doc = "`write(|w| ..)` method takes [scapt0::W](scapt0::W) writer structure"]
impl crate::Writable for SCAPT0 {}
#[doc = "Capture Register A"]
pub mod scapt0;
#[doc = "Capture Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt1](scapt1) module"]
pub type SCAPT1 = crate::Reg<u32, _SCAPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAPT1;
#[doc = "`read()` method returns [scapt1::R](scapt1::R) reader structure"]
impl crate::Readable for SCAPT1 {}
#[doc = "`write(|w| ..)` method takes [scapt1::W](scapt1::W) writer structure"]
impl crate::Writable for SCAPT1 {}
#[doc = "Capture Register B"]
pub mod scapt1;
#[doc = "Capture Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt2](scapt2) module"]
pub type SCAPT2 = crate::Reg<u32, _SCAPT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAPT2;
#[doc = "`read()` method returns [scapt2::R](scapt2::R) reader structure"]
impl crate::Readable for SCAPT2 {}
#[doc = "`write(|w| ..)` method takes [scapt2::W](scapt2::W) writer structure"]
impl crate::Writable for SCAPT2 {}
#[doc = "Capture Register C"]
pub mod scapt2;
#[doc = "Capture Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scapt3](scapt3) module"]
pub type SCAPT3 = crate::Reg<u32, _SCAPT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCAPT3;
#[doc = "`read()` method returns [scapt3::R](scapt3::R) reader structure"]
impl crate::Readable for SCAPT3 {}
#[doc = "`write(|w| ..)` method takes [scapt3::W](scapt3::W) writer structure"]
impl crate::Writable for SCAPT3 {}
#[doc = "Capture Register D"]
pub mod scapt3;
#[doc = "System Timer NVRAM_A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr0](snvr0) module"]
pub type SNVR0 = crate::Reg<u32, _SNVR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNVR0;
#[doc = "`read()` method returns [snvr0::R](snvr0::R) reader structure"]
impl crate::Readable for SNVR0 {}
#[doc = "`write(|w| ..)` method takes [snvr0::W](snvr0::W) writer structure"]
impl crate::Writable for SNVR0 {}
#[doc = "System Timer NVRAM_A Register"]
pub mod snvr0;
#[doc = "System Timer NVRAM_B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr1](snvr1) module"]
pub type SNVR1 = crate::Reg<u32, _SNVR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNVR1;
#[doc = "`read()` method returns [snvr1::R](snvr1::R) reader structure"]
impl crate::Readable for SNVR1 {}
#[doc = "`write(|w| ..)` method takes [snvr1::W](snvr1::W) writer structure"]
impl crate::Writable for SNVR1 {}
#[doc = "System Timer NVRAM_B Register"]
pub mod snvr1;
#[doc = "System Timer NVRAM_C Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr2](snvr2) module"]
pub type SNVR2 = crate::Reg<u32, _SNVR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNVR2;
#[doc = "`read()` method returns [snvr2::R](snvr2::R) reader structure"]
impl crate::Readable for SNVR2 {}
#[doc = "`write(|w| ..)` method takes [snvr2::W](snvr2::W) writer structure"]
impl crate::Writable for SNVR2 {}
#[doc = "System Timer NVRAM_C Register"]
pub mod snvr2;
#[doc = "Counter/Timer Interrupts: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Counter/Timer Interrupts: Enable"]
pub mod inten;
#[doc = "Counter/Timer Interrupts: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "Counter/Timer Interrupts: Status"]
pub mod intstat;
#[doc = "Counter/Timer Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "Counter/Timer Interrupts: Clear"]
pub mod intclr;
#[doc = "Counter/Timer Interrupts: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "Counter/Timer Interrupts: Set"]
pub mod intset;
#[doc = "STIMER Interrupt registers: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stminten](stminten) module"]
pub type STMINTEN = crate::Reg<u32, _STMINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMINTEN;
#[doc = "`read()` method returns [stminten::R](stminten::R) reader structure"]
impl crate::Readable for STMINTEN {}
#[doc = "`write(|w| ..)` method takes [stminten::W](stminten::W) writer structure"]
impl crate::Writable for STMINTEN {}
#[doc = "STIMER Interrupt registers: Enable"]
pub mod stminten;
#[doc = "STIMER Interrupt registers: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmintstat](stmintstat) module"]
pub type STMINTSTAT = crate::Reg<u32, _STMINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMINTSTAT;
#[doc = "`read()` method returns [stmintstat::R](stmintstat::R) reader structure"]
impl crate::Readable for STMINTSTAT {}
#[doc = "`write(|w| ..)` method takes [stmintstat::W](stmintstat::W) writer structure"]
impl crate::Writable for STMINTSTAT {}
#[doc = "STIMER Interrupt registers: Status"]
pub mod stmintstat;
#[doc = "STIMER Interrupt registers: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmintclr](stmintclr) module"]
pub type STMINTCLR = crate::Reg<u32, _STMINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMINTCLR;
#[doc = "`read()` method returns [stmintclr::R](stmintclr::R) reader structure"]
impl crate::Readable for STMINTCLR {}
#[doc = "`write(|w| ..)` method takes [stmintclr::W](stmintclr::W) writer structure"]
impl crate::Writable for STMINTCLR {}
#[doc = "STIMER Interrupt registers: Clear"]
pub mod stmintclr;
#[doc = "STIMER Interrupt registers: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmintset](stmintset) module"]
pub type STMINTSET = crate::Reg<u32, _STMINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMINTSET;
#[doc = "`read()` method returns [stmintset::R](stmintset::R) reader structure"]
impl crate::Readable for STMINTSET {}
#[doc = "`write(|w| ..)` method takes [stmintset::W](stmintset::W) writer structure"]
impl crate::Writable for STMINTSET {}
#[doc = "STIMER Interrupt registers: Set"]
pub mod stmintset;
