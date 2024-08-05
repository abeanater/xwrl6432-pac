#[doc = "Register `RES03` reader"]
pub type R = crate::R<Res03Spec>;
#[doc = "Register `RES03` writer"]
pub type W = crate::W<Res03Spec>;
#[doc = "Field `RES03` reader - 31:0\\]
Reserved"]
pub type Res03R = crate::FieldReader<u32>;
#[doc = "Field `RES03` writer - 31:0\\]
Reserved"]
pub type Res03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res03(&self) -> Res03R {
        Res03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res03(&mut self) -> Res03W<Res03Spec> {
        Res03W::new(self, 0)
    }
}
#[doc = "RES03\n\nYou can [`read`](crate::Reg::read) this register and get [`res03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res03Spec;
impl crate::RegisterSpec for Res03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res03::R`](R) reader structure"]
impl crate::Readable for Res03Spec {}
#[doc = "`write(|w| ..)` method takes [`res03::W`](W) writer structure"]
impl crate::Writable for Res03Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES03 to value 0"]
impl crate::Resettable for Res03Spec {
    const RESET_VALUE: u32 = 0;
}
