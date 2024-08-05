#[doc = "Register `SPI_DATA2` reader"]
pub type R = crate::R<SpiData2Spec>;
#[doc = "Register `SPI_DATA2` writer"]
pub type W = crate::W<SpiData2Spec>;
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
    pub fn data(&mut self) -> DataW<SpiData2Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "SPI Data Register (SPIDR2)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_data2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_data2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiData2Spec;
impl crate::RegisterSpec for SpiData2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_data2::R`](R) reader structure"]
impl crate::Readable for SpiData2Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_data2::W`](W) writer structure"]
impl crate::Writable for SpiData2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_DATA2 to value 0"]
impl crate::Resettable for SpiData2Spec {
    const RESET_VALUE: u32 = 0;
}
