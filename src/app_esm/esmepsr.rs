#[doc = "Register `ESMEPSR` reader"]
pub type R = crate::R<EsmepsrSpec>;
#[doc = "Register `ESMEPSR` writer"]
pub type W = crate::W<EsmepsrSpec>;
#[doc = "Field `EPSF` reader - 0:0\\]
ERROR Pin Status Flag. Provides status information for the ERROR Pin. Read/Write in User and Privileged mode. 0 Read: ERROR Pin is low (active) if any error has occurred. Write: Writes have no effect. 1 Read: ERROR Pin is high if no error has occurred. Write: Writes have no effect. Note: This flag will be set to 1 after PORRST. The value will be unchanged after nRST. The ERROR pin status remains un-changed during after nRST."]
pub type EpsfR = crate::BitReader;
#[doc = "Field `EPSF` writer - 0:0\\]
ERROR Pin Status Flag. Provides status information for the ERROR Pin. Read/Write in User and Privileged mode. 0 Read: ERROR Pin is low (active) if any error has occurred. Write: Writes have no effect. 1 Read: ERROR Pin is high if no error has occurred. Write: Writes have no effect. Note: This flag will be set to 1 after PORRST. The value will be unchanged after nRST. The ERROR pin status remains un-changed during after nRST."]
pub type EpsfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ERROR Pin Status Flag. Provides status information for the ERROR Pin. Read/Write in User and Privileged mode. 0 Read: ERROR Pin is low (active) if any error has occurred. Write: Writes have no effect. 1 Read: ERROR Pin is high if no error has occurred. Write: Writes have no effect. Note: This flag will be set to 1 after PORRST. The value will be unchanged after nRST. The ERROR pin status remains un-changed during after nRST."]
    #[inline(always)]
    pub fn epsf(&self) -> EpsfR {
        EpsfR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ERROR Pin Status Flag. Provides status information for the ERROR Pin. Read/Write in User and Privileged mode. 0 Read: ERROR Pin is low (active) if any error has occurred. Write: Writes have no effect. 1 Read: ERROR Pin is high if no error has occurred. Write: Writes have no effect. Note: This flag will be set to 1 after PORRST. The value will be unchanged after nRST. The ERROR pin status remains un-changed during after nRST."]
    #[inline(always)]
    #[must_use]
    pub fn epsf(&mut self) -> EpsfW<EsmepsrSpec> {
        EpsfW::new(self, 0)
    }
}
#[doc = "ESM ERROR Pin Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esmepsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmepsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsmepsrSpec;
impl crate::RegisterSpec for EsmepsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmepsr::R`](R) reader structure"]
impl crate::Readable for EsmepsrSpec {}
#[doc = "`write(|w| ..)` method takes [`esmepsr::W`](W) writer structure"]
impl crate::Writable for EsmepsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMEPSR to value 0"]
impl crate::Resettable for EsmepsrSpec {
    const RESET_VALUE: u32 = 0;
}
