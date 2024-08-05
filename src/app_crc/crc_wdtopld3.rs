#[doc = "Register `CRC_WDTOPLD3` reader"]
pub type R = crate::R<CrcWdtopld3Spec>;
#[doc = "Register `CRC_WDTOPLD3` writer"]
pub type W = crate::W<CrcWdtopld3Spec>;
#[doc = "Field `NU44` reader - 23:0\\]
Reserved"]
pub type Nu44R = crate::FieldReader<u32>;
#[doc = "Field `NU44` writer - 23:0\\]
Reserved"]
pub type Nu44W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `Reserved1` reader - 31:24\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 31:24\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu44(&self) -> Nu44R {
        Nu44R::new(self.bits & 0x00ff_ffff)
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
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu44(&mut self) -> Nu44W<CrcWdtopld3Spec> {
        Nu44W::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcWdtopld3Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Channel 3 timeout pre-load value to check if within a given time DMA initiates a block transfer\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_wdtopld3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_wdtopld3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcWdtopld3Spec;
impl crate::RegisterSpec for CrcWdtopld3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_wdtopld3::R`](R) reader structure"]
impl crate::Readable for CrcWdtopld3Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_wdtopld3::W`](W) writer structure"]
impl crate::Writable for CrcWdtopld3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_WDTOPLD3 to value 0"]
impl crate::Resettable for CrcWdtopld3Spec {
    const RESET_VALUE: u32 = 0;
}
