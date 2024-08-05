#[doc = "Register `FEC_CM3_CTI_ITCTRL` reader"]
pub type R = crate::R<FecCm3CtiItctrlSpec>;
#[doc = "Register `FEC_CM3_CTI_ITCTRL` writer"]
pub type W = crate::W<FecCm3CtiItctrlSpec>;
#[doc = "Field `FEC_CM3_CTI_ITCTRL` reader - "]
pub type FecCm3CtiItctrlR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_ITCTRL` writer - "]
pub type FecCm3CtiItctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_itctrl(&self) -> FecCm3CtiItctrlR {
        FecCm3CtiItctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_itctrl(&mut self) -> FecCm3CtiItctrlW<FecCm3CtiItctrlSpec> {
        FecCm3CtiItctrlW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_ITCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiItctrlSpec;
impl crate::RegisterSpec for FecCm3CtiItctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_itctrl::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiItctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_itctrl::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiItctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_ITCTRL to value 0"]
impl crate::Resettable for FecCm3CtiItctrlSpec {
    const RESET_VALUE: u32 = 0;
}
