#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory and Core Voltage Supply Source Select Register"]
    pub supplysrc: SUPPLYSRC,
    #[doc = "0x04 - Power Status Register for MCU supplies and peripherals"]
    pub powerstatus: POWERSTATUS,
    #[doc = "0x08 - DEVICE ENABLES for SHELBY"]
    pub deviceen: DEVICEEN,
    #[doc = "0x0c - Powerdown an SRAM Banks in Deep Sleep mode"]
    pub srampwdinsleep: SRAMPWDINSLEEP,
    #[doc = "0x10 - Disables individual banks of the MEMORY array"]
    pub memen: MEMEN,
    #[doc = "0x14 - POWER ON Status"]
    pub pwronstatus: PWRONSTATUS,
    #[doc = "0x18 - SRAM Control register"]
    pub sramctrl: SRAMCTRL,
    #[doc = "0x1c - Power Status Register for ADC Block"]
    pub adcstatus: ADCSTATUS,
    #[doc = "0x20 - Power Optimization Control Bits"]
    pub miscopt: MISCOPT,
}
#[doc = "Memory and Core Voltage Supply Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supplysrc](supplysrc) module"]
pub type SUPPLYSRC = crate::Reg<u32, _SUPPLYSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPPLYSRC;
#[doc = "`read()` method returns [supplysrc::R](supplysrc::R) reader structure"]
impl crate::Readable for SUPPLYSRC {}
#[doc = "`write(|w| ..)` method takes [supplysrc::W](supplysrc::W) writer structure"]
impl crate::Writable for SUPPLYSRC {}
#[doc = "Memory and Core Voltage Supply Source Select Register"]
pub mod supplysrc;
#[doc = "Power Status Register for MCU supplies and peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerstatus](powerstatus) module"]
pub type POWERSTATUS = crate::Reg<u32, _POWERSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWERSTATUS;
#[doc = "`read()` method returns [powerstatus::R](powerstatus::R) reader structure"]
impl crate::Readable for POWERSTATUS {}
#[doc = "`write(|w| ..)` method takes [powerstatus::W](powerstatus::W) writer structure"]
impl crate::Writable for POWERSTATUS {}
#[doc = "Power Status Register for MCU supplies and peripherals"]
pub mod powerstatus;
#[doc = "DEVICE ENABLES for SHELBY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceen](deviceen) module"]
pub type DEVICEEN = crate::Reg<u32, _DEVICEEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEEN;
#[doc = "`read()` method returns [deviceen::R](deviceen::R) reader structure"]
impl crate::Readable for DEVICEEN {}
#[doc = "`write(|w| ..)` method takes [deviceen::W](deviceen::W) writer structure"]
impl crate::Writable for DEVICEEN {}
#[doc = "DEVICE ENABLES for SHELBY"]
pub mod deviceen;
#[doc = "Powerdown an SRAM Banks in Deep Sleep mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srampwdinsleep](srampwdinsleep) module"]
pub type SRAMPWDINSLEEP = crate::Reg<u32, _SRAMPWDINSLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMPWDINSLEEP;
#[doc = "`read()` method returns [srampwdinsleep::R](srampwdinsleep::R) reader structure"]
impl crate::Readable for SRAMPWDINSLEEP {}
#[doc = "`write(|w| ..)` method takes [srampwdinsleep::W](srampwdinsleep::W) writer structure"]
impl crate::Writable for SRAMPWDINSLEEP {}
#[doc = "Powerdown an SRAM Banks in Deep Sleep mode"]
pub mod srampwdinsleep;
#[doc = "Disables individual banks of the MEMORY array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memen](memen) module"]
pub type MEMEN = crate::Reg<u32, _MEMEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMEN;
#[doc = "`read()` method returns [memen::R](memen::R) reader structure"]
impl crate::Readable for MEMEN {}
#[doc = "`write(|w| ..)` method takes [memen::W](memen::W) writer structure"]
impl crate::Writable for MEMEN {}
#[doc = "Disables individual banks of the MEMORY array"]
pub mod memen;
#[doc = "POWER ON Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwronstatus](pwronstatus) module"]
pub type PWRONSTATUS = crate::Reg<u32, _PWRONSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRONSTATUS;
#[doc = "`read()` method returns [pwronstatus::R](pwronstatus::R) reader structure"]
impl crate::Readable for PWRONSTATUS {}
#[doc = "`write(|w| ..)` method takes [pwronstatus::W](pwronstatus::W) writer structure"]
impl crate::Writable for PWRONSTATUS {}
#[doc = "POWER ON Status"]
pub mod pwronstatus;
#[doc = "SRAM Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramctrl](sramctrl) module"]
pub type SRAMCTRL = crate::Reg<u32, _SRAMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRAMCTRL;
#[doc = "`read()` method returns [sramctrl::R](sramctrl::R) reader structure"]
impl crate::Readable for SRAMCTRL {}
#[doc = "`write(|w| ..)` method takes [sramctrl::W](sramctrl::W) writer structure"]
impl crate::Writable for SRAMCTRL {}
#[doc = "SRAM Control register"]
pub mod sramctrl;
#[doc = "Power Status Register for ADC Block\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcstatus](adcstatus) module"]
pub type ADCSTATUS = crate::Reg<u32, _ADCSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCSTATUS;
#[doc = "`read()` method returns [adcstatus::R](adcstatus::R) reader structure"]
impl crate::Readable for ADCSTATUS {}
#[doc = "`write(|w| ..)` method takes [adcstatus::W](adcstatus::W) writer structure"]
impl crate::Writable for ADCSTATUS {}
#[doc = "Power Status Register for ADC Block"]
pub mod adcstatus;
#[doc = "Power Optimization Control Bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miscopt](miscopt) module"]
pub type MISCOPT = crate::Reg<u32, _MISCOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISCOPT;
#[doc = "`read()` method returns [miscopt::R](miscopt::R) reader structure"]
impl crate::Readable for MISCOPT {}
#[doc = "`write(|w| ..)` method takes [miscopt::W](miscopt::W) writer structure"]
impl crate::Writable for MISCOPT {}
#[doc = "Power Optimization Control Bits"]
pub mod miscopt;
