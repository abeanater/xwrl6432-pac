#[doc = "Register `RES12` reader"]
pub type R = crate::R<Res12Spec>;
#[doc = "Register `RES12` writer"]
pub type W = crate::W<Res12Spec>;
#[doc = "Field `RES12` reader - 31:0\\]
Reserved"]
pub type Res12R = crate::FieldReader<u32>;
#[doc = "Field `RES12` writer - 31:0\\]
Reserved"]
pub type Res12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res12(&self) -> Res12R {
        Res12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res12(&mut self) -> Res12W<Res12Spec> {
        Res12W::new(self, 0)
    }
}
#[doc = "RES12\n\nYou can [`read`](crate::Reg::read) this register and get [`res12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res12Spec;
impl crate::RegisterSpec for Res12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res12::R`](R) reader structure"]
impl crate::Readable for Res12Spec {}
#[doc = "`write(|w| ..)` method takes [`res12::W`](W) writer structure"]
impl crate::Writable for Res12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES12 to value 0"]
impl crate::Resettable for Res12Spec {
    const RESET_VALUE: u32 = 0;
}
