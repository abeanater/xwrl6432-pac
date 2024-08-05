#[doc = "Register `ONEMCU_TPIU_LAR` reader"]
pub type R = crate::R<OnemcuTpiuLarSpec>;
#[doc = "Register `ONEMCU_TPIU_LAR` writer"]
pub type W = crate::W<OnemcuTpiuLarSpec>;
#[doc = "Field `ONEMCU_TPIU_LAR` reader - 31:0\\]
Lock status"]
pub type OnemcuTpiuLarR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_LAR` writer - 31:0\\]
Lock status"]
pub type OnemcuTpiuLarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Lock status"]
    #[inline(always)]
    pub fn onemcu_tpiu_lar(&self) -> OnemcuTpiuLarR {
        OnemcuTpiuLarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Lock status"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_lar(&mut self) -> OnemcuTpiuLarW<OnemcuTpiuLarSpec> {
        OnemcuTpiuLarW::new(self, 0)
    }
}
#[doc = "Lock status\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_lar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_lar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuLarSpec;
impl crate::RegisterSpec for OnemcuTpiuLarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_lar::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuLarSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_lar::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuLarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_LAR to value 0"]
impl crate::Resettable for OnemcuTpiuLarSpec {
    const RESET_VALUE: u32 = 0;
}
