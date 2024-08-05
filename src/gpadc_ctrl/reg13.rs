#[doc = "Register `REG13` reader"]
pub type R = crate::R<Reg13Spec>;
#[doc = "Register `REG13` writer"]
pub type W = crate::W<Reg13Spec>;
#[doc = "Field `SPARE_WR2` reader - 31:0\\]
TI reserved"]
pub type SpareWr2R = crate::FieldReader<u32>;
#[doc = "Field `SPARE_WR2` writer - 31:0\\]
TI reserved"]
pub type SpareWr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    pub fn spare_wr2(&self) -> SpareWr2R {
        SpareWr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn spare_wr2(&mut self) -> SpareWr2W<Reg13Spec> {
        SpareWr2W::new(self, 0)
    }
}
#[doc = "REG13\n\nYou can [`read`](crate::Reg::read) this register and get [`reg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg13Spec;
impl crate::RegisterSpec for Reg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg13::R`](R) reader structure"]
impl crate::Readable for Reg13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg13::W`](W) writer structure"]
impl crate::Writable for Reg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG13 to value 0"]
impl crate::Resettable for Reg13Spec {
    const RESET_VALUE: u32 = 0;
}
