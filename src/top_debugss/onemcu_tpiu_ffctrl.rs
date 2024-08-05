#[doc = "Register `ONEMCU_TPIU_FFCTRL` reader"]
pub type R = crate::R<OnemcuTpiuFfctrlSpec>;
#[doc = "Register `ONEMCU_TPIU_FFCTRL` writer"]
pub type W = crate::W<OnemcuTpiuFfctrlSpec>;
#[doc = "Field `ONEMCU_TPIU_FFCTRL` reader - 31:0\\]
Formatter and flush control"]
pub type OnemcuTpiuFfctrlR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_FFCTRL` writer - 31:0\\]
Formatter and flush control"]
pub type OnemcuTpiuFfctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Formatter and flush control"]
    #[inline(always)]
    pub fn onemcu_tpiu_ffctrl(&self) -> OnemcuTpiuFfctrlR {
        OnemcuTpiuFfctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Formatter and flush control"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_ffctrl(&mut self) -> OnemcuTpiuFfctrlW<OnemcuTpiuFfctrlSpec> {
        OnemcuTpiuFfctrlW::new(self, 0)
    }
}
#[doc = "Formatter and flush control\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ffctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ffctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuFfctrlSpec;
impl crate::RegisterSpec for OnemcuTpiuFfctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_ffctrl::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuFfctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_ffctrl::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuFfctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_FFCTRL to value 0"]
impl crate::Resettable for OnemcuTpiuFfctrlSpec {
    const RESET_VALUE: u32 = 0;
}
