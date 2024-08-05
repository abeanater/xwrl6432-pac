#[doc = "Register `APP_CM4_CTI_Lock_Access_Register` reader"]
pub type R = crate::R<AppCm4CtiLockAccessRegisterSpec>;
#[doc = "Register `APP_CM4_CTI_Lock_Access_Register` writer"]
pub type W = crate::W<AppCm4CtiLockAccessRegisterSpec>;
#[doc = "Field `APP_CM4_CTI_Lock_Access_Register` reader - "]
pub type AppCm4CtiLockAccessRegisterR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_Lock_Access_Register` writer - "]
pub type AppCm4CtiLockAccessRegisterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_lock_access_register(&self) -> AppCm4CtiLockAccessRegisterR {
        AppCm4CtiLockAccessRegisterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_lock_access_register(
        &mut self,
    ) -> AppCm4CtiLockAccessRegisterW<AppCm4CtiLockAccessRegisterSpec> {
        AppCm4CtiLockAccessRegisterW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_Lock_Access_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_lock_access_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_lock_access_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiLockAccessRegisterSpec;
impl crate::RegisterSpec for AppCm4CtiLockAccessRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_lock_access_register::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiLockAccessRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_lock_access_register::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiLockAccessRegisterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_Lock_Access_Register to value 0"]
impl crate::Resettable for AppCm4CtiLockAccessRegisterSpec {
    const RESET_VALUE: u32 = 0;
}
