#[doc = "Register `REG19` reader"]
pub type R = crate::R<Reg19Spec>;
#[doc = "Register `REG19` writer"]
pub type W = crate::W<Reg19Spec>;
#[doc = "Field `GPADC_SAMPLES_FRAME` reader - 15:0\\]
Total number of GPADC samples collected in a frame"]
pub type GpadcSamplesFrameR = crate::FieldReader<u16>;
#[doc = "Field `GPADC_SAMPLES_FRAME` writer - 15:0\\]
Total number of GPADC samples collected in a frame"]
pub type GpadcSamplesFrameW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - 31:16\\]
TI reserved"]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 31:16\\]
TI reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Total number of GPADC samples collected in a frame"]
    #[inline(always)]
    pub fn gpadc_samples_frame(&self) -> GpadcSamplesFrameR {
        GpadcSamplesFrameR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
TI reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Total number of GPADC samples collected in a frame"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_samples_frame(&mut self) -> GpadcSamplesFrameW<Reg19Spec> {
        GpadcSamplesFrameW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Reg19Spec> {
        NuW::new(self, 16)
    }
}
#[doc = "REG19\n\nYou can [`read`](crate::Reg::read) this register and get [`reg19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg19Spec;
impl crate::RegisterSpec for Reg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg19::R`](R) reader structure"]
impl crate::Readable for Reg19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg19::W`](W) writer structure"]
impl crate::Writable for Reg19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG19 to value 0"]
impl crate::Resettable for Reg19Spec {
    const RESET_VALUE: u32 = 0;
}
