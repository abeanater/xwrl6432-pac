#[doc = "Register `HW_SPARE_RW3` reader"]
pub type R = crate::R<HwSpareRw3Spec>;
#[doc = "Register `HW_SPARE_RW3` writer"]
pub type W = crate::W<HwSpareRw3Spec>;
#[doc = "Field `hw_spare_rw3` reader - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRw3R = crate::FieldReader<u32>;
#[doc = "Field `hw_spare_rw3` writer - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRw3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    pub fn hw_spare_rw3(&self) -> HwSpareRw3R {
        HwSpareRw3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    #[must_use]
    pub fn hw_spare_rw3(&mut self) -> HwSpareRw3W<HwSpareRw3Spec> {
        HwSpareRw3W::new(self, 0)
    }
}
#[doc = "HW_SPARE_RW3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSpareRw3Spec;
impl crate::RegisterSpec for HwSpareRw3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_spare_rw3::R`](R) reader structure"]
impl crate::Readable for HwSpareRw3Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_spare_rw3::W`](W) writer structure"]
impl crate::Writable for HwSpareRw3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_SPARE_RW3 to value 0"]
impl crate::Resettable for HwSpareRw3Spec {
    const RESET_VALUE: u32 = 0;
}
