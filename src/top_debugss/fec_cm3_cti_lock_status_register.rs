#[doc = "Register `FEC_CM3_CTI_Lock_Status_Register` reader"]
pub type R = crate::R<FecCm3CtiLockStatusRegisterSpec>;
#[doc = "Register `FEC_CM3_CTI_Lock_Status_Register` writer"]
pub type W = crate::W<FecCm3CtiLockStatusRegisterSpec>;
#[doc = "Field `FEC_CM3_CTI_Lock_Status_Register` reader - "]
pub type FecCm3CtiLockStatusRegisterR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_Lock_Status_Register` writer - "]
pub type FecCm3CtiLockStatusRegisterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_lock_status_register(&self) -> FecCm3CtiLockStatusRegisterR {
        FecCm3CtiLockStatusRegisterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_lock_status_register(
        &mut self,
    ) -> FecCm3CtiLockStatusRegisterW<FecCm3CtiLockStatusRegisterSpec> {
        FecCm3CtiLockStatusRegisterW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_Lock_Status_Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_lock_status_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_lock_status_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiLockStatusRegisterSpec;
impl crate::RegisterSpec for FecCm3CtiLockStatusRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_lock_status_register::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiLockStatusRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_lock_status_register::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiLockStatusRegisterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_Lock_Status_Register to value 0"]
impl crate::Resettable for FecCm3CtiLockStatusRegisterSpec {
    const RESET_VALUE: u32 = 0;
}
