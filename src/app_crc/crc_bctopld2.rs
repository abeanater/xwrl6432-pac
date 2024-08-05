#[doc = "Register `CRC_BCTOPLD2` reader"]
pub type R = crate::R<CrcBctopld2Spec>;
#[doc = "Register `CRC_BCTOPLD2` writer"]
pub type W = crate::W<CrcBctopld2Spec>;
#[doc = "Field `CRC_BCTOPLD2` reader - 23:0\\]
Channel 2 Block Complete Timeout Counter Preload Regis- ter. This register contains the number of clock cycles within which the CRC for an entire block needs to complete before a timeout interrupt is generated."]
pub type CrcBctopld2R = crate::FieldReader<u32>;
#[doc = "Field `CRC_BCTOPLD2` writer - 23:0\\]
Channel 2 Block Complete Timeout Counter Preload Regis- ter. This register contains the number of clock cycles within which the CRC for an entire block needs to complete before a timeout interrupt is generated."]
pub type CrcBctopld2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved1` reader - 31:24\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 31:24\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 2 Block Complete Timeout Counter Preload Regis- ter. This register contains the number of clock cycles within which the CRC for an entire block needs to complete before a timeout interrupt is generated."]
    #[inline(always)]
    pub fn crc_bctopld2(&self) -> CrcBctopld2R {
        CrcBctopld2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Channel 2 Block Complete Timeout Counter Preload Regis- ter. This register contains the number of clock cycles within which the CRC for an entire block needs to complete before a timeout interrupt is generated."]
    #[inline(always)]
    #[must_use]
    pub fn crc_bctopld2(&mut self) -> CrcBctopld2W<CrcBctopld2Spec> {
        CrcBctopld2W::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcBctopld2Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Channel 2 timeout pre-load value to check if one block of patterns are compressed with a given time\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_bctopld2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_bctopld2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcBctopld2Spec;
impl crate::RegisterSpec for CrcBctopld2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_bctopld2::R`](R) reader structure"]
impl crate::Readable for CrcBctopld2Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_bctopld2::W`](W) writer structure"]
impl crate::Writable for CrcBctopld2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_BCTOPLD2 to value 0"]
impl crate::Resettable for CrcBctopld2Spec {
    const RESET_VALUE: u32 = 0;
}
