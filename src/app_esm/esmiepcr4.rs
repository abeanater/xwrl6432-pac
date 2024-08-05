#[doc = "Register `ESMIEPCR4` reader"]
pub type R = crate::R<Esmiepcr4Spec>;
#[doc = "Register `ESMIEPCR4` writer"]
pub type W = crate::W<Esmiepcr4Spec>;
#[doc = "Field `IEPCLR` reader - 31:0\\]
Disable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding set bit in the ESMIEPSR4 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Disables failure influence on ERROR pin and clears the corresponding set bit in the ESMIEPSR4 register."]
pub type IepclrR = crate::FieldReader<u32>;
#[doc = "Field `IEPCLR` writer - 31:0\\]
Disable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding set bit in the ESMIEPSR4 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Disables failure influence on ERROR pin and clears the corresponding set bit in the ESMIEPSR4 register."]
pub type IepclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Disable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding set bit in the ESMIEPSR4 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Disables failure influence on ERROR pin and clears the corresponding set bit in the ESMIEPSR4 register."]
    #[inline(always)]
    pub fn iepclr(&self) -> IepclrR {
        IepclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Disable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding set bit in the ESMIEPSR4 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Disables failure influence on ERROR pin and clears the corresponding set bit in the ESMIEPSR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn iepclr(&mut self) -> IepclrW<Esmiepcr4Spec> {
        IepclrW::new(self, 0)
    }
}
#[doc = "ESM Disable ERROR Pin Action/Response Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmiepcr4Spec;
impl crate::RegisterSpec for Esmiepcr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmiepcr4::R`](R) reader structure"]
impl crate::Readable for Esmiepcr4Spec {}
#[doc = "`write(|w| ..)` method takes [`esmiepcr4::W`](W) writer structure"]
impl crate::Writable for Esmiepcr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMIEPCR4 to value 0"]
impl crate::Resettable for Esmiepcr4Spec {
    const RESET_VALUE: u32 = 0;
}
