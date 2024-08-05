#[doc = "Register `ESMSR3` reader"]
pub type R = crate::R<Esmsr3Spec>;
#[doc = "Register `ESMSR3` writer"]
pub type W = crate::W<Esmsr3Spec>;
#[doc = "Field `ESF` reader - 31:0\\]
Error Status Flag. Provides status information on a pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred. Write: Leaves the bit unchanged. 1 Read: Error occurred. Write: Clears the bit."]
pub type EsfR = crate::FieldReader<u32>;
#[doc = "Field `ESF` writer - 31:0\\]
Error Status Flag. Provides status information on a pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred. Write: Leaves the bit unchanged. 1 Read: Error occurred. Write: Clears the bit."]
pub type EsfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Error Status Flag. Provides status information on a pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred. Write: Leaves the bit unchanged. 1 Read: Error occurred. Write: Clears the bit."]
    #[inline(always)]
    pub fn esf(&self) -> EsfR {
        EsfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Error Status Flag. Provides status information on a pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred. Write: Leaves the bit unchanged. 1 Read: Error occurred. Write: Clears the bit."]
    #[inline(always)]
    #[must_use]
    pub fn esf(&mut self) -> EsfW<Esmsr3Spec> {
        EsfW::new(self, 0)
    }
}
#[doc = "ESM Status Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`esmsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmsr3Spec;
impl crate::RegisterSpec for Esmsr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmsr3::R`](R) reader structure"]
impl crate::Readable for Esmsr3Spec {}
#[doc = "`write(|w| ..)` method takes [`esmsr3::W`](W) writer structure"]
impl crate::Writable for Esmsr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMSR3 to value 0"]
impl crate::Resettable for Esmsr3Spec {
    const RESET_VALUE: u32 = 0;
}
