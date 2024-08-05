#[doc = "Register `SCIINTVECT0` reader"]
pub type R = crate::R<Sciintvect0Spec>;
#[doc = "Register `SCIINTVECT0` writer"]
pub type W = crate::W<Sciintvect0Spec>;
#[doc = "Field `INTVECT0` reader - 3:0\\]
Interrupt vector offset for INT0"]
pub type Intvect0R = crate::FieldReader;
#[doc = "Field `INTVECT0` writer - 3:0\\]
Interrupt vector offset for INT0"]
pub type Intvect0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Interrupt vector offset for INT0"]
    #[inline(always)]
    pub fn intvect0(&self) -> Intvect0R {
        Intvect0R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Interrupt vector offset for INT0"]
    #[inline(always)]
    #[must_use]
    pub fn intvect0(&mut self) -> Intvect0W<Sciintvect0Spec> {
        Intvect0W::new(self, 0)
    }
}
#[doc = "SCI Interrupt Offset Vector 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciintvect0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciintvect0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sciintvect0Spec;
impl crate::RegisterSpec for Sciintvect0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciintvect0::R`](R) reader structure"]
impl crate::Readable for Sciintvect0Spec {}
#[doc = "`write(|w| ..)` method takes [`sciintvect0::W`](W) writer structure"]
impl crate::Writable for Sciintvect0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIINTVECT0 to value 0"]
impl crate::Resettable for Sciintvect0Spec {
    const RESET_VALUE: u32 = 0;
}
