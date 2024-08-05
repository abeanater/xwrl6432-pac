#[doc = "Register `DCTRIPSEL_DCACTL` reader"]
pub type R = crate::R<DctripselDcactlSpec>;
#[doc = "Register `DCTRIPSEL_DCACTL` writer"]
pub type W = crate::W<DctripselDcactlSpec>;
#[doc = "Field `DCTRIPSEL_DCAHCOMPSEL` reader - 3:0\\]
Digital Compare A High Input Select Defines the source for the DCAH input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
pub type DctripselDcahcompselR = crate::FieldReader;
#[doc = "Field `DCTRIPSEL_DCAHCOMPSEL` writer - 3:0\\]
Digital Compare A High Input Select Defines the source for the DCAH input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
pub type DctripselDcahcompselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DCTRIPSEL_DCALCOMPSEL` reader - 7:4\\]
Digital Compare A Low Input Select Defines the source for the DCAL input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
pub type DctripselDcalcompselR = crate::FieldReader;
#[doc = "Field `DCTRIPSEL_DCALCOMPSEL` writer - 7:4\\]
Digital Compare A Low Input Select Defines the source for the DCAL input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
pub type DctripselDcalcompselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DCTRIPSEL_DCBHCOMPSEL` reader - 11:8\\]
Digital Compare B High Input Select Defines the source for the DCBH input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
pub type DctripselDcbhcompselR = crate::FieldReader;
#[doc = "Field `DCTRIPSEL_DCBHCOMPSEL` writer - 11:8\\]
Digital Compare B High Input Select Defines the source for the DCBH input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
pub type DctripselDcbhcompselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DCTRIPSEL_DCBLCOMPSEL` reader - 15:12\\]
Digital Compare B Low Input Select Defines the source for the DCBL input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
pub type DctripselDcblcompselR = crate::FieldReader;
#[doc = "Field `DCTRIPSEL_DCBLCOMPSEL` writer - 15:12\\]
Digital Compare B Low Input Select Defines the source for the DCBL input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
pub type DctripselDcblcompselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DCACTL_EVT1SRCSEL` reader - 16:16\\]
DCAEVT1 Source Signal Select 0 Source Is DCAEVT1 Signal 1 Source Is DCEVTFILT Signal"]
pub type DcactlEvt1srcselR = crate::BitReader;
#[doc = "Field `DCACTL_EVT1SRCSEL` writer - 16:16\\]
DCAEVT1 Source Signal Select 0 Source Is DCAEVT1 Signal 1 Source Is DCEVTFILT Signal"]
pub type DcactlEvt1srcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACTL_EVT1FRC_SYNCSEL` reader - 17:17\\]
DCAEVT1 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
pub type DcactlEvt1frcSyncselR = crate::BitReader;
#[doc = "Field `DCACTL_EVT1FRC_SYNCSEL` writer - 17:17\\]
DCAEVT1 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
pub type DcactlEvt1frcSyncselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACTL_EVT1SOCE` reader - 18:18\\]
DCAEVT1 SOC, Enable/Disable 0 SOC Generation Disabled 1 SOC Generation Enabled"]
pub type DcactlEvt1soceR = crate::BitReader;
#[doc = "Field `DCACTL_EVT1SOCE` writer - 18:18\\]
DCAEVT1 SOC, Enable/Disable 0 SOC Generation Disabled 1 SOC Generation Enabled"]
pub type DcactlEvt1soceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACTL_EVT1SYNCE` reader - 19:19\\]
DCAEVT1 SYNC, Enable/Disable 0 SYNC Generation Disabled 1 SYNC Generation Enabled"]
pub type DcactlEvt1synceR = crate::BitReader;
#[doc = "Field `DCACTL_EVT1SYNCE` writer - 19:19\\]
DCAEVT1 SYNC, Enable/Disable 0 SYNC Generation Disabled 1 SYNC Generation Enabled"]
pub type DcactlEvt1synceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 23:20\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 23:20\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DCACTL_EVT2SRCSEL` reader - 24:24\\]
DCAEVT2 Source Signal Select 0 Source Is DCAEVT2 Signal 1 Source Is DCEVTFILT Signal"]
pub type DcactlEvt2srcselR = crate::BitReader;
#[doc = "Field `DCACTL_EVT2SRCSEL` writer - 24:24\\]
DCAEVT2 Source Signal Select 0 Source Is DCAEVT2 Signal 1 Source Is DCEVTFILT Signal"]
pub type DcactlEvt2srcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACTL_EVT2FRC_SYNCSEL` reader - 25:25\\]
DCAEVT2 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
pub type DcactlEvt2frcSyncselR = crate::BitReader;
#[doc = "Field `DCACTL_EVT2FRC_SYNCSEL` writer - 25:25\\]
DCAEVT2 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
pub type DcactlEvt2frcSyncselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 31:26\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 31:26\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Digital Compare A High Input Select Defines the source for the DCAH input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
    #[inline(always)]
    pub fn dctripsel_dcahcompsel(&self) -> DctripselDcahcompselR {
        DctripselDcahcompselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Digital Compare A Low Input Select Defines the source for the DCAL input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
    #[inline(always)]
    pub fn dctripsel_dcalcompsel(&self) -> DctripselDcalcompselR {
        DctripselDcalcompselR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Digital Compare B High Input Select Defines the source for the DCBH input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
    #[inline(always)]
    pub fn dctripsel_dcbhcompsel(&self) -> DctripselDcbhcompselR {
        DctripselDcbhcompselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Digital Compare B Low Input Select Defines the source for the DCBL input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
    #[inline(always)]
    pub fn dctripsel_dcblcompsel(&self) -> DctripselDcblcompselR {
        DctripselDcblcompselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
DCAEVT1 Source Signal Select 0 Source Is DCAEVT1 Signal 1 Source Is DCEVTFILT Signal"]
    #[inline(always)]
    pub fn dcactl_evt1srcsel(&self) -> DcactlEvt1srcselR {
        DcactlEvt1srcselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
DCAEVT1 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
    #[inline(always)]
    pub fn dcactl_evt1frc_syncsel(&self) -> DcactlEvt1frcSyncselR {
        DcactlEvt1frcSyncselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
DCAEVT1 SOC, Enable/Disable 0 SOC Generation Disabled 1 SOC Generation Enabled"]
    #[inline(always)]
    pub fn dcactl_evt1soce(&self) -> DcactlEvt1soceR {
        DcactlEvt1soceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
DCAEVT1 SYNC, Enable/Disable 0 SYNC Generation Disabled 1 SYNC Generation Enabled"]
    #[inline(always)]
    pub fn dcactl_evt1synce(&self) -> DcactlEvt1synceR {
        DcactlEvt1synceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
DCAEVT2 Source Signal Select 0 Source Is DCAEVT2 Signal 1 Source Is DCEVTFILT Signal"]
    #[inline(always)]
    pub fn dcactl_evt2srcsel(&self) -> DcactlEvt2srcselR {
        DcactlEvt2srcselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
DCAEVT2 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
    #[inline(always)]
    pub fn dcactl_evt2frc_syncsel(&self) -> DcactlEvt2frcSyncselR {
        DcactlEvt2frcSyncselR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Digital Compare A High Input Select Defines the source for the DCAH input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn dctripsel_dcahcompsel(&mut self) -> DctripselDcahcompselW<DctripselDcactlSpec> {
        DctripselDcahcompselW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Digital Compare A Low Input Select Defines the source for the DCAL input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn dctripsel_dcalcompsel(&mut self) -> DctripselDcalcompselW<DctripselDcactlSpec> {
        DctripselDcalcompselW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Digital Compare B High Input Select Defines the source for the DCBH input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn dctripsel_dcbhcompsel(&mut self) -> DctripselDcbhcompselW<DctripselDcactlSpec> {
        DctripselDcbhcompselW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Digital Compare B Low Input Select Defines the source for the DCBL input. The TZ signals, when used as trip signals, are treated as normal inputs and can be defined as active high or active low. 0 TZ1 input 1h TZ2 input 2h TZ3 input All other values Values not shown are reserved. If a device does not have a particular comparitor, then that option is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn dctripsel_dcblcompsel(&mut self) -> DctripselDcblcompselW<DctripselDcactlSpec> {
        DctripselDcblcompselW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
DCAEVT1 Source Signal Select 0 Source Is DCAEVT1 Signal 1 Source Is DCEVTFILT Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcactl_evt1srcsel(&mut self) -> DcactlEvt1srcselW<DctripselDcactlSpec> {
        DcactlEvt1srcselW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
DCAEVT1 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcactl_evt1frc_syncsel(&mut self) -> DcactlEvt1frcSyncselW<DctripselDcactlSpec> {
        DcactlEvt1frcSyncselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
DCAEVT1 SOC, Enable/Disable 0 SOC Generation Disabled 1 SOC Generation Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dcactl_evt1soce(&mut self) -> DcactlEvt1soceW<DctripselDcactlSpec> {
        DcactlEvt1soceW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
DCAEVT1 SYNC, Enable/Disable 0 SYNC Generation Disabled 1 SYNC Generation Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dcactl_evt1synce(&mut self) -> DcactlEvt1synceW<DctripselDcactlSpec> {
        DcactlEvt1synceW::new(self, 19)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DctripselDcactlSpec> {
        Reserved1W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
DCAEVT2 Source Signal Select 0 Source Is DCAEVT2 Signal 1 Source Is DCEVTFILT Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcactl_evt2srcsel(&mut self) -> DcactlEvt2srcselW<DctripselDcactlSpec> {
        DcactlEvt2srcselW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
DCAEVT2 Force Synchronization Signal Select 0 Source Is Synchronous Signal 1 Source Is Asynchronous Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcactl_evt2frc_syncsel(&mut self) -> DcactlEvt2frcSyncselW<DctripselDcactlSpec> {
        DcactlEvt2frcSyncselW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<DctripselDcactlSpec> {
        Reserved2W::new(self, 26)
    }
}
#[doc = "Digital Compare Trip Select Register/ Digital Compare A Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dctripsel_dcactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctripsel_dcactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctripselDcactlSpec;
impl crate::RegisterSpec for DctripselDcactlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctripsel_dcactl::R`](R) reader structure"]
impl crate::Readable for DctripselDcactlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctripsel_dcactl::W`](W) writer structure"]
impl crate::Writable for DctripselDcactlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTRIPSEL_DCACTL to value 0"]
impl crate::Resettable for DctripselDcactlSpec {
    const RESET_VALUE: u32 = 0;
}
