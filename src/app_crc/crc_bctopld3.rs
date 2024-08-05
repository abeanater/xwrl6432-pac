#[doc = "Register `CRC_BCTOPLD3` reader"]
pub type R = crate::R<CrcBctopld3Spec>;
#[doc = "Register `CRC_BCTOPLD3` writer"]
pub type W = crate::W<CrcBctopld3Spec>;
#[doc = "Field `NU45` reader - 23:0\\]
Reserved"]
pub type Nu45R = crate::FieldReader<u32>;
#[doc = "Field `NU45` writer - 23:0\\]
Reserved"]
pub type Nu45W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
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
    pub fn nu45(&self) -> Nu45R {
        Nu45R::new(self.bits & 0x00ff_ffff)
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
    pub fn nu45(&mut self) -> Nu45W<CrcBctopld3Spec> {
        Nu45W::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcBctopld3Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Channel 3 timeout pre-load value to check if one block of patterns are compressed with a given time\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_bctopld3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_bctopld3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcBctopld3Spec;
impl crate::RegisterSpec for CrcBctopld3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_bctopld3::R`](R) reader structure"]
impl crate::Readable for CrcBctopld3Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_bctopld3::W`](W) writer structure"]
impl crate::Writable for CrcBctopld3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_BCTOPLD3 to value 0"]
impl crate::Resettable for CrcBctopld3Spec {
    const RESET_VALUE: u32 = 0;
}
