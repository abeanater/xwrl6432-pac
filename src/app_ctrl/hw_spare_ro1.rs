#[doc = "Register `HW_SPARE_RO1` reader"]
pub type R = crate::R<HwSpareRo1Spec>;
#[doc = "Register `HW_SPARE_RO1` writer"]
pub type W = crate::W<HwSpareRo1Spec>;
#[doc = "Field `hw_spare_ro1` reader - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRo1R = crate::FieldReader<u32>;
#[doc = "Field `hw_spare_ro1` writer - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRo1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    pub fn hw_spare_ro1(&self) -> HwSpareRo1R {
        HwSpareRo1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    #[must_use]
    pub fn hw_spare_ro1(&mut self) -> HwSpareRo1W<HwSpareRo1Spec> {
        HwSpareRo1W::new(self, 0)
    }
}
#[doc = "HW_SPARE_RO1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_ro1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_ro1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSpareRo1Spec;
impl crate::RegisterSpec for HwSpareRo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_spare_ro1::R`](R) reader structure"]
impl crate::Readable for HwSpareRo1Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_spare_ro1::W`](W) writer structure"]
impl crate::Writable for HwSpareRo1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_SPARE_RO1 to value 0"]
impl crate::Resettable for HwSpareRo1Spec {
    const RESET_VALUE: u32 = 0;
}
