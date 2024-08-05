#[doc = "Register `ESMIECR4` reader"]
pub type R = crate::R<Esmiecr4Spec>;
#[doc = "Register `ESMIECR4` writer"]
pub type W = crate::W<Esmiecr4Spec>;
#[doc = "Field `INTENCLR` reader - 31:0\\]
Clear Interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding set bit in the ESMIESR4 register unchanged. 1 Read: Interrupt is enabled. Write: Disables interrupt and clears the corresponding set bit in the ESMIESR4 register."]
pub type IntenclrR = crate::FieldReader<u32>;
#[doc = "Field `INTENCLR` writer - 31:0\\]
Clear Interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding set bit in the ESMIESR4 register unchanged. 1 Read: Interrupt is enabled. Write: Disables interrupt and clears the corresponding set bit in the ESMIESR4 register."]
pub type IntenclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Clear Interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding set bit in the ESMIESR4 register unchanged. 1 Read: Interrupt is enabled. Write: Disables interrupt and clears the corresponding set bit in the ESMIESR4 register."]
    #[inline(always)]
    pub fn intenclr(&self) -> IntenclrR {
        IntenclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Clear Interrupt Enable Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt is disabled. Write: Leaves the bit and the corresponding set bit in the ESMIESR4 register unchanged. 1 Read: Interrupt is enabled. Write: Disables interrupt and clears the corresponding set bit in the ESMIESR4 register."]
    #[inline(always)]
    #[must_use]
    pub fn intenclr(&mut self) -> IntenclrW<Esmiecr4Spec> {
        IntenclrW::new(self, 0)
    }
}
#[doc = "ESM Interrupt Enable Clear/Status Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`esmiecr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmiecr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmiecr4Spec;
impl crate::RegisterSpec for Esmiecr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmiecr4::R`](R) reader structure"]
impl crate::Readable for Esmiecr4Spec {}
#[doc = "`write(|w| ..)` method takes [`esmiecr4::W`](W) writer structure"]
impl crate::Writable for Esmiecr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMIECR4 to value 0"]
impl crate::Resettable for Esmiecr4Spec {
    const RESET_VALUE: u32 = 0;
}
