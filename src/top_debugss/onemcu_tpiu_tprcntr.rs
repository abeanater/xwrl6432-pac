#[doc = "Register `ONEMCU_TPIU_TPRCNTR` reader"]
pub type R = crate::R<OnemcuTpiuTprcntrSpec>;
#[doc = "Register `ONEMCU_TPIU_TPRCNTR` writer"]
pub type W = crate::W<OnemcuTpiuTprcntrSpec>;
#[doc = "Field `ONEMCU_TPIU_TPRCNTR` reader - 31:0\\]
Test pattern repeat counter"]
pub type OnemcuTpiuTprcntrR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_TPRCNTR` writer - 31:0\\]
Test pattern repeat counter"]
pub type OnemcuTpiuTprcntrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Test pattern repeat counter"]
    #[inline(always)]
    pub fn onemcu_tpiu_tprcntr(&self) -> OnemcuTpiuTprcntrR {
        OnemcuTpiuTprcntrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Test pattern repeat counter"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_tprcntr(&mut self) -> OnemcuTpiuTprcntrW<OnemcuTpiuTprcntrSpec> {
        OnemcuTpiuTprcntrW::new(self, 0)
    }
}
#[doc = "Test pattern repeat counter\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_tprcntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_tprcntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuTprcntrSpec;
impl crate::RegisterSpec for OnemcuTpiuTprcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_tprcntr::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuTprcntrSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_tprcntr::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuTprcntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_TPRCNTR to value 0"]
impl crate::Resettable for OnemcuTpiuTprcntrSpec {
    const RESET_VALUE: u32 = 0;
}
