#[doc = "Register `ONEMCU_TPIU_CLAIMSET` reader"]
pub type R = crate::R<OnemcuTpiuClaimsetSpec>;
#[doc = "Register `ONEMCU_TPIU_CLAIMSET` writer"]
pub type W = crate::W<OnemcuTpiuClaimsetSpec>;
#[doc = "Field `ONEMCU_TPIU_CLAIMSET` reader - 31:0\\]
Claim Tag Set"]
pub type OnemcuTpiuClaimsetR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_CLAIMSET` writer - 31:0\\]
Claim Tag Set"]
pub type OnemcuTpiuClaimsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim Tag Set"]
    #[inline(always)]
    pub fn onemcu_tpiu_claimset(&self) -> OnemcuTpiuClaimsetR {
        OnemcuTpiuClaimsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim Tag Set"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_claimset(&mut self) -> OnemcuTpiuClaimsetW<OnemcuTpiuClaimsetSpec> {
        OnemcuTpiuClaimsetW::new(self, 0)
    }
}
#[doc = "Claim Tag Set\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_claimset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_claimset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuClaimsetSpec;
impl crate::RegisterSpec for OnemcuTpiuClaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_claimset::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuClaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_claimset::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuClaimsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_CLAIMSET to value 0"]
impl crate::Resettable for OnemcuTpiuClaimsetSpec {
    const RESET_VALUE: u32 = 0;
}
