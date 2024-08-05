#[doc = "Register `ONEMCU_TPIU_CLAIMCLR` reader"]
pub type R = crate::R<OnemcuTpiuClaimclrSpec>;
#[doc = "Register `ONEMCU_TPIU_CLAIMCLR` writer"]
pub type W = crate::W<OnemcuTpiuClaimclrSpec>;
#[doc = "Field `ONEMCU_TPIU_CLAIMCLR` reader - 31:0\\]
Claim Tag Clear"]
pub type OnemcuTpiuClaimclrR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_CLAIMCLR` writer - 31:0\\]
Claim Tag Clear"]
pub type OnemcuTpiuClaimclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim Tag Clear"]
    #[inline(always)]
    pub fn onemcu_tpiu_claimclr(&self) -> OnemcuTpiuClaimclrR {
        OnemcuTpiuClaimclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim Tag Clear"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_claimclr(&mut self) -> OnemcuTpiuClaimclrW<OnemcuTpiuClaimclrSpec> {
        OnemcuTpiuClaimclrW::new(self, 0)
    }
}
#[doc = "Claim Tag Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_claimclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_claimclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuClaimclrSpec;
impl crate::RegisterSpec for OnemcuTpiuClaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_claimclr::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuClaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_claimclr::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuClaimclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_CLAIMCLR to value 0"]
impl crate::Resettable for OnemcuTpiuClaimclrSpec {
    const RESET_VALUE: u32 = 0;
}
