#[doc = "Register `APP_CM4_CTI_APPCLEAR` reader"]
pub type R = crate::R<AppCm4CtiAppclearSpec>;
#[doc = "Register `APP_CM4_CTI_APPCLEAR` writer"]
pub type W = crate::W<AppCm4CtiAppclearSpec>;
#[doc = "Field `APP_CM4_CTI_APPCLEAR` reader - "]
pub type AppCm4CtiAppclearR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_APPCLEAR` writer - "]
pub type AppCm4CtiAppclearW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_appclear(&self) -> AppCm4CtiAppclearR {
        AppCm4CtiAppclearR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_appclear(&mut self) -> AppCm4CtiAppclearW<AppCm4CtiAppclearSpec> {
        AppCm4CtiAppclearW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_APPCLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_appclear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_appclear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiAppclearSpec;
impl crate::RegisterSpec for AppCm4CtiAppclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_appclear::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiAppclearSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_appclear::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiAppclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_APPCLEAR to value 0"]
impl crate::Resettable for AppCm4CtiAppclearSpec {
    const RESET_VALUE: u32 = 0;
}