#[doc = "Register `RES15` reader"]
pub type R = crate::R<Res15Spec>;
#[doc = "Register `RES15` writer"]
pub type W = crate::W<Res15Spec>;
#[doc = "Field `RES15` reader - 31:0\\]
Reserved"]
pub type Res15R = crate::FieldReader<u32>;
#[doc = "Field `RES15` writer - 31:0\\]
Reserved"]
pub type Res15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res15(&self) -> Res15R {
        Res15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res15(&mut self) -> Res15W<Res15Spec> {
        Res15W::new(self, 0)
    }
}
#[doc = "RES15\n\nYou can [`read`](crate::Reg::read) this register and get [`res15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res15Spec;
impl crate::RegisterSpec for Res15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res15::R`](R) reader structure"]
impl crate::Readable for Res15Spec {}
#[doc = "`write(|w| ..)` method takes [`res15::W`](W) writer structure"]
impl crate::Writable for Res15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES15 to value 0"]
impl crate::Resettable for Res15Spec {
    const RESET_VALUE: u32 = 0;
}
