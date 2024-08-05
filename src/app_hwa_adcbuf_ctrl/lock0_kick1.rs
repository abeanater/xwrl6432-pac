#[doc = "Register `LOCK0_KICK1` reader"]
pub type R = crate::R<Lock0Kick1Spec>;
#[doc = "Register `LOCK0_KICK1` writer"]
pub type W = crate::W<Lock0Kick1Spec>;
#[doc = "Field `KICK1_COMPONENT` reader - 31:0\\]
- KICK1 component"]
pub type Kick1ComponentR = crate::FieldReader<u32>;
#[doc = "Field `KICK1_COMPONENT` writer - 31:0\\]
- KICK1 component"]
pub type Kick1ComponentW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
- KICK1 component"]
    #[inline(always)]
    pub fn kick1_component(&self) -> Kick1ComponentR {
        Kick1ComponentR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
- KICK1 component"]
    #[inline(always)]
    #[must_use]
    pub fn kick1_component(&mut self) -> Kick1ComponentW<Lock0Kick1Spec> {
        Kick1ComponentW::new(self, 0)
    }
}
#[doc = "- KICK1 component\n\nYou can [`read`](crate::Reg::read) this register and get [`lock0_kick1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock0_kick1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lock0Kick1Spec;
impl crate::RegisterSpec for Lock0Kick1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock0_kick1::R`](R) reader structure"]
impl crate::Readable for Lock0Kick1Spec {}
#[doc = "`write(|w| ..)` method takes [`lock0_kick1::W`](W) writer structure"]
impl crate::Writable for Lock0Kick1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK0_KICK1 to value 0"]
impl crate::Resettable for Lock0Kick1Spec {
    const RESET_VALUE: u32 = 0;
}
