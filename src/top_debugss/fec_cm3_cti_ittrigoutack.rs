#[doc = "Register `FEC_CM3_CTI_ITTRIGOUTACK` reader"]
pub type R = crate::R<FecCm3CtiIttrigoutackSpec>;
#[doc = "Register `FEC_CM3_CTI_ITTRIGOUTACK` writer"]
pub type W = crate::W<FecCm3CtiIttrigoutackSpec>;
#[doc = "Field `FEC_CM3_CTI_ITTRIGOUTACK` reader - "]
pub type FecCm3CtiIttrigoutackR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_ITTRIGOUTACK` writer - "]
pub type FecCm3CtiIttrigoutackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_ittrigoutack(&self) -> FecCm3CtiIttrigoutackR {
        FecCm3CtiIttrigoutackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_ittrigoutack(
        &mut self,
    ) -> FecCm3CtiIttrigoutackW<FecCm3CtiIttrigoutackSpec> {
        FecCm3CtiIttrigoutackW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_ITTRIGOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_ittrigoutack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_ittrigoutack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiIttrigoutackSpec;
impl crate::RegisterSpec for FecCm3CtiIttrigoutackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_ittrigoutack::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiIttrigoutackSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_ittrigoutack::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiIttrigoutackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_ITTRIGOUTACK to value 0"]
impl crate::Resettable for FecCm3CtiIttrigoutackSpec {
    const RESET_VALUE: u32 = 0;
}
