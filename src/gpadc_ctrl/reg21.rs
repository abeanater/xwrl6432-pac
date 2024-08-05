#[doc = "Register `REG21` reader"]
pub type R = crate::R<Reg21Spec>;
#[doc = "Register `REG21` writer"]
pub type W = crate::W<Reg21Spec>;
#[doc = "Field `SPARE_RD2` reader - 31:0\\]
TI reserved"]
pub type SpareRd2R = crate::FieldReader<u32>;
#[doc = "Field `SPARE_RD2` writer - 31:0\\]
TI reserved"]
pub type SpareRd2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    pub fn spare_rd2(&self) -> SpareRd2R {
        SpareRd2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn spare_rd2(&mut self) -> SpareRd2W<Reg21Spec> {
        SpareRd2W::new(self, 0)
    }
}
#[doc = "REG21\n\nYou can [`read`](crate::Reg::read) this register and get [`reg21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg21Spec;
impl crate::RegisterSpec for Reg21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg21::R`](R) reader structure"]
impl crate::Readable for Reg21Spec {}
#[doc = "`write(|w| ..)` method takes [`reg21::W`](W) writer structure"]
impl crate::Writable for Reg21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG21 to value 0"]
impl crate::Resettable for Reg21Spec {
    const RESET_VALUE: u32 = 0;
}
