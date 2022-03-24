#[doc = "Register `VNPCCR` reader"]
pub struct R(crate::R<VNPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VNPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VNPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VNPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VNPCCR` writer"]
pub struct W(crate::W<VNPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VNPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<VNPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VNPCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NPSIZE` reader - Null Packet Size"]
pub struct NPSIZE_R(crate::FieldReader<u16, u16>);
impl NPSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPSIZE` writer - Null Packet Size"]
pub struct NPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> NPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Null Packet Size"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Null Packet Size"]
    #[inline(always)]
    pub fn npsize(&mut self) -> NPSIZE_W {
        NPSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video Null Packet Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vnpccr](index.html) module"]
pub struct VNPCCR_SPEC;
impl crate::RegisterSpec for VNPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vnpccr::R](R) reader structure"]
impl crate::Readable for VNPCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vnpccr::W](W) writer structure"]
impl crate::Writable for VNPCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VNPCCR to value 0"]
impl crate::Resettable for VNPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
