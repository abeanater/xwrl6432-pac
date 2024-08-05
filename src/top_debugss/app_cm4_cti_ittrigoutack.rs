#[doc = "Register `APP_CM4_CTI_ITTRIGOUTACK` reader"]
pub type R = crate::R<AppCm4CtiIttrigoutackSpec>;
#[doc = "Register `APP_CM4_CTI_ITTRIGOUTACK` writer"]
pub type W = crate::W<AppCm4CtiIttrigoutackSpec>;
#[doc = "Field `APP_CM4_CTI_ITTRIGOUTACK` reader - "]
pub type AppCm4CtiIttrigoutackR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_ITTRIGOUTACK` writer - "]
pub type AppCm4CtiIttrigoutackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_ittrigoutack(&self) -> AppCm4CtiIttrigoutackR {
        AppCm4CtiIttrigoutackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_ittrigoutack(
        &mut self,
    ) -> AppCm4CtiIttrigoutackW<AppCm4CtiIttrigoutackSpec> {
        AppCm4CtiIttrigoutackW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_ITTRIGOUTACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_ittrigoutack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_ittrigoutack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiIttrigoutackSpec;
impl crate::RegisterSpec for AppCm4CtiIttrigoutackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_ittrigoutack::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiIttrigoutackSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_ittrigoutack::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiIttrigoutackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_ITTRIGOUTACK to value 0"]
impl crate::Resettable for AppCm4CtiIttrigoutackSpec {
    const RESET_VALUE: u32 = 0;
}
