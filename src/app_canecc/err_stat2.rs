#[doc = "Register `ERR_STAT2` reader"]
pub type R = crate::R<ErrStat2Spec>;
#[doc = "Register `ERR_STAT2` writer"]
pub type W = crate::W<ErrStat2Spec>;
#[doc = "Field `ECC_ROW` reader - 31:0\\]
TI Internal : Row address where the single or double-bit error has occurred"]
pub type EccRowR = crate::FieldReader<u32>;
#[doc = "Field `ECC_ROW` writer - 31:0\\]
TI Internal : Row address where the single or double-bit error has occurred"]
pub type EccRowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal : Row address where the single or double-bit error has occurred"]
    #[inline(always)]
    pub fn ecc_row(&self) -> EccRowR {
        EccRowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TI Internal : Row address where the single or double-bit error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_row(&mut self) -> EccRowW<ErrStat2Spec> {
        EccRowW::new(self, 0)
    }
}
#[doc = "ERR_STAT2\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStat2Spec;
impl crate::RegisterSpec for ErrStat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat2::R`](R) reader structure"]
impl crate::Readable for ErrStat2Spec {}
#[doc = "`write(|w| ..)` method takes [`err_stat2::W`](W) writer structure"]
impl crate::Writable for ErrStat2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_STAT2 to value 0"]
impl crate::Resettable for ErrStat2Spec {
    const RESET_VALUE: u32 = 0;
}
