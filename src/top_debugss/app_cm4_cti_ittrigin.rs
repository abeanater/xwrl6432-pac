#[doc = "Register `APP_CM4_CTI_ITTRIGIN` reader"]
pub type R = crate::R<AppCm4CtiIttriginSpec>;
#[doc = "Register `APP_CM4_CTI_ITTRIGIN` writer"]
pub type W = crate::W<AppCm4CtiIttriginSpec>;
#[doc = "Field `APP_CM4_CTI_ITTRIGIN` reader - "]
pub type AppCm4CtiIttriginR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_ITTRIGIN` writer - "]
pub type AppCm4CtiIttriginW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_ittrigin(&self) -> AppCm4CtiIttriginR {
        AppCm4CtiIttriginR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_ittrigin(&mut self) -> AppCm4CtiIttriginW<AppCm4CtiIttriginSpec> {
        AppCm4CtiIttriginW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_ITTRIGIN\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_ittrigin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_ittrigin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiIttriginSpec;
impl crate::RegisterSpec for AppCm4CtiIttriginSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_ittrigin::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiIttriginSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_ittrigin::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiIttriginSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_ITTRIGIN to value 0"]
impl crate::Resettable for AppCm4CtiIttriginSpec {
    const RESET_VALUE: u32 = 0;
}
