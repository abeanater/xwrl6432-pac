#[doc = "Register `CRC_SCOUNT_REG4` reader"]
pub type R = crate::R<CrcScountReg4Spec>;
#[doc = "Register `CRC_SCOUNT_REG4` writer"]
pub type W = crate::W<CrcScountReg4Spec>;
#[doc = "Field `NU55` reader - 15:0\\]
Reserved"]
pub type Nu55R = crate::FieldReader<u16>;
#[doc = "Field `NU55` writer - 15:0\\]
Reserved"]
pub type Nu55W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved1` reader - 31:16\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:16\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu55(&self) -> Nu55R {
        Nu55R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu55(&mut self) -> Nu55W<CrcScountReg4Spec> {
        Nu55W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcScountReg4Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "Channel 4 preload register for the sector count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_scount_reg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_scount_reg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcScountReg4Spec;
impl crate::RegisterSpec for CrcScountReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_scount_reg4::R`](R) reader structure"]
impl crate::Readable for CrcScountReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_scount_reg4::W`](W) writer structure"]
impl crate::Writable for CrcScountReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_SCOUNT_REG4 to value 0"]
impl crate::Resettable for CrcScountReg4Spec {
    const RESET_VALUE: u32 = 0;
}
