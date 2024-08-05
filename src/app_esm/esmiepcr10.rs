#[doc = "Register `ESMIEPCR10` reader"]
pub type R = crate::R<Esmiepcr10Spec>;
#[doc = "Register `ESMIEPCR10` writer"]
pub type W = crate::W<Esmiepcr10Spec>;
#[doc = "Field `IEPCLR` reader - 31:0\\]
Disable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding set bit in the ESMIEPSR10 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Disables failure influence on ERROR pin and clears the corresponding set bit in the ESMIEPSR10 register."]
pub type IepclrR = crate::FieldReader<u32>;
#[doc = "Field `IEPCLR` writer - 31:0\\]
Disable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding set bit in the ESMIEPSR10 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Disables failure influence on ERROR pin and clears the corresponding set bit in the ESMIEPSR10 register."]
pub type IepclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Disable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding set bit in the ESMIEPSR10 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Disables failure influence on ERROR pin and clears the corresponding set bit in the ESMIEPSR10 register."]
    #[inline(always)]
    pub fn iepclr(&self) -> IepclrR {
        IepclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Disable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding set bit in the ESMIEPSR10 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Disables failure influence on ERROR pin and clears the corresponding set bit in the ESMIEPSR10 register."]
    #[inline(always)]
    #[must_use]
    pub fn iepclr(&mut self) -> IepclrW<Esmiepcr10Spec> {
        IepclrW::new(self, 0)
    }
}
#[doc = "ESM Disable ERROR Pin Action/Response Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepcr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepcr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmiepcr10Spec;
impl crate::RegisterSpec for Esmiepcr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmiepcr10::R`](R) reader structure"]
impl crate::Readable for Esmiepcr10Spec {}
#[doc = "`write(|w| ..)` method takes [`esmiepcr10::W`](W) writer structure"]
impl crate::Writable for Esmiepcr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMIEPCR10 to value 0"]
impl crate::Resettable for Esmiepcr10Spec {
    const RESET_VALUE: u32 = 0;
}
