#[doc = "Register `NDAT2` reader"]
pub type R = crate::R<Ndat2Spec>;
#[doc = "Register `NDAT2` writer"]
pub type W = crate::W<Ndat2Spec>;
#[doc = "Field `ND32_63` reader - 31:0\\]
New Data 32-63"]
pub type Nd32_63R = crate::FieldReader<u32>;
#[doc = "Field `ND32_63` writer - 31:0\\]
New Data 32-63"]
pub type Nd32_63W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
New Data 32-63"]
    #[inline(always)]
    pub fn nd32_63(&self) -> Nd32_63R {
        Nd32_63R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
New Data 32-63"]
    #[inline(always)]
    #[must_use]
    pub fn nd32_63(&mut self) -> Nd32_63W<Ndat2Spec> {
        Nd32_63W::new(self, 0)
    }
}
#[doc = "NDAT2\n\nYou can [`read`](crate::Reg::read) this register and get [`ndat2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndat2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ndat2Spec;
impl crate::RegisterSpec for Ndat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndat2::R`](R) reader structure"]
impl crate::Readable for Ndat2Spec {}
#[doc = "`write(|w| ..)` method takes [`ndat2::W`](W) writer structure"]
impl crate::Writable for Ndat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NDAT2 to value 0"]
impl crate::Resettable for Ndat2Spec {
    const RESET_VALUE: u32 = 0;
}
