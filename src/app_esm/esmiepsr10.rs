#[doc = "Register `ESMIEPSR10` reader"]
pub type R = crate::R<Esmiepsr10Spec>;
#[doc = "Register `ESMIEPSR10` writer"]
pub type W = crate::W<Esmiepsr10Spec>;
#[doc = "Field `IEPSET` reader - 31:0\\]
Enable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding clear bit in the ESMIEPCR10 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Enables failure influence on ERROR pin and sets the corresponding clear bit in the ESMIEPCR10 register."]
pub type IepsetR = crate::FieldReader<u32>;
#[doc = "Field `IEPSET` writer - 31:0\\]
Enable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding clear bit in the ESMIEPCR10 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Enables failure influence on ERROR pin and sets the corresponding clear bit in the ESMIEPCR10 register."]
pub type IepsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Enable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding clear bit in the ESMIEPCR10 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Enables failure influence on ERROR pin and sets the corresponding clear bit in the ESMIEPCR10 register."]
    #[inline(always)]
    pub fn iepset(&self) -> IepsetR {
        IepsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Enable ERROR Pin Action/Response on Group 1. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Failure on channel x has no influence on ERROR pin. Write: Leaves the bit and the corresponding clear bit in the ESMIEPCR10 register unchanged. 1 Read: Failure on channel x has influence on ERROR pin. Write: Enables failure influence on ERROR pin and sets the corresponding clear bit in the ESMIEPCR10 register."]
    #[inline(always)]
    #[must_use]
    pub fn iepset(&mut self) -> IepsetW<Esmiepsr10Spec> {
        IepsetW::new(self, 0)
    }
}
#[doc = "ESM Enable ERROR Pin Action/Response Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiepsr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiepsr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmiepsr10Spec;
impl crate::RegisterSpec for Esmiepsr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmiepsr10::R`](R) reader structure"]
impl crate::Readable for Esmiepsr10Spec {}
#[doc = "`write(|w| ..)` method takes [`esmiepsr10::W`](W) writer structure"]
impl crate::Writable for Esmiepsr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMIEPSR10 to value 0"]
impl crate::Resettable for Esmiepsr10Spec {
    const RESET_VALUE: u32 = 0;
}
