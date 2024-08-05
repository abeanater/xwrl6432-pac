#[doc = "Register `PSA_SIGREGL2` reader"]
pub type R = crate::R<PsaSigregl2Spec>;
#[doc = "Register `PSA_SIGREGL2` writer"]
pub type W = crate::W<PsaSigregl2Spec>;
#[doc = "Field `PSASIG2_31_0` reader - 31:0\\]
Channel 2 PSA Signature Low Register. This register contains the value stored at PSASIG2\\[31:0\\]
register."]
pub type Psasig2_31_0R = crate::FieldReader<u32>;
#[doc = "Field `PSASIG2_31_0` writer - 31:0\\]
Channel 2 PSA Signature Low Register. This register contains the value stored at PSASIG2\\[31:0\\]
register."]
pub type Psasig2_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Signature Low Register. This register contains the value stored at PSASIG2\\[31:0\\]
register."]
    #[inline(always)]
    pub fn psasig2_31_0(&self) -> Psasig2_31_0R {
        Psasig2_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Signature Low Register. This register contains the value stored at PSASIG2\\[31:0\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psasig2_31_0(&mut self) -> Psasig2_31_0W<PsaSigregl2Spec> {
        Psasig2_31_0W::new(self, 0)
    }
}
#[doc = "Channel 2 PSA signature low register\n\nYou can [`read`](crate::Reg::read) this register and get [`psa_sigregl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psa_sigregl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsaSigregl2Spec;
impl crate::RegisterSpec for PsaSigregl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psa_sigregl2::R`](R) reader structure"]
impl crate::Readable for PsaSigregl2Spec {}
#[doc = "`write(|w| ..)` method takes [`psa_sigregl2::W`](W) writer structure"]
impl crate::Writable for PsaSigregl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSA_SIGREGL2 to value 0"]
impl crate::Resettable for PsaSigregl2Spec {
    const RESET_VALUE: u32 = 0;
}
