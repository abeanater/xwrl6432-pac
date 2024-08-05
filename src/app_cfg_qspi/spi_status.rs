#[doc = "Register `SPI_STATUS` reader"]
pub type R = crate::R<SpiStatusSpec>;
#[doc = "Register `SPI_STATUS` writer"]
pub type W = crate::W<SpiStatusSpec>;
#[doc = "Field `BUSY` reader - 0:0\\]
Busy bit. Active transfer in progress. This bit is only set during an active word transfer. Between words, the bit will clear to signal that it is ok to read/write the data registers. 0- Idle 1- Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - 0:0\\]
Busy bit. Active transfer in progress. This bit is only set during an active word transfer. Between words, the bit will clear to signal that it is ok to read/write the data registers. 0- Idle 1- Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WC` reader - 1:1\\]
Word complete. This bit is set after each word transfer is completed. 0- Word transfer is not complete 1- Word transfer is complete This bit is reset when the SPI Status Register is read"]
pub type WcR = crate::BitReader;
#[doc = "Field `WC` writer - 1:1\\]
Word complete. This bit is set after each word transfer is completed. 0- Word transfer is not complete 1- Word transfer is complete This bit is reset when the SPI Status Register is read"]
pub type WcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC` reader - 2:2\\]
Frame complete. This bit is set after all of the requested words have been transmitted. 0- Transfer is not complete 1- Transfer is complete This bit is reset when the SPI Status Register is read"]
pub type FcR = crate::BitReader;
#[doc = "Field `FC` writer - 2:2\\]
Frame complete. This bit is set after all of the requested words have been transmitted. 0- Transfer is not complete 1- Transfer is complete This bit is reset when the SPI Status Register is read"]
pub type FcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 15:3\\]
Always read as 0"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 15:3\\]
Always read as 0"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `WDCNT` reader - 27:16\\]
Word count. This field will reflect the 1-4096 words transferred"]
pub type WdcntR = crate::FieldReader<u16>;
#[doc = "Field `WDCNT` writer - 27:16\\]
Word count. This field will reflect the 1-4096 words transferred"]
pub type WdcntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `Reserved2` reader - 31:28\\]
Always read as 0"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 31:28\\]
Always read as 0"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Busy bit. Active transfer in progress. This bit is only set during an active word transfer. Between words, the bit will clear to signal that it is ok to read/write the data registers. 0- Idle 1- Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word complete. This bit is set after each word transfer is completed. 0- Word transfer is not complete 1- Word transfer is complete This bit is reset when the SPI Status Register is read"]
    #[inline(always)]
    pub fn wc(&self) -> WcR {
        WcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Frame complete. This bit is set after all of the requested words have been transmitted. 0- Transfer is not complete 1- Transfer is complete This bit is reset when the SPI Status Register is read"]
    #[inline(always)]
    pub fn fc(&self) -> FcR {
        FcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Word count. This field will reflect the 1-4096 words transferred"]
    #[inline(always)]
    pub fn wdcnt(&self) -> WdcntR {
        WdcntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Busy bit. Active transfer in progress. This bit is only set during an active word transfer. Between words, the bit will clear to signal that it is ok to read/write the data registers. 0- Idle 1- Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<SpiStatusSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word complete. This bit is set after each word transfer is completed. 0- Word transfer is not complete 1- Word transfer is complete This bit is reset when the SPI Status Register is read"]
    #[inline(always)]
    #[must_use]
    pub fn wc(&mut self) -> WcW<SpiStatusSpec> {
        WcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Frame complete. This bit is set after all of the requested words have been transmitted. 0- Transfer is not complete 1- Transfer is complete This bit is reset when the SPI Status Register is read"]
    #[inline(always)]
    #[must_use]
    pub fn fc(&mut self) -> FcW<SpiStatusSpec> {
        FcW::new(self, 2)
    }
    #[doc = "Bits 3:15 - 15:3\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SpiStatusSpec> {
        Reserved1W::new(self, 3)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Word count. This field will reflect the 1-4096 words transferred"]
    #[inline(always)]
    #[must_use]
    pub fn wdcnt(&mut self) -> WdcntW<SpiStatusSpec> {
        WdcntW::new(self, 16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SpiStatusSpec> {
        Reserved2W::new(self, 28)
    }
}
#[doc = "SPI Status Register (SPISR)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiStatusSpec;
impl crate::RegisterSpec for SpiStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_status::R`](R) reader structure"]
impl crate::Readable for SpiStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_status::W`](W) writer structure"]
impl crate::Writable for SpiStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_STATUS to value 0"]
impl crate::Resettable for SpiStatusSpec {
    const RESET_VALUE: u32 = 0;
}
