#[doc = "Register `RES02` reader"]
pub type R = crate::R<Res02Spec>;
#[doc = "Register `RES02` writer"]
pub type W = crate::W<Res02Spec>;
#[doc = "Field `RES02` reader - 31:0\\]
Reserved"]
pub type Res02R = crate::FieldReader<u32>;
#[doc = "Field `RES02` writer - 31:0\\]
Reserved"]
pub type Res02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res02(&self) -> Res02R {
        Res02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res02(&mut self) -> Res02W<Res02Spec> {
        Res02W::new(self, 0)
    }
}
#[doc = "RES02\n\nYou can [`read`](crate::Reg::read) this register and get [`res02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res02Spec;
impl crate::RegisterSpec for Res02Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res02::R`](R) reader structure"]
impl crate::Readable for Res02Spec {}
#[doc = "`write(|w| ..)` method takes [`res02::W`](W) writer structure"]
impl crate::Writable for Res02Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES02 to value 0"]
impl crate::Resettable for Res02Spec {
    const RESET_VALUE: u32 = 0;
}
