#[doc = "Register `HW_SPARE_RW5` reader"]
pub type R = crate::R<HwSpareRw5Spec>;
#[doc = "Register `HW_SPARE_RW5` writer"]
pub type W = crate::W<HwSpareRw5Spec>;
#[doc = "Field `hw_spare_rw5` reader - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRw5R = crate::FieldReader<u32>;
#[doc = "Field `hw_spare_rw5` writer - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRw5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    pub fn hw_spare_rw5(&self) -> HwSpareRw5R {
        HwSpareRw5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    #[must_use]
    pub fn hw_spare_rw5(&mut self) -> HwSpareRw5W<HwSpareRw5Spec> {
        HwSpareRw5W::new(self, 0)
    }
}
#[doc = "HW_SPARE_RW5\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSpareRw5Spec;
impl crate::RegisterSpec for HwSpareRw5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_spare_rw5::R`](R) reader structure"]
impl crate::Readable for HwSpareRw5Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_spare_rw5::W`](W) writer structure"]
impl crate::Writable for HwSpareRw5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_SPARE_RW5 to value 0"]
impl crate::Resettable for HwSpareRw5Spec {
    const RESET_VALUE: u32 = 0;
}
