#[doc = "Register `HW_SPARE_REG3` reader"]
pub type R = crate::R<HwSpareReg3Spec>;
#[doc = "Register `HW_SPARE_REG3` writer"]
pub type W = crate::W<HwSpareReg3Spec>;
#[doc = "Field `NU` reader - 31:0\\]
Resereved for R&amp;D"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:0\\]
Resereved for R&amp;D"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Resereved for R&amp;D"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Resereved for R&amp;D"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<HwSpareReg3Spec> {
        NuW::new(self, 0)
    }
}
#[doc = "HW_SPARE_REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSpareReg3Spec;
impl crate::RegisterSpec for HwSpareReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_spare_reg3::R`](R) reader structure"]
impl crate::Readable for HwSpareReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_spare_reg3::W`](W) writer structure"]
impl crate::Writable for HwSpareReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_SPARE_REG3 to value 0"]
impl crate::Resettable for HwSpareReg3Spec {
    const RESET_VALUE: u32 = 0;
}
