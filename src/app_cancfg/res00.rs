#[doc = "Register `RES00` reader"]
pub type R = crate::R<Res00Spec>;
#[doc = "Register `RES00` writer"]
pub type W = crate::W<Res00Spec>;
#[doc = "Field `RES00` reader - 31:0\\]
Reserved"]
pub type Res00R = crate::FieldReader<u32>;
#[doc = "Field `RES00` writer - 31:0\\]
Reserved"]
pub type Res00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res00(&self) -> Res00R {
        Res00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res00(&mut self) -> Res00W<Res00Spec> {
        Res00W::new(self, 0)
    }
}
#[doc = "RES00\n\nYou can [`read`](crate::Reg::read) this register and get [`res00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res00Spec;
impl crate::RegisterSpec for Res00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res00::R`](R) reader structure"]
impl crate::Readable for Res00Spec {}
#[doc = "`write(|w| ..)` method takes [`res00::W`](W) writer structure"]
impl crate::Writable for Res00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES00 to value 0"]
impl crate::Resettable for Res00Spec {
    const RESET_VALUE: u32 = 0;
}
