#[doc = "Register `ONEMCU_CTI_Lock_Status_Register` reader"]
pub type R = crate::R<OnemcuCtiLockStatusRegisterSpec>;
#[doc = "Register `ONEMCU_CTI_Lock_Status_Register` writer"]
pub type W = crate::W<OnemcuCtiLockStatusRegisterSpec>;
#[doc = "Field `ONEMCU_CTI_Lock_Status_Register` reader - "]
pub type OnemcuCtiLockStatusRegisterR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_Lock_Status_Register` writer - "]
pub type OnemcuCtiLockStatusRegisterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_lock_status_register(&self) -> OnemcuCtiLockStatusRegisterR {
        OnemcuCtiLockStatusRegisterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_lock_status_register(
        &mut self,
    ) -> OnemcuCtiLockStatusRegisterW<OnemcuCtiLockStatusRegisterSpec> {
        OnemcuCtiLockStatusRegisterW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_Lock_Status_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_lock_status_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_lock_status_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiLockStatusRegisterSpec;
impl crate::RegisterSpec for OnemcuCtiLockStatusRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_lock_status_register::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiLockStatusRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_lock_status_register::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiLockStatusRegisterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_Lock_Status_Register to value 0"]
impl crate::Resettable for OnemcuCtiLockStatusRegisterSpec {
    const RESET_VALUE: u32 = 0;
}
