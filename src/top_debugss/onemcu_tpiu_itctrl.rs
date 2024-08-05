#[doc = "Register `ONEMCU_TPIU_ITCTRL` reader"]
pub type R = crate::R<OnemcuTpiuItctrlSpec>;
#[doc = "Register `ONEMCU_TPIU_ITCTRL` writer"]
pub type W = crate::W<OnemcuTpiuItctrlSpec>;
#[doc = "Field `ONEMCU_TPIU_ITCTRL` reader - 31:0\\]
Integration Mode Control Register"]
pub type OnemcuTpiuItctrlR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_ITCTRL` writer - 31:0\\]
Integration Mode Control Register"]
pub type OnemcuTpiuItctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Mode Control Register"]
    #[inline(always)]
    pub fn onemcu_tpiu_itctrl(&self) -> OnemcuTpiuItctrlR {
        OnemcuTpiuItctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Integration Mode Control Register"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_itctrl(&mut self) -> OnemcuTpiuItctrlW<OnemcuTpiuItctrlSpec> {
        OnemcuTpiuItctrlW::new(self, 0)
    }
}
#[doc = "Integration Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuItctrlSpec;
impl crate::RegisterSpec for OnemcuTpiuItctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_itctrl::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuItctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_itctrl::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuItctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_ITCTRL to value 0"]
impl crate::Resettable for OnemcuTpiuItctrlSpec {
    const RESET_VALUE: u32 = 0;
}
