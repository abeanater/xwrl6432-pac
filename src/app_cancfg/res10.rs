#[doc = "Register `RES10` reader"]
pub type R = crate::R<Res10Spec>;
#[doc = "Register `RES10` writer"]
pub type W = crate::W<Res10Spec>;
#[doc = "Field `RES10` reader - 31:0\\]
Reserved"]
pub type Res10R = crate::FieldReader<u32>;
#[doc = "Field `RES10` writer - 31:0\\]
Reserved"]
pub type Res10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res10(&self) -> Res10R {
        Res10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res10(&mut self) -> Res10W<Res10Spec> {
        Res10W::new(self, 0)
    }
}
#[doc = "RES10\n\nYou can [`read`](crate::Reg::read) this register and get [`res10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res10Spec;
impl crate::RegisterSpec for Res10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res10::R`](R) reader structure"]
impl crate::Readable for Res10Spec {}
#[doc = "`write(|w| ..)` method takes [`res10::W`](W) writer structure"]
impl crate::Writable for Res10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES10 to value 0"]
impl crate::Resettable for Res10Spec {
    const RESET_VALUE: u32 = 0;
}
