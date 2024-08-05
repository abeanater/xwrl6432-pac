#[doc = "Register `ONEMCU_CTI_Claim_Tag_Clear` reader"]
pub type R = crate::R<OnemcuCtiClaimTagClearSpec>;
#[doc = "Register `ONEMCU_CTI_Claim_Tag_Clear` writer"]
pub type W = crate::W<OnemcuCtiClaimTagClearSpec>;
#[doc = "Field `ONEMCU_CTI_Claim_Tag_Clear` reader - "]
pub type OnemcuCtiClaimTagClearR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_Claim_Tag_Clear` writer - "]
pub type OnemcuCtiClaimTagClearW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn onemcu_cti_claim_tag_clear(&self) -> OnemcuCtiClaimTagClearR {
        OnemcuCtiClaimTagClearR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_claim_tag_clear(
        &mut self,
    ) -> OnemcuCtiClaimTagClearW<OnemcuCtiClaimTagClearSpec> {
        OnemcuCtiClaimTagClearW::new(self, 0)
    }
}
#[doc = "ONEMCU_CTI_Claim_Tag_Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_claim_tag_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_claim_tag_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiClaimTagClearSpec;
impl crate::RegisterSpec for OnemcuCtiClaimTagClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_claim_tag_clear::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiClaimTagClearSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_claim_tag_clear::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiClaimTagClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_Claim_Tag_Clear to value 0"]
impl crate::Resettable for OnemcuCtiClaimTagClearSpec {
    const RESET_VALUE: u32 = 0;
}
