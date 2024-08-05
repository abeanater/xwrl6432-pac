#[doc = "Register `CRC_PCOUNT_REG3` reader"]
pub type R = crate::R<CrcPcountReg3Spec>;
#[doc = "Register `CRC_PCOUNT_REG3` writer"]
pub type W = crate::W<CrcPcountReg3Spec>;
#[doc = "Field `NU41` reader - 19:0\\]
Reserved"]
pub type Nu41R = crate::FieldReader<u32>;
#[doc = "Field `NU41` writer - 19:0\\]
Reserved"]
pub type Nu41W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `Reserved1` reader - 31:20\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:20\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu41(&self) -> Nu41R {
        Nu41R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu41(&mut self) -> Nu41W<CrcPcountReg3Spec> {
        Nu41W::new(self, 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcPcountReg3Spec> {
        Reserved1W::new(self, 20)
    }
}
#[doc = "Channel 3 preload register for the pattern count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pcount_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pcount_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcPcountReg3Spec;
impl crate::RegisterSpec for CrcPcountReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_pcount_reg3::R`](R) reader structure"]
impl crate::Readable for CrcPcountReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_pcount_reg3::W`](W) writer structure"]
impl crate::Writable for CrcPcountReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_PCOUNT_REG3 to value 0"]
impl crate::Resettable for CrcPcountReg3Spec {
    const RESET_VALUE: u32 = 0;
}
