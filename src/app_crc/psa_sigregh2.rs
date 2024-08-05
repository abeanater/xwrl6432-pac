#[doc = "Register `PSA_SIGREGH2` reader"]
pub type R = crate::R<PsaSigregh2Spec>;
#[doc = "Register `PSA_SIGREGH2` writer"]
pub type W = crate::W<PsaSigregh2Spec>;
#[doc = "Field `PSA_SIG2_63_32` reader - 31:0\\]
Channel 2 PSA Signature High Register. This register contains the value stored at PSASIG2\\[63:32\\]
register."]
pub type PsaSig2_63_32R = crate::FieldReader<u32>;
#[doc = "Field `PSA_SIG2_63_32` writer - 31:0\\]
Channel 2 PSA Signature High Register. This register contains the value stored at PSASIG2\\[63:32\\]
register."]
pub type PsaSig2_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Signature High Register. This register contains the value stored at PSASIG2\\[63:32\\]
register."]
    #[inline(always)]
    pub fn psa_sig2_63_32(&self) -> PsaSig2_63_32R {
        PsaSig2_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Signature High Register. This register contains the value stored at PSASIG2\\[63:32\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psa_sig2_63_32(&mut self) -> PsaSig2_63_32W<PsaSigregh2Spec> {
        PsaSig2_63_32W::new(self, 0)
    }
}
#[doc = "Channel 2 PSA signature high register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregh2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregh2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSigregh2Spec;
impl crate::RegisterSpec for PsaSigregh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_sigregh2::R`](R) reader structure"]
impl crate::Readable for PsaSigregh2Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_sigregh2::W`](W) writer structure"]
impl crate::Writable for PsaSigregh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SIGREGH2 to value 0"]
impl crate::Resettable for PsaSigregh2Spec {
    const RESET_VALUE: u32 = 0;
}
