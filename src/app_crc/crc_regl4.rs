#[doc = "Register `CRC_REGL4` reader"]
pub type R = crate::R<CrcRegl4Spec>;
#[doc = "Register `CRC_REGL4` writer"]
pub type W = crate::W<CrcRegl4Spec>;
#[doc = "Field `NU61` reader - 31:0\\]
Reserved"]
pub type Nu61R = crate::FieldReader<u32>;
#[doc = "Field `NU61` writer - 31:0\\]
Reserved"]
pub type Nu61W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu61(&self) -> Nu61R {
        Nu61R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu61(&mut self) -> Nu61W<CrcRegl4Spec> {
        Nu61W::new(self, 0)
    }
}
#[doc = "Channel 4 CRC value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRegl4Spec;
impl crate::RegisterSpec for CrcRegl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_regl4::R`](R) reader structure"]
impl crate::Readable for CrcRegl4Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_regl4::W`](W) writer structure"]
impl crate::Writable for CrcRegl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_REGL4 to value 0"]
impl crate::Resettable for CrcRegl4Spec {
    const RESET_VALUE: u32 = 0;
}
