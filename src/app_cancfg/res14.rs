#[doc = "Register `RES14` reader"]
pub type R = crate::R<Res14Spec>;
#[doc = "Register `RES14` writer"]
pub type W = crate::W<Res14Spec>;
#[doc = "Field `RES14` reader - 31:0\\]
Reserved"]
pub type Res14R = crate::FieldReader<u32>;
#[doc = "Field `RES14` writer - 31:0\\]
Reserved"]
pub type Res14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res14(&self) -> Res14R {
        Res14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res14(&mut self) -> Res14W<Res14Spec> {
        Res14W::new(self, 0)
    }
}
#[doc = "RES14\n\nYou can [`read`](crate::Reg::read) this register and get [`res14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res14Spec;
impl crate::RegisterSpec for Res14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res14::R`](R) reader structure"]
impl crate::Readable for Res14Spec {}
#[doc = "`write(|w| ..)` method takes [`res14::W`](W) writer structure"]
impl crate::Writable for Res14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES14 to value 0"]
impl crate::Resettable for Res14Spec {
    const RESET_VALUE: u32 = 0;
}
