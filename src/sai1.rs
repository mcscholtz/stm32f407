#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - SAI AConfiguration register 1"]
    pub sai_acr1: SAI_ACR1,
    #[doc = "0x08 - SAI AConfiguration register 2"]
    pub sai_acr2: SAI_ACR2,
    #[doc = "0x0c - SAI AFrame configuration register"]
    pub sai_afrcr: SAI_AFRCR,
    #[doc = "0x10 - SAI ASlot register"]
    pub sai_aslotr: SAI_ASLOTR,
    #[doc = "0x14 - SAI AInterrupt mask register2"]
    pub sai_aim: SAI_AIM,
    #[doc = "0x18 - SAI AStatus register"]
    pub sai_asr: SAI_ASR,
    #[doc = "0x1c - SAI AClear flag register"]
    pub sai_aclrfr: SAI_ACLRFR,
    #[doc = "0x20 - SAI AData register"]
    pub sai_adr: SAI_ADR,
    #[doc = "0x24 - SAI BConfiguration register 1"]
    pub sai_bcr1: SAI_BCR1,
    #[doc = "0x28 - SAI BConfiguration register 2"]
    pub sai_bcr2: SAI_BCR2,
    #[doc = "0x2c - SAI BFrame configuration register"]
    pub sai_bfrcr: SAI_BFRCR,
    #[doc = "0x30 - SAI BSlot register"]
    pub sai_bslotr: SAI_BSLOTR,
    #[doc = "0x34 - SAI BInterrupt mask register2"]
    pub sai_bim: SAI_BIM,
    #[doc = "0x38 - SAI BStatus register"]
    pub sai_bsr: SAI_BSR,
    #[doc = "0x3c - SAI BClear flag register"]
    pub sai_bclrfr: SAI_BCLRFR,
    #[doc = "0x40 - SAI BData register"]
    pub sai_bdr: SAI_BDR,
}
#[doc = "SAI AConfiguration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_acr1](sai_acr1) module"]
pub type SAI_ACR1 = crate::Reg<u32, _SAI_ACR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ACR1;
#[doc = "`read()` method returns [sai_acr1::R](sai_acr1::R) reader structure"]
impl crate::Readable for SAI_ACR1 {}
#[doc = "`write(|w| ..)` method takes [sai_acr1::W](sai_acr1::W) writer structure"]
impl crate::Writable for SAI_ACR1 {}
#[doc = "SAI AConfiguration register 1"]
pub mod sai_acr1;
#[doc = "SAI BConfiguration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bcr1](sai_bcr1) module"]
pub type SAI_BCR1 = crate::Reg<u32, _SAI_BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BCR1;
#[doc = "`read()` method returns [sai_bcr1::R](sai_bcr1::R) reader structure"]
impl crate::Readable for SAI_BCR1 {}
#[doc = "`write(|w| ..)` method takes [sai_bcr1::W](sai_bcr1::W) writer structure"]
impl crate::Writable for SAI_BCR1 {}
#[doc = "SAI BConfiguration register 1"]
pub mod sai_bcr1;
#[doc = "SAI AConfiguration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_acr2](sai_acr2) module"]
pub type SAI_ACR2 = crate::Reg<u32, _SAI_ACR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ACR2;
#[doc = "`read()` method returns [sai_acr2::R](sai_acr2::R) reader structure"]
impl crate::Readable for SAI_ACR2 {}
#[doc = "`write(|w| ..)` method takes [sai_acr2::W](sai_acr2::W) writer structure"]
impl crate::Writable for SAI_ACR2 {}
#[doc = "SAI AConfiguration register 2"]
pub mod sai_acr2;
#[doc = "SAI BConfiguration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bcr2](sai_bcr2) module"]
pub type SAI_BCR2 = crate::Reg<u32, _SAI_BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BCR2;
#[doc = "`read()` method returns [sai_bcr2::R](sai_bcr2::R) reader structure"]
impl crate::Readable for SAI_BCR2 {}
#[doc = "`write(|w| ..)` method takes [sai_bcr2::W](sai_bcr2::W) writer structure"]
impl crate::Writable for SAI_BCR2 {}
#[doc = "SAI BConfiguration register 2"]
pub mod sai_bcr2;
#[doc = "SAI AFrame configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_afrcr](sai_afrcr) module"]
pub type SAI_AFRCR = crate::Reg<u32, _SAI_AFRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_AFRCR;
#[doc = "`read()` method returns [sai_afrcr::R](sai_afrcr::R) reader structure"]
impl crate::Readable for SAI_AFRCR {}
#[doc = "`write(|w| ..)` method takes [sai_afrcr::W](sai_afrcr::W) writer structure"]
impl crate::Writable for SAI_AFRCR {}
#[doc = "SAI AFrame configuration register"]
pub mod sai_afrcr;
#[doc = "SAI BFrame configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bfrcr](sai_bfrcr) module"]
pub type SAI_BFRCR = crate::Reg<u32, _SAI_BFRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BFRCR;
#[doc = "`read()` method returns [sai_bfrcr::R](sai_bfrcr::R) reader structure"]
impl crate::Readable for SAI_BFRCR {}
#[doc = "`write(|w| ..)` method takes [sai_bfrcr::W](sai_bfrcr::W) writer structure"]
impl crate::Writable for SAI_BFRCR {}
#[doc = "SAI BFrame configuration register"]
pub mod sai_bfrcr;
#[doc = "SAI ASlot register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aslotr](sai_aslotr) module"]
pub type SAI_ASLOTR = crate::Reg<u32, _SAI_ASLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ASLOTR;
#[doc = "`read()` method returns [sai_aslotr::R](sai_aslotr::R) reader structure"]
impl crate::Readable for SAI_ASLOTR {}
#[doc = "`write(|w| ..)` method takes [sai_aslotr::W](sai_aslotr::W) writer structure"]
impl crate::Writable for SAI_ASLOTR {}
#[doc = "SAI ASlot register"]
pub mod sai_aslotr;
#[doc = "SAI BSlot register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bslotr](sai_bslotr) module"]
pub type SAI_BSLOTR = crate::Reg<u32, _SAI_BSLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BSLOTR;
#[doc = "`read()` method returns [sai_bslotr::R](sai_bslotr::R) reader structure"]
impl crate::Readable for SAI_BSLOTR {}
#[doc = "`write(|w| ..)` method takes [sai_bslotr::W](sai_bslotr::W) writer structure"]
impl crate::Writable for SAI_BSLOTR {}
#[doc = "SAI BSlot register"]
pub mod sai_bslotr;
#[doc = "SAI AInterrupt mask register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aim](sai_aim) module"]
pub type SAI_AIM = crate::Reg<u32, _SAI_AIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_AIM;
#[doc = "`read()` method returns [sai_aim::R](sai_aim::R) reader structure"]
impl crate::Readable for SAI_AIM {}
#[doc = "`write(|w| ..)` method takes [sai_aim::W](sai_aim::W) writer structure"]
impl crate::Writable for SAI_AIM {}
#[doc = "SAI AInterrupt mask register2"]
pub mod sai_aim;
#[doc = "SAI BInterrupt mask register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bim](sai_bim) module"]
pub type SAI_BIM = crate::Reg<u32, _SAI_BIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BIM;
#[doc = "`read()` method returns [sai_bim::R](sai_bim::R) reader structure"]
impl crate::Readable for SAI_BIM {}
#[doc = "`write(|w| ..)` method takes [sai_bim::W](sai_bim::W) writer structure"]
impl crate::Writable for SAI_BIM {}
#[doc = "SAI BInterrupt mask register2"]
pub mod sai_bim;
#[doc = "SAI AStatus register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_asr](sai_asr) module"]
pub type SAI_ASR = crate::Reg<u32, _SAI_ASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ASR;
#[doc = "`read()` method returns [sai_asr::R](sai_asr::R) reader structure"]
impl crate::Readable for SAI_ASR {}
#[doc = "SAI AStatus register"]
pub mod sai_asr;
#[doc = "SAI BStatus register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bsr](sai_bsr) module"]
pub type SAI_BSR = crate::Reg<u32, _SAI_BSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BSR;
#[doc = "`read()` method returns [sai_bsr::R](sai_bsr::R) reader structure"]
impl crate::Readable for SAI_BSR {}
#[doc = "SAI BStatus register"]
pub mod sai_bsr;
#[doc = "SAI AClear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aclrfr](sai_aclrfr) module"]
pub type SAI_ACLRFR = crate::Reg<u32, _SAI_ACLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ACLRFR;
#[doc = "`read()` method returns [sai_aclrfr::R](sai_aclrfr::R) reader structure"]
impl crate::Readable for SAI_ACLRFR {}
#[doc = "`write(|w| ..)` method takes [sai_aclrfr::W](sai_aclrfr::W) writer structure"]
impl crate::Writable for SAI_ACLRFR {}
#[doc = "SAI AClear flag register"]
pub mod sai_aclrfr;
#[doc = "SAI BClear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bclrfr](sai_bclrfr) module"]
pub type SAI_BCLRFR = crate::Reg<u32, _SAI_BCLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BCLRFR;
#[doc = "`read()` method returns [sai_bclrfr::R](sai_bclrfr::R) reader structure"]
impl crate::Readable for SAI_BCLRFR {}
#[doc = "`write(|w| ..)` method takes [sai_bclrfr::W](sai_bclrfr::W) writer structure"]
impl crate::Writable for SAI_BCLRFR {}
#[doc = "SAI BClear flag register"]
pub mod sai_bclrfr;
#[doc = "SAI AData register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_adr](sai_adr) module"]
pub type SAI_ADR = crate::Reg<u32, _SAI_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ADR;
#[doc = "`read()` method returns [sai_adr::R](sai_adr::R) reader structure"]
impl crate::Readable for SAI_ADR {}
#[doc = "`write(|w| ..)` method takes [sai_adr::W](sai_adr::W) writer structure"]
impl crate::Writable for SAI_ADR {}
#[doc = "SAI AData register"]
pub mod sai_adr;
#[doc = "SAI BData register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bdr](sai_bdr) module"]
pub type SAI_BDR = crate::Reg<u32, _SAI_BDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BDR;
#[doc = "`read()` method returns [sai_bdr::R](sai_bdr::R) reader structure"]
impl crate::Readable for SAI_BDR {}
#[doc = "`write(|w| ..)` method takes [sai_bdr::W](sai_bdr::W) writer structure"]
impl crate::Writable for SAI_BDR {}
#[doc = "SAI BData register"]
pub mod sai_bdr;
