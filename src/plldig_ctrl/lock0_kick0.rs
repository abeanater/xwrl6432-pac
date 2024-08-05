#[doc = "Register `LOCK0_KICK0` reader"]
pub type R = crate::R<Lock0Kick0Spec>;
#[doc = "Register `LOCK0_KICK0` writer"]
pub type W = crate::W<Lock0Kick0Spec>;
#[doc = "Field `KICK0_COMPONENT` reader - 31:0\\]
- KICK0 component"]
pub type Kick0ComponentR = crate::FieldReader<u32>;
#[doc = "Field `KICK0_COMPONENT` writer - 31:0\\]
- KICK0 component"]
pub type Kick0ComponentW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
- KICK0 component"]
    #[inline(always)]
    pub fn kick0_component(&self) -> Kick0ComponentR {
        Kick0ComponentR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
- KICK0 component"]
    #[inline(always)]
    #[must_use]
    pub fn kick0_component(&mut self) -> Kick0ComponentW<Lock0Kick0Spec> {
        Kick0ComponentW::new(self, 0)
    }
}
#[doc = "- KICK0 component\n\nYou can [`read`](crate::Reg::read) this register and get [`lock0_kick0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock0_kick0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lock0Kick0Spec;
impl crate::RegisterSpec for Lock0Kick0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock0_kick0::R`](R) reader structure"]
impl crate::Readable for Lock0Kick0Spec {}
#[doc = "`write(|w| ..)` method takes [`lock0_kick0::W`](W) writer structure"]
impl crate::Writable for Lock0Kick0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK0_KICK0 to value 0"]
impl crate::Resettable for Lock0Kick0Spec {
    const RESET_VALUE: u32 = 0;
}
