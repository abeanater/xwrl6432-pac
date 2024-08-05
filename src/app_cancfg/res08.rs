#[doc = "Register `RES08` reader"]
pub type R = crate::R<Res08Spec>;
#[doc = "Register `RES08` writer"]
pub type W = crate::W<Res08Spec>;
#[doc = "Field `RES08` reader - 31:0\\]
Reserved"]
pub type Res08R = crate::FieldReader<u32>;
#[doc = "Field `RES08` writer - 31:0\\]
Reserved"]
pub type Res08W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res08(&self) -> Res08R {
        Res08R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res08(&mut self) -> Res08W<Res08Spec> {
        Res08W::new(self, 0)
    }
}
#[doc = "RES08\n\nYou can [`read`](crate::Reg::read) this register and get [`res08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res08Spec;
impl crate::RegisterSpec for Res08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res08::R`](R) reader structure"]
impl crate::Readable for Res08Spec {}
#[doc = "`write(|w| ..)` method takes [`res08::W`](W) writer structure"]
impl crate::Writable for Res08Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES08 to value 0"]
impl crate::Resettable for Res08Spec {
    const RESET_VALUE: u32 = 0;
}
