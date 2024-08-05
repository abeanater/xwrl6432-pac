#[doc = "Register `PSA_SECSIGREGL2` reader"]
pub type R = crate::R<PsaSecsigregl2Spec>;
#[doc = "Register `PSA_SECSIGREGL2` writer"]
pub type W = crate::W<PsaSecsigregl2Spec>;
#[doc = "Field `PSASECSIG2_31_0` reader - 31:0\\]
Channel 2 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG2\\[31:0\\]
register."]
pub type Psasecsig2_31_0R = crate::FieldReader<u32>;
#[doc = "Field `PSASECSIG2_31_0` writer - 31:0\\]
Channel 2 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG2\\[31:0\\]
register."]
pub type Psasecsig2_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG2\\[31:0\\]
register."]
    #[inline(always)]
    pub fn psasecsig2_31_0(&self) -> Psasecsig2_31_0R {
        Psasecsig2_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG2\\[31:0\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psasecsig2_31_0(&mut self) -> Psasecsig2_31_0W<PsaSecsigregl2Spec> {
        Psasecsig2_31_0W::new(self, 0)
    }
}
#[doc = "Channel 2 PSA sector signature low regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSecsigregl2Spec;
impl crate::RegisterSpec for PsaSecsigregl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_secsigregl2::R`](R) reader structure"]
impl crate::Readable for PsaSecsigregl2Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_secsigregl2::W`](W) writer structure"]
impl crate::Writable for PsaSecsigregl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SECSIGREGL2 to value 0"]
impl crate::Resettable for PsaSecsigregl2Spec {
    const RESET_VALUE: u32 = 0;
}
