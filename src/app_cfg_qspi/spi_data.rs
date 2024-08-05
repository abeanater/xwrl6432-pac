#[doc = "Register `SPI_DATA` reader"]
pub type R = crate::R<SpiDataSpec>;
#[doc = "Register `SPI_DATA` writer"]
pub type W = crate::W<SpiDataSpec>;
#[doc = "Field `DATA` reader - 31:0\\]
Data register for read and write operations"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Data register for read and write operations"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Data register for read and write operations"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Data register for read and write operations"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<SpiDataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "SPI Data Register (SPIDR)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiDataSpec;
impl crate::RegisterSpec for SpiDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_data::R`](R) reader structure"]
impl crate::Readable for SpiDataSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_data::W`](W) writer structure"]
impl crate::Writable for SpiDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_DATA to value 0"]
impl crate::Resettable for SpiDataSpec {
    const RESET_VALUE: u32 = 0;
}
