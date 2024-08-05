#[doc = "Register `CRC_BUSY` reader"]
pub type R = crate::R<CrcBusySpec>;
#[doc = "Register `CRC_BUSY` writer"]
pub type W = crate::W<CrcBusySpec>;
#[doc = "Field `CH1_BUSY` reader - 0:0\\]
CH1_BUSY. During AUTO mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch1BusyR = crate::BitReader;
#[doc = "Field `CH1_BUSY` writer - 0:0\\]
CH1_BUSY. During AUTO mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch1BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 7:1\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 7:1\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Ch2_BUSY` reader - 8:8\\]
Ch2_BUSY. During AUTO mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch2BusyR = crate::BitReader;
#[doc = "Field `Ch2_BUSY` writer - 8:8\\]
Ch2_BUSY. During AUTO mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
pub type Ch2BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 15:9\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 15:9\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU39` reader - 16:16\\]
Reserved"]
pub type Nu39R = crate::BitReader;
#[doc = "Field `NU39` writer - 16:16\\]
Reserved"]
pub type Nu39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 23:17\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 23:17\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU40` reader - 24:24\\]
Reserved"]
pub type Nu40R = crate::BitReader;
#[doc = "Field `NU40` writer - 24:24\\]
Reserved"]
pub type Nu40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - 31:25\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 31:25\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CH1_BUSY. During AUTO mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    pub fn ch1_busy(&self) -> Ch1BusyR {
        Ch1BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Ch2_BUSY. During AUTO mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    pub fn ch2_busy(&self) -> Ch2BusyR {
        Ch2BusyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu39(&self) -> Nu39R {
        Nu39R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Reserved"]
    #[inline(always)]
    pub fn nu40(&self) -> Nu40R {
        Nu40R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CH1_BUSY. During AUTO mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_busy(&mut self) -> Ch1BusyW<CrcBusySpec> {
        Ch1BusyW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcBusySpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Ch2_BUSY. During AUTO mode, the busy flag is set when the first data pattern of the block is compressed and remains set until the the last data pattern of the block is compressed. The flag is cleared when the last data pattern of the block is compressed."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_busy(&mut self) -> Ch2BusyW<CrcBusySpec> {
        Ch2BusyW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CrcBusySpec> {
        Reserved2W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu39(&mut self) -> Nu39W<CrcBusySpec> {
        Nu39W::new(self, 16)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CrcBusySpec> {
        Reserved3W::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu40(&mut self) -> Nu40W<CrcBusySpec> {
        Nu40W::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CrcBusySpec> {
        Reserved4W::new(self, 25)
    }
}
#[doc = "Contains the busy flag for each channel\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_busy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_busy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcBusySpec;
impl crate::RegisterSpec for CrcBusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_busy::R`](R) reader structure"]
impl crate::Readable for CrcBusySpec {}
#[doc = "`write(|w| ..)` method takes [`crc_busy::W`](W) writer structure"]
impl crate::Writable for CrcBusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_BUSY to value 0"]
impl crate::Resettable for CrcBusySpec {
    const RESET_VALUE: u32 = 0;
}
