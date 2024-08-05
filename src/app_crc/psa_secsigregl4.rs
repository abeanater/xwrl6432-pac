#[doc = "Register `PSA_SECSIGREGL4` reader"]
pub type R = crate::R<PsaSecsigregl4Spec>;
#[doc = "Register `PSA_SECSIGREGL4` writer"]
pub type W = crate::W<PsaSecsigregl4Spec>;
#[doc = "Field `NU63` reader - 31:0\\]
Reserved"]
pub type Nu63R = crate::FieldReader<u32>;
#[doc = "Field `NU63` writer - 31:0\\]
Reserved"]
pub type Nu63W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu63(&self) -> Nu63R {
        Nu63R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu63(&mut self) -> Nu63W<PsaSecsigregl4Spec> {
        Nu63W::new(self, 0)
    }
}
#[doc = "Channel 4 PSA sector signature low regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSecsigregl4Spec;
impl crate::RegisterSpec for PsaSecsigregl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_secsigregl4::R`](R) reader structure"]
impl crate::Readable for PsaSecsigregl4Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_secsigregl4::W`](W) writer structure"]
impl crate::Writable for PsaSecsigregl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SECSIGREGL4 to value 0"]
impl crate::Resettable for PsaSecsigregl4Spec {
    const RESET_VALUE: u32 = 0;
}
