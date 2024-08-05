#[doc = "Register `SPI_HOST_IRQ` reader"]
pub type R = crate::R<SpiHostIrqSpec>;
#[doc = "Register `SPI_HOST_IRQ` writer"]
pub type W = crate::W<SpiHostIrqSpec>;
#[doc = "Field `host_irq` reader - 0:0\\]
RESERVED"]
pub type HostIrqR = crate::BitReader;
#[doc = "Field `host_irq` writer - 0:0\\]
RESERVED"]
pub type HostIrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RESERVED"]
    #[inline(always)]
    pub fn host_irq(&self) -> HostIrqR {
        HostIrqR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn host_irq(&mut self) -> HostIrqW<SpiHostIrqSpec> {
        HostIrqW::new(self, 0)
    }
}
#[doc = "SPI_HOST_IRQ\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_host_irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_host_irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiHostIrqSpec;
impl crate::RegisterSpec for SpiHostIrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_host_irq::R`](R) reader structure"]
impl crate::Readable for SpiHostIrqSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_host_irq::W`](W) writer structure"]
impl crate::Writable for SpiHostIrqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_HOST_IRQ to value 0"]
impl crate::Resettable for SpiHostIrqSpec {
    const RESET_VALUE: u32 = 0;
}
