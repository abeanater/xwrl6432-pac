#[doc = "Register `PSA_SIGREGH3` reader"]
pub type R = crate::R<PsaSigregh3Spec>;
#[doc = "Register `PSA_SIGREGH3` writer"]
pub type W = crate::W<PsaSigregh3Spec>;
#[doc = "Field `NU47` reader - 31:0\\]
Reserved"]
pub type Nu47R = crate::FieldReader<u32>;
#[doc = "Field `NU47` writer - 31:0\\]
Reserved"]
pub type Nu47W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu47(&self) -> Nu47R {
        Nu47R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu47(&mut self) -> Nu47W<PsaSigregh3Spec> {
        Nu47W::new(self, 0)
    }
}
#[doc = "Channel 3 PSA signature high register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregh3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregh3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSigregh3Spec;
impl crate::RegisterSpec for PsaSigregh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_sigregh3::R`](R) reader structure"]
impl crate::Readable for PsaSigregh3Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_sigregh3::W`](W) writer structure"]
impl crate::Writable for PsaSigregh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SIGREGH3 to value 0"]
impl crate::Resettable for PsaSigregh3Spec {
    const RESET_VALUE: u32 = 0;
}
