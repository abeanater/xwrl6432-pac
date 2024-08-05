#[doc = "Register `IRQENABLE` reader"]
pub type R = crate::R<IrqenableSpec>;
#[doc = "Register `IRQENABLE` writer"]
pub type W = crate::W<IrqenableSpec>;
#[doc = "Field `TX0_EMPTY_ENABLE` reader - 0:0\\]
Transmitter register Empty Interrupt Enable Ch 0 - (RW )"]
pub type Tx0EmptyEnableR = crate::BitReader;
#[doc = "Field `TX0_EMPTY_ENABLE` writer - 0:0\\]
Transmitter register Empty Interrupt Enable Ch 0 - (RW )"]
pub type Tx0EmptyEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX0_UNDERFLOW_ENABLE` reader - 1:1\\]
Transmitter register Underflow Interrupt Enable Ch 0 - (RW )"]
pub type Tx0UnderflowEnableR = crate::BitReader;
#[doc = "Field `TX0_UNDERFLOW_ENABLE` writer - 1:1\\]
Transmitter register Underflow Interrupt Enable Ch 0 - (RW )"]
pub type Tx0UnderflowEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX0_FULL_ENABLE` reader - 2:2\\]
Receiver register Full Interrupt Enable Ch 0 - (RW )"]
pub type Rx0FullEnableR = crate::BitReader;
#[doc = "Field `RX0_FULL_ENABLE` writer - 2:2\\]
Receiver register Full Interrupt Enable Ch 0 - (RW )"]
pub type Rx0FullEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX0_OVERFLOW_ENABLE` reader - 3:3\\]
Receiver register Overflow Interrupt Enable Ch 0 - (RW )"]
pub type Rx0OverflowEnableR = crate::BitReader;
#[doc = "Field `RX0_OVERFLOW_ENABLE` writer - 3:3\\]
Receiver register Overflow Interrupt Enable Ch 0 - (RW )"]
pub type Rx0OverflowEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX1_EMPTY_ENABLE` reader - 4:4\\]
Transmitter register Empty Interrupt Enable Ch 1 - (RW )"]
pub type Tx1EmptyEnableR = crate::BitReader;
#[doc = "Field `TX1_EMPTY_ENABLE` writer - 4:4\\]
Transmitter register Empty Interrupt Enable Ch 1 - (RW )"]
pub type Tx1EmptyEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX1_UNDERFLOW_ENABLE` reader - 5:5\\]
Transmitter register Underflow Interrupt Enable Ch 1 - (RW )"]
pub type Tx1UnderflowEnableR = crate::BitReader;
#[doc = "Field `TX1_UNDERFLOW_ENABLE` writer - 5:5\\]
Transmitter register Underflow Interrupt Enable Ch 1 - (RW )"]
pub type Tx1UnderflowEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX1_FULL_ENABLE` reader - 6:6\\]
Receiver register Full Interrupt Enable Ch 1 - (RW )"]
pub type Rx1FullEnableR = crate::BitReader;
#[doc = "Field `RX1_FULL_ENABLE` writer - 6:6\\]
Receiver register Full Interrupt Enable Ch 1 - (RW )"]
pub type Rx1FullEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_3` reader - 7:7\\]
Reads return 0 - (RO )"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED_3` writer - 7:7\\]
Reads return 0 - (RO )"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX2_EMPTY_ENABLE` reader - 8:8\\]
Transmitter register Empty Interrupt Enable Ch 2 - (RW )"]
pub type Tx2EmptyEnableR = crate::BitReader;
#[doc = "Field `TX2_EMPTY_ENABLE` writer - 8:8\\]
Transmitter register Empty Interrupt Enable Ch 2 - (RW )"]
pub type Tx2EmptyEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX2_UNDERFLOW_ENABLE` reader - 9:9\\]
Transmitter register Underflow Interrupt Enable Ch 2 - (RW )"]
pub type Tx2UnderflowEnableR = crate::BitReader;
#[doc = "Field `TX2_UNDERFLOW_ENABLE` writer - 9:9\\]
Transmitter register Underflow Interrupt Enable Ch 2 - (RW )"]
pub type Tx2UnderflowEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX2_FULL_ENABLE` reader - 10:10\\]
Receiver register Full Interrupt Enable Ch 2 - (RW )"]
pub type Rx2FullEnableR = crate::BitReader;
#[doc = "Field `RX2_FULL_ENABLE` writer - 10:10\\]
Receiver register Full Interrupt Enable Ch 2 - (RW )"]
pub type Rx2FullEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_6` reader - 11:11\\]
Reads return 0 - (RO )"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `RESERVED_6` writer - 11:11\\]
Reads return 0 - (RO )"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX3_EMPTY_ENABLE` reader - 12:12\\]
Transmitter register Empty Interrupt Enable Ch3 - (RW )"]
pub type Tx3EmptyEnableR = crate::BitReader;
#[doc = "Field `TX3_EMPTY_ENABLE` writer - 12:12\\]
Transmitter register Empty Interrupt Enable Ch3 - (RW )"]
pub type Tx3EmptyEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX3_UNDERFLOW_ENABLE` reader - 13:13\\]
Transmitter register Underflow Interrupt Enable Ch 3 - (RW )"]
pub type Tx3UnderflowEnableR = crate::BitReader;
#[doc = "Field `TX3_UNDERFLOW_ENABLE` writer - 13:13\\]
Transmitter register Underflow Interrupt Enable Ch 3 - (RW )"]
pub type Tx3UnderflowEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX3_FULL_ENABLE` reader - 14:14\\]
Receiver register Full Interrupt Enable Ch 3 - (RW )"]
pub type Rx3FullEnableR = crate::BitReader;
#[doc = "Field `RX3_FULL_ENABLE` writer - 14:14\\]
Receiver register Full Interrupt Enable Ch 3 - (RW )"]
pub type Rx3FullEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_4` reader - 15:15\\]
Reads returns 0 - (RO )"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RESERVED_4` writer - 15:15\\]
Reads returns 0 - (RO )"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKE` reader - 16:16\\]
Wake Up event interrupt Enable in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]
- (RW )"]
pub type WkeR = crate::BitReader;
#[doc = "Field `WKE` writer - 16:16\\]
Wake Up event interrupt Enable in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]
- (RW )"]
pub type WkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOW_ENABLE` reader - 17:17\\]
End of Word count Interrupt Enable - (RW )"]
pub type EowEnableR = crate::BitReader;
#[doc = "Field `EOW_ENABLE` writer - 17:17\\]
End of Word count Interrupt Enable - (RW )"]
pub type EowEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_5` reader - 31:18\\]
Reads return 0 - (RO )"]
pub type Reserved5R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED_5` writer - 31:18\\]
Reads return 0 - (RO )"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmitter register Empty Interrupt Enable Ch 0 - (RW )"]
    #[inline(always)]
    pub fn tx0_empty_enable(&self) -> Tx0EmptyEnableR {
        Tx0EmptyEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmitter register Underflow Interrupt Enable Ch 0 - (RW )"]
    #[inline(always)]
    pub fn tx0_underflow_enable(&self) -> Tx0UnderflowEnableR {
        Tx0UnderflowEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receiver register Full Interrupt Enable Ch 0 - (RW )"]
    #[inline(always)]
    pub fn rx0_full_enable(&self) -> Rx0FullEnableR {
        Rx0FullEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receiver register Overflow Interrupt Enable Ch 0 - (RW )"]
    #[inline(always)]
    pub fn rx0_overflow_enable(&self) -> Rx0OverflowEnableR {
        Rx0OverflowEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmitter register Empty Interrupt Enable Ch 1 - (RW )"]
    #[inline(always)]
    pub fn tx1_empty_enable(&self) -> Tx1EmptyEnableR {
        Tx1EmptyEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmitter register Underflow Interrupt Enable Ch 1 - (RW )"]
    #[inline(always)]
    pub fn tx1_underflow_enable(&self) -> Tx1UnderflowEnableR {
        Tx1UnderflowEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receiver register Full Interrupt Enable Ch 1 - (RW )"]
    #[inline(always)]
    pub fn rx1_full_enable(&self) -> Rx1FullEnableR {
        Rx1FullEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reads return 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmitter register Empty Interrupt Enable Ch 2 - (RW )"]
    #[inline(always)]
    pub fn tx2_empty_enable(&self) -> Tx2EmptyEnableR {
        Tx2EmptyEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter register Underflow Interrupt Enable Ch 2 - (RW )"]
    #[inline(always)]
    pub fn tx2_underflow_enable(&self) -> Tx2UnderflowEnableR {
        Tx2UnderflowEnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Receiver register Full Interrupt Enable Ch 2 - (RW )"]
    #[inline(always)]
    pub fn rx2_full_enable(&self) -> Rx2FullEnableR {
        Rx2FullEnableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Reads return 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Transmitter register Empty Interrupt Enable Ch3 - (RW )"]
    #[inline(always)]
    pub fn tx3_empty_enable(&self) -> Tx3EmptyEnableR {
        Tx3EmptyEnableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Transmitter register Underflow Interrupt Enable Ch 3 - (RW )"]
    #[inline(always)]
    pub fn tx3_underflow_enable(&self) -> Tx3UnderflowEnableR {
        Tx3UnderflowEnableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver register Full Interrupt Enable Ch 3 - (RW )"]
    #[inline(always)]
    pub fn rx3_full_enable(&self) -> Rx3FullEnableR {
        Rx3FullEnableR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Wake Up event interrupt Enable in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]
- (RW )"]
    #[inline(always)]
    pub fn wke(&self) -> WkeR {
        WkeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
End of Word count Interrupt Enable - (RW )"]
    #[inline(always)]
    pub fn eow_enable(&self) -> EowEnableR {
        EowEnableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reads return 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmitter register Empty Interrupt Enable Ch 0 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tx0_empty_enable(&mut self) -> Tx0EmptyEnableW<IrqenableSpec> {
        Tx0EmptyEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmitter register Underflow Interrupt Enable Ch 0 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tx0_underflow_enable(&mut self) -> Tx0UnderflowEnableW<IrqenableSpec> {
        Tx0UnderflowEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Receiver register Full Interrupt Enable Ch 0 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn rx0_full_enable(&mut self) -> Rx0FullEnableW<IrqenableSpec> {
        Rx0FullEnableW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receiver register Overflow Interrupt Enable Ch 0 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn rx0_overflow_enable(&mut self) -> Rx0OverflowEnableW<IrqenableSpec> {
        Rx0OverflowEnableW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmitter register Empty Interrupt Enable Ch 1 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tx1_empty_enable(&mut self) -> Tx1EmptyEnableW<IrqenableSpec> {
        Tx1EmptyEnableW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmitter register Underflow Interrupt Enable Ch 1 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tx1_underflow_enable(&mut self) -> Tx1UnderflowEnableW<IrqenableSpec> {
        Tx1UnderflowEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Receiver register Full Interrupt Enable Ch 1 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn rx1_full_enable(&mut self) -> Rx1FullEnableW<IrqenableSpec> {
        Rx1FullEnableW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Reads return 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_3(&mut self) -> Reserved3W<IrqenableSpec> {
        Reserved3W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmitter register Empty Interrupt Enable Ch 2 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tx2_empty_enable(&mut self) -> Tx2EmptyEnableW<IrqenableSpec> {
        Tx2EmptyEnableW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter register Underflow Interrupt Enable Ch 2 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tx2_underflow_enable(&mut self) -> Tx2UnderflowEnableW<IrqenableSpec> {
        Tx2UnderflowEnableW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Receiver register Full Interrupt Enable Ch 2 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn rx2_full_enable(&mut self) -> Rx2FullEnableW<IrqenableSpec> {
        Rx2FullEnableW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Reads return 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_6(&mut self) -> Reserved6W<IrqenableSpec> {
        Reserved6W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Transmitter register Empty Interrupt Enable Ch3 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tx3_empty_enable(&mut self) -> Tx3EmptyEnableW<IrqenableSpec> {
        Tx3EmptyEnableW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Transmitter register Underflow Interrupt Enable Ch 3 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tx3_underflow_enable(&mut self) -> Tx3UnderflowEnableW<IrqenableSpec> {
        Tx3UnderflowEnableW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver register Full Interrupt Enable Ch 3 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn rx3_full_enable(&mut self) -> Rx3FullEnableW<IrqenableSpec> {
        Rx3FullEnableW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_4(&mut self) -> Reserved4W<IrqenableSpec> {
        Reserved4W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Wake Up event interrupt Enable in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn wke(&mut self) -> WkeW<IrqenableSpec> {
        WkeW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
End of Word count Interrupt Enable - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn eow_enable(&mut self) -> EowEnableW<IrqenableSpec> {
        EowEnableW::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reads return 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_5(&mut self) -> Reserved5W<IrqenableSpec> {
        Reserved5W::new(self, 18)
    }
}
#[doc = "This register allows to enable/disable the module internal sources of interrupt on an event-by-event basis.\n\nYou can [`read`](crate::Reg::read) this register and get [`irqenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqenableSpec;
impl crate::RegisterSpec for IrqenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqenable::R`](R) reader structure"]
impl crate::Readable for IrqenableSpec {}
#[doc = "`write(|w| ..)` method takes [`irqenable::W`](W) writer structure"]
impl crate::Writable for IrqenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQENABLE to value 0"]
impl crate::Resettable for IrqenableSpec {
    const RESET_VALUE: u32 = 0;
}
