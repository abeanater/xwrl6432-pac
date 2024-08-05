#[doc = "Register `HW_REG6` reader"]
pub type R = crate::R<HwReg6Spec>;
#[doc = "Register `HW_REG6` writer"]
pub type W = crate::W<HwReg6Spec>;
#[doc = "Field `hwreg6` reader - 31:0\\]
HW reserved Regsiter"]
pub type Hwreg6R = crate::FieldReader<u32>;
#[doc = "Field `hwreg6` writer - 31:0\\]
HW reserved Regsiter"]
pub type Hwreg6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HW reserved Regsiter"]
    #[inline(always)]
    pub fn hwreg6(&self) -> Hwreg6R {
        Hwreg6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HW reserved Regsiter"]
    #[inline(always)]
    #[must_use]
    pub fn hwreg6(&mut self) -> Hwreg6W<HwReg6Spec> {
        Hwreg6W::new(self, 0)
    }
}
#[doc = "HW_REG6\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwReg6Spec;
impl crate::RegisterSpec for HwReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_reg6::R`](R) reader structure"]
impl crate::Readable for HwReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_reg6::W`](W) writer structure"]
impl crate::Writable for HwReg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_REG6 to value 0"]
impl crate::Resettable for HwReg6Spec {
    const RESET_VALUE: u32 = 0;
}
