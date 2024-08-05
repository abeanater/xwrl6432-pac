#[doc = "Register `FEC_CM3_CTI_ITCHOUT` reader"]
pub type R = crate::R<FecCm3CtiItchoutSpec>;
#[doc = "Register `FEC_CM3_CTI_ITCHOUT` writer"]
pub type W = crate::W<FecCm3CtiItchoutSpec>;
#[doc = "Field `FEC_CM3_CTI_ITCHOUT` reader - "]
pub type FecCm3CtiItchoutR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_ITCHOUT` writer - "]
pub type FecCm3CtiItchoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_itchout(&self) -> FecCm3CtiItchoutR {
        FecCm3CtiItchoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_itchout(&mut self) -> FecCm3CtiItchoutW<FecCm3CtiItchoutSpec> {
        FecCm3CtiItchoutW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_ITCHOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itchout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itchout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiItchoutSpec;
impl crate::RegisterSpec for FecCm3CtiItchoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_itchout::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiItchoutSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_itchout::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiItchoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_ITCHOUT to value 0"]
impl crate::Resettable for FecCm3CtiItchoutSpec {
    const RESET_VALUE: u32 = 0;
}
