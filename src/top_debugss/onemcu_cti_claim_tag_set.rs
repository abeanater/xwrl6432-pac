#[doc = "Register `ONEMCU_CTI_Claim_Tag_Set` reader"]
pub type R = crate::R<OnemcuCtiClaimTagSetSpec>;
#[doc = "Register `ONEMCU_CTI_Claim_Tag_Set` writer"]
pub type W = crate::W<OnemcuCtiClaimTagSetSpec>;
#[doc = "Field `ONEMCU_CTI_Claim_Tag_Set` reader - "]
pub type OnemcuCtiClaimTagSetR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_Claim_Tag_Set` writer - "]
pub type OnemcuCtiClaimTagSetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_claim_tag_set(&self) -> OnemcuCtiClaimTagSetR {
        OnemcuCtiClaimTagSetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_claim_tag_set(&mut self) -> OnemcuCtiClaimTagSetW<OnemcuCtiClaimTagSetSpec> {
        OnemcuCtiClaimTagSetW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_Claim_Tag_Set\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_claim_tag_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_claim_tag_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiClaimTagSetSpec;
impl crate::RegisterSpec for OnemcuCtiClaimTagSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_claim_tag_set::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiClaimTagSetSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_claim_tag_set::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiClaimTagSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_Claim_Tag_Set to value 0"]
impl crate::Resettable for OnemcuCtiClaimTagSetSpec {
    const RESET_VALUE: u32 = 0;
}
