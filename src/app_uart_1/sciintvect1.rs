#[doc = "Register `SCIINTVECT1` reader"]
pub type R = crate::R<Sciintvect1Spec>;
#[doc = "Register `SCIINTVECT1` writer"]
pub type W = crate::W<Sciintvect1Spec>;
#[doc = "Field `INTVECT1` reader - 3:0\\]
Interrupt vector offset for INT1"]
pub type Intvect1R = crate::FieldReader;
#[doc = "Field `INTVECT1` writer - 3:0\\]
Interrupt vector offset for INT1"]
pub type Intvect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Interrupt vector offset for INT1"]
    #[inline(always)]
    pub fn intvect1(&self) -> Intvect1R {
        Intvect1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Interrupt vector offset for INT1"]
    #[inline(always)]
    #[must_use]
    pub fn intvect1(&mut self) -> Intvect1W<Sciintvect1Spec> {
        Intvect1W::new(self, 0)
    }
}
#[doc = "SCI Interrupt Offset Vector 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciintvect1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciintvect1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sciintvect1Spec;
impl crate::RegisterSpec for Sciintvect1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciintvect1::R`](R) reader structure"]
impl crate::Readable for Sciintvect1Spec {}
#[doc = "`write(|w| ..)` method takes [`sciintvect1::W`](W) writer structure"]
impl crate::Writable for Sciintvect1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIINTVECT1 to value 0"]
impl crate::Resettable for Sciintvect1Spec {
    const RESET_VALUE: u32 = 0;
}
