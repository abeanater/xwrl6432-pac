#[doc = "Register `ONEMCU_TPIU_STSTPTRN` reader"]
pub type R = crate::R<OnemcuTpiuStstptrnSpec>;
#[doc = "Register `ONEMCU_TPIU_STSTPTRN` writer"]
pub type W = crate::W<OnemcuTpiuStstptrnSpec>;
#[doc = "Field `ONEMCU_TPIU_STSTPTRN` reader - 31:0\\]
Supported test pattern/modes"]
pub type OnemcuTpiuStstptrnR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_STSTPTRN` writer - 31:0\\]
Supported test pattern/modes"]
pub type OnemcuTpiuStstptrnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Supported test pattern/modes"]
    #[inline(always)]
    pub fn onemcu_tpiu_ststptrn(&self) -> OnemcuTpiuStstptrnR {
        OnemcuTpiuStstptrnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Supported test pattern/modes"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_ststptrn(&mut self) -> OnemcuTpiuStstptrnW<OnemcuTpiuStstptrnSpec> {
        OnemcuTpiuStstptrnW::new(self, 0)
    }
}
#[doc = "Supported test pattern/modes\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_ststptrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_ststptrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuStstptrnSpec;
impl crate::RegisterSpec for OnemcuTpiuStstptrnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_ststptrn::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuStstptrnSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_ststptrn::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuStstptrnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_STSTPTRN to value 0"]
impl crate::Resettable for OnemcuTpiuStstptrnSpec {
    const RESET_VALUE: u32 = 0;
}
