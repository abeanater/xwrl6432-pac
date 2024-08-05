#[doc = "Register `ONEMCU_TPIU_SPORTSZ` reader"]
pub type R = crate::R<OnemcuTpiuSportszSpec>;
#[doc = "Register `ONEMCU_TPIU_SPORTSZ` writer"]
pub type W = crate::W<OnemcuTpiuSportszSpec>;
#[doc = "Field `ONEMCU_TPIU_SPORTSZ` reader - 31:0\\]
Supported port sizes"]
pub type OnemcuTpiuSportszR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_SPORTSZ` writer - 31:0\\]
Supported port sizes"]
pub type OnemcuTpiuSportszW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Supported port sizes"]
    #[inline(always)]
    pub fn onemcu_tpiu_sportsz(&self) -> OnemcuTpiuSportszR {
        OnemcuTpiuSportszR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Supported port sizes"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_sportsz(&mut self) -> OnemcuTpiuSportszW<OnemcuTpiuSportszSpec> {
        OnemcuTpiuSportszW::new(self, 0)
    }
}
#[doc = "Supported port sizes\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_sportsz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_sportsz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuSportszSpec;
impl crate::RegisterSpec for OnemcuTpiuSportszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_sportsz::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuSportszSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_sportsz::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuSportszSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_SPORTSZ to value 0"]
impl crate::Resettable for OnemcuTpiuSportszSpec {
    const RESET_VALUE: u32 = 0;
}
