#[doc = "Register `PSA_SIGREGH4` reader"]
pub type R = crate::R<PsaSigregh4Spec>;
#[doc = "Register `PSA_SIGREGH4` writer"]
pub type W = crate::W<PsaSigregh4Spec>;
#[doc = "Field `NU60` reader - 31:0\\]
Reserved"]
pub type Nu60R = crate::FieldReader<u32>;
#[doc = "Field `NU60` writer - 31:0\\]
Reserved"]
pub type Nu60W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu60(&self) -> Nu60R {
        Nu60R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu60(&mut self) -> Nu60W<PsaSigregh4Spec> {
        Nu60W::new(self, 0)
    }
}
#[doc = "Channel 4 PSA signature high register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregh4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregh4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSigregh4Spec;
impl crate::RegisterSpec for PsaSigregh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_sigregh4::R`](R) reader structure"]
impl crate::Readable for PsaSigregh4Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_sigregh4::W`](W) writer structure"]
impl crate::Writable for PsaSigregh4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SIGREGH4 to value 0"]
impl crate::Resettable for PsaSigregh4Spec {
    const RESET_VALUE: u32 = 0;
}
