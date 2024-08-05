#[doc = "Register `FEC_CM3_CTI_Authentication_Status` reader"]
pub type R = crate::R<FecCm3CtiAuthenticationStatusSpec>;
#[doc = "Register `FEC_CM3_CTI_Authentication_Status` writer"]
pub type W = crate::W<FecCm3CtiAuthenticationStatusSpec>;
#[doc = "Field `FEC_CM3_CTI_Authentication_Status` reader - "]
pub type FecCm3CtiAuthenticationStatusR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_Authentication_Status` writer - "]
pub type FecCm3CtiAuthenticationStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_authentication_status(&self) -> FecCm3CtiAuthenticationStatusR {
        FecCm3CtiAuthenticationStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_authentication_status(
        &mut self,
    ) -> FecCm3CtiAuthenticationStatusW<FecCm3CtiAuthenticationStatusSpec> {
        FecCm3CtiAuthenticationStatusW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_Authentication_Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_authentication_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_authentication_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiAuthenticationStatusSpec;
impl crate::RegisterSpec for FecCm3CtiAuthenticationStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_authentication_status::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiAuthenticationStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_authentication_status::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiAuthenticationStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_Authentication_Status to value 0"]
impl crate::Resettable for FecCm3CtiAuthenticationStatusSpec {
    const RESET_VALUE: u32 = 0;
}
