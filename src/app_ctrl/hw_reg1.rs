#[doc = "Register `HW_REG1` reader"]
pub type R = crate::R<HwReg1Spec>;
#[doc = "Register `HW_REG1` writer"]
pub type W = crate::W<HwReg1Spec>;
#[doc = "Field `hwreg1` reader - 31:0\\]
HW reserved Regsiter"]
pub type Hwreg1R = crate::FieldReader<u32>;
#[doc = "Field `hwreg1` writer - 31:0\\]
HW reserved Regsiter"]
pub type Hwreg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
HW reserved Regsiter"]
    #[inline(always)]
    pub fn hwreg1(&self) -> Hwreg1R {
        Hwreg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
HW reserved Regsiter"]
    #[inline(always)]
    #[must_use]
    pub fn hwreg1(&mut self) -> Hwreg1W<HwReg1Spec> {
        Hwreg1W::new(self, 0)
    }
}
#[doc = "HW_REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwReg1Spec;
impl crate::RegisterSpec for HwReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_reg1::R`](R) reader structure"]
impl crate::Readable for HwReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_reg1::W`](W) writer structure"]
impl crate::Writable for HwReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_REG1 to value 0"]
impl crate::Resettable for HwReg1Spec {
    const RESET_VALUE: u32 = 0;
}
