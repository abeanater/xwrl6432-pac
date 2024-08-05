#[doc = "Register `CRC_REGH4` reader"]
pub type R = crate::R<CrcRegh4Spec>;
#[doc = "Register `CRC_REGH4` writer"]
pub type W = crate::W<CrcRegh4Spec>;
#[doc = "Field `NU62` reader - 31:0\\]
Reserved"]
pub type Nu62R = crate::FieldReader<u32>;
#[doc = "Field `NU62` writer - 31:0\\]
Reserved"]
pub type Nu62W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu62(&self) -> Nu62R {
        Nu62R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu62(&mut self) -> Nu62W<CrcRegh4Spec> {
        Nu62W::new(self, 0)
    }
}
#[doc = "Channel 4 CRC value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regh4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regh4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRegh4Spec;
impl crate::RegisterSpec for CrcRegh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_regh4::R`](R) reader structure"]
impl crate::Readable for CrcRegh4Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_regh4::W`](W) writer structure"]
impl crate::Writable for CrcRegh4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_REGH4 to value 0"]
impl crate::Resettable for CrcRegh4Spec {
    const RESET_VALUE: u32 = 0;
}
