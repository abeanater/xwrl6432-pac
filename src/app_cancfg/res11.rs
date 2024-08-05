#[doc = "Register `RES11` reader"]
pub type R = crate::R<Res11Spec>;
#[doc = "Register `RES11` writer"]
pub type W = crate::W<Res11Spec>;
#[doc = "Field `RES11` reader - 31:0\\]
Reserved"]
pub type Res11R = crate::FieldReader<u32>;
#[doc = "Field `RES11` writer - 31:0\\]
Reserved"]
pub type Res11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res11(&self) -> Res11R {
        Res11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res11(&mut self) -> Res11W<Res11Spec> {
        Res11W::new(self, 0)
    }
}
#[doc = "RES11\n\nYou can [`read`](crate::Reg::read) this register and get [`res11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res11Spec;
impl crate::RegisterSpec for Res11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res11::R`](R) reader structure"]
impl crate::Readable for Res11Spec {}
#[doc = "`write(|w| ..)` method takes [`res11::W`](W) writer structure"]
impl crate::Writable for Res11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES11 to value 0"]
impl crate::Resettable for Res11Spec {
    const RESET_VALUE: u32 = 0;
}
