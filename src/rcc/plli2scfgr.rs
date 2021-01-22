#[doc = "Reader of register PLLI2SCFGR"]
pub type R = crate::R<u32, super::PLLI2SCFGR>;
#[doc = "Writer for register PLLI2SCFGR"]
pub type W = crate::W<u32, super::PLLI2SCFGR>;
#[doc = "Register PLLI2SCFGR `reset()`'s with value 0x2000_3000"]
impl crate::ResetValue for super::PLLI2SCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_3000
    }
}
#[doc = "Reader of field `PLLI2SRx`"]
pub type PLLI2SRX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLI2SRx`"]
pub struct PLLI2SRX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `PLLI2SNx`"]
pub type PLLI2SNX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PLLI2SNx`"]
pub struct PLLI2SNX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SNX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | (((value as u32) & 0x01ff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2srx(&self) -> PLLI2SRX_R {
        PLLI2SRX_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2snx(&self) -> PLLI2SNX_R {
        PLLI2SNX_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2srx(&mut self) -> PLLI2SRX_W {
        PLLI2SRX_W { w: self }
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2snx(&mut self) -> PLLI2SNX_W {
        PLLI2SNX_W { w: self }
    }
}
