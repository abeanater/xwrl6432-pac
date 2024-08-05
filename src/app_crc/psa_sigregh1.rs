#[doc = "Register `PSA_SIGREGH1` reader"]
pub type R = crate::R<PsaSigregh1Spec>;
#[doc = "Register `PSA_SIGREGH1` writer"]
pub type W = crate::W<PsaSigregh1Spec>;
#[doc = "Field `PSA_SIG1_63_32` reader - 31:0\\]
Channel 1 PSA Signature High Register. This register contains the value stored at PSASIG1\\[63:32\\]
register."]
pub type PsaSig1_63_32R = crate::FieldReader<u32>;
#[doc = "Field `PSA_SIG1_63_32` writer - 31:0\\]
Channel 1 PSA Signature High Register. This register contains the value stored at PSASIG1\\[63:32\\]
register."]
pub type PsaSig1_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 PSA Signature High Register. This register contains the value stored at PSASIG1\\[63:32\\]
register."]
    #[inline(always)]
    pub fn psa_sig1_63_32(&self) -> PsaSig1_63_32R {
        PsaSig1_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 PSA Signature High Register. This register contains the value stored at PSASIG1\\[63:32\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psa_sig1_63_32(&mut self) -> PsaSig1_63_32W<PsaSigregh1Spec> {
        PsaSig1_63_32W::new(self, 0)
    }
}
#[doc = "Channel 1 PSA signature high register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregh1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregh1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSigregh1Spec;
impl crate::RegisterSpec for PsaSigregh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_sigregh1::R`](R) reader structure"]
impl crate::Readable for PsaSigregh1Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_sigregh1::W`](W) writer structure"]
impl crate::Writable for PsaSigregh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SIGREGH1 to value 0"]
impl crate::Resettable for PsaSigregh1Spec {
    const RESET_VALUE: u32 = 0;
}
