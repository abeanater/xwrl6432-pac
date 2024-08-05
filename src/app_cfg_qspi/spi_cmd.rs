#[doc = "Register `SPI_CMD` reader"]
pub type R = crate::R<SpiCmdSpec>;
#[doc = "Register `SPI_CMD` writer"]
pub type W = crate::W<SpiCmdSpec>;
#[doc = "Field `FLEN` reader - 11:0\\]
Frame Length 0- 1 word 1- 2 words ΓÇª 4095 ΓÇô 4096 words"]
pub type FlenR = crate::FieldReader<u16>;
#[doc = "Field `FLEN` writer - 11:0\\]
Frame Length 0- 1 word 1- 2 words ΓÇª 4095 ΓÇô 4096 words"]
pub type FlenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `Reserved1` reader - 13:12\\]
Always read as 0"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 13:12\\]
Always read as 0"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WIRQ` reader - 14:14\\]
Word count interrupt enable"]
pub type WirqR = crate::BitReader;
#[doc = "Field `WIRQ` writer - 14:14\\]
Word count interrupt enable"]
pub type WirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIRQ` reader - 15:15\\]
Frame count interrupt enable"]
pub type FirqR = crate::BitReader;
#[doc = "Field `FIRQ` writer - 15:15\\]
Frame count interrupt enable"]
pub type FirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD` reader - 18:16\\]
Transfer command 000- Reserved 001- 4 pin Read Single 010- 4 pin Write Single 011- 4 pin Read Dual 100 ΓÇô Reserved 101 ΓÇô 3 pin Read Single 110 ΓÇô 3 pin Write Single 111 ΓÇô 6 pin Read Quad"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - 18:16\\]
Transfer command 000- Reserved 001- 4 pin Read Single 010- 4 pin Write Single 011- 4 pin Read Dual 100 ΓÇô Reserved 101 ΓÇô 3 pin Read Single 110 ΓÇô 3 pin Write Single 111 ΓÇô 6 pin Read Quad"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WLEN` reader - 25:19\\]
Word length. Sets the size of the individual transfers from 1 ΓÇô 128 bits 0- 1 bit 1- 2 bits ΓÇª 127 ΓÇô 128 bits"]
pub type WlenR = crate::FieldReader;
#[doc = "Field `WLEN` writer - 25:19\\]
Word length. Sets the size of the individual transfers from 1 ΓÇô 128 bits 0- 1 bit 1- 2 bits ΓÇª 127 ΓÇô 128 bits"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - 27:26\\]
Always read as 0"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 27:26\\]
Always read as 0"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSNUM` reader - 29:28\\]
Device select. Sets the active chip select for the transfer 00- Chip Select 0 active 01- Chip Select 1 active 10- Chip Select 2 active 11- Chip Select 3 active"]
pub type CsnumR = crate::FieldReader;
#[doc = "Field `CSNUM` writer - 29:28\\]
Device select. Sets the active chip select for the transfer 00- Chip Select 0 active 01- Chip Select 1 active 10- Chip Select 2 active 11- Chip Select 3 active"]
pub type CsnumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - 31:30\\]
Always read as 0"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 31:30\\]
Always read as 0"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Frame Length 0- 1 word 1- 2 words ΓÇª 4095 ΓÇô 4096 words"]
    #[inline(always)]
    pub fn flen(&self) -> FlenR {
        FlenR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Word count interrupt enable"]
    #[inline(always)]
    pub fn wirq(&self) -> WirqR {
        WirqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Frame count interrupt enable"]
    #[inline(always)]
    pub fn firq(&self) -> FirqR {
        FirqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Transfer command 000- Reserved 001- 4 pin Read Single 010- 4 pin Write Single 011- 4 pin Read Dual 100 ΓÇô Reserved 101 ΓÇô 3 pin Read Single 110 ΓÇô 3 pin Write Single 111 ΓÇô 6 pin Read Quad"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:25 - 25:19\\]
Word length. Sets the size of the individual transfers from 1 ΓÇô 128 bits 0- 1 bit 1- 2 bits ΓÇª 127 ΓÇô 128 bits"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 19) & 0x7f) as u8)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Device select. Sets the active chip select for the transfer 00- Chip Select 0 active 01- Chip Select 1 active 10- Chip Select 2 active 11- Chip Select 3 active"]
    #[inline(always)]
    pub fn csnum(&self) -> CsnumR {
        CsnumR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Frame Length 0- 1 word 1- 2 words ΓÇª 4095 ΓÇô 4096 words"]
    #[inline(always)]
    #[must_use]
    pub fn flen(&mut self) -> FlenW<SpiCmdSpec> {
        FlenW::new(self, 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SpiCmdSpec> {
        Reserved1W::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
Word count interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wirq(&mut self) -> WirqW<SpiCmdSpec> {
        WirqW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Frame count interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn firq(&mut self) -> FirqW<SpiCmdSpec> {
        FirqW::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Transfer command 000- Reserved 001- 4 pin Read Single 010- 4 pin Write Single 011- 4 pin Read Dual 100 ΓÇô Reserved 101 ΓÇô 3 pin Read Single 110 ΓÇô 3 pin Write Single 111 ΓÇô 6 pin Read Quad"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<SpiCmdSpec> {
        CmdW::new(self, 16)
    }
    #[doc = "Bits 19:25 - 25:19\\]
Word length. Sets the size of the individual transfers from 1 ΓÇô 128 bits 0- 1 bit 1- 2 bits ΓÇª 127 ΓÇô 128 bits"]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WlenW<SpiCmdSpec> {
        WlenW::new(self, 19)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SpiCmdSpec> {
        Reserved2W::new(self, 26)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Device select. Sets the active chip select for the transfer 00- Chip Select 0 active 01- Chip Select 1 active 10- Chip Select 2 active 11- Chip Select 3 active"]
    #[inline(always)]
    #[must_use]
    pub fn csnum(&mut self) -> CsnumW<SpiCmdSpec> {
        CsnumW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SpiCmdSpec> {
        Reserved3W::new(self, 30)
    }
}
#[doc = "SPI Command Register (SPICR)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCmdSpec;
impl crate::RegisterSpec for SpiCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_cmd::R`](R) reader structure"]
impl crate::Readable for SpiCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_cmd::W`](W) writer structure"]
impl crate::Writable for SpiCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CMD to value 0"]
impl crate::Resettable for SpiCmdSpec {
    const RESET_VALUE: u32 = 0;
}
