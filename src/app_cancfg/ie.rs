#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "Field `RF0NE` reader - 0:0\\]
Rx FIFO 0 New Message Interrupt Enable"]
pub type Rf0neR = crate::BitReader;
#[doc = "Field `RF0NE` writer - 0:0\\]
Rx FIFO 0 New Message Interrupt Enable"]
pub type Rf0neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0WE` reader - 1:1\\]
Rx FIFO 0 Watermark Reached Interrupt Enable"]
pub type Rf0weR = crate::BitReader;
#[doc = "Field `RF0WE` writer - 1:1\\]
Rx FIFO 0 Watermark Reached Interrupt Enable"]
pub type Rf0weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0FE` reader - 2:2\\]
Rx FIFO 0 Full Interrupt Enable"]
pub type Rf0feR = crate::BitReader;
#[doc = "Field `RF0FE` writer - 2:2\\]
Rx FIFO 0 Full Interrupt Enable"]
pub type Rf0feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0LE` reader - 3:3\\]
Rx FIFO 0 Message Lost Interrupt Enable"]
pub type Rf0leR = crate::BitReader;
#[doc = "Field `RF0LE` writer - 3:3\\]
Rx FIFO 0 Message Lost Interrupt Enable"]
pub type Rf0leW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1NE` reader - 4:4\\]
Rx FIFO 1 New Message Interrupt Enable"]
pub type Rf1neR = crate::BitReader;
#[doc = "Field `RF1NE` writer - 4:4\\]
Rx FIFO 1 New Message Interrupt Enable"]
pub type Rf1neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1WE` reader - 5:5\\]
Rx FIFO 1 Watermark Reached Interrupt Enable"]
pub type Rf1weR = crate::BitReader;
#[doc = "Field `RF1WE` writer - 5:5\\]
Rx FIFO 1 Watermark Reached Interrupt Enable"]
pub type Rf1weW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1FE` reader - 6:6\\]
Rx FIFO 1 Full Interrupt Enable"]
pub type Rf1feR = crate::BitReader;
#[doc = "Field `RF1FE` writer - 6:6\\]
Rx FIFO 1 Full Interrupt Enable"]
pub type Rf1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1LE` reader - 7:7\\]
rx FIFO 1 Message Lost Interrupt Enable"]
pub type Rf1leR = crate::BitReader;
#[doc = "Field `RF1LE` writer - 7:7\\]
rx FIFO 1 Message Lost Interrupt Enable"]
pub type Rf1leW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPME` reader - 8:8\\]
High Priority message Interrupt Enable"]
pub type HpmeR = crate::BitReader;
#[doc = "Field `HPME` writer - 8:8\\]
High Priority message Interrupt Enable"]
pub type HpmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCE` reader - 9:9\\]
Transmission Completed Interrupt Enable"]
pub type TceR = crate::BitReader;
#[doc = "Field `TCE` writer - 9:9\\]
Transmission Completed Interrupt Enable"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCFE` reader - 10:10\\]
Transmission Cancellation Finishied Interrupt Enable"]
pub type TcfeR = crate::BitReader;
#[doc = "Field `TCFE` writer - 10:10\\]
Transmission Cancellation Finishied Interrupt Enable"]
pub type TcfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFEE` reader - 11:11\\]
Tx FIFO Empty Interrupt Enable"]
pub type TfeeR = crate::BitReader;
#[doc = "Field `TFEE` writer - 11:11\\]
Tx FIFO Empty Interrupt Enable"]
pub type TfeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFNE` reader - 12:12\\]
Tx Event FIFO New Entry Interrupt Enable"]
pub type TefneR = crate::BitReader;
#[doc = "Field `TEFNE` writer - 12:12\\]
Tx Event FIFO New Entry Interrupt Enable"]
pub type TefneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFWE` reader - 13:13\\]
Tx Event FIFO Watermark Reached Interrupt enable"]
pub type TefweR = crate::BitReader;
#[doc = "Field `TEFWE` writer - 13:13\\]
Tx Event FIFO Watermark Reached Interrupt enable"]
pub type TefweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFFE` reader - 14:14\\]
Tx Event FIFO Full Interrupt Enable"]
pub type TeffeR = crate::BitReader;
#[doc = "Field `TEFFE` writer - 14:14\\]
Tx Event FIFO Full Interrupt Enable"]
pub type TeffeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFLE` reader - 15:15\\]
Tx Event FIFO Event Lost Interrupt Enable"]
pub type TefleR = crate::BitReader;
#[doc = "Field `TEFLE` writer - 15:15\\]
Tx Event FIFO Event Lost Interrupt Enable"]
pub type TefleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSWE` reader - 16:16\\]
Timestamp Wraparound Interrupt Enable"]
pub type TsweR = crate::BitReader;
#[doc = "Field `TSWE` writer - 16:16\\]
Timestamp Wraparound Interrupt Enable"]
pub type TsweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAFE` reader - 17:17\\]
Message RAM Access Failure Interrupt Enable"]
pub type MrafeR = crate::BitReader;
#[doc = "Field `MRAFE` writer - 17:17\\]
Message RAM Access Failure Interrupt Enable"]
pub type MrafeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOOE` reader - 18:18\\]
Timeout Occurred Interrupt Enable"]
pub type TooeR = crate::BitReader;
#[doc = "Field `TOOE` writer - 18:18\\]
Timeout Occurred Interrupt Enable"]
pub type TooeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRX` reader - 19:19\\]
Message stored to Dedicated Rx Buffer Interrupt Enable"]
pub type DrxR = crate::BitReader;
#[doc = "Field `DRX` writer - 19:19\\]
Message stored to Dedicated Rx Buffer Interrupt Enable"]
pub type DrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BECE` reader - 20:20\\]
Bit Error Corrected Interrupt Enable"]
pub type BeceR = crate::BitReader;
#[doc = "Field `BECE` writer - 20:20\\]
Bit Error Corrected Interrupt Enable"]
pub type BeceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEUE` reader - 21:21\\]
Bit Error Uncorrected Interrupt Enable"]
pub type BeueR = crate::BitReader;
#[doc = "Field `BEUE` writer - 21:21\\]
Bit Error Uncorrected Interrupt Enable"]
pub type BeueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELOE` reader - 22:22\\]
Error Logging Overflow Interrupt Enable"]
pub type EloeR = crate::BitReader;
#[doc = "Field `ELOE` writer - 22:22\\]
Error Logging Overflow Interrupt Enable"]
pub type EloeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPE` reader - 23:23\\]
Error Passive Interrupt Enable"]
pub type EpeR = crate::BitReader;
#[doc = "Field `EPE` writer - 23:23\\]
Error Passive Interrupt Enable"]
pub type EpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWE` reader - 24:24\\]
Warning Status Interrupt Enable"]
pub type EweR = crate::BitReader;
#[doc = "Field `EWE` writer - 24:24\\]
Warning Status Interrupt Enable"]
pub type EweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOE` reader - 25:25\\]
Bus_Off Status Interrupt Enable"]
pub type BoeR = crate::BitReader;
#[doc = "Field `BOE` writer - 25:25\\]
Bus_Off Status Interrupt Enable"]
pub type BoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDIE` reader - 26:26\\]
Watchdog Interrupt Enable"]
pub type WdieR = crate::BitReader;
#[doc = "Field `WDIE` writer - 26:26\\]
Watchdog Interrupt Enable"]
pub type WdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEAE` reader - 27:27\\]
Protocol Error in Arbitration Phase Interrupt Enable"]
pub type PeaeR = crate::BitReader;
#[doc = "Field `PEAE` writer - 27:27\\]
Protocol Error in Arbitration Phase Interrupt Enable"]
pub type PeaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEDE` reader - 28:28\\]
Protocol Error in Data Phase Interrupt Enable"]
pub type PedeR = crate::BitReader;
#[doc = "Field `PEDE` writer - 28:28\\]
Protocol Error in Data Phase Interrupt Enable"]
pub type PedeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARAE` reader - 29:29\\]
Accees to Reserve Address Interrupt Enable"]
pub type AraeR = crate::BitReader;
#[doc = "Field `ARAE` writer - 29:29\\]
Accees to Reserve Address Interrupt Enable"]
pub type AraeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU31` reader - 31:30\\]
Reserved"]
pub type Nu31R = crate::FieldReader;
#[doc = "Field `NU31` writer - 31:30\\]
Reserved"]
pub type Nu31W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Rx FIFO 0 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> Rf0neR {
        Rf0neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Rx FIFO 0 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn rf0we(&self) -> Rf0weR {
        Rf0weR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Rx FIFO 0 Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> Rf0feR {
        Rf0feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Rx FIFO 0 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> Rf0leR {
        Rf0leR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Rx FIFO 1 New Message Interrupt Enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> Rf1neR {
        Rf1neR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Rx FIFO 1 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    pub fn rf1we(&self) -> Rf1weR {
        Rf1weR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Rx FIFO 1 Full Interrupt Enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> Rf1feR {
        Rf1feR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
rx FIFO 1 Message Lost Interrupt Enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> Rf1leR {
        Rf1leR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
High Priority message Interrupt Enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HpmeR {
        HpmeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmission Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmission Cancellation Finishied Interrupt Enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TcfeR {
        TcfeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tfee(&self) -> TfeeR {
        TfeeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Tx Event FIFO New Entry Interrupt Enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TefneR {
        TefneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Tx Event FIFO Watermark Reached Interrupt enable"]
    #[inline(always)]
    pub fn tefwe(&self) -> TefweR {
        TefweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Tx Event FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TeffeR {
        TeffeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Tx Event FIFO Event Lost Interrupt Enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TefleR {
        TefleR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Timestamp Wraparound Interrupt Enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TsweR {
        TsweR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Message RAM Access Failure Interrupt Enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MrafeR {
        MrafeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Timeout Occurred Interrupt Enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TooeR {
        TooeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Message stored to Dedicated Rx Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn drx(&self) -> DrxR {
        DrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    pub fn bece(&self) -> BeceR {
        BeceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub fn beue(&self) -> BeueR {
        BeueR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Error Logging Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn eloe(&self) -> EloeR {
        EloeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epe(&self) -> EpeR {
        EpeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Warning Status Interrupt Enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EweR {
        EweR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Bus_Off Status Interrupt Enable"]
    #[inline(always)]
    pub fn boe(&self) -> BoeR {
        BoeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WdieR {
        WdieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Protocol Error in Arbitration Phase Interrupt Enable"]
    #[inline(always)]
    pub fn peae(&self) -> PeaeR {
        PeaeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Protocol Error in Data Phase Interrupt Enable"]
    #[inline(always)]
    pub fn pede(&self) -> PedeR {
        PedeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Accees to Reserve Address Interrupt Enable"]
    #[inline(always)]
    pub fn arae(&self) -> AraeR {
        AraeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Reserved"]
    #[inline(always)]
    pub fn nu31(&self) -> Nu31R {
        Nu31R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Rx FIFO 0 New Message Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0ne(&mut self) -> Rf0neW<IeSpec> {
        Rf0neW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Rx FIFO 0 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0we(&mut self) -> Rf0weW<IeSpec> {
        Rf0weW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Rx FIFO 0 Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0fe(&mut self) -> Rf0feW<IeSpec> {
        Rf0feW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Rx FIFO 0 Message Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf0le(&mut self) -> Rf0leW<IeSpec> {
        Rf0leW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Rx FIFO 1 New Message Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1ne(&mut self) -> Rf1neW<IeSpec> {
        Rf1neW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Rx FIFO 1 Watermark Reached Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1we(&mut self) -> Rf1weW<IeSpec> {
        Rf1weW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Rx FIFO 1 Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1fe(&mut self) -> Rf1feW<IeSpec> {
        Rf1feW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
rx FIFO 1 Message Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rf1le(&mut self) -> Rf1leW<IeSpec> {
        Rf1leW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
High Priority message Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpme(&mut self) -> HpmeW<IeSpec> {
        HpmeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmission Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TceW<IeSpec> {
        TceW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmission Cancellation Finishied Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcfe(&mut self) -> TcfeW<IeSpec> {
        TcfeW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfee(&mut self) -> TfeeW<IeSpec> {
        TfeeW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Tx Event FIFO New Entry Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefne(&mut self) -> TefneW<IeSpec> {
        TefneW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Tx Event FIFO Watermark Reached Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefwe(&mut self) -> TefweW<IeSpec> {
        TefweW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Tx Event FIFO Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teffe(&mut self) -> TeffeW<IeSpec> {
        TeffeW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Tx Event FIFO Event Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tefle(&mut self) -> TefleW<IeSpec> {
        TefleW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Timestamp Wraparound Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tswe(&mut self) -> TsweW<IeSpec> {
        TsweW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Message RAM Access Failure Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mrafe(&mut self) -> MrafeW<IeSpec> {
        MrafeW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Timeout Occurred Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tooe(&mut self) -> TooeW<IeSpec> {
        TooeW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Message stored to Dedicated Rx Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drx(&mut self) -> DrxW<IeSpec> {
        DrxW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bece(&mut self) -> BeceW<IeSpec> {
        BeceW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn beue(&mut self) -> BeueW<IeSpec> {
        BeueW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Error Logging Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eloe(&mut self) -> EloeW<IeSpec> {
        EloeW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Error Passive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EpeW<IeSpec> {
        EpeW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Warning Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ewe(&mut self) -> EweW<IeSpec> {
        EweW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Bus_Off Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn boe(&mut self) -> BoeW<IeSpec> {
        BoeW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Watchdog Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdie(&mut self) -> WdieW<IeSpec> {
        WdieW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Protocol Error in Arbitration Phase Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peae(&mut self) -> PeaeW<IeSpec> {
        PeaeW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Protocol Error in Data Phase Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pede(&mut self) -> PedeW<IeSpec> {
        PedeW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Accees to Reserve Address Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arae(&mut self) -> AraeW<IeSpec> {
        AraeW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu31(&mut self) -> Nu31W<IeSpec> {
        Nu31W::new(self, 30)
    }
}
#[doc = "IE\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IeSpec {
    const RESET_VALUE: u32 = 0;
}
