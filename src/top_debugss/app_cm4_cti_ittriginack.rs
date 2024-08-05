#[doc = "Register `APP_CM4_CTI_ITTRIGINACK` reader"]
pub type R = crate::R<AppCm4CtiIttriginackSpec>;
#[doc = "Register `APP_CM4_CTI_ITTRIGINACK` writer"]
pub type W = crate::W<AppCm4CtiIttriginackSpec>;
#[doc = "Field `APP_CM4_CTI_ITTRIGINACK` reader - "]
pub type AppCm4CtiIttriginackR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_ITTRIGINACK` writer - "]
pub type AppCm4CtiIttriginackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_ittriginack(&self) -> AppCm4CtiIttriginackR {
        AppCm4CtiIttriginackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_ittriginack(&mut self) -> AppCm4CtiIttriginackW<AppCm4CtiIttriginackSpec> {
        AppCm4CtiIttriginackW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_ITTRIGINACK\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_ittriginack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_ittriginack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiIttriginackSpec;
impl crate::RegisterSpec for AppCm4CtiIttriginackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_ittriginack::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiIttriginackSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_ittriginack::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiIttriginackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_ITTRIGINACK to value 0"]
impl crate::Resettable for AppCm4CtiIttriginackSpec {
    const RESET_VALUE: u32 = 0;
}
