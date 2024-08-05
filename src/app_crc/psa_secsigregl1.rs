#[doc = "Register `PSA_SECSIGREGL1` reader"]
pub type R = crate::R<PsaSecsigregl1Spec>;
#[doc = "Register `PSA_SECSIGREGL1` writer"]
pub type W = crate::W<PsaSecsigregl1Spec>;
#[doc = "Field `PSASECSIG1_31_0` reader - 31:0\\]
Channel 1 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG1\\[31:0\\]
register."]
pub type Psasecsig1_31_0R = crate::FieldReader<u32>;
#[doc = "Field `PSASECSIG1_31_0` writer - 31:0\\]
Channel 1 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG1\\[31:0\\]
register."]
pub type Psasecsig1_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG1\\[31:0\\]
register."]
    #[inline(always)]
    pub fn psasecsig1_31_0(&self) -> Psasecsig1_31_0R {
        Psasecsig1_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG1\\[31:0\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psasecsig1_31_0(&mut self) -> Psasecsig1_31_0W<PsaSecsigregl1Spec> {
        Psasecsig1_31_0W::new(self, 0)
    }
}
#[doc = "Channel 1 PSA sector signature low regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSecsigregl1Spec;
impl crate::RegisterSpec for PsaSecsigregl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_secsigregl1::R`](R) reader structure"]
impl crate::Readable for PsaSecsigregl1Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_secsigregl1::W`](W) writer structure"]
impl crate::Writable for PsaSecsigregl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SECSIGREGL1 to value 0"]
impl crate::Resettable for PsaSecsigregl1Spec {
    const RESET_VALUE: u32 = 0;
}
