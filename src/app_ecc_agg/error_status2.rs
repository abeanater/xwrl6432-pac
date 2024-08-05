#[doc = "Register `ERROR_STATUS2` reader"]
pub type R = crate::R<ErrorStatus2Spec>;
#[doc = "Register `ERROR_STATUS2` writer"]
pub type W = crate::W<ErrorStatus2Spec>;
#[doc = "Field `ECC_ROW` reader - 31:0\\]
Row address where the single or double-bit error has occurred - (RO )"]
pub type EccRowR = crate::FieldReader<u32>;
#[doc = "Field `ECC_ROW` writer - 31:0\\]
Row address where the single or double-bit error has occurred - (RO )"]
pub type EccRowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Row address where the single or double-bit error has occurred - (RO )"]
    #[inline(always)]
    pub fn ecc_row(&self) -> EccRowR {
        EccRowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Row address where the single or double-bit error has occurred - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_row(&mut self) -> EccRowW<ErrorStatus2Spec> {
        EccRowW::new(self, 0)
    }
}
#[doc = "ECC Error Status2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_status2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorStatus2Spec;
impl crate::RegisterSpec for ErrorStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_status2::R`](R) reader structure"]
impl crate::Readable for ErrorStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`error_status2::W`](W) writer structure"]
impl crate::Writable for ErrorStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERROR_STATUS2 to value 0"]
impl crate::Resettable for ErrorStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
