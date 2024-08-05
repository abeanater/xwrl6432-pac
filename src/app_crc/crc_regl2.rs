#[doc = "Register `CRC_REGL2` reader"]
pub type R = crate::R<CrcRegl2Spec>;
#[doc = "Register `CRC_REGL2` writer"]
pub type W = crate::W<CrcRegl2Spec>;
#[doc = "Field `CRC2_31_0` reader - 31:0\\]
Channel 2 CRC Value Low Register. This register contains the current known good signature value stored at CRC2\\[31:0\\]
regis- ter."]
pub type Crc2_31_0R = crate::FieldReader<u32>;
#[doc = "Field `CRC2_31_0` writer - 31:0\\]
Channel 2 CRC Value Low Register. This register contains the current known good signature value stored at CRC2\\[31:0\\]
regis- ter."]
pub type Crc2_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 CRC Value Low Register. This register contains the current known good signature value stored at CRC2\\[31:0\\]
regis- ter."]
    #[inline(always)]
    pub fn crc2_31_0(&self) -> Crc2_31_0R {
        Crc2_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 CRC Value Low Register. This register contains the current known good signature value stored at CRC2\\[31:0\\]
regis- ter."]
    #[inline(always)]
    #[must_use]
    pub fn crc2_31_0(&mut self) -> Crc2_31_0W<CrcRegl2Spec> {
        Crc2_31_0W::new(self, 0)
    }
}
#[doc = "Channel 2 CRC value low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_regl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_regl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRegl2Spec;
impl crate::RegisterSpec for CrcRegl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_regl2::R`](R) reader structure"]
impl crate::Readable for CrcRegl2Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_regl2::W`](W) writer structure"]
impl crate::Writable for CrcRegl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_REGL2 to value 0"]
impl crate::Resettable for CrcRegl2Spec {
    const RESET_VALUE: u32 = 0;
}
