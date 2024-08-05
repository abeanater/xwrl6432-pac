#[doc = "Register `SCIGCR1` reader"]
pub type R = crate::R<Scigcr1Spec>;
#[doc = "Register `SCIGCR1` writer"]
pub type W = crate::W<Scigcr1Spec>;
#[doc = "Field `COMM_MODE` reader - 0:0\\]
SCI communication mode bit (0=Idle-line mode, 1=Address-bit mode)"]
pub type CommModeR = crate::BitReader;
#[doc = "Field `COMM_MODE` writer - 0:0\\]
SCI communication mode bit (0=Idle-line mode, 1=Address-bit mode)"]
pub type CommModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMING_MODE` reader - 1:1\\]
SCI timing mode bit (0=Isosynchronous timing,1=Asynchronous timing)"]
pub type TimingModeR = crate::BitReader;
#[doc = "Field `TIMING_MODE` writer - 1:1\\]
SCI timing mode bit (0=Isosynchronous timing,1=Asynchronous timing)"]
pub type TimingModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ENA` reader - 2:2\\]
SCI parity enable"]
pub type ParityEnaR = crate::BitReader;
#[doc = "Field `PARITY_ENA` writer - 2:2\\]
SCI parity enable"]
pub type ParityEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY` reader - 3:3\\]
SCI parity odd/even selection"]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - 3:3\\]
SCI parity odd/even selection"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - 4:4\\]
SCI number of stop bits"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - 4:4\\]
SCI number of stop bits"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOCK` reader - 5:5\\]
SCI internal clock enable"]
pub type ClockR = crate::BitReader;
#[doc = "Field `CLOCK` writer - 5:5\\]
SCI internal clock enable"]
pub type ClockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 6:6\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 6:6\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_nRESET` reader - 7:7\\]
Software reset (active low)"]
pub type SwNResetR = crate::BitReader;
#[doc = "Field `SW_nRESET` writer - 7:7\\]
Software reset (active low)"]
pub type SwNResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - 8:8\\]
In a multiprocessor configuration, this bit controls the receive sleep function. Clearing this bit brings the SCI out of sleep mode"]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - 8:8\\]
In a multiprocessor configuration, this bit controls the receive sleep function. Clearing this bit brings the SCI out of sleep mode"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWERDOWN` reader - 9:9\\]
When the POWERDOWN bit is set, the SCI attempts to enter local low-power mode"]
pub type PowerdownR = crate::BitReader;
#[doc = "Field `POWERDOWN` writer - 9:9\\]
When the POWERDOWN bit is set, the SCI attempts to enter local low-power mode"]
pub type PowerdownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 15:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 15:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LOOP_BACK` reader - 16:16\\]
Enable bit for loopback mode"]
pub type LoopBackR = crate::BitReader;
#[doc = "Field `LOOP_BACK` writer - 16:16\\]
Enable bit for loopback mode"]
pub type LoopBackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - 17:17\\]
This bit has an effect only when a program is being debugged with an emulator, and it determines how the SCI operates when the program is suspended"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - 17:17\\]
This bit has an effect only when a program is being debugged with an emulator, and it determines how the SCI operates when the program is suspended"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 23:18\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 23:18\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RXENA` reader - 24:24\\]
Allows the receiver to transfer data from the shift buffer to the receive buffer"]
pub type RxenaR = crate::BitReader;
#[doc = "Field `RXENA` writer - 24:24\\]
Allows the receiver to transfer data from the shift buffer to the receive buffer"]
pub type RxenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENA` reader - 25:25\\]
Data is transferred from SCITD to SCITXSHF only when the TXENA bit is set"]
pub type TxenaR = crate::BitReader;
#[doc = "Field `TXENA` writer - 25:25\\]
Data is transferred from SCITD to SCITXSHF only when the TXENA bit is set"]
pub type TxenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:26\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 31:26\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SCI communication mode bit (0=Idle-line mode, 1=Address-bit mode)"]
    #[inline(always)]
    pub fn comm_mode(&self) -> CommModeR {
        CommModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SCI timing mode bit (0=Isosynchronous timing,1=Asynchronous timing)"]
    #[inline(always)]
    pub fn timing_mode(&self) -> TimingModeR {
        TimingModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SCI parity enable"]
    #[inline(always)]
    pub fn parity_ena(&self) -> ParityEnaR {
        ParityEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
SCI parity odd/even selection"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SCI number of stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
SCI internal clock enable"]
    #[inline(always)]
    pub fn clock(&self) -> ClockR {
        ClockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software reset (active low)"]
    #[inline(always)]
    pub fn sw_n_reset(&self) -> SwNResetR {
        SwNResetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
In a multiprocessor configuration, this bit controls the receive sleep function. Clearing this bit brings the SCI out of sleep mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
When the POWERDOWN bit is set, the SCI attempts to enter local low-power mode"]
    #[inline(always)]
    pub fn powerdown(&self) -> PowerdownR {
        PowerdownR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable bit for loopback mode"]
    #[inline(always)]
    pub fn loop_back(&self) -> LoopBackR {
        LoopBackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
This bit has an effect only when a program is being debugged with an emulator, and it determines how the SCI operates when the program is suspended"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Allows the receiver to transfer data from the shift buffer to the receive buffer"]
    #[inline(always)]
    pub fn rxena(&self) -> RxenaR {
        RxenaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Data is transferred from SCITD to SCITXSHF only when the TXENA bit is set"]
    #[inline(always)]
    pub fn txena(&self) -> TxenaR {
        TxenaR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SCI communication mode bit (0=Idle-line mode, 1=Address-bit mode)"]
    #[inline(always)]
    #[must_use]
    pub fn comm_mode(&mut self) -> CommModeW<Scigcr1Spec> {
        CommModeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SCI timing mode bit (0=Isosynchronous timing,1=Asynchronous timing)"]
    #[inline(always)]
    #[must_use]
    pub fn timing_mode(&mut self) -> TimingModeW<Scigcr1Spec> {
        TimingModeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
SCI parity enable"]
    #[inline(always)]
    #[must_use]
    pub fn parity_ena(&mut self) -> ParityEnaW<Scigcr1Spec> {
        ParityEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
SCI parity odd/even selection"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<Scigcr1Spec> {
        ParityW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
SCI number of stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<Scigcr1Spec> {
        StopW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
SCI internal clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock(&mut self) -> ClockW<Scigcr1Spec> {
        ClockW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scigcr1Spec> {
        Reserved1W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software reset (active low)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_n_reset(&mut self) -> SwNResetW<Scigcr1Spec> {
        SwNResetW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
In a multiprocessor configuration, this bit controls the receive sleep function. Clearing this bit brings the SCI out of sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<Scigcr1Spec> {
        SleepW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
When the POWERDOWN bit is set, the SCI attempts to enter local low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn powerdown(&mut self) -> PowerdownW<Scigcr1Spec> {
        PowerdownW::new(self, 9)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Scigcr1Spec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable bit for loopback mode"]
    #[inline(always)]
    #[must_use]
    pub fn loop_back(&mut self) -> LoopBackW<Scigcr1Spec> {
        LoopBackW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
This bit has an effect only when a program is being debugged with an emulator, and it determines how the SCI operates when the program is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<Scigcr1Spec> {
        ContW::new(self, 17)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Scigcr1Spec> {
        Reserved3W::new(self, 18)
    }
    #[doc = "Bit 24 - 24:24\\]
Allows the receiver to transfer data from the shift buffer to the receive buffer"]
    #[inline(always)]
    #[must_use]
    pub fn rxena(&mut self) -> RxenaW<Scigcr1Spec> {
        RxenaW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Data is transferred from SCITD to SCITXSHF only when the TXENA bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn txena(&mut self) -> TxenaW<Scigcr1Spec> {
        TxenaW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<Scigcr1Spec> {
        Reserved4W::new(self, 26)
    }
}
#[doc = "The SCIGCR1 register defines the frame format, protocol, and communication mode used by the SCI\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scigcr1Spec;
impl crate::RegisterSpec for Scigcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scigcr1::R`](R) reader structure"]
impl crate::Readable for Scigcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`scigcr1::W`](W) writer structure"]
impl crate::Writable for Scigcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIGCR1 to value 0"]
impl crate::Resettable for Scigcr1Spec {
    const RESET_VALUE: u32 = 0;
}
