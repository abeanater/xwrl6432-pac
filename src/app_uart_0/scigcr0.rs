#[doc = "Register `SCIGCR0` reader"]
pub type R = crate::R<Scigcr0Spec>;
#[doc = "Register `SCIGCR0` writer"]
pub type W = crate::W<Scigcr0Spec>;
#[doc = "Field `RESET` reader - 0:0\\]
GIO reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - 0:0\\]
GIO reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
GIO reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
GIO reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<Scigcr0Spec> {
        ResetW::new(self, 0)
    }
}
#[doc = "The SCIGCR0 register defines the module reset\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scigcr0Spec;
impl crate::RegisterSpec for Scigcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scigcr0::R`](R) reader structure"]
impl crate::Readable for Scigcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`scigcr0::W`](W) writer structure"]
impl crate::Writable for Scigcr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIGCR0 to value 0"]
impl crate::Resettable for Scigcr0Spec {
    const RESET_VALUE: u32 = 0;
}
