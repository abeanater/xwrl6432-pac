#[doc = "Register `RES06` reader"]
pub type R = crate::R<Res06Spec>;
#[doc = "Register `RES06` writer"]
pub type W = crate::W<Res06Spec>;
#[doc = "Field `RES06` reader - 31:0\\]
Reserved"]
pub type Res06R = crate::FieldReader<u32>;
#[doc = "Field `RES06` writer - 31:0\\]
Reserved"]
pub type Res06W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res06(&self) -> Res06R {
        Res06R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res06(&mut self) -> Res06W<Res06Spec> {
        Res06W::new(self, 0)
    }
}
#[doc = "RES06\n\nYou can [`read`](crate::Reg::read) this register and get [`res06::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res06::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res06Spec;
impl crate::RegisterSpec for Res06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res06::R`](R) reader structure"]
impl crate::Readable for Res06Spec {}
#[doc = "`write(|w| ..)` method takes [`res06::W`](W) writer structure"]
impl crate::Writable for Res06Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES06 to value 0"]
impl crate::Resettable for Res06Spec {
    const RESET_VALUE: u32 = 0;
}
