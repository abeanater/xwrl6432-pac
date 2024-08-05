#[doc = "Register `PSA_SECSIGREGL3` reader"]
pub type R = crate::R<PsaSecsigregl3Spec>;
#[doc = "Register `PSA_SECSIGREGL3` writer"]
pub type W = crate::W<PsaSecsigregl3Spec>;
#[doc = "Field `NU50` reader - 31:0\\]
Reserved"]
pub type Nu50R = crate::FieldReader<u32>;
#[doc = "Field `NU50` writer - 31:0\\]
Reserved"]
pub type Nu50W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu50(&self) -> Nu50R {
        Nu50R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu50(&mut self) -> Nu50W<PsaSecsigregl3Spec> {
        Nu50W::new(self, 0)
    }
}
#[doc = "Channel 3 PSA sector signature low regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSecsigregl3Spec;
impl crate::RegisterSpec for PsaSecsigregl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_secsigregl3::R`](R) reader structure"]
impl crate::Readable for PsaSecsigregl3Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_secsigregl3::W`](W) writer structure"]
impl crate::Writable for PsaSecsigregl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SECSIGREGL3 to value 0"]
impl crate::Resettable for PsaSecsigregl3Spec {
    const RESET_VALUE: u32 = 0;
}
