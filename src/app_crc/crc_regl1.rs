#[doc = "Register `CRC_REGL1` reader"]
pub type R = crate::R<CrcRegl1Spec>;
#[doc = "Register `CRC_REGL1` writer"]
pub type W = crate::W<CrcRegl1Spec>;
#[doc = "Field `CRC1_31_0` reader - 31:0\\]
Channel 1 CRC Value Low Register. This register contains the current known good signature value stored at CRC1\\[31:0\\]
regis- ter."]
pub type Crc1_31_0R = crate::FieldReader<u32>;
#[doc = "Field `CRC1_31_0` writer - 31:0\\]
Channel 1 CRC Value Low Register. This register contains the current known good signature value stored at CRC1\\[31:0\\]
regis- ter."]
pub type Crc1_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 CRC Value Low Register. This register contains the current known good signature value stored at CRC1\\[31:0\\]
regis- ter."]
    #[inline(always)]
    pub fn crc1_31_0(&self) -> Crc1_31_0R {
        Crc1_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 CRC Value Low Register. This register contains the current known good signature value stored at CRC1\\[31:0\\]
regis- ter."]
    #[inline(always)]
    #[must_use]
    pub fn crc1_31_0(&mut self) -> Crc1_31_0W<CrcRegl1Spec> {
        Crc1_31_0W::new(self, 0)
    }
}
#[doc = "Channel 1 CRC value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRegl1Spec;
impl crate::RegisterSpec for CrcRegl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_regl1::R`](R) reader structure"]
impl crate::Readable for CrcRegl1Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_regl1::W`](W) writer structure"]
impl crate::Writable for CrcRegl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_REGL1 to value 0"]
impl crate::Resettable for CrcRegl1Spec {
    const RESET_VALUE: u32 = 0;
}
