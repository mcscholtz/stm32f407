#[doc = "Reader of register AHB3LPENR"]
pub type R = crate::R<u32, super::AHB3LPENR>;
#[doc = "Writer for register AHB3LPENR"]
pub type W = crate::W<u32, super::AHB3LPENR>;
#[doc = "Register AHB3LPENR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::AHB3LPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `FSMCLPEN`"]
pub type FSMCLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSMCLPEN`"]
pub struct FSMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCLPEN_W<'a> {
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
    #[doc = "Bit 0 - Flexible static memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&self) -> FSMCLPEN_R {
        FSMCLPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&mut self) -> FSMCLPEN_W {
        FSMCLPEN_W { w: self }
    }
}
