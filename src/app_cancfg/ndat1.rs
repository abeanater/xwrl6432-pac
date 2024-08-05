#[doc = "Register `NDAT1` reader"]
pub type R = crate::R<Ndat1Spec>;
#[doc = "Register `NDAT1` writer"]
pub type W = crate::W<Ndat1Spec>;
#[doc = "Field `ND0_31` reader - 31:0\\]
New Data 0-31"]
pub type Nd0_31R = crate::FieldReader<u32>;
#[doc = "Field `ND0_31` writer - 31:0\\]
New Data 0-31"]
pub type Nd0_31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
New Data 0-31"]
    #[inline(always)]
    pub fn nd0_31(&self) -> Nd0_31R {
        Nd0_31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
New Data 0-31"]
    #[inline(always)]
    #[must_use]
    pub fn nd0_31(&mut self) -> Nd0_31W<Ndat1Spec> {
        Nd0_31W::new(self, 0)
    }
}
#[doc = "NDAT1\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ndat1Spec;
impl crate::RegisterSpec for Ndat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndat1::R`](R) reader structure"]
impl crate::Readable for Ndat1Spec {}
#[doc = "`write(|w| ..)` method takes [`ndat1::W`](W) writer structure"]
impl crate::Writable for Ndat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NDAT1 to value 0"]
impl crate::Resettable for Ndat1Spec {
    const RESET_VALUE: u32 = 0;
}
