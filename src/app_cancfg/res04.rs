#[doc = "Register `RES04` reader"]
pub type R = crate::R<Res04Spec>;
#[doc = "Register `RES04` writer"]
pub type W = crate::W<Res04Spec>;
#[doc = "Field `RES04` reader - 31:0\\]
Reserved"]
pub type Res04R = crate::FieldReader<u32>;
#[doc = "Field `RES04` writer - 31:0\\]
Reserved"]
pub type Res04W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res04(&self) -> Res04R {
        Res04R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res04(&mut self) -> Res04W<Res04Spec> {
        Res04W::new(self, 0)
    }
}
#[doc = "RES04\n\nYou can [`read`](crate::Reg::read) this register and get [`res04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res04Spec;
impl crate::RegisterSpec for Res04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res04::R`](R) reader structure"]
impl crate::Readable for Res04Spec {}
#[doc = "`write(|w| ..)` method takes [`res04::W`](W) writer structure"]
impl crate::Writable for Res04Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES04 to value 0"]
impl crate::Resettable for Res04Spec {
    const RESET_VALUE: u32 = 0;
}
