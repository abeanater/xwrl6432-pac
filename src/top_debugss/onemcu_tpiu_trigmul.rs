#[doc = "Register `ONEMCU_TPIU_TRIGMUL` reader"]
pub type R = crate::R<OnemcuTpiuTrigmulSpec>;
#[doc = "Register `ONEMCU_TPIU_TRIGMUL` writer"]
pub type W = crate::W<OnemcuTpiuTrigmulSpec>;
#[doc = "Field `ONEMCU_TPIU_TRIGMUL` reader - 31:0\\]
Trigger multiplier"]
pub type OnemcuTpiuTrigmulR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_TRIGMUL` writer - 31:0\\]
Trigger multiplier"]
pub type OnemcuTpiuTrigmulW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Trigger multiplier"]
    #[inline(always)]
    pub fn onemcu_tpiu_trigmul(&self) -> OnemcuTpiuTrigmulR {
        OnemcuTpiuTrigmulR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Trigger multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_trigmul(&mut self) -> OnemcuTpiuTrigmulW<OnemcuTpiuTrigmulSpec> {
        OnemcuTpiuTrigmulW::new(self, 0)
    }
}
#[doc = "Trigger multiplier\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_trigmul::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_trigmul::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuTrigmulSpec;
impl crate::RegisterSpec for OnemcuTpiuTrigmulSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_trigmul::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuTrigmulSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_trigmul::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuTrigmulSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_TRIGMUL to value 0"]
impl crate::Resettable for OnemcuTpiuTrigmulSpec {
    const RESET_VALUE: u32 = 0;
}
