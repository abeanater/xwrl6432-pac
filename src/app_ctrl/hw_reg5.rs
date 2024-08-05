#[doc = "Register `HW_REG5` reader"]
pub type R = crate::R<HwReg5Spec>;
#[doc = "Register `HW_REG5` writer"]
pub type W = crate::W<HwReg5Spec>;
#[doc = "Field `hwreg5` reader - 31:0\\]
HW reserved Regsiter"]
pub type Hwreg5R = crate::FieldReader<u32>;
#[doc = "Field `hwreg5` writer - 31:0\\]
HW reserved Regsiter"]
pub type Hwreg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HW reserved Regsiter"]
    #[inline(always)]
    pub fn hwreg5(&self) -> Hwreg5R {
        Hwreg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HW reserved Regsiter"]
    #[inline(always)]
    #[must_use]
    pub fn hwreg5(&mut self) -> Hwreg5W<HwReg5Spec> {
        Hwreg5W::new(self, 0)
    }
}
#[doc = "HW_REG5\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwReg5Spec;
impl crate::RegisterSpec for HwReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_reg5::R`](R) reader structure"]
impl crate::Readable for HwReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_reg5::W`](W) writer structure"]
impl crate::Writable for HwReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_REG5 to value 0"]
impl crate::Resettable for HwReg5Spec {
    const RESET_VALUE: u32 = 0;
}
