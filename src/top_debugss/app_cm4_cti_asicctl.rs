#[doc = "Register `APP_CM4_CTI_ASICCTL` reader"]
pub type R = crate::R<AppCm4CtiAsicctlSpec>;
#[doc = "Register `APP_CM4_CTI_ASICCTL` writer"]
pub type W = crate::W<AppCm4CtiAsicctlSpec>;
#[doc = "Field `APP_CM4_CTI_ASICCTL` reader - "]
pub type AppCm4CtiAsicctlR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_ASICCTL` writer - "]
pub type AppCm4CtiAsicctlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_asicctl(&self) -> AppCm4CtiAsicctlR {
        AppCm4CtiAsicctlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_asicctl(&mut self) -> AppCm4CtiAsicctlW<AppCm4CtiAsicctlSpec> {
        AppCm4CtiAsicctlW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_ASICCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_asicctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_asicctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiAsicctlSpec;
impl crate::RegisterSpec for AppCm4CtiAsicctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_asicctl::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiAsicctlSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_asicctl::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiAsicctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_ASICCTL to value 0"]
impl crate::Resettable for AppCm4CtiAsicctlSpec {
    const RESET_VALUE: u32 = 0;
}
