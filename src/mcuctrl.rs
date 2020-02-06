#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip Information Register"]
    pub chip_info: CHIP_INFO,
    #[doc = "0x04 - Unique Chip ID 0"]
    pub chipid0: CHIPID0,
    #[doc = "0x08 - Unique Chip ID 1"]
    pub chipid1: CHIPID1,
    #[doc = "0x0c - Chip Revision"]
    pub chiprev: CHIPREV,
    #[doc = "0x10 - Unique Vendor ID"]
    pub vendorid: VENDORID,
    #[doc = "0x14 - Debugger Access Control"]
    pub debugger: DEBUGGER,
    _reserved6: [u8; 72usize],
    #[doc = "0x60 - Analog Buck Control"]
    pub buck: BUCK,
    _reserved7: [u8; 4usize],
    #[doc = "0x68 - Buck control reg 3"]
    pub buck3: BUCK3,
    _reserved8: [u8; 20usize],
    #[doc = "0x80 - Analog LDO Reg 1"]
    pub ldoreg1: LDOREG1,
    _reserved9: [u8; 4usize],
    #[doc = "0x88 - LDO Control Register 3"]
    pub ldoreg3: LDOREG3,
    _reserved10: [u8; 116usize],
    #[doc = "0x100 - BOD and PDR control Register"]
    pub bodporctrl: BODPORCTRL,
    #[doc = "0x104 - ADC Power Up Delay Control"]
    pub adcpwrdly: ADCPWRDLY,
    _reserved12: [u8; 4usize],
    #[doc = "0x10c - ADC Calibration Control"]
    pub adccal: ADCCAL,
    #[doc = "0x110 - ADC Battery Load Enable"]
    pub adcbattload: ADCBATTLOAD,
    #[doc = "0x114 - Trim settings for Core and Mem buck modules"]
    pub bucktrim: BUCKTRIM,
    _reserved15: [u8; 12usize],
    #[doc = "0x124 - XTAL Oscillator General Control"]
    pub xtalgenctrl: XTALGENCTRL,
    _reserved16: [u8; 120usize],
    #[doc = "0x1a0 - Determines whether the bootloader code is visible at address 0x00000000"]
    pub bootloaderlow: BOOTLOADERLOW,
    #[doc = "0x1a4 - Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
    pub shadowvalid: SHADOWVALID,
    _reserved18: [u8; 24usize],
    #[doc = "0x1c0 - ICODE bus address which was present when a bus fault occurred."]
    pub icodefaultaddr: ICODEFAULTADDR,
    #[doc = "0x1c4 - DCODE bus address which was present when a bus fault occurred."]
    pub dcodefaultaddr: DCODEFAULTADDR,
    #[doc = "0x1c8 - System bus address which was present when a bus fault occurred."]
    pub sysfaultaddr: SYSFAULTADDR,
    #[doc = "0x1cc - Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
    pub faultstatus: FAULTSTATUS,
    #[doc = "0x1d0 - Enable the fault capture registers"]
    pub faultcaptureen: FAULTCAPTUREEN,
    _reserved23: [u8; 44usize],
    #[doc = "0x200 - Read-only debug register 1"]
    pub dbgr1: DBGR1,
    #[doc = "0x204 - Read-only debug register 2"]
    pub dbgr2: DBGR2,
    _reserved25: [u8; 24usize],
    #[doc = "0x220 - Control bit to enable/disable the PMU"]
    pub pmuenable: PMUENABLE,
    _reserved26: [u8; 44usize],
    #[doc = "0x250 - TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
    pub tpiuctrl: TPIUCTRL,
}
#[doc = "Chip Information Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_info](chip_info) module"]
pub type CHIP_INFO = crate::Reg<u32, _CHIP_INFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIP_INFO;
#[doc = "`read()` method returns [chip_info::R](chip_info::R) reader structure"]
impl crate::Readable for CHIP_INFO {}
#[doc = "`write(|w| ..)` method takes [chip_info::W](chip_info::W) writer structure"]
impl crate::Writable for CHIP_INFO {}
#[doc = "Chip Information Register"]
pub mod chip_info;
#[doc = "Unique Chip ID 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid0](chipid0) module"]
pub type CHIPID0 = crate::Reg<u32, _CHIPID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPID0;
#[doc = "`read()` method returns [chipid0::R](chipid0::R) reader structure"]
impl crate::Readable for CHIPID0 {}
#[doc = "`write(|w| ..)` method takes [chipid0::W](chipid0::W) writer structure"]
impl crate::Writable for CHIPID0 {}
#[doc = "Unique Chip ID 0"]
pub mod chipid0;
#[doc = "Unique Chip ID 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid1](chipid1) module"]
pub type CHIPID1 = crate::Reg<u32, _CHIPID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPID1;
#[doc = "`read()` method returns [chipid1::R](chipid1::R) reader structure"]
impl crate::Readable for CHIPID1 {}
#[doc = "`write(|w| ..)` method takes [chipid1::W](chipid1::W) writer structure"]
impl crate::Writable for CHIPID1 {}
#[doc = "Unique Chip ID 1"]
pub mod chipid1;
#[doc = "Chip Revision\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chiprev](chiprev) module"]
pub type CHIPREV = crate::Reg<u32, _CHIPREV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPREV;
#[doc = "`read()` method returns [chiprev::R](chiprev::R) reader structure"]
impl crate::Readable for CHIPREV {}
#[doc = "`write(|w| ..)` method takes [chiprev::W](chiprev::W) writer structure"]
impl crate::Writable for CHIPREV {}
#[doc = "Chip Revision"]
pub mod chiprev;
#[doc = "Unique Vendor ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vendorid](vendorid) module"]
pub type VENDORID = crate::Reg<u32, _VENDORID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VENDORID;
#[doc = "`read()` method returns [vendorid::R](vendorid::R) reader structure"]
impl crate::Readable for VENDORID {}
#[doc = "`write(|w| ..)` method takes [vendorid::W](vendorid::W) writer structure"]
impl crate::Writable for VENDORID {}
#[doc = "Unique Vendor ID"]
pub mod vendorid;
#[doc = "Debugger Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debugger](debugger) module"]
pub type DEBUGGER = crate::Reg<u32, _DEBUGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUGGER;
#[doc = "`read()` method returns [debugger::R](debugger::R) reader structure"]
impl crate::Readable for DEBUGGER {}
#[doc = "`write(|w| ..)` method takes [debugger::W](debugger::W) writer structure"]
impl crate::Writable for DEBUGGER {}
#[doc = "Debugger Access Control"]
pub mod debugger;
#[doc = "Analog Buck Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buck](buck) module"]
pub type BUCK = crate::Reg<u32, _BUCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUCK;
#[doc = "`read()` method returns [buck::R](buck::R) reader structure"]
impl crate::Readable for BUCK {}
#[doc = "`write(|w| ..)` method takes [buck::W](buck::W) writer structure"]
impl crate::Writable for BUCK {}
#[doc = "Analog Buck Control"]
pub mod buck;
#[doc = "Buck control reg 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buck3](buck3) module"]
pub type BUCK3 = crate::Reg<u32, _BUCK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUCK3;
#[doc = "`read()` method returns [buck3::R](buck3::R) reader structure"]
impl crate::Readable for BUCK3 {}
#[doc = "`write(|w| ..)` method takes [buck3::W](buck3::W) writer structure"]
impl crate::Writable for BUCK3 {}
#[doc = "Buck control reg 3"]
pub mod buck3;
#[doc = "Analog LDO Reg 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldoreg1](ldoreg1) module"]
pub type LDOREG1 = crate::Reg<u32, _LDOREG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDOREG1;
#[doc = "`read()` method returns [ldoreg1::R](ldoreg1::R) reader structure"]
impl crate::Readable for LDOREG1 {}
#[doc = "`write(|w| ..)` method takes [ldoreg1::W](ldoreg1::W) writer structure"]
impl crate::Writable for LDOREG1 {}
#[doc = "Analog LDO Reg 1"]
pub mod ldoreg1;
#[doc = "LDO Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldoreg3](ldoreg3) module"]
pub type LDOREG3 = crate::Reg<u32, _LDOREG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDOREG3;
#[doc = "`read()` method returns [ldoreg3::R](ldoreg3::R) reader structure"]
impl crate::Readable for LDOREG3 {}
#[doc = "`write(|w| ..)` method takes [ldoreg3::W](ldoreg3::W) writer structure"]
impl crate::Writable for LDOREG3 {}
#[doc = "LDO Control Register 3"]
pub mod ldoreg3;
#[doc = "BOD and PDR control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodporctrl](bodporctrl) module"]
pub type BODPORCTRL = crate::Reg<u32, _BODPORCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BODPORCTRL;
#[doc = "`read()` method returns [bodporctrl::R](bodporctrl::R) reader structure"]
impl crate::Readable for BODPORCTRL {}
#[doc = "`write(|w| ..)` method takes [bodporctrl::W](bodporctrl::W) writer structure"]
impl crate::Writable for BODPORCTRL {}
#[doc = "BOD and PDR control Register"]
pub mod bodporctrl;
#[doc = "ADC Power Up Delay Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcpwrdly](adcpwrdly) module"]
pub type ADCPWRDLY = crate::Reg<u32, _ADCPWRDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCPWRDLY;
#[doc = "`read()` method returns [adcpwrdly::R](adcpwrdly::R) reader structure"]
impl crate::Readable for ADCPWRDLY {}
#[doc = "`write(|w| ..)` method takes [adcpwrdly::W](adcpwrdly::W) writer structure"]
impl crate::Writable for ADCPWRDLY {}
#[doc = "ADC Power Up Delay Control"]
pub mod adcpwrdly;
#[doc = "ADC Calibration Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adccal](adccal) module"]
pub type ADCCAL = crate::Reg<u32, _ADCCAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCAL;
#[doc = "`read()` method returns [adccal::R](adccal::R) reader structure"]
impl crate::Readable for ADCCAL {}
#[doc = "`write(|w| ..)` method takes [adccal::W](adccal::W) writer structure"]
impl crate::Writable for ADCCAL {}
#[doc = "ADC Calibration Control"]
pub mod adccal;
#[doc = "ADC Battery Load Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcbattload](adcbattload) module"]
pub type ADCBATTLOAD = crate::Reg<u32, _ADCBATTLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCBATTLOAD;
#[doc = "`read()` method returns [adcbattload::R](adcbattload::R) reader structure"]
impl crate::Readable for ADCBATTLOAD {}
#[doc = "`write(|w| ..)` method takes [adcbattload::W](adcbattload::W) writer structure"]
impl crate::Writable for ADCBATTLOAD {}
#[doc = "ADC Battery Load Enable"]
pub mod adcbattload;
#[doc = "Trim settings for Core and Mem buck modules\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bucktrim](bucktrim) module"]
pub type BUCKTRIM = crate::Reg<u32, _BUCKTRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUCKTRIM;
#[doc = "`read()` method returns [bucktrim::R](bucktrim::R) reader structure"]
impl crate::Readable for BUCKTRIM {}
#[doc = "`write(|w| ..)` method takes [bucktrim::W](bucktrim::W) writer structure"]
impl crate::Writable for BUCKTRIM {}
#[doc = "Trim settings for Core and Mem buck modules"]
pub mod bucktrim;
#[doc = "XTAL Oscillator General Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalgenctrl](xtalgenctrl) module"]
pub type XTALGENCTRL = crate::Reg<u32, _XTALGENCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTALGENCTRL;
#[doc = "`read()` method returns [xtalgenctrl::R](xtalgenctrl::R) reader structure"]
impl crate::Readable for XTALGENCTRL {}
#[doc = "`write(|w| ..)` method takes [xtalgenctrl::W](xtalgenctrl::W) writer structure"]
impl crate::Writable for XTALGENCTRL {}
#[doc = "XTAL Oscillator General Control"]
pub mod xtalgenctrl;
#[doc = "Determines whether the bootloader code is visible at address 0x00000000\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootloaderlow](bootloaderlow) module"]
pub type BOOTLOADERLOW = crate::Reg<u32, _BOOTLOADERLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOTLOADERLOW;
#[doc = "`read()` method returns [bootloaderlow::R](bootloaderlow::R) reader structure"]
impl crate::Readable for BOOTLOADERLOW {}
#[doc = "`write(|w| ..)` method takes [bootloaderlow::W](bootloaderlow::W) writer structure"]
impl crate::Writable for BOOTLOADERLOW {}
#[doc = "Determines whether the bootloader code is visible at address 0x00000000"]
pub mod bootloaderlow;
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shadowvalid](shadowvalid) module"]
pub type SHADOWVALID = crate::Reg<u32, _SHADOWVALID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHADOWVALID;
#[doc = "`read()` method returns [shadowvalid::R](shadowvalid::R) reader structure"]
impl crate::Readable for SHADOWVALID {}
#[doc = "`write(|w| ..)` method takes [shadowvalid::W](shadowvalid::W) writer structure"]
impl crate::Writable for SHADOWVALID {}
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
pub mod shadowvalid;
#[doc = "ICODE bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icodefaultaddr](icodefaultaddr) module"]
pub type ICODEFAULTADDR = crate::Reg<u32, _ICODEFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICODEFAULTADDR;
#[doc = "`read()` method returns [icodefaultaddr::R](icodefaultaddr::R) reader structure"]
impl crate::Readable for ICODEFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [icodefaultaddr::W](icodefaultaddr::W) writer structure"]
impl crate::Writable for ICODEFAULTADDR {}
#[doc = "ICODE bus address which was present when a bus fault occurred."]
pub mod icodefaultaddr;
#[doc = "DCODE bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcodefaultaddr](dcodefaultaddr) module"]
pub type DCODEFAULTADDR = crate::Reg<u32, _DCODEFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCODEFAULTADDR;
#[doc = "`read()` method returns [dcodefaultaddr::R](dcodefaultaddr::R) reader structure"]
impl crate::Readable for DCODEFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [dcodefaultaddr::W](dcodefaultaddr::W) writer structure"]
impl crate::Writable for DCODEFAULTADDR {}
#[doc = "DCODE bus address which was present when a bus fault occurred."]
pub mod dcodefaultaddr;
#[doc = "System bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysfaultaddr](sysfaultaddr) module"]
pub type SYSFAULTADDR = crate::Reg<u32, _SYSFAULTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSFAULTADDR;
#[doc = "`read()` method returns [sysfaultaddr::R](sysfaultaddr::R) reader structure"]
impl crate::Readable for SYSFAULTADDR {}
#[doc = "`write(|w| ..)` method takes [sysfaultaddr::W](sysfaultaddr::W) writer structure"]
impl crate::Writable for SYSFAULTADDR {}
#[doc = "System bus address which was present when a bus fault occurred."]
pub mod sysfaultaddr;
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultstatus](faultstatus) module"]
pub type FAULTSTATUS = crate::Reg<u32, _FAULTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULTSTATUS;
#[doc = "`read()` method returns [faultstatus::R](faultstatus::R) reader structure"]
impl crate::Readable for FAULTSTATUS {}
#[doc = "`write(|w| ..)` method takes [faultstatus::W](faultstatus::W) writer structure"]
impl crate::Writable for FAULTSTATUS {}
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
pub mod faultstatus;
#[doc = "Enable the fault capture registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultcaptureen](faultcaptureen) module"]
pub type FAULTCAPTUREEN = crate::Reg<u32, _FAULTCAPTUREEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULTCAPTUREEN;
#[doc = "`read()` method returns [faultcaptureen::R](faultcaptureen::R) reader structure"]
impl crate::Readable for FAULTCAPTUREEN {}
#[doc = "`write(|w| ..)` method takes [faultcaptureen::W](faultcaptureen::W) writer structure"]
impl crate::Writable for FAULTCAPTUREEN {}
#[doc = "Enable the fault capture registers"]
pub mod faultcaptureen;
#[doc = "Read-only debug register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgr1](dbgr1) module"]
pub type DBGR1 = crate::Reg<u32, _DBGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGR1;
#[doc = "`read()` method returns [dbgr1::R](dbgr1::R) reader structure"]
impl crate::Readable for DBGR1 {}
#[doc = "`write(|w| ..)` method takes [dbgr1::W](dbgr1::W) writer structure"]
impl crate::Writable for DBGR1 {}
#[doc = "Read-only debug register 1"]
pub mod dbgr1;
#[doc = "Read-only debug register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgr2](dbgr2) module"]
pub type DBGR2 = crate::Reg<u32, _DBGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGR2;
#[doc = "`read()` method returns [dbgr2::R](dbgr2::R) reader structure"]
impl crate::Readable for DBGR2 {}
#[doc = "`write(|w| ..)` method takes [dbgr2::W](dbgr2::W) writer structure"]
impl crate::Writable for DBGR2 {}
#[doc = "Read-only debug register 2"]
pub mod dbgr2;
#[doc = "Control bit to enable/disable the PMU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmuenable](pmuenable) module"]
pub type PMUENABLE = crate::Reg<u32, _PMUENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUENABLE;
#[doc = "`read()` method returns [pmuenable::R](pmuenable::R) reader structure"]
impl crate::Readable for PMUENABLE {}
#[doc = "`write(|w| ..)` method takes [pmuenable::W](pmuenable::W) writer structure"]
impl crate::Writable for PMUENABLE {}
#[doc = "Control bit to enable/disable the PMU"]
pub mod pmuenable;
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpiuctrl](tpiuctrl) module"]
pub type TPIUCTRL = crate::Reg<u32, _TPIUCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPIUCTRL;
#[doc = "`read()` method returns [tpiuctrl::R](tpiuctrl::R) reader structure"]
impl crate::Readable for TPIUCTRL {}
#[doc = "`write(|w| ..)` method takes [tpiuctrl::W](tpiuctrl::W) writer structure"]
impl crate::Writable for TPIUCTRL {}
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
pub mod tpiuctrl;
