#[doc = "Register `RESERVED1` reader"]
pub type R = crate::R<Reserved1Spec>;
#[doc = "Register `RESERVED1` writer"]
pub type W = crate::W<Reserved1Spec>;
#[doc = "Field `rwres` reader - "]
pub type RwresR = crate::FieldReader;
#[doc = "Field `rwres` writer - "]
pub type RwresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rores` reader - "]
pub type RoresR = crate::FieldReader;
#[doc = "Field `rores` writer - "]
pub type RoresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `wphres` reader - "]
pub type WphresR = crate::FieldReader;
#[doc = "Field `wphres` writer - "]
pub type WphresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rwres(&self) -> RwresR {
        RwresR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rores(&self) -> RoresR {
        RoresR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wphres(&self) -> WphresR {
        WphresR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rwres(&mut self) -> RwresW<Reserved1Spec> {
        RwresW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn rores(&mut self) -> RoresW<Reserved1Spec> {
        RoresW::new(self, 8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn wphres(&mut self) -> WphresW<Reserved1Spec> {
        WphresW::new(self, 24)
    }
}
#[doc = "RESERVED1\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved1Spec;
impl crate::RegisterSpec for Reserved1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved1::R`](R) reader structure"]
impl crate::Readable for Reserved1Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved1::W`](W) writer structure"]
impl crate::Writable for Reserved1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED1 to value 0"]
impl crate::Resettable for Reserved1Spec {
    const RESET_VALUE: u32 = 0;
}
