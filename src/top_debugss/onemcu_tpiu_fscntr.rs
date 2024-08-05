#[doc = "Register `ONEMCU_TPIU_FSCNTR` reader"]
pub type R = crate::R<OnemcuTpiuFscntrSpec>;
#[doc = "Register `ONEMCU_TPIU_FSCNTR` writer"]
pub type W = crate::W<OnemcuTpiuFscntrSpec>;
#[doc = "Field `ONEMCU_TPIU_FSCNTR` reader - 31:0\\]
Formatter synchronization counter"]
pub type OnemcuTpiuFscntrR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_FSCNTR` writer - 31:0\\]
Formatter synchronization counter"]
pub type OnemcuTpiuFscntrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Formatter synchronization counter"]
    #[inline(always)]
    pub fn onemcu_tpiu_fscntr(&self) -> OnemcuTpiuFscntrR {
        OnemcuTpiuFscntrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Formatter synchronization counter"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_fscntr(&mut self) -> OnemcuTpiuFscntrW<OnemcuTpiuFscntrSpec> {
        OnemcuTpiuFscntrW::new(self, 0)
    }
}
#[doc = "Formatter synchronization counter\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_fscntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_fscntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuFscntrSpec;
impl crate::RegisterSpec for OnemcuTpiuFscntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_fscntr::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuFscntrSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_fscntr::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuFscntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_FSCNTR to value 0"]
impl crate::Resettable for OnemcuTpiuFscntrSpec {
    const RESET_VALUE: u32 = 0;
}
