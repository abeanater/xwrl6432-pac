#[doc = "Register `CRC_REGH1` reader"]
pub type R = crate::R<CrcRegh1Spec>;
#[doc = "Register `CRC_REGH1` writer"]
pub type W = crate::W<CrcRegh1Spec>;
#[doc = "Field `CRC1_63_32` reader - 31:0\\]
Channel 1 CRC Value High Register. This register contains the current known good signature value stored at CRC1\\[63:32\\]
regis- ter."]
pub type Crc1_63_32R = crate::FieldReader<u32>;
#[doc = "Field `CRC1_63_32` writer - 31:0\\]
Channel 1 CRC Value High Register. This register contains the current known good signature value stored at CRC1\\[63:32\\]
regis- ter."]
pub type Crc1_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 CRC Value High Register. This register contains the current known good signature value stored at CRC1\\[63:32\\]
regis- ter."]
    #[inline(always)]
    pub fn crc1_63_32(&self) -> Crc1_63_32R {
        Crc1_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 CRC Value High Register. This register contains the current known good signature value stored at CRC1\\[63:32\\]
regis- ter."]
    #[inline(always)]
    #[must_use]
    pub fn crc1_63_32(&mut self) -> Crc1_63_32W<CrcRegh1Spec> {
        Crc1_63_32W::new(self, 0)
    }
}
#[doc = "Channel 1 CRC value high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regh1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regh1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRegh1Spec;
impl crate::RegisterSpec for CrcRegh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_regh1::R`](R) reader structure"]
impl crate::Readable for CrcRegh1Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_regh1::W`](W) writer structure"]
impl crate::Writable for CrcRegh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_REGH1 to value 0"]
impl crate::Resettable for CrcRegh1Spec {
    const RESET_VALUE: u32 = 0;
}
