#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Synchronization Size Configuration Register"]
    pub sscr: SSCR,
    #[doc = "0x0c - Back Porch Configuration Register"]
    pub bpcr: BPCR,
    #[doc = "0x10 - Active Width Configuration Register"]
    pub awcr: AWCR,
    #[doc = "0x14 - Total Width Configuration Register"]
    pub twcr: TWCR,
    #[doc = "0x18 - Global Control Register"]
    pub gcr: GCR,
    _reserved5: [u8; 8usize],
    #[doc = "0x24 - Shadow Reload Configuration Register"]
    pub srcr: SRCR,
    _reserved6: [u8; 4usize],
    #[doc = "0x2c - Background Color Configuration Register"]
    pub bccr: BCCR,
    _reserved7: [u8; 4usize],
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x38 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x3c - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x40 - Line Interrupt Position Configuration Register"]
    pub lipcr: LIPCR,
    #[doc = "0x44 - Current Position Status Register"]
    pub cpsr: CPSR,
    #[doc = "0x48 - Current Display Status Register"]
    pub cdsr: CDSR,
    _reserved13: [u8; 56usize],
    #[doc = "0x84 - Layerx Control Register"]
    pub l1cr: L1CR,
    #[doc = "0x88 - Layerx Window Horizontal Position Configuration Register"]
    pub l1whpcr: L1WHPCR,
    #[doc = "0x8c - Layerx Window Vertical Position Configuration Register"]
    pub l1wvpcr: L1WVPCR,
    #[doc = "0x90 - Layerx Color Keying Configuration Register"]
    pub l1ckcr: L1CKCR,
    #[doc = "0x94 - Layerx Pixel Format Configuration Register"]
    pub l1pfcr: L1PFCR,
    #[doc = "0x98 - Layerx Constant Alpha Configuration Register"]
    pub l1cacr: L1CACR,
    #[doc = "0x9c - Layerx Default Color Configuration Register"]
    pub l1dccr: L1DCCR,
    #[doc = "0xa0 - Layerx Blending Factors Configuration Register"]
    pub l1bfcr: L1BFCR,
    _reserved21: [u8; 8usize],
    #[doc = "0xac - Layerx Color Frame Buffer Address Register"]
    pub l1cfbar: L1CFBAR,
    #[doc = "0xb0 - Layerx Color Frame Buffer Length Register"]
    pub l1cfblr: L1CFBLR,
    #[doc = "0xb4 - Layerx ColorFrame Buffer Line Number Register"]
    pub l1cfblnr: L1CFBLNR,
    _reserved24: [u8; 12usize],
    #[doc = "0xc4 - Layerx CLUT Write Register"]
    pub l1clutwr: L1CLUTWR,
    _reserved25: [u8; 60usize],
    #[doc = "0x104 - Layerx Control Register"]
    pub l2cr: L2CR,
    #[doc = "0x108 - Layerx Window Horizontal Position Configuration Register"]
    pub l2whpcr: L2WHPCR,
    #[doc = "0x10c - Layerx Window Vertical Position Configuration Register"]
    pub l2wvpcr: L2WVPCR,
    #[doc = "0x110 - Layerx Color Keying Configuration Register"]
    pub l2ckcr: L2CKCR,
    #[doc = "0x114 - Layerx Pixel Format Configuration Register"]
    pub l2pfcr: L2PFCR,
    #[doc = "0x118 - Layerx Constant Alpha Configuration Register"]
    pub l2cacr: L2CACR,
    #[doc = "0x11c - Layerx Default Color Configuration Register"]
    pub l2dccr: L2DCCR,
    #[doc = "0x120 - Layerx Blending Factors Configuration Register"]
    pub l2bfcr: L2BFCR,
    _reserved33: [u8; 8usize],
    #[doc = "0x12c - Layerx Color Frame Buffer Address Register"]
    pub l2cfbar: L2CFBAR,
    #[doc = "0x130 - Layerx Color Frame Buffer Length Register"]
    pub l2cfblr: L2CFBLR,
    #[doc = "0x134 - Layerx ColorFrame Buffer Line Number Register"]
    pub l2cfblnr: L2CFBLNR,
    _reserved36: [u8; 12usize],
    #[doc = "0x144 - Layerx CLUT Write Register"]
    pub l2clutwr: L2CLUTWR,
}
#[doc = "Synchronization Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscr](sscr) module"]
pub type SSCR = crate::Reg<u32, _SSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCR;
#[doc = "`read()` method returns [sscr::R](sscr::R) reader structure"]
impl crate::Readable for SSCR {}
#[doc = "`write(|w| ..)` method takes [sscr::W](sscr::W) writer structure"]
impl crate::Writable for SSCR {}
#[doc = "Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "Back Porch Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpcr](bpcr) module"]
pub type BPCR = crate::Reg<u32, _BPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPCR;
#[doc = "`read()` method returns [bpcr::R](bpcr::R) reader structure"]
impl crate::Readable for BPCR {}
#[doc = "`write(|w| ..)` method takes [bpcr::W](bpcr::W) writer structure"]
impl crate::Writable for BPCR {}
#[doc = "Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "Active Width Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awcr](awcr) module"]
pub type AWCR = crate::Reg<u32, _AWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWCR;
#[doc = "`read()` method returns [awcr::R](awcr::R) reader structure"]
impl crate::Readable for AWCR {}
#[doc = "`write(|w| ..)` method takes [awcr::W](awcr::W) writer structure"]
impl crate::Writable for AWCR {}
#[doc = "Active Width Configuration Register"]
pub mod awcr;
#[doc = "Total Width Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twcr](twcr) module"]
pub type TWCR = crate::Reg<u32, _TWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWCR;
#[doc = "`read()` method returns [twcr::R](twcr::R) reader structure"]
impl crate::Readable for TWCR {}
#[doc = "`write(|w| ..)` method takes [twcr::W](twcr::W) writer structure"]
impl crate::Writable for TWCR {}
#[doc = "Total Width Configuration Register"]
pub mod twcr;
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](gcr) module"]
pub type GCR = crate::Reg<u32, _GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCR;
#[doc = "`read()` method returns [gcr::R](gcr::R) reader structure"]
impl crate::Readable for GCR {}
#[doc = "`write(|w| ..)` method takes [gcr::W](gcr::W) writer structure"]
impl crate::Writable for GCR {}
#[doc = "Global Control Register"]
pub mod gcr;
#[doc = "Shadow Reload Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcr](srcr) module"]
pub type SRCR = crate::Reg<u32, _SRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR;
#[doc = "`read()` method returns [srcr::R](srcr::R) reader structure"]
impl crate::Readable for SRCR {}
#[doc = "`write(|w| ..)` method takes [srcr::W](srcr::W) writer structure"]
impl crate::Writable for SRCR {}
#[doc = "Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "Background Color Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bccr](bccr) module"]
pub type BCCR = crate::Reg<u32, _BCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCCR;
#[doc = "`read()` method returns [bccr::R](bccr::R) reader structure"]
impl crate::Readable for BCCR {}
#[doc = "`write(|w| ..)` method takes [bccr::W](bccr::W) writer structure"]
impl crate::Writable for BCCR {}
#[doc = "Background Color Configuration Register"]
pub mod bccr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Line Interrupt Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lipcr](lipcr) module"]
pub type LIPCR = crate::Reg<u32, _LIPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIPCR;
#[doc = "`read()` method returns [lipcr::R](lipcr::R) reader structure"]
impl crate::Readable for LIPCR {}
#[doc = "`write(|w| ..)` method takes [lipcr::W](lipcr::W) writer structure"]
impl crate::Writable for LIPCR {}
#[doc = "Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "Current Position Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpsr](cpsr) module"]
pub type CPSR = crate::Reg<u32, _CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPSR;
#[doc = "`read()` method returns [cpsr::R](cpsr::R) reader structure"]
impl crate::Readable for CPSR {}
#[doc = "Current Position Status Register"]
pub mod cpsr;
#[doc = "Current Display Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdsr](cdsr) module"]
pub type CDSR = crate::Reg<u32, _CDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDSR;
#[doc = "`read()` method returns [cdsr::R](cdsr::R) reader structure"]
impl crate::Readable for CDSR {}
#[doc = "Current Display Status Register"]
pub mod cdsr;
#[doc = "Layerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1cr](l1cr) module"]
pub type L1CR = crate::Reg<u32, _L1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CR;
#[doc = "`read()` method returns [l1cr::R](l1cr::R) reader structure"]
impl crate::Readable for L1CR {}
#[doc = "`write(|w| ..)` method takes [l1cr::W](l1cr::W) writer structure"]
impl crate::Writable for L1CR {}
#[doc = "Layerx Control Register"]
pub mod l1cr;
#[doc = "Layerx Window Horizontal Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1whpcr](l1whpcr) module"]
pub type L1WHPCR = crate::Reg<u32, _L1WHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1WHPCR;
#[doc = "`read()` method returns [l1whpcr::R](l1whpcr::R) reader structure"]
impl crate::Readable for L1WHPCR {}
#[doc = "`write(|w| ..)` method takes [l1whpcr::W](l1whpcr::W) writer structure"]
impl crate::Writable for L1WHPCR {}
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l1whpcr;
#[doc = "Layerx Window Vertical Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1wvpcr](l1wvpcr) module"]
pub type L1WVPCR = crate::Reg<u32, _L1WVPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1WVPCR;
#[doc = "`read()` method returns [l1wvpcr::R](l1wvpcr::R) reader structure"]
impl crate::Readable for L1WVPCR {}
#[doc = "`write(|w| ..)` method takes [l1wvpcr::W](l1wvpcr::W) writer structure"]
impl crate::Writable for L1WVPCR {}
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l1wvpcr;
#[doc = "Layerx Color Keying Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1ckcr](l1ckcr) module"]
pub type L1CKCR = crate::Reg<u32, _L1CKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CKCR;
#[doc = "`read()` method returns [l1ckcr::R](l1ckcr::R) reader structure"]
impl crate::Readable for L1CKCR {}
#[doc = "`write(|w| ..)` method takes [l1ckcr::W](l1ckcr::W) writer structure"]
impl crate::Writable for L1CKCR {}
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l1ckcr;
#[doc = "Layerx Pixel Format Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1pfcr](l1pfcr) module"]
pub type L1PFCR = crate::Reg<u32, _L1PFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1PFCR;
#[doc = "`read()` method returns [l1pfcr::R](l1pfcr::R) reader structure"]
impl crate::Readable for L1PFCR {}
#[doc = "`write(|w| ..)` method takes [l1pfcr::W](l1pfcr::W) writer structure"]
impl crate::Writable for L1PFCR {}
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l1pfcr;
#[doc = "Layerx Constant Alpha Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1cacr](l1cacr) module"]
pub type L1CACR = crate::Reg<u32, _L1CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CACR;
#[doc = "`read()` method returns [l1cacr::R](l1cacr::R) reader structure"]
impl crate::Readable for L1CACR {}
#[doc = "`write(|w| ..)` method takes [l1cacr::W](l1cacr::W) writer structure"]
impl crate::Writable for L1CACR {}
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l1cacr;
#[doc = "Layerx Default Color Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1dccr](l1dccr) module"]
pub type L1DCCR = crate::Reg<u32, _L1DCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1DCCR;
#[doc = "`read()` method returns [l1dccr::R](l1dccr::R) reader structure"]
impl crate::Readable for L1DCCR {}
#[doc = "`write(|w| ..)` method takes [l1dccr::W](l1dccr::W) writer structure"]
impl crate::Writable for L1DCCR {}
#[doc = "Layerx Default Color Configuration Register"]
pub mod l1dccr;
#[doc = "Layerx Blending Factors Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1bfcr](l1bfcr) module"]
pub type L1BFCR = crate::Reg<u32, _L1BFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1BFCR;
#[doc = "`read()` method returns [l1bfcr::R](l1bfcr::R) reader structure"]
impl crate::Readable for L1BFCR {}
#[doc = "`write(|w| ..)` method takes [l1bfcr::W](l1bfcr::W) writer structure"]
impl crate::Writable for L1BFCR {}
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l1bfcr;
#[doc = "Layerx Color Frame Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1cfbar](l1cfbar) module"]
pub type L1CFBAR = crate::Reg<u32, _L1CFBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CFBAR;
#[doc = "`read()` method returns [l1cfbar::R](l1cfbar::R) reader structure"]
impl crate::Readable for L1CFBAR {}
#[doc = "`write(|w| ..)` method takes [l1cfbar::W](l1cfbar::W) writer structure"]
impl crate::Writable for L1CFBAR {}
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l1cfbar;
#[doc = "Layerx Color Frame Buffer Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1cfblr](l1cfblr) module"]
pub type L1CFBLR = crate::Reg<u32, _L1CFBLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CFBLR;
#[doc = "`read()` method returns [l1cfblr::R](l1cfblr::R) reader structure"]
impl crate::Readable for L1CFBLR {}
#[doc = "`write(|w| ..)` method takes [l1cfblr::W](l1cfblr::W) writer structure"]
impl crate::Writable for L1CFBLR {}
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l1cfblr;
#[doc = "Layerx ColorFrame Buffer Line Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1cfblnr](l1cfblnr) module"]
pub type L1CFBLNR = crate::Reg<u32, _L1CFBLNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CFBLNR;
#[doc = "`read()` method returns [l1cfblnr::R](l1cfblnr::R) reader structure"]
impl crate::Readable for L1CFBLNR {}
#[doc = "`write(|w| ..)` method takes [l1cfblnr::W](l1cfblnr::W) writer structure"]
impl crate::Writable for L1CFBLNR {}
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l1cfblnr;
#[doc = "Layerx CLUT Write Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1clutwr](l1clutwr) module"]
pub type L1CLUTWR = crate::Reg<u32, _L1CLUTWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CLUTWR;
#[doc = "`write(|w| ..)` method takes [l1clutwr::W](l1clutwr::W) writer structure"]
impl crate::Writable for L1CLUTWR {}
#[doc = "Layerx CLUT Write Register"]
pub mod l1clutwr;
#[doc = "Layerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2cr](l2cr) module"]
pub type L2CR = crate::Reg<u32, _L2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CR;
#[doc = "`read()` method returns [l2cr::R](l2cr::R) reader structure"]
impl crate::Readable for L2CR {}
#[doc = "`write(|w| ..)` method takes [l2cr::W](l2cr::W) writer structure"]
impl crate::Writable for L2CR {}
#[doc = "Layerx Control Register"]
pub mod l2cr;
#[doc = "Layerx Window Horizontal Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2whpcr](l2whpcr) module"]
pub type L2WHPCR = crate::Reg<u32, _L2WHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2WHPCR;
#[doc = "`read()` method returns [l2whpcr::R](l2whpcr::R) reader structure"]
impl crate::Readable for L2WHPCR {}
#[doc = "`write(|w| ..)` method takes [l2whpcr::W](l2whpcr::W) writer structure"]
impl crate::Writable for L2WHPCR {}
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l2whpcr;
#[doc = "Layerx Window Vertical Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2wvpcr](l2wvpcr) module"]
pub type L2WVPCR = crate::Reg<u32, _L2WVPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2WVPCR;
#[doc = "`read()` method returns [l2wvpcr::R](l2wvpcr::R) reader structure"]
impl crate::Readable for L2WVPCR {}
#[doc = "`write(|w| ..)` method takes [l2wvpcr::W](l2wvpcr::W) writer structure"]
impl crate::Writable for L2WVPCR {}
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l2wvpcr;
#[doc = "Layerx Color Keying Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2ckcr](l2ckcr) module"]
pub type L2CKCR = crate::Reg<u32, _L2CKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CKCR;
#[doc = "`read()` method returns [l2ckcr::R](l2ckcr::R) reader structure"]
impl crate::Readable for L2CKCR {}
#[doc = "`write(|w| ..)` method takes [l2ckcr::W](l2ckcr::W) writer structure"]
impl crate::Writable for L2CKCR {}
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l2ckcr;
#[doc = "Layerx Pixel Format Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2pfcr](l2pfcr) module"]
pub type L2PFCR = crate::Reg<u32, _L2PFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2PFCR;
#[doc = "`read()` method returns [l2pfcr::R](l2pfcr::R) reader structure"]
impl crate::Readable for L2PFCR {}
#[doc = "`write(|w| ..)` method takes [l2pfcr::W](l2pfcr::W) writer structure"]
impl crate::Writable for L2PFCR {}
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l2pfcr;
#[doc = "Layerx Constant Alpha Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2cacr](l2cacr) module"]
pub type L2CACR = crate::Reg<u32, _L2CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CACR;
#[doc = "`read()` method returns [l2cacr::R](l2cacr::R) reader structure"]
impl crate::Readable for L2CACR {}
#[doc = "`write(|w| ..)` method takes [l2cacr::W](l2cacr::W) writer structure"]
impl crate::Writable for L2CACR {}
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l2cacr;
#[doc = "Layerx Default Color Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2dccr](l2dccr) module"]
pub type L2DCCR = crate::Reg<u32, _L2DCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2DCCR;
#[doc = "`read()` method returns [l2dccr::R](l2dccr::R) reader structure"]
impl crate::Readable for L2DCCR {}
#[doc = "`write(|w| ..)` method takes [l2dccr::W](l2dccr::W) writer structure"]
impl crate::Writable for L2DCCR {}
#[doc = "Layerx Default Color Configuration Register"]
pub mod l2dccr;
#[doc = "Layerx Blending Factors Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2bfcr](l2bfcr) module"]
pub type L2BFCR = crate::Reg<u32, _L2BFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2BFCR;
#[doc = "`read()` method returns [l2bfcr::R](l2bfcr::R) reader structure"]
impl crate::Readable for L2BFCR {}
#[doc = "`write(|w| ..)` method takes [l2bfcr::W](l2bfcr::W) writer structure"]
impl crate::Writable for L2BFCR {}
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l2bfcr;
#[doc = "Layerx Color Frame Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2cfbar](l2cfbar) module"]
pub type L2CFBAR = crate::Reg<u32, _L2CFBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CFBAR;
#[doc = "`read()` method returns [l2cfbar::R](l2cfbar::R) reader structure"]
impl crate::Readable for L2CFBAR {}
#[doc = "`write(|w| ..)` method takes [l2cfbar::W](l2cfbar::W) writer structure"]
impl crate::Writable for L2CFBAR {}
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l2cfbar;
#[doc = "Layerx Color Frame Buffer Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2cfblr](l2cfblr) module"]
pub type L2CFBLR = crate::Reg<u32, _L2CFBLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CFBLR;
#[doc = "`read()` method returns [l2cfblr::R](l2cfblr::R) reader structure"]
impl crate::Readable for L2CFBLR {}
#[doc = "`write(|w| ..)` method takes [l2cfblr::W](l2cfblr::W) writer structure"]
impl crate::Writable for L2CFBLR {}
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l2cfblr;
#[doc = "Layerx ColorFrame Buffer Line Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2cfblnr](l2cfblnr) module"]
pub type L2CFBLNR = crate::Reg<u32, _L2CFBLNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CFBLNR;
#[doc = "`read()` method returns [l2cfblnr::R](l2cfblnr::R) reader structure"]
impl crate::Readable for L2CFBLNR {}
#[doc = "`write(|w| ..)` method takes [l2cfblnr::W](l2cfblnr::W) writer structure"]
impl crate::Writable for L2CFBLNR {}
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l2cfblnr;
#[doc = "Layerx CLUT Write Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2clutwr](l2clutwr) module"]
pub type L2CLUTWR = crate::Reg<u32, _L2CLUTWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CLUTWR;
#[doc = "`write(|w| ..)` method takes [l2clutwr::W](l2clutwr::W) writer structure"]
impl crate::Writable for L2CLUTWR {}
#[doc = "Layerx CLUT Write Register"]
pub mod l2clutwr;
