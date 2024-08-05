#[doc = "Register `ONEMCU_CTI_Lock_Access_Register` reader"]
pub type R = crate::R<OnemcuCtiLockAccessRegisterSpec>;
#[doc = "Register `ONEMCU_CTI_Lock_Access_Register` writer"]
pub type W = crate::W<OnemcuCtiLockAccessRegisterSpec>;
#[doc = "Field `ONEMCU_CTI_Lock_Access_Register` reader - "]
pub type OnemcuCtiLockAccessRegisterR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_Lock_Access_Register` writer - "]
pub type OnemcuCtiLockAccessRegisterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_lock_access_register(&self) -> OnemcuCtiLockAccessRegisterR {
        OnemcuCtiLockAccessRegisterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_lock_access_register(
        &mut self,
    ) -> OnemcuCtiLockAccessRegisterW<OnemcuCtiLockAccessRegisterSpec> {
        OnemcuCtiLockAccessRegisterW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_Lock_Access_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_lock_access_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_lock_access_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiLockAccessRegisterSpec;
impl crate::RegisterSpec for OnemcuCtiLockAccessRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_lock_access_register::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiLockAccessRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_lock_access_register::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiLockAccessRegisterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_Lock_Access_Register to value 0"]
impl crate::Resettable for OnemcuCtiLockAccessRegisterSpec {
    const RESET_VALUE: u32 = 0;
}
