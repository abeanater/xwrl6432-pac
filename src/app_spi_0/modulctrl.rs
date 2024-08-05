#[doc = "Register `MODULCTRL` reader"]
pub type R = crate::R<ModulctrlSpec>;
#[doc = "Register `MODULCTRL` writer"]
pub type W = crate::W<ModulctrlSpec>;
#[doc = "Field `SINGLE` reader - 0:0\\]
Single channel / Multi Channel \\[master mode only\\]
- (RW )"]
pub type SingleR = crate::BitReader;
#[doc = "Field `SINGLE` writer - 0:0\\]
Single channel / Multi Channel \\[master mode only\\]
- (RW )"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN34` reader - 1:1\\]
Pin mode selection: This register is used to configure the SPI pin mode in master or slave mode If asserted the controller only use SIMOSOMI and SPICLK clock pin for spi transfers - (RW )"]
pub type Pin34R = crate::BitReader;
#[doc = "Field `PIN34` writer - 1:1\\]
Pin mode selection: This register is used to configure the SPI pin mode in master or slave mode If asserted the controller only use SIMOSOMI and SPICLK clock pin for spi transfers - (RW )"]
pub type Pin34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - 2:2\\]
Master/ Slave - (RW )"]
pub type MsR = crate::BitReader;
#[doc = "Field `MS` writer - 2:2\\]
Master/ Slave - (RW )"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTEM_TEST` reader - 3:3\\]
Enables the system test mode - (RW )"]
pub type SystemTestR = crate::BitReader;
#[doc = "Field `SYSTEM_TEST` writer - 3:3\\]
Enables the system test mode - (RW )"]
pub type SystemTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITDLY` reader - 6:4\\]
Initial spi delay for first transfer: This register is an option only available in SINGLE master mode The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled This Delay is based on SPI output frequency clock No clock output provided to the boundary and chip select is not active in 4 pin mode within this period - (RW )"]
pub type InitdlyR = crate::FieldReader;
#[doc = "Field `INITDLY` writer - 6:4\\]
Initial spi delay for first transfer: This register is an option only available in SINGLE master mode The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled This Delay is based on SPI output frequency clock No clock output provided to the boundary and chip select is not active in 4 pin mode within this period - (RW )"]
pub type InitdlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MOA` reader - 7:7\\]
Multiple word ocp access: This register can only be used when a channel is enabled using a FIFO It allows the system to perform multiple SPI word access for a single 32-bit OCP word access This is possible for WL &lt; 16 - (RW )"]
pub type MoaR = crate::BitReader;
#[doc = "Field `MOA` writer - 7:7\\]
Multiple word ocp access: This register can only be used when a channel is enabled using a FIFO It allows the system to perform multiple SPI word access for a single 32-bit OCP word access This is possible for WL &lt; 16 - (RW )"]
pub type MoaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDAA` reader - 8:8\\]
FIFO DMA Address 256-bit aligned This register is used when a FIFO is managed by the module and DMA connected to the controller provides only 256 bit aligned address If this bit is set the enabled channel which uses the FIFO has its datas managed through MCSPI_DAFTX and MCSPI_DAFRX registers instead of MCSPI_TX\\[i\\]
and MCSPI_RX\\[i\\]
registers - (RW )"]
pub type FdaaR = crate::BitReader;
#[doc = "Field `FDAA` writer - 8:8\\]
FIFO DMA Address 256-bit aligned This register is used when a FIFO is managed by the module and DMA connected to the controller provides only 256 bit aligned address If this bit is set the enabled channel which uses the FIFO has its datas managed through MCSPI_DAFTX and MCSPI_DAFRX registers instead of MCSPI_TX\\[i\\]
and MCSPI_RX\\[i\\]
registers - (RW )"]
pub type FdaaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_11` reader - 31:9\\]
Reads returns 0 - (RO )"]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_11` writer - 31:9\\]
Reads returns 0 - (RO )"]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Single channel / Multi Channel \\[master mode only\\]
- (RW )"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Pin mode selection: This register is used to configure the SPI pin mode in master or slave mode If asserted the controller only use SIMOSOMI and SPICLK clock pin for spi transfers - (RW )"]
    #[inline(always)]
    pub fn pin34(&self) -> Pin34R {
        Pin34R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Master/ Slave - (RW )"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the system test mode - (RW )"]
    #[inline(always)]
    pub fn system_test(&self) -> SystemTestR {
        SystemTestR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Initial spi delay for first transfer: This register is an option only available in SINGLE master mode The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled This Delay is based on SPI output frequency clock No clock output provided to the boundary and chip select is not active in 4 pin mode within this period - (RW )"]
    #[inline(always)]
    pub fn initdly(&self) -> InitdlyR {
        InitdlyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Multiple word ocp access: This register can only be used when a channel is enabled using a FIFO It allows the system to perform multiple SPI word access for a single 32-bit OCP word access This is possible for WL &lt; 16 - (RW )"]
    #[inline(always)]
    pub fn moa(&self) -> MoaR {
        MoaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
FIFO DMA Address 256-bit aligned This register is used when a FIFO is managed by the module and DMA connected to the controller provides only 256 bit aligned address If this bit is set the enabled channel which uses the FIFO has its datas managed through MCSPI_DAFTX and MCSPI_DAFRX registers instead of MCSPI_TX\\[i\\]
and MCSPI_RX\\[i\\]
registers - (RW )"]
    #[inline(always)]
    pub fn fdaa(&self) -> FdaaR {
        FdaaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Single channel / Multi Channel \\[master mode only\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SingleW<ModulctrlSpec> {
        SingleW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Pin mode selection: This register is used to configure the SPI pin mode in master or slave mode If asserted the controller only use SIMOSOMI and SPICLK clock pin for spi transfers - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn pin34(&mut self) -> Pin34W<ModulctrlSpec> {
        Pin34W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Master/ Slave - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<ModulctrlSpec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the system test mode - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn system_test(&mut self) -> SystemTestW<ModulctrlSpec> {
        SystemTestW::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Initial spi delay for first transfer: This register is an option only available in SINGLE master mode The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled This Delay is based on SPI output frequency clock No clock output provided to the boundary and chip select is not active in 4 pin mode within this period - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn initdly(&mut self) -> InitdlyW<ModulctrlSpec> {
        InitdlyW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
Multiple word ocp access: This register can only be used when a channel is enabled using a FIFO It allows the system to perform multiple SPI word access for a single 32-bit OCP word access This is possible for WL &lt; 16 - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn moa(&mut self) -> MoaW<ModulctrlSpec> {
        MoaW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
FIFO DMA Address 256-bit aligned This register is used when a FIFO is managed by the module and DMA connected to the controller provides only 256 bit aligned address If this bit is set the enabled channel which uses the FIFO has its datas managed through MCSPI_DAFTX and MCSPI_DAFRX registers instead of MCSPI_TX\\[i\\]
and MCSPI_RX\\[i\\]
registers - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn fdaa(&mut self) -> FdaaW<ModulctrlSpec> {
        FdaaW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_11(&mut self) -> Reserved11W<ModulctrlSpec> {
        Reserved11W::new(self, 9)
    }
}
#[doc = "This register is dedicated to the configuration of the serial port interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`modulctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modulctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModulctrlSpec;
impl crate::RegisterSpec for ModulctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modulctrl::R`](R) reader structure"]
impl crate::Readable for ModulctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`modulctrl::W`](W) writer structure"]
impl crate::Writable for ModulctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODULCTRL to value 0"]
impl crate::Resettable for ModulctrlSpec {
    const RESET_VALUE: u32 = 0;
}
