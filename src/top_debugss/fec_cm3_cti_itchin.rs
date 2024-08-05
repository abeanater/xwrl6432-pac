#[doc = "Register `FEC_CM3_CTI_ITCHIN` reader"]
pub type R = crate::R<FecCm3CtiItchinSpec>;
#[doc = "Register `FEC_CM3_CTI_ITCHIN` writer"]
pub type W = crate::W<FecCm3CtiItchinSpec>;
#[doc = "Field `FEC_CM3_CTI_ITCHIN` reader - "]
pub type FecCm3CtiItchinR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_ITCHIN` writer - "]
pub type FecCm3CtiItchinW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_itchin(&self) -> FecCm3CtiItchinR {
        FecCm3CtiItchinR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_itchin(&mut self) -> FecCm3CtiItchinW<FecCm3CtiItchinSpec> {
        FecCm3CtiItchinW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_ITCHIN\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itchin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itchin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiItchinSpec;
impl crate::RegisterSpec for FecCm3CtiItchinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_itchin::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiItchinSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_itchin::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiItchinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_ITCHIN to value 0"]
impl crate::Resettable for FecCm3CtiItchinSpec {
    const RESET_VALUE: u32 = 0;
}
