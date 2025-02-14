#[doc = "Register `ONEMCU_TPIU_CPORTSZ` reader"]
pub type R = crate::R<OnemcuTpiuCportszSpec>;
#[doc = "Register `ONEMCU_TPIU_CPORTSZ` writer"]
pub type W = crate::W<OnemcuTpiuCportszSpec>;
#[doc = "Field `ONEMCU_TPIU_CPORTSZ` reader - 31:0\\]
Current port size"]
pub type OnemcuTpiuCportszR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_CPORTSZ` writer - 31:0\\]
Current port size"]
pub type OnemcuTpiuCportszW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current port size"]
    #[inline(always)]
    pub fn onemcu_tpiu_cportsz(&self) -> OnemcuTpiuCportszR {
        OnemcuTpiuCportszR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current port size"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_cportsz(&mut self) -> OnemcuTpiuCportszW<OnemcuTpiuCportszSpec> {
        OnemcuTpiuCportszW::new(self, 0)
    }
}
#[doc = "Current port size\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_cportsz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_cportsz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuCportszSpec;
impl crate::RegisterSpec for OnemcuTpiuCportszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_cportsz::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuCportszSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_cportsz::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuCportszSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_CPORTSZ to value 0"]
impl crate::Resettable for OnemcuTpiuCportszSpec {
    const RESET_VALUE: u32 = 0;
}
