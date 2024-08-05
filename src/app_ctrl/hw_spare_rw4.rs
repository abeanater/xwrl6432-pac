#[doc = "Register `HW_SPARE_RW4` reader"]
pub type R = crate::R<HwSpareRw4Spec>;
#[doc = "Register `HW_SPARE_RW4` writer"]
pub type W = crate::W<HwSpareRw4Spec>;
#[doc = "Field `hw_spare_rw4` reader - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRw4R = crate::FieldReader<u32>;
#[doc = "Field `hw_spare_rw4` writer - 31:0\\]
Reserved for HW R&amp;D"]
pub type HwSpareRw4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    pub fn hw_spare_rw4(&self) -> HwSpareRw4R {
        HwSpareRw4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for HW R&amp;D"]
    #[inline(always)]
    #[must_use]
    pub fn hw_spare_rw4(&mut self) -> HwSpareRw4W<HwSpareRw4Spec> {
        HwSpareRw4W::new(self, 0)
    }
}
#[doc = "HW_SPARE_RW4\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSpareRw4Spec;
impl crate::RegisterSpec for HwSpareRw4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_spare_rw4::R`](R) reader structure"]
impl crate::Readable for HwSpareRw4Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_spare_rw4::W`](W) writer structure"]
impl crate::Writable for HwSpareRw4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_SPARE_RW4 to value 0"]
impl crate::Resettable for HwSpareRw4Spec {
    const RESET_VALUE: u32 = 0;
}
