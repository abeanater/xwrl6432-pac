#[doc = "Register `ERR_CTRL1` reader"]
pub type R = crate::R<ErrCtrl1Spec>;
#[doc = "Register `ERR_CTRL1` writer"]
pub type W = crate::W<ErrCtrl1Spec>;
#[doc = "Field `ECC_ROW` reader - 31:0\\]
TI Internal : Row address where single or double-bit error needs to be applied. This is ignored if force_n_row is set"]
pub type EccRowR = crate::FieldReader<u32>;
#[doc = "Field `ECC_ROW` writer - 31:0\\]
TI Internal : Row address where single or double-bit error needs to be applied. This is ignored if force_n_row is set"]
pub type EccRowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal : Row address where single or double-bit error needs to be applied. This is ignored if force_n_row is set"]
    #[inline(always)]
    pub fn ecc_row(&self) -> EccRowR {
        EccRowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal : Row address where single or double-bit error needs to be applied. This is ignored if force_n_row is set"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_row(&mut self) -> EccRowW<ErrCtrl1Spec> {
        EccRowW::new(self, 0)
    }
}
#[doc = "ERR_CTRL1\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrCtrl1Spec;
impl crate::RegisterSpec for ErrCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_ctrl1::R`](R) reader structure"]
impl crate::Readable for ErrCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`err_ctrl1::W`](W) writer structure"]
impl crate::Writable for ErrCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_CTRL1 to value 0"]
impl crate::Resettable for ErrCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
