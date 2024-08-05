#[doc = "Register `RES09` reader"]
pub type R = crate::R<Res09Spec>;
#[doc = "Register `RES09` writer"]
pub type W = crate::W<Res09Spec>;
#[doc = "Field `RES09` reader - 31:0\\]
Reserved"]
pub type Res09R = crate::FieldReader<u32>;
#[doc = "Field `RES09` writer - 31:0\\]
Reserved"]
pub type Res09W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res09(&self) -> Res09R {
        Res09R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res09(&mut self) -> Res09W<Res09Spec> {
        Res09W::new(self, 0)
    }
}
#[doc = "RES09\n\nYou can [`read`](crate::Reg::read) this register and get [`res09::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res09::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res09Spec;
impl crate::RegisterSpec for Res09Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res09::R`](R) reader structure"]
impl crate::Readable for Res09Spec {}
#[doc = "`write(|w| ..)` method takes [`res09::W`](W) writer structure"]
impl crate::Writable for Res09Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES09 to value 0"]
impl crate::Resettable for Res09Spec {
    const RESET_VALUE: u32 = 0;
}
