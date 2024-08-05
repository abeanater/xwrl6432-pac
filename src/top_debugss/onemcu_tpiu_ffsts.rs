#[doc = "Register `ONEMCU_TPIU_FFSTS` reader"]
pub type R = crate::R<OnemcuTpiuFfstsSpec>;
#[doc = "Register `ONEMCU_TPIU_FFSTS` writer"]
pub type W = crate::W<OnemcuTpiuFfstsSpec>;
#[doc = "Field `ONEMCU_TPIU_FFSTS` reader - 31:0\\]
Formatter and flush status"]
pub type OnemcuTpiuFfstsR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_FFSTS` writer - 31:0\\]
Formatter and flush status"]
pub type OnemcuTpiuFfstsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Formatter and flush status"]
    #[inline(always)]
    pub fn onemcu_tpiu_ffsts(&self) -> OnemcuTpiuFfstsR {
        OnemcuTpiuFfstsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Formatter and flush status"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_ffsts(&mut self) -> OnemcuTpiuFfstsW<OnemcuTpiuFfstsSpec> {
        OnemcuTpiuFfstsW::new(self, 0)
    }
}
#[doc = "Formatter and flush status\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ffsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ffsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuFfstsSpec;
impl crate::RegisterSpec for OnemcuTpiuFfstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_ffsts::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuFfstsSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_ffsts::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuFfstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_FFSTS to value 0"]
impl crate::Resettable for OnemcuTpiuFfstsSpec {
    const RESET_VALUE: u32 = 0;
}
