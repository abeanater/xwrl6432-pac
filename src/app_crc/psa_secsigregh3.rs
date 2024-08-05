#[doc = "Register `PSA_SECSIGREGH3` reader"]
pub type R = crate::R<PsaSecsigregh3Spec>;
#[doc = "Register `PSA_SECSIGREGH3` writer"]
pub type W = crate::W<PsaSecsigregh3Spec>;
#[doc = "Field `NU51` reader - 31:0\\]
Reserved"]
pub type Nu51R = crate::FieldReader<u32>;
#[doc = "Field `NU51` writer - 31:0\\]
Reserved"]
pub type Nu51W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu51(&self) -> Nu51R {
        Nu51R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu51(&mut self) -> Nu51W<PsaSecsigregh3Spec> {
        Nu51W::new(self, 0)
    }
}
#[doc = "Channel 3 PSA sector signature high regis-ter\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_secsigregh3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_secsigregh3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSecsigregh3Spec;
impl crate::RegisterSpec for PsaSecsigregh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_secsigregh3::R`](R) reader structure"]
impl crate::Readable for PsaSecsigregh3Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_secsigregh3::W`](W) writer structure"]
impl crate::Writable for PsaSecsigregh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SECSIGREGH3 to value 0"]
impl crate::Resettable for PsaSecsigregh3Spec {
    const RESET_VALUE: u32 = 0;
}
