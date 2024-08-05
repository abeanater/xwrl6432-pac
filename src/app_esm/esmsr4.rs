#[doc = "Register `ESMSR4` reader"]
pub type R = crate::R<Esmsr4Spec>;
#[doc = "Register `ESMSR4` writer"]
pub type W = crate::W<Esmsr4Spec>;
#[doc = "Field `ESF` reader - 31:0\\]
Error Status Flag. Provides status information on a pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred; no interrupt is pending. Write: Leaves the bit unchanged. 1 Read: Error occurred; interrupt is pending. Write: Clears the bit. Note: After nRST, if one of these flags are set and the corresponding interrupt are enabled, the interrupt service routine will be called."]
pub type EsfR = crate::FieldReader<u32>;
#[doc = "Field `ESF` writer - 31:0\\]
Error Status Flag. Provides status information on a pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred; no interrupt is pending. Write: Leaves the bit unchanged. 1 Read: Error occurred; interrupt is pending. Write: Clears the bit. Note: After nRST, if one of these flags are set and the corresponding interrupt are enabled, the interrupt service routine will be called."]
pub type EsfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Error Status Flag. Provides status information on a pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred; no interrupt is pending. Write: Leaves the bit unchanged. 1 Read: Error occurred; interrupt is pending. Write: Clears the bit. Note: After nRST, if one of these flags are set and the corresponding interrupt are enabled, the interrupt service routine will be called."]
    #[inline(always)]
    pub fn esf(&self) -> EsfR {
        EsfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Error Status Flag. Provides status information on a pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred; no interrupt is pending. Write: Leaves the bit unchanged. 1 Read: Error occurred; interrupt is pending. Write: Clears the bit. Note: After nRST, if one of these flags are set and the corresponding interrupt are enabled, the interrupt service routine will be called."]
    #[inline(always)]
    #[must_use]
    pub fn esf(&mut self) -> EsfW<Esmsr4Spec> {
        EsfW::new(self, 0)
    }
}
#[doc = "ESM Status Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmsr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmsr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmsr4Spec;
impl crate::RegisterSpec for Esmsr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmsr4::R`](R) reader structure"]
impl crate::Readable for Esmsr4Spec {}
#[doc = "`write(|w| ..)` method takes [`esmsr4::W`](W) writer structure"]
impl crate::Writable for Esmsr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMSR4 to value 0"]
impl crate::Resettable for Esmsr4Spec {
    const RESET_VALUE: u32 = 0;
}
