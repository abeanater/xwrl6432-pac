#[doc = "Register `PSA_SECSIGREGH4` reader"]
pub type R = crate::R<PsaSecsigregh4Spec>;
#[doc = "Register `PSA_SECSIGREGH4` writer"]
pub type W = crate::W<PsaSecsigregh4Spec>;
#[doc = "Field `NU64` reader - 31:0\\]
Reserved"]
pub type Nu64R = crate::FieldReader<u32>;
#[doc = "Field `NU64` writer - 31:0\\]
Reserved"]
pub type Nu64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu64(&self) -> Nu64R {
        Nu64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu64(&mut self) -> Nu64W<PsaSecsigregh4Spec> {
        Nu64W::new(self, 0)
    }
}
#[doc = "Channel 4 PSA sector signature high regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregh4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregh4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSecsigregh4Spec;
impl crate::RegisterSpec for PsaSecsigregh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_secsigregh4::R`](R) reader structure"]
impl crate::Readable for PsaSecsigregh4Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_secsigregh4::W`](W) writer structure"]
impl crate::Writable for PsaSecsigregh4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SECSIGREGH4 to value 0"]
impl crate::Resettable for PsaSecsigregh4Spec {
    const RESET_VALUE: u32 = 0;
}
