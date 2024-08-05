#[doc = "Register `SPI_CLOCK_CNTRL` reader"]
pub type R = crate::R<SpiClockCntrlSpec>;
#[doc = "Register `SPI_CLOCK_CNTRL` writer"]
pub type W = crate::W<SpiClockCntrlSpec>;
#[doc = "Field `DCLK_DIV` reader - 15:0\\]
Serial data clock divide by ratio"]
pub type DclkDivR = crate::FieldReader<u16>;
#[doc = "Field `DCLK_DIV` writer - 15:0\\]
Serial data clock divide by ratio"]
pub type DclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CLKEN` reader - 31:31\\]
Clock Enable. 0- Data clock is turned off 1- Data clock is enabled"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - 31:31\\]
Clock Enable. 0- Data clock is turned off 1- Data clock is enabled"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Serial data clock divide by ratio"]
    #[inline(always)]
    pub fn dclk_div(&self) -> DclkDivR {
        DclkDivR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Clock Enable. 0- Data clock is turned off 1- Data clock is enabled"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Serial data clock divide by ratio"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_div(&mut self) -> DclkDivW<SpiClockCntrlSpec> {
        DclkDivW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Clock Enable. 0- Data clock is turned off 1- Data clock is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<SpiClockCntrlSpec> {
        ClkenW::new(self, 31)
    }
}
#[doc = "SPI Clock Control Register (SPICC)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_clock_cntrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_clock_cntrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiClockCntrlSpec;
impl crate::RegisterSpec for SpiClockCntrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_clock_cntrl::R`](R) reader structure"]
impl crate::Readable for SpiClockCntrlSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_clock_cntrl::W`](W) writer structure"]
impl crate::Writable for SpiClockCntrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CLOCK_CNTRL to value 0"]
impl crate::Resettable for SpiClockCntrlSpec {
    const RESET_VALUE: u32 = 0;
}
