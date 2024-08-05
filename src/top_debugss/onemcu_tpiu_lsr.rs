#[doc = "Register `ONEMCU_TPIU_LSR` reader"]
pub type R = crate::R<OnemcuTpiuLsrSpec>;
#[doc = "Register `ONEMCU_TPIU_LSR` writer"]
pub type W = crate::W<OnemcuTpiuLsrSpec>;
#[doc = "Field `ONEMCU_TPIU_LSR` reader - 31:0\\]
Lock Access"]
pub type OnemcuTpiuLsrR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_LSR` writer - 31:0\\]
Lock Access"]
pub type OnemcuTpiuLsrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Lock Access"]
    #[inline(always)]
    pub fn onemcu_tpiu_lsr(&self) -> OnemcuTpiuLsrR {
        OnemcuTpiuLsrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Lock Access"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_lsr(&mut self) -> OnemcuTpiuLsrW<OnemcuTpiuLsrSpec> {
        OnemcuTpiuLsrW::new(self, 0)
    }
}
#[doc = "Lock Access\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_lsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_lsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuLsrSpec;
impl crate::RegisterSpec for OnemcuTpiuLsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_lsr::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuLsrSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_lsr::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuLsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_LSR to value 0"]
impl crate::Resettable for OnemcuTpiuLsrSpec {
    const RESET_VALUE: u32 = 0;
}
