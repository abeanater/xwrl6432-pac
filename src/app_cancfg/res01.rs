#[doc = "Register `RES01` reader"]
pub type R = crate::R<Res01Spec>;
#[doc = "Register `RES01` writer"]
pub type W = crate::W<Res01Spec>;
#[doc = "Field `RES01` reader - 31:0\\]
Reserved"]
pub type Res01R = crate::FieldReader<u32>;
#[doc = "Field `RES01` writer - 31:0\\]
Reserved"]
pub type Res01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res01(&self) -> Res01R {
        Res01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res01(&mut self) -> Res01W<Res01Spec> {
        Res01W::new(self, 0)
    }
}
#[doc = "RES01\n\nYou can [`read`](crate::Reg::read) this register and get [`res01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res01Spec;
impl crate::RegisterSpec for Res01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res01::R`](R) reader structure"]
impl crate::Readable for Res01Spec {}
#[doc = "`write(|w| ..)` method takes [`res01::W`](W) writer structure"]
impl crate::Writable for Res01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES01 to value 0"]
impl crate::Resettable for Res01Spec {
    const RESET_VALUE: u32 = 0;
}
