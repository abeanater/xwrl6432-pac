#[doc = "Register `ESMSSR2` reader"]
pub type R = crate::R<Esmssr2Spec>;
#[doc = "Register `ESMSSR2` writer"]
pub type W = crate::W<Esmssr2Spec>;
#[doc = "Field `ESF` reader - 31:0\\]
Error Status Flag. Shadow register for status information on pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred. Write: Leaves the bit unchanged. 1 Read: Error occurred. Write: Clears the bit. ESMSR2 is not impacted by this action. Note: Errors are stored until they are cleared by the software or at power-on reset (PORRST)."]
pub type EsfR = crate::FieldReader<u32>;
#[doc = "Field `ESF` writer - 31:0\\]
Error Status Flag. Shadow register for status information on pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred. Write: Leaves the bit unchanged. 1 Read: Error occurred. Write: Clears the bit. ESMSR2 is not impacted by this action. Note: Errors are stored until they are cleared by the software or at power-on reset (PORRST)."]
pub type EsfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Error Status Flag. Shadow register for status information on pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred. Write: Leaves the bit unchanged. 1 Read: Error occurred. Write: Clears the bit. ESMSR2 is not impacted by this action. Note: Errors are stored until they are cleared by the software or at power-on reset (PORRST)."]
    #[inline(always)]
    pub fn esf(&self) -> EsfR {
        EsfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Error Status Flag. Shadow register for status information on pending error. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: No error occurred. Write: Leaves the bit unchanged. 1 Read: Error occurred. Write: Clears the bit. ESMSR2 is not impacted by this action. Note: Errors are stored until they are cleared by the software or at power-on reset (PORRST)."]
    #[inline(always)]
    #[must_use]
    pub fn esf(&mut self) -> EsfW<Esmssr2Spec> {
        EsfW::new(self, 0)
    }
}
#[doc = "ESM Status Shadow Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`esmssr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmssr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmssr2Spec;
impl crate::RegisterSpec for Esmssr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmssr2::R`](R) reader structure"]
impl crate::Readable for Esmssr2Spec {}
#[doc = "`write(|w| ..)` method takes [`esmssr2::W`](W) writer structure"]
impl crate::Writable for Esmssr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMSSR2 to value 0"]
impl crate::Resettable for Esmssr2Spec {
    const RESET_VALUE: u32 = 0;
}
