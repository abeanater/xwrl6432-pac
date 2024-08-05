#[doc = "Register `ONEMCU_TPIU_STRIGM` reader"]
pub type R = crate::R<OnemcuTpiuStrigmSpec>;
#[doc = "Register `ONEMCU_TPIU_STRIGM` writer"]
pub type W = crate::W<OnemcuTpiuStrigmSpec>;
#[doc = "Field `ONEMCU_TPIU_STRIGM` reader - 31:0\\]
Supported trigger modes"]
pub type OnemcuTpiuStrigmR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_STRIGM` writer - 31:0\\]
Supported trigger modes"]
pub type OnemcuTpiuStrigmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Supported trigger modes"]
    #[inline(always)]
    pub fn onemcu_tpiu_strigm(&self) -> OnemcuTpiuStrigmR {
        OnemcuTpiuStrigmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Supported trigger modes"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_strigm(&mut self) -> OnemcuTpiuStrigmW<OnemcuTpiuStrigmSpec> {
        OnemcuTpiuStrigmW::new(self, 0)
    }
}
#[doc = "Supported trigger modes\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_strigm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_strigm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuStrigmSpec;
impl crate::RegisterSpec for OnemcuTpiuStrigmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_strigm::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuStrigmSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_strigm::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuStrigmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_STRIGM to value 0"]
impl crate::Resettable for OnemcuTpiuStrigmSpec {
    const RESET_VALUE: u32 = 0;
}
