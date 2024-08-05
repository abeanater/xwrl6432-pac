#[doc = "Register `CRC_REGL3` reader"]
pub type R = crate::R<CrcRegl3Spec>;
#[doc = "Register `CRC_REGL3` writer"]
pub type W = crate::W<CrcRegl3Spec>;
#[doc = "Field `NU48` reader - 31:0\\]
Reserved"]
pub type Nu48R = crate::FieldReader<u32>;
#[doc = "Field `NU48` writer - 31:0\\]
Reserved"]
pub type Nu48W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu48(&self) -> Nu48R {
        Nu48R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu48(&mut self) -> Nu48W<CrcRegl3Spec> {
        Nu48W::new(self, 0)
    }
}
#[doc = "Channel 3 CRC value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRegl3Spec;
impl crate::RegisterSpec for CrcRegl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_regl3::R`](R) reader structure"]
impl crate::Readable for CrcRegl3Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_regl3::W`](W) writer structure"]
impl crate::Writable for CrcRegl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_REGL3 to value 0"]
impl crate::Resettable for CrcRegl3Spec {
    const RESET_VALUE: u32 = 0;
}
