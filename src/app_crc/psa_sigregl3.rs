#[doc = "Register `PSA_SIGREGL3` reader"]
pub type R = crate::R<PsaSigregl3Spec>;
#[doc = "Register `PSA_SIGREGL3` writer"]
pub type W = crate::W<PsaSigregl3Spec>;
#[doc = "Field `NU46` reader - 31:0\\]
Reserved"]
pub type Nu46R = crate::FieldReader<u32>;
#[doc = "Field `NU46` writer - 31:0\\]
Reserved"]
pub type Nu46W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu46(&self) -> Nu46R {
        Nu46R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu46(&mut self) -> Nu46W<PsaSigregl3Spec> {
        Nu46W::new(self, 0)
    }
}
#[doc = "Channel 3 PSA signature low register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSigregl3Spec;
impl crate::RegisterSpec for PsaSigregl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_sigregl3::R`](R) reader structure"]
impl crate::Readable for PsaSigregl3Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_sigregl3::W`](W) writer structure"]
impl crate::Writable for PsaSigregl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SIGREGL3 to value 0"]
impl crate::Resettable for PsaSigregl3Spec {
    const RESET_VALUE: u32 = 0;
}
