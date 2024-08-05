#[doc = "Register `ONEMCU_CTI_ITCTRL` reader"]
pub type R = crate::R<OnemcuCtiItctrlSpec>;
#[doc = "Register `ONEMCU_CTI_ITCTRL` writer"]
pub type W = crate::W<OnemcuCtiItctrlSpec>;
#[doc = "Field `ONEMCU_CTI_ITCTRL` reader - "]
pub type OnemcuCtiItctrlR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_ITCTRL` writer - "]
pub type OnemcuCtiItctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_itctrl(&self) -> OnemcuCtiItctrlR {
        OnemcuCtiItctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_itctrl(&mut self) -> OnemcuCtiItctrlW<OnemcuCtiItctrlSpec> {
        OnemcuCtiItctrlW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_ITCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiItctrlSpec;
impl crate::RegisterSpec for OnemcuCtiItctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_itctrl::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiItctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_itctrl::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiItctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_ITCTRL to value 0"]
impl crate::Resettable for OnemcuCtiItctrlSpec {
    const RESET_VALUE: u32 = 0;
}
