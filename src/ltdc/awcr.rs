#[doc = "Reader of register AWCR"]
pub type R = crate::R<u32, super::AWCR>;
#[doc = "Writer for register AWCR"]
pub type W = crate::W<u32, super::AWCR>;
#[doc = "Register AWCR `reset()`'s with value 0"]
impl crate::ResetValue for super::AWCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AAV`"]
pub type AAV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AAV`"]
pub struct AAV_W<'a> {
    w: &'a mut W,
}
impl<'a> AAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AAH`"]
pub type AAH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AAH`"]
pub struct AAH_W<'a> {
    w: &'a mut W,
}
impl<'a> AAH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25 - AAV"]
    #[inline(always)]
    pub fn aav(&self) -> AAV_R {
        AAV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - AAV"]
    #[inline(always)]
    pub fn aav(&mut self) -> AAV_W {
        AAV_W { w: self }
    }
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn aah(&mut self) -> AAH_W {
        AAH_W { w: self }
    }
}
