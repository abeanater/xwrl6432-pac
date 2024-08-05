#[doc = "Register `CRC_REGH3` reader"]
pub type R = crate::R<CrcRegh3Spec>;
#[doc = "Register `CRC_REGH3` writer"]
pub type W = crate::W<CrcRegh3Spec>;
#[doc = "Field `NU49` reader - 31:0\\]
Reserved"]
pub type Nu49R = crate::FieldReader<u32>;
#[doc = "Field `NU49` writer - 31:0\\]
Reserved"]
pub type Nu49W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu49(&self) -> Nu49R {
        Nu49R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu49(&mut self) -> Nu49W<CrcRegh3Spec> {
        Nu49W::new(self, 0)
    }
}
#[doc = "Channel 3 CRC value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regh3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regh3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRegh3Spec;
impl crate::RegisterSpec for CrcRegh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_regh3::R`](R) reader structure"]
impl crate::Readable for CrcRegh3Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_regh3::W`](W) writer structure"]
impl crate::Writable for CrcRegh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_REGH3 to value 0"]
impl crate::Resettable for CrcRegh3Spec {
    const RESET_VALUE: u32 = 0;
}
