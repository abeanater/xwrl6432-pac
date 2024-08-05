#[doc = "Register `PSA_SECSIGREGH2` reader"]
pub type R = crate::R<PsaSecsigregh2Spec>;
#[doc = "Register `PSA_SECSIGREGH2` writer"]
pub type W = crate::W<PsaSecsigregh2Spec>;
#[doc = "Field `PSASECSIG2_63_32` reader - 31:0\\]
Channel 2 PSA Sector Signature High Register. This register contains the value stored at PSASECSIG2\\[63:32\\]
register."]
pub type Psasecsig2_63_32R = crate::FieldReader<u32>;
#[doc = "Field `PSASECSIG2_63_32` writer - 31:0\\]
Channel 2 PSA Sector Signature High Register. This register contains the value stored at PSASECSIG2\\[63:32\\]
register."]
pub type Psasecsig2_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Sector Signature High Register. This register contains the value stored at PSASECSIG2\\[63:32\\]
register."]
    #[inline(always)]
    pub fn psasecsig2_63_32(&self) -> Psasecsig2_63_32R {
        Psasecsig2_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Sector Signature High Register. This register contains the value stored at PSASECSIG2\\[63:32\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psasecsig2_63_32(&mut self) -> Psasecsig2_63_32W<PsaSecsigregh2Spec> {
        Psasecsig2_63_32W::new(self, 0)
    }
}
#[doc = "Channel 2 PSA sector signature high regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregh2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregh2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSecsigregh2Spec;
impl crate::RegisterSpec for PsaSecsigregh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_secsigregh2::R`](R) reader structure"]
impl crate::Readable for PsaSecsigregh2Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_secsigregh2::W`](W) writer structure"]
impl crate::Writable for PsaSecsigregh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SECSIGREGH2 to value 0"]
impl crate::Resettable for PsaSecsigregh2Spec {
    const RESET_VALUE: u32 = 0;
}
