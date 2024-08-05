#[doc = "Register `HW_REG7` reader"]
pub type R = crate::R<HwReg7Spec>;
#[doc = "Register `HW_REG7` writer"]
pub type W = crate::W<HwReg7Spec>;
#[doc = "Field `hwreg7` reader - 31:0\\]
HW reserved Regsiter"]
pub type Hwreg7R = crate::FieldReader<u32>;
#[doc = "Field `hwreg7` writer - 31:0\\]
HW reserved Regsiter"]
pub type Hwreg7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HW reserved Regsiter"]
    #[inline(always)]
    pub fn hwreg7(&self) -> Hwreg7R {
        Hwreg7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HW reserved Regsiter"]
    #[inline(always)]
    #[must_use]
    pub fn hwreg7(&mut self) -> Hwreg7W<HwReg7Spec> {
        Hwreg7W::new(self, 0)
    }
}
#[doc = "HW_REG7\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwReg7Spec;
impl crate::RegisterSpec for HwReg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_reg7::R`](R) reader structure"]
impl crate::Readable for HwReg7Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_reg7::W`](W) writer structure"]
impl crate::Writable for HwReg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_REG7 to value 0"]
impl crate::Resettable for HwReg7Spec {
    const RESET_VALUE: u32 = 0;
}
