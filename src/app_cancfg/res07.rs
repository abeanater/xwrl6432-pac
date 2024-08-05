#[doc = "Register `RES07` reader"]
pub type R = crate::R<Res07Spec>;
#[doc = "Register `RES07` writer"]
pub type W = crate::W<Res07Spec>;
#[doc = "Field `RES07` reader - 31:0\\]
Reserved"]
pub type Res07R = crate::FieldReader<u32>;
#[doc = "Field `RES07` writer - 31:0\\]
Reserved"]
pub type Res07W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res07(&self) -> Res07R {
        Res07R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res07(&mut self) -> Res07W<Res07Spec> {
        Res07W::new(self, 0)
    }
}
#[doc = "RES07\n\nYou can [`read`](crate::Reg::read) this register and get [`res07::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res07::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res07Spec;
impl crate::RegisterSpec for Res07Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res07::R`](R) reader structure"]
impl crate::Readable for Res07Spec {}
#[doc = "`write(|w| ..)` method takes [`res07::W`](W) writer structure"]
impl crate::Writable for Res07Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES07 to value 0"]
impl crate::Resettable for Res07Spec {
    const RESET_VALUE: u32 = 0;
}
