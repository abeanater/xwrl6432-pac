#[doc = "Register `ERROR_CTRL1` reader"]
pub type R = crate::R<ErrorCtrl1Spec>;
#[doc = "Register `ERROR_CTRL1` writer"]
pub type W = crate::W<ErrorCtrl1Spec>;
#[doc = "Field `ECC_ROW` reader - 31:0\\]
Row address where single or double-bit error needs to be applied. This is ignored if force_n_row is set - (RW )"]
pub type EccRowR = crate::FieldReader<u32>;
#[doc = "Field `ECC_ROW` writer - 31:0\\]
Row address where single or double-bit error needs to be applied. This is ignored if force_n_row is set - (RW )"]
pub type EccRowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Row address where single or double-bit error needs to be applied. This is ignored if force_n_row is set - (RW )"]
    #[inline(always)]
    pub fn ecc_row(&self) -> EccRowR {
        EccRowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Row address where single or double-bit error needs to be applied. This is ignored if force_n_row is set - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_row(&mut self) -> EccRowW<ErrorCtrl1Spec> {
        EccRowW::new(self, 0)
    }
}
#[doc = "ECC Error Control1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorCtrl1Spec;
impl crate::RegisterSpec for ErrorCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_ctrl1::R`](R) reader structure"]
impl crate::Readable for ErrorCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`error_ctrl1::W`](W) writer structure"]
impl crate::Writable for ErrorCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERROR_CTRL1 to value 0"]
impl crate::Resettable for ErrorCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
