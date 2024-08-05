#[doc = "Register `RES16` reader"]
pub type R = crate::R<Res16Spec>;
#[doc = "Register `RES16` writer"]
pub type W = crate::W<Res16Spec>;
#[doc = "Field `RES16` reader - 31:0\\]
Reserved"]
pub type Res16R = crate::FieldReader<u32>;
#[doc = "Field `RES16` writer - 31:0\\]
Reserved"]
pub type Res16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn res16(&self) -> Res16R {
        Res16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res16(&mut self) -> Res16W<Res16Spec> {
        Res16W::new(self, 0)
    }
}
#[doc = "RES16\n\nYou can [`read`](crate::Reg::read) this register and get [`res16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res16Spec;
impl crate::RegisterSpec for Res16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res16::R`](R) reader structure"]
impl crate::Readable for Res16Spec {}
#[doc = "`write(|w| ..)` method takes [`res16::W`](W) writer structure"]
impl crate::Writable for Res16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES16 to value 0"]
impl crate::Resettable for Res16Spec {
    const RESET_VALUE: u32 = 0;
}
