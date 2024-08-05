#[doc = "Register `ONEMCU_CTI_Authentication_Status` reader"]
pub type R = crate::R<OnemcuCtiAuthenticationStatusSpec>;
#[doc = "Register `ONEMCU_CTI_Authentication_Status` writer"]
pub type W = crate::W<OnemcuCtiAuthenticationStatusSpec>;
#[doc = "Field `ONEMCU_CTI_Authentication_Status` reader - "]
pub type OnemcuCtiAuthenticationStatusR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_Authentication_Status` writer - "]
pub type OnemcuCtiAuthenticationStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_authentication_status(&self) -> OnemcuCtiAuthenticationStatusR {
        OnemcuCtiAuthenticationStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_authentication_status(
        &mut self,
    ) -> OnemcuCtiAuthenticationStatusW<OnemcuCtiAuthenticationStatusSpec> {
        OnemcuCtiAuthenticationStatusW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_Authentication_Status\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_authentication_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_authentication_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiAuthenticationStatusSpec;
impl crate::RegisterSpec for OnemcuCtiAuthenticationStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_authentication_status::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiAuthenticationStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_authentication_status::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiAuthenticationStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_Authentication_Status to value 0"]
impl crate::Resettable for OnemcuCtiAuthenticationStatusSpec {
    const RESET_VALUE: u32 = 0;
}
