#[doc = "Register `PLATFORM_SIGNATURE` reader"]
pub type R = crate::R<PlatformSignatureSpec>;
#[doc = "Register `PLATFORM_SIGNATURE` writer"]
pub type W = crate::W<PlatformSignatureSpec>;
#[doc = "Field `signature` reader - 31:0\\]
Platform signature to identify the platform"]
pub type SignatureR = crate::FieldReader<u32>;
#[doc = "Field `signature` writer - 31:0\\]
Platform signature to identify the platform"]
pub type SignatureW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Platform signature to identify the platform"]
    #[inline(always)]
    pub fn signature(&self) -> SignatureR {
        SignatureR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Platform signature to identify the platform"]
    #[inline(always)]
    #[must_use]
    pub fn signature(&mut self) -> SignatureW<PlatformSignatureSpec> {
        SignatureW::new(self, 0)
    }
}
#[doc = "PLATFORM_SIGNATURE\n\nYou can [`read`](crate::Reg::read) this register and get [`platform_signature::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`platform_signature::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlatformSignatureSpec;
impl crate::RegisterSpec for PlatformSignatureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`platform_signature::R`](R) reader structure"]
impl crate::Readable for PlatformSignatureSpec {}
#[doc = "`write(|w| ..)` method takes [`platform_signature::W`](W) writer structure"]
impl crate::Writable for PlatformSignatureSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLATFORM_SIGNATURE to value 0"]
impl crate::Resettable for PlatformSignatureSpec {
    const RESET_VALUE: u32 = 0;
}
