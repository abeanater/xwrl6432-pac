#[doc = "Register `CRC_CTRL2` reader"]
pub type R = crate::R<CrcCtrl2Spec>;
#[doc = "Register `CRC_CTRL2` writer"]
pub type W = crate::W<CrcCtrl2Spec>;
#[doc = "Field `CH1_MODE` reader - 1:0\\]
Channel 1 Mode: 0 0 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register 0 1 = AUTO mode 1 0 = reserved 1 1 = Full-CPU mode"]
pub type Ch1ModeR = crate::FieldReader;
#[doc = "Field `CH1_MODE` writer - 1:0\\]
Channel 1 Mode: 0 0 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register 0 1 = AUTO mode 1 0 = reserved 1 1 = Full-CPU mode"]
pub type Ch1ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - 3:2\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 3:2\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_TRACEEN` reader - 4:4\\]
Channel 1 Data Trace Enable. When set, the channel is put into data trace mode. The channel snoops on the CPU VBUSM, ITCM, DTCM buses for any read transaction. Any read data on these buses is compressed by the PSA Signature Register. When suspend is on, the PSA Signature Register does not compress any read data on these buses. 0 = Data Trace disable 1 = Data Trace enable"]
pub type Ch1TraceenR = crate::BitReader;
#[doc = "Field `CH1_TRACEEN` writer - 4:4\\]
Channel 1 Data Trace Enable. When set, the channel is put into data trace mode. The channel snoops on the CPU VBUSM, ITCM, DTCM buses for any read transaction. Any read data on these buses is compressed by the PSA Signature Register. When suspend is on, the PSA Signature Register does not compress any read data on these buses. 0 = Data Trace disable 1 = Data Trace enable"]
pub type Ch1TraceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 7:5\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 7:5\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2_MODE` reader - 9:8\\]
Channel 2 Mode: 0 0 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register 0 1 = AUTO mode 1 0 = reserved 1 1 = Full-CPU mode"]
pub type Ch2ModeR = crate::FieldReader;
#[doc = "Field `CH2_MODE` writer - 9:8\\]
Channel 2 Mode: 0 0 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register 0 1 = AUTO mode 1 0 = reserved 1 1 = Full-CPU mode"]
pub type Ch2ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - 15:10\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 15:10\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU13` reader - 17:16\\]
Reserved"]
pub type Nu13R = crate::FieldReader;
#[doc = "Field `NU13` writer - 17:16\\]
Reserved"]
pub type Nu13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved4` reader - 23:18\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 23:18\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU14` reader - 25:24\\]
Reserved"]
pub type Nu14R = crate::FieldReader;
#[doc = "Field `NU14` writer - 25:24\\]
Reserved"]
pub type Nu14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved5` reader - 31:26\\]
Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - 31:26\\]
Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Channel 1 Mode: 0 0 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register 0 1 = AUTO mode 1 0 = reserved 1 1 = Full-CPU mode"]
    #[inline(always)]
    pub fn ch1_mode(&self) -> Ch1ModeR {
        Ch1ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Data Trace Enable. When set, the channel is put into data trace mode. The channel snoops on the CPU VBUSM, ITCM, DTCM buses for any read transaction. Any read data on these buses is compressed by the PSA Signature Register. When suspend is on, the PSA Signature Register does not compress any read data on these buses. 0 = Data Trace disable 1 = Data Trace enable"]
    #[inline(always)]
    pub fn ch1_traceen(&self) -> Ch1TraceenR {
        Ch1TraceenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Channel 2 Mode: 0 0 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register 0 1 = AUTO mode 1 0 = reserved 1 1 = Full-CPU mode"]
    #[inline(always)]
    pub fn ch2_mode(&self) -> Ch2ModeR {
        Ch2ModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu13(&self) -> Nu13R {
        Nu13R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Reserved"]
    #[inline(always)]
    pub fn nu14(&self) -> Nu14R {
        Nu14R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Channel 1 Mode: 0 0 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register 0 1 = AUTO mode 1 0 = reserved 1 1 = Full-CPU mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_mode(&mut self) -> Ch1ModeW<CrcCtrl2Spec> {
        Ch1ModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcCtrl2Spec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Data Trace Enable. When set, the channel is put into data trace mode. The channel snoops on the CPU VBUSM, ITCM, DTCM buses for any read transaction. Any read data on these buses is compressed by the PSA Signature Register. When suspend is on, the PSA Signature Register does not compress any read data on these buses. 0 = Data Trace disable 1 = Data Trace enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_traceen(&mut self) -> Ch1TraceenW<CrcCtrl2Spec> {
        Ch1TraceenW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CrcCtrl2Spec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Channel 2 Mode: 0 0 = Data Capture mode. In this mode, the PSA Signature Register does not compress data when it is written. Any data written to PSA Signature Register is simply captured by PSA Signature Register without any compression. This mode can be used to plant seed value into the PSA register 0 1 = AUTO mode 1 0 = reserved 1 1 = Full-CPU mode"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_mode(&mut self) -> Ch2ModeW<CrcCtrl2Spec> {
        Ch2ModeW::new(self, 8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CrcCtrl2Spec> {
        Reserved3W::new(self, 10)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu13(&mut self) -> Nu13W<CrcCtrl2Spec> {
        Nu13W::new(self, 16)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CrcCtrl2Spec> {
        Reserved4W::new(self, 18)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu14(&mut self) -> Nu14W<CrcCtrl2Spec> {
        Nu14W::new(self, 24)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<CrcCtrl2Spec> {
        Reserved5W::new(self, 26)
    }
}
#[doc = "Contains channel mode, data trace enable control bits\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCtrl2Spec;
impl crate::RegisterSpec for CrcCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ctrl2::R`](R) reader structure"]
impl crate::Readable for CrcCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_ctrl2::W`](W) writer structure"]
impl crate::Writable for CrcCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CTRL2 to value 0"]
impl crate::Resettable for CrcCtrl2Spec {
    const RESET_VALUE: u32 = 0;
}
