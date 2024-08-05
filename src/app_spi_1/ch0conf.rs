#[doc = "Register `CH0CONF` reader"]
pub type R = crate::R<Ch0confSpec>;
#[doc = "Register `CH0CONF` writer"]
pub type W = crate::W<Ch0confSpec>;
#[doc = "Field `PHA` reader - 0:0\\]
SPICLK phase - (RW )"]
pub type PhaR = crate::BitReader;
#[doc = "Field `PHA` writer - 0:0\\]
SPICLK phase - (RW )"]
pub type PhaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - 1:1\\]
SPICLK polarity - (RW )"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - 1:1\\]
SPICLK polarity - (RW )"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKD` reader - 5:2\\]
Frequency divider for SPICLK \\[only when the module is a Master SPI device\\]
A programmable clock divider divides the SPI reference clock \\[CLKSPIREF\\]
with a 4-bit value and results in a new clock SPICLK available to shift-in and shift-out data By default the clock divider ratio has a power of two granularity when MCSPI_CHCONF\\[CLKG\\]
is cleared Otherwise this register is the 4 LSB bit of a 12-bit register concatenated with clock divider extension MCSPI_CHCTRL\\[EXTCLK\\]
registerThe value description below defines the clock ratio when MCSPI_CHCONF\\[CLKG\\]
is set to 0 - (RW )"]
pub type ClkdR = crate::FieldReader;
#[doc = "Field `CLKD` writer - 5:2\\]
Frequency divider for SPICLK \\[only when the module is a Master SPI device\\]
A programmable clock divider divides the SPI reference clock \\[CLKSPIREF\\]
with a 4-bit value and results in a new clock SPICLK available to shift-in and shift-out data By default the clock divider ratio has a power of two granularity when MCSPI_CHCONF\\[CLKG\\]
is cleared Otherwise this register is the 4 LSB bit of a 12-bit register concatenated with clock divider extension MCSPI_CHCTRL\\[EXTCLK\\]
registerThe value description below defines the clock ratio when MCSPI_CHCONF\\[CLKG\\]
is set to 0 - (RW )"]
pub type ClkdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPOL` reader - 6:6\\]
SPIEN polarity - (RW )"]
pub type EpolR = crate::BitReader;
#[doc = "Field `EPOL` writer - 6:6\\]
SPIEN polarity - (RW )"]
pub type EpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WL` reader - 11:7\\]
SPI word length - (RW )"]
pub type WlR = crate::FieldReader;
#[doc = "Field `WL` writer - 11:7\\]
SPI word length - (RW )"]
pub type WlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRM` reader - 13:12\\]
Transmit/Receive modes - (RW )"]
pub type TrmR = crate::FieldReader;
#[doc = "Field `TRM` writer - 13:12\\]
Transmit/Receive modes - (RW )"]
pub type TrmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMAW` reader - 14:14\\]
DMA Write request The DMA Write request line is asserted when The channel is enabled and the transmitter register of the channel is empty The DMA Write request line is deasserted on load completion of the transmitter register of the channel - (RW )"]
pub type DmawR = crate::BitReader;
#[doc = "Field `DMAW` writer - 14:14\\]
DMA Write request The DMA Write request line is asserted when The channel is enabled and the transmitter register of the channel is empty The DMA Write request line is deasserted on load completion of the transmitter register of the channel - (RW )"]
pub type DmawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - 15:15\\]
DMA Read request The DMA Read request line is asserted when the channel is enabled and a new data is available in the receive register of the channel The DMA Read request line is deasserted on read completion of the receive register of the channel - (RW )"]
pub type DmarR = crate::BitReader;
#[doc = "Field `DMAR` writer - 15:15\\]
DMA Read request The DMA Read request line is asserted when the channel is enabled and a new data is available in the receive register of the channel The DMA Read request line is deasserted on read completion of the receive register of the channel - (RW )"]
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPE0` reader - 16:16\\]
Transmission Enable for data line 0 \\[SPIDATAGZEN\\[0\\]\\]
- (RW )"]
pub type Dpe0R = crate::BitReader;
#[doc = "Field `DPE0` writer - 16:16\\]
Transmission Enable for data line 0 \\[SPIDATAGZEN\\[0\\]\\]
- (RW )"]
pub type Dpe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPE1` reader - 17:17\\]
Transmission Enable for data line 1 \\[SPIDATAGZEN\\[1\\]\\]
- (RW )"]
pub type Dpe1R = crate::BitReader;
#[doc = "Field `DPE1` writer - 17:17\\]
Transmission Enable for data line 1 \\[SPIDATAGZEN\\[1\\]\\]
- (RW )"]
pub type Dpe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IS` reader - 18:18\\]
Input Select - (RW )"]
pub type IsR = crate::BitReader;
#[doc = "Field `IS` writer - 18:18\\]
Input Select - (RW )"]
pub type IsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TURBO` reader - 19:19\\]
Turbo mode - (RW )"]
pub type TurboR = crate::BitReader;
#[doc = "Field `TURBO` writer - 19:19\\]
Turbo mode - (RW )"]
pub type TurboW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE` reader - 20:20\\]
Manual SPIEN assertion to keep SPIEN active between SPI words \\[single channel master mode only\\]
- (RW )"]
pub type ForceR = crate::BitReader;
#[doc = "Field `FORCE` writer - 20:20\\]
Manual SPIEN assertion to keep SPIEN active between SPI words \\[single channel master mode only\\]
- (RW )"]
pub type ForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIENSLV` reader - 22:21\\]
Channel 0 only and slave mode only: SPI slave select signal detection Reserved bits for other cases - (RW )"]
pub type SpienslvR = crate::FieldReader;
#[doc = "Field `SPIENSLV` writer - 22:21\\]
Channel 0 only and slave mode only: SPI slave select signal detection Reserved bits for other cases - (RW )"]
pub type SpienslvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SBE` reader - 23:23\\]
Start bit enable for SPI transfer - (RW )"]
pub type SbeR = crate::BitReader;
#[doc = "Field `SBE` writer - 23:23\\]
Start bit enable for SPI transfer - (RW )"]
pub type SbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBPOL` reader - 24:24\\]
Start bit polarity - (RW )"]
pub type SbpolR = crate::BitReader;
#[doc = "Field `SBPOL` writer - 24:24\\]
Start bit polarity - (RW )"]
pub type SbpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCS0` reader - 26:25\\]
Chip Select Time Control This 2-bits field defines the number of interface clock cycles between CS toggling and first or last edge of SPI clock - (RW )"]
pub type Tcs0R = crate::FieldReader;
#[doc = "Field `TCS0` writer - 26:25\\]
Chip Select Time Control This 2-bits field defines the number of interface clock cycles between CS toggling and first or last edge of SPI clock - (RW )"]
pub type Tcs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FFEW` reader - 27:27\\]
FIFO enabled for Transmit:Only one channel can have this bit field set - (RW )"]
pub type FfewR = crate::BitReader;
#[doc = "Field `FFEW` writer - 27:27\\]
FIFO enabled for Transmit:Only one channel can have this bit field set - (RW )"]
pub type FfewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFER` reader - 28:28\\]
FIFO enabled for receive:Only one channel can have this bit field set - (RW )"]
pub type FferR = crate::BitReader;
#[doc = "Field `FFER` writer - 28:28\\]
FIFO enabled for receive:Only one channel can have this bit field set - (RW )"]
pub type FferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKG` reader - 29:29\\]
Clock divider granularity This register defines the granularity of channel clock divider: power of two or one clock cycle granularity When this bit is set the register MCSPI_CHCTRL\\[EXTCLK\\]
must be configured to reach a maximum of 4096 clock divider ratio Then The clock divider ratio is a concatenation of MCSPI_CHCONF\\[CLKD\\]
and MCSPI_CHCTRL\\[EXTCLK\\]
values - (RW )"]
pub type ClkgR = crate::BitReader;
#[doc = "Field `CLKG` writer - 29:29\\]
Clock divider granularity This register defines the granularity of channel clock divider: power of two or one clock cycle granularity When this bit is set the register MCSPI_CHCTRL\\[EXTCLK\\]
must be configured to reach a maximum of 4096 clock divider ratio Then The clock divider ratio is a concatenation of MCSPI_CHCONF\\[CLKD\\]
and MCSPI_CHCTRL\\[EXTCLK\\]
values - (RW )"]
pub type ClkgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_0` reader - 31:30\\]
read returns 0 - (RO )"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED_0` writer - 31:30\\]
read returns 0 - (RO )"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SPICLK phase - (RW )"]
    #[inline(always)]
    pub fn pha(&self) -> PhaR {
        PhaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SPICLK polarity - (RW )"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - 5:2\\]
Frequency divider for SPICLK \\[only when the module is a Master SPI device\\]
A programmable clock divider divides the SPI reference clock \\[CLKSPIREF\\]
with a 4-bit value and results in a new clock SPICLK available to shift-in and shift-out data By default the clock divider ratio has a power of two granularity when MCSPI_CHCONF\\[CLKG\\]
is cleared Otherwise this register is the 4 LSB bit of a 12-bit register concatenated with clock divider extension MCSPI_CHCTRL\\[EXTCLK\\]
registerThe value description below defines the clock ratio when MCSPI_CHCONF\\[CLKG\\]
is set to 0 - (RW )"]
    #[inline(always)]
    pub fn clkd(&self) -> ClkdR {
        ClkdR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
SPIEN polarity - (RW )"]
    #[inline(always)]
    pub fn epol(&self) -> EpolR {
        EpolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - 11:7\\]
SPI word length - (RW )"]
    #[inline(always)]
    pub fn wl(&self) -> WlR {
        WlR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Transmit/Receive modes - (RW )"]
    #[inline(always)]
    pub fn trm(&self) -> TrmR {
        TrmR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
DMA Write request The DMA Write request line is asserted when The channel is enabled and the transmitter register of the channel is empty The DMA Write request line is deasserted on load completion of the transmitter register of the channel - (RW )"]
    #[inline(always)]
    pub fn dmaw(&self) -> DmawR {
        DmawR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
DMA Read request The DMA Read request line is asserted when the channel is enabled and a new data is available in the receive register of the channel The DMA Read request line is deasserted on read completion of the receive register of the channel - (RW )"]
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Transmission Enable for data line 0 \\[SPIDATAGZEN\\[0\\]\\]
- (RW )"]
    #[inline(always)]
    pub fn dpe0(&self) -> Dpe0R {
        Dpe0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Transmission Enable for data line 1 \\[SPIDATAGZEN\\[1\\]\\]
- (RW )"]
    #[inline(always)]
    pub fn dpe1(&self) -> Dpe1R {
        Dpe1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input Select - (RW )"]
    #[inline(always)]
    pub fn is(&self) -> IsR {
        IsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Turbo mode - (RW )"]
    #[inline(always)]
    pub fn turbo(&self) -> TurboR {
        TurboR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Manual SPIEN assertion to keep SPIEN active between SPI words \\[single channel master mode only\\]
- (RW )"]
    #[inline(always)]
    pub fn force(&self) -> ForceR {
        ForceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Channel 0 only and slave mode only: SPI slave select signal detection Reserved bits for other cases - (RW )"]
    #[inline(always)]
    pub fn spienslv(&self) -> SpienslvR {
        SpienslvR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Start bit enable for SPI transfer - (RW )"]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Start bit polarity - (RW )"]
    #[inline(always)]
    pub fn sbpol(&self) -> SbpolR {
        SbpolR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Chip Select Time Control This 2-bits field defines the number of interface clock cycles between CS toggling and first or last edge of SPI clock - (RW )"]
    #[inline(always)]
    pub fn tcs0(&self) -> Tcs0R {
        Tcs0R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
FIFO enabled for Transmit:Only one channel can have this bit field set - (RW )"]
    #[inline(always)]
    pub fn ffew(&self) -> FfewR {
        FfewR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
FIFO enabled for receive:Only one channel can have this bit field set - (RW )"]
    #[inline(always)]
    pub fn ffer(&self) -> FferR {
        FferR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Clock divider granularity This register defines the granularity of channel clock divider: power of two or one clock cycle granularity When this bit is set the register MCSPI_CHCTRL\\[EXTCLK\\]
must be configured to reach a maximum of 4096 clock divider ratio Then The clock divider ratio is a concatenation of MCSPI_CHCONF\\[CLKD\\]
and MCSPI_CHCTRL\\[EXTCLK\\]
values - (RW )"]
    #[inline(always)]
    pub fn clkg(&self) -> ClkgR {
        ClkgR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
read returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SPICLK phase - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn pha(&mut self) -> PhaW<Ch0confSpec> {
        PhaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SPICLK polarity - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> PolW<Ch0confSpec> {
        PolW::new(self, 1)
    }
    #[doc = "Bits 2:5 - 5:2\\]
Frequency divider for SPICLK \\[only when the module is a Master SPI device\\]
A programmable clock divider divides the SPI reference clock \\[CLKSPIREF\\]
with a 4-bit value and results in a new clock SPICLK available to shift-in and shift-out data By default the clock divider ratio has a power of two granularity when MCSPI_CHCONF\\[CLKG\\]
is cleared Otherwise this register is the 4 LSB bit of a 12-bit register concatenated with clock divider extension MCSPI_CHCTRL\\[EXTCLK\\]
registerThe value description below defines the clock ratio when MCSPI_CHCONF\\[CLKG\\]
is set to 0 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn clkd(&mut self) -> ClkdW<Ch0confSpec> {
        ClkdW::new(self, 2)
    }
    #[doc = "Bit 6 - 6:6\\]
SPIEN polarity - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn epol(&mut self) -> EpolW<Ch0confSpec> {
        EpolW::new(self, 6)
    }
    #[doc = "Bits 7:11 - 11:7\\]
SPI word length - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn wl(&mut self) -> WlW<Ch0confSpec> {
        WlW::new(self, 7)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Transmit/Receive modes - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn trm(&mut self) -> TrmW<Ch0confSpec> {
        TrmW::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
DMA Write request The DMA Write request line is asserted when The channel is enabled and the transmitter register of the channel is empty The DMA Write request line is deasserted on load completion of the transmitter register of the channel - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn dmaw(&mut self) -> DmawW<Ch0confSpec> {
        DmawW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
DMA Read request The DMA Read request line is asserted when the channel is enabled and a new data is available in the receive register of the channel The DMA Read request line is deasserted on read completion of the receive register of the channel - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DmarW<Ch0confSpec> {
        DmarW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Transmission Enable for data line 0 \\[SPIDATAGZEN\\[0\\]\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn dpe0(&mut self) -> Dpe0W<Ch0confSpec> {
        Dpe0W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Transmission Enable for data line 1 \\[SPIDATAGZEN\\[1\\]\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn dpe1(&mut self) -> Dpe1W<Ch0confSpec> {
        Dpe1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input Select - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn is(&mut self) -> IsW<Ch0confSpec> {
        IsW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Turbo mode - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn turbo(&mut self) -> TurboW<Ch0confSpec> {
        TurboW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Manual SPIEN assertion to keep SPIEN active between SPI words \\[single channel master mode only\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn force(&mut self) -> ForceW<Ch0confSpec> {
        ForceW::new(self, 20)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Channel 0 only and slave mode only: SPI slave select signal detection Reserved bits for other cases - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn spienslv(&mut self) -> SpienslvW<Ch0confSpec> {
        SpienslvW::new(self, 21)
    }
    #[doc = "Bit 23 - 23:23\\]
Start bit enable for SPI transfer - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn sbe(&mut self) -> SbeW<Ch0confSpec> {
        SbeW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Start bit polarity - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn sbpol(&mut self) -> SbpolW<Ch0confSpec> {
        SbpolW::new(self, 24)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Chip Select Time Control This 2-bits field defines the number of interface clock cycles between CS toggling and first or last edge of SPI clock - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn tcs0(&mut self) -> Tcs0W<Ch0confSpec> {
        Tcs0W::new(self, 25)
    }
    #[doc = "Bit 27 - 27:27\\]
FIFO enabled for Transmit:Only one channel can have this bit field set - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ffew(&mut self) -> FfewW<Ch0confSpec> {
        FfewW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
FIFO enabled for receive:Only one channel can have this bit field set - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ffer(&mut self) -> FferW<Ch0confSpec> {
        FferW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Clock divider granularity This register defines the granularity of channel clock divider: power of two or one clock cycle granularity When this bit is set the register MCSPI_CHCTRL\\[EXTCLK\\]
must be configured to reach a maximum of 4096 clock divider ratio Then The clock divider ratio is a concatenation of MCSPI_CHCONF\\[CLKD\\]
and MCSPI_CHCTRL\\[EXTCLK\\]
values - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn clkg(&mut self) -> ClkgW<Ch0confSpec> {
        ClkgW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
read returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_0(&mut self) -> Reserved0W<Ch0confSpec> {
        Reserved0W::new(self, 30)
    }
}
#[doc = "This register is dedicated to the configuration of the channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0confSpec;
impl crate::RegisterSpec for Ch0confSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0conf::R`](R) reader structure"]
impl crate::Readable for Ch0confSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0conf::W`](W) writer structure"]
impl crate::Writable for Ch0confSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0CONF to value 0"]
impl crate::Resettable for Ch0confSpec {
    const RESET_VALUE: u32 = 0;
}
