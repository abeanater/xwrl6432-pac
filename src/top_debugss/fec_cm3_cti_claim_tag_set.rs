#[doc = "Register `FEC_CM3_CTI_Claim_Tag_Set` reader"]
pub type R = crate::R<FecCm3CtiClaimTagSetSpec>;
#[doc = "Register `FEC_CM3_CTI_Claim_Tag_Set` writer"]
pub type W = crate::W<FecCm3CtiClaimTagSetSpec>;
#[doc = "Field `FEC_CM3_CTI_Claim_Tag_Set` reader - "]
pub type FecCm3CtiClaimTagSetR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_Claim_Tag_Set` writer - "]
pub type FecCm3CtiClaimTagSetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_claim_tag_set(&self) -> FecCm3CtiClaimTagSetR {
        FecCm3CtiClaimTagSetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_claim_tag_set(&mut self) -> FecCm3CtiClaimTagSetW<FecCm3CtiClaimTagSetSpec> {
        FecCm3CtiClaimTagSetW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_Claim_Tag_Set\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_claim_tag_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_claim_tag_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiClaimTagSetSpec;
impl crate::RegisterSpec for FecCm3CtiClaimTagSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_claim_tag_set::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiClaimTagSetSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_claim_tag_set::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiClaimTagSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_Claim_Tag_Set to value 0"]
impl crate::Resettable for FecCm3CtiClaimTagSetSpec {
    const RESET_VALUE: u32 = 0;
}
