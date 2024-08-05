#[doc = "Register `SPI_SETUP2` reader"]
pub type R = crate::R<SpiSetup2Spec>;
#[doc = "Register `SPI_SETUP2` writer"]
pub type W = crate::W<SpiSetup2Spec>;
#[doc = "Field `RCMD` reader - 7:0\\]
Read Command"]
pub type RcmdR = crate::FieldReader;
#[doc = "Field `RCMD` writer - 7:0\\]
Read Command"]
pub type RcmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUM_A_BYTES` reader - 9:8\\]
Number of address bytes to be sent. 0 = 1 byte; 1 = 2 bytes; 2 = 3 bytes; 3 = 4 bytes"]
pub type NumABytesR = crate::FieldReader;
#[doc = "Field `NUM_A_BYTES` writer - 9:8\\]
Number of address bytes to be sent. 0 = 1 byte; 1 = 2 bytes; 2 = 3 bytes; 3 = 4 bytes"]
pub type NumABytesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NUM_D_BYTES` reader - 11:10\\]
Number of dummy bytes to be used for fast read. 0 = use the value in NUM_D_BITS 1 = use 8 bits; 2 = use 16 bits; 3 = use 24 bits"]
pub type NumDBytesR = crate::FieldReader;
#[doc = "Field `NUM_D_BYTES` writer - 11:10\\]
Number of dummy bytes to be used for fast read. 0 = use the value in NUM_D_BITS 1 = use 8 bits; 2 = use 16 bits; 3 = use 24 bits"]
pub type NumDBytesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `READ_TYPE` reader - 13:12\\]
Determines if the read command is a single, dual or quad read mode command 00 ΓÇô Normal read (all data input on spi_din) 01 ΓÇô Dual read (odd bytes input on spi_din; even on spi_dout) 10 ΓÇô Normal read (all data input on spi_din) 11 ΓÇô Quad read (uses spi_qdin0/1)"]
pub type ReadTypeR = crate::FieldReader;
#[doc = "Field `READ_TYPE` writer - 13:12\\]
Determines if the read command is a single, dual or quad read mode command 00 ΓÇô Normal read (all data input on spi_din) 01 ΓÇô Dual read (odd bytes input on spi_din; even on spi_dout) 10 ΓÇô Normal read (all data input on spi_din) 11 ΓÇô Quad read (uses spi_qdin0/1)"]
pub type ReadTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - 15:14\\]
Always read as 0"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 15:14\\]
Always read as 0"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WCMD` reader - 23:16\\]
Write Command"]
pub type WcmdR = crate::FieldReader;
#[doc = "Field `WCMD` writer - 23:16\\]
Write Command"]
pub type WcmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUM_D_BITS` reader - 28:24\\]
Number of dummy bits to use if NUM_D_BYTES = 0"]
pub type NumDBitsR = crate::FieldReader;
#[doc = "Field `NUM_D_BITS` writer - 28:24\\]
Number of dummy bits to use if NUM_D_BYTES = 0"]
pub type NumDBitsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved2` reader - 31:29\\]
Always read as 0"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 31:29\\]
Always read as 0"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read Command"]
    #[inline(always)]
    pub fn rcmd(&self) -> RcmdR {
        RcmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Number of address bytes to be sent. 0 = 1 byte; 1 = 2 bytes; 2 = 3 bytes; 3 = 4 bytes"]
    #[inline(always)]
    pub fn num_a_bytes(&self) -> NumABytesR {
        NumABytesR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Number of dummy bytes to be used for fast read. 0 = use the value in NUM_D_BITS 1 = use 8 bits; 2 = use 16 bits; 3 = use 24 bits"]
    #[inline(always)]
    pub fn num_d_bytes(&self) -> NumDBytesR {
        NumDBytesR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Determines if the read command is a single, dual or quad read mode command 00 ΓÇô Normal read (all data input on spi_din) 01 ΓÇô Dual read (odd bytes input on spi_din; even on spi_dout) 10 ΓÇô Normal read (all data input on spi_din) 11 ΓÇô Quad read (uses spi_qdin0/1)"]
    #[inline(always)]
    pub fn read_type(&self) -> ReadTypeR {
        ReadTypeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Write Command"]
    #[inline(always)]
    pub fn wcmd(&self) -> WcmdR {
        WcmdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Number of dummy bits to use if NUM_D_BYTES = 0"]
    #[inline(always)]
    pub fn num_d_bits(&self) -> NumDBitsR {
        NumDBitsR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Always read as 0"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read Command"]
    #[inline(always)]
    #[must_use]
    pub fn rcmd(&mut self) -> RcmdW<SpiSetup2Spec> {
        RcmdW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Number of address bytes to be sent. 0 = 1 byte; 1 = 2 bytes; 2 = 3 bytes; 3 = 4 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn num_a_bytes(&mut self) -> NumABytesW<SpiSetup2Spec> {
        NumABytesW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Number of dummy bytes to be used for fast read. 0 = use the value in NUM_D_BITS 1 = use 8 bits; 2 = use 16 bits; 3 = use 24 bits"]
    #[inline(always)]
    #[must_use]
    pub fn num_d_bytes(&mut self) -> NumDBytesW<SpiSetup2Spec> {
        NumDBytesW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Determines if the read command is a single, dual or quad read mode command 00 ΓÇô Normal read (all data input on spi_din) 01 ΓÇô Dual read (odd bytes input on spi_din; even on spi_dout) 10 ΓÇô Normal read (all data input on spi_din) 11 ΓÇô Quad read (uses spi_qdin0/1)"]
    #[inline(always)]
    #[must_use]
    pub fn read_type(&mut self) -> ReadTypeW<SpiSetup2Spec> {
        ReadTypeW::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SpiSetup2Spec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Write Command"]
    #[inline(always)]
    #[must_use]
    pub fn wcmd(&mut self) -> WcmdW<SpiSetup2Spec> {
        WcmdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Number of dummy bits to use if NUM_D_BYTES = 0"]
    #[inline(always)]
    #[must_use]
    pub fn num_d_bits(&mut self) -> NumDBitsW<SpiSetup2Spec> {
        NumDBitsW::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Always read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SpiSetup2Spec> {
        Reserved2W::new(self, 29)
    }
}
#[doc = "Memory Mapped SPI Setup2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_setup2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_setup2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSetup2Spec;
impl crate::RegisterSpec for SpiSetup2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_setup2::R`](R) reader structure"]
impl crate::Readable for SpiSetup2Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_setup2::W`](W) writer structure"]
impl crate::Writable for SpiSetup2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_SETUP2 to value 0"]
impl crate::Resettable for SpiSetup2Spec {
    const RESET_VALUE: u32 = 0;
}
