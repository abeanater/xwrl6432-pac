#[doc = "Register `RES13` reader"]
pub type R = crate::R<Res13Spec>;
#[doc = "Register `RES13` writer"]
pub type W = crate::W<Res13Spec>;
#[doc = "Field `RES13` reader - 31:0\\]
Reserved"]
pub type Res13R = crate::FieldReader<u32>;
#[doc = "Field `RES13` writer - 31:0\\]
Reserved"]
pub type Res13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res13(&self) -> Res13R {
        Res13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res13(&mut self) -> Res13W<Res13Spec> {
        Res13W::new(self, 0)
    }
}
#[doc = "RES13\n\nYou can [`read`](crate::Reg::read) this register and get [`res13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res13Spec;
impl crate::RegisterSpec for Res13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res13::R`](R) reader structure"]
impl crate::Readable for Res13Spec {}
#[doc = "`write(|w| ..)` method takes [`res13::W`](W) writer structure"]
impl crate::Writable for Res13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES13 to value 0"]
impl crate::Resettable for Res13Spec {
    const RESET_VALUE: u32 = 0;
}
