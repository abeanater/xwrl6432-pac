#[doc = "Register `ONEMCU_TPIU_CTSTPTRN` reader"]
pub type R = crate::R<OnemcuTpiuCtstptrnSpec>;
#[doc = "Register `ONEMCU_TPIU_CTSTPTRN` writer"]
pub type W = crate::W<OnemcuTpiuCtstptrnSpec>;
#[doc = "Field `ONEMCU_TPIU_CTSTPTRN` reader - 31:0\\]
Current test pattern/mode"]
pub type OnemcuTpiuCtstptrnR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_CTSTPTRN` writer - 31:0\\]
Current test pattern/mode"]
pub type OnemcuTpiuCtstptrnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current test pattern/mode"]
    #[inline(always)]
    pub fn onemcu_tpiu_ctstptrn(&self) -> OnemcuTpiuCtstptrnR {
        OnemcuTpiuCtstptrnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current test pattern/mode"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_ctstptrn(&mut self) -> OnemcuTpiuCtstptrnW<OnemcuTpiuCtstptrnSpec> {
        OnemcuTpiuCtstptrnW::new(self, 0)
    }
}
#[doc = "Current test pattern/mode\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ctstptrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ctstptrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuCtstptrnSpec;
impl crate::RegisterSpec for OnemcuTpiuCtstptrnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_ctstptrn::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuCtstptrnSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_ctstptrn::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuCtstptrnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_CTSTPTRN to value 0"]
impl crate::Resettable for OnemcuTpiuCtstptrnSpec {
    const RESET_VALUE: u32 = 0;
}
