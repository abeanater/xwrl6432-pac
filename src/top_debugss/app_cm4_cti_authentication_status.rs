#[doc = "Register `APP_CM4_CTI_Authentication_Status` reader"]
pub type R = crate::R<AppCm4CtiAuthenticationStatusSpec>;
#[doc = "Register `APP_CM4_CTI_Authentication_Status` writer"]
pub type W = crate::W<AppCm4CtiAuthenticationStatusSpec>;
#[doc = "Field `APP_CM4_CTI_Authentication_Status` reader - "]
pub type AppCm4CtiAuthenticationStatusR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_Authentication_Status` writer - "]
pub type AppCm4CtiAuthenticationStatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_authentication_status(&self) -> AppCm4CtiAuthenticationStatusR {
        AppCm4CtiAuthenticationStatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_authentication_status(
        &mut self,
    ) -> AppCm4CtiAuthenticationStatusW<AppCm4CtiAuthenticationStatusSpec> {
        AppCm4CtiAuthenticationStatusW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_Authentication_Status\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_authentication_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_authentication_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiAuthenticationStatusSpec;
impl crate::RegisterSpec for AppCm4CtiAuthenticationStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_authentication_status::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiAuthenticationStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_authentication_status::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiAuthenticationStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_Authentication_Status to value 0"]
impl crate::Resettable for AppCm4CtiAuthenticationStatusSpec {
    const RESET_VALUE: u32 = 0;
}
