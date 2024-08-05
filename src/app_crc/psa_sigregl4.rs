#[doc = "Register `PSA_SIGREGL4` reader"]
pub type R = crate::R<PsaSigregl4Spec>;
#[doc = "Register `PSA_SIGREGL4` writer"]
pub type W = crate::W<PsaSigregl4Spec>;
#[doc = "Field `NU59` reader - 31:0\\]
Reserved"]
pub type Nu59R = crate::FieldReader<u32>;
#[doc = "Field `NU59` writer - 31:0\\]
Reserved"]
pub type Nu59W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu59(&self) -> Nu59R {
        Nu59R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu59(&mut self) -> Nu59W<PsaSigregl4Spec> {
        Nu59W::new(self, 0)
    }
}
#[doc = "Channel 4 PSA signature low register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSigregl4Spec;
impl crate::RegisterSpec for PsaSigregl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_sigregl4::R`](R) reader structure"]
impl crate::Readable for PsaSigregl4Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_sigregl4::W`](W) writer structure"]
impl crate::Writable for PsaSigregl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SIGREGL4 to value 0"]
impl crate::Resettable for PsaSigregl4Spec {
    const RESET_VALUE: u32 = 0;
}
