#[doc = "Register `SPI_SWITCH` reader"]
pub type R = crate::R<SpiSwitchSpec>;
#[doc = "Register `SPI_SWITCH` writer"]
pub type W = crate::W<SpiSwitchSpec>;
#[doc = "Field `MMPT_S` reader - 0:0\\]
MMPT select. If 0 (default) config port has is selected to control config of core SPI module. If 1, Memory Mapped Protocol Translator is selected to control config port of core SPI module."]
pub type MmptSR = crate::BitReader;
#[doc = "Field `MMPT_S` writer - 0:0\\]
MMPT select. If 0 (default) config port has is selected to control config of core SPI module. If 1, Memory Mapped Protocol Translator is selected to control config port of core SPI module."]
pub type MmptSW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MM_INT_EN` reader - 1:1\\]
Memory Mapped mode interrupt enable. 0 ΓÇô Interrupts are disabled during memory mapped operations 1 ΓÇô Word Count interrupt is enabled for memory mapped operations"]
pub type MmIntEnR = crate::BitReader;
#[doc = "Field `MM_INT_EN` writer - 1:1\\]
Memory Mapped mode interrupt enable. 0 ΓÇô Interrupts are disabled during memory mapped operations 1 ΓÇô Word Count interrupt is enabled for memory mapped operations"]
pub type MmIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MMPT select. If 0 (default) config port has is selected to control config of core SPI module. If 1, Memory Mapped Protocol Translator is selected to control config port of core SPI module."]
    #[inline(always)]
    pub fn mmpt_s(&self) -> MmptSR {
        MmptSR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Mapped mode interrupt enable. 0 ΓÇô Interrupts are disabled during memory mapped operations 1 ΓÇô Word Count interrupt is enabled for memory mapped operations"]
    #[inline(always)]
    pub fn mm_int_en(&self) -> MmIntEnR {
        MmIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MMPT select. If 0 (default) config port has is selected to control config of core SPI module. If 1, Memory Mapped Protocol Translator is selected to control config port of core SPI module."]
    #[inline(always)]
    #[must_use]
    pub fn mmpt_s(&mut self) -> MmptSW<SpiSwitchSpec> {
        MmptSW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Mapped mode interrupt enable. 0 ΓÇô Interrupts are disabled during memory mapped operations 1 ΓÇô Word Count interrupt is enabled for memory mapped operations"]
    #[inline(always)]
    #[must_use]
    pub fn mm_int_en(&mut self) -> MmIntEnW<SpiSwitchSpec> {
        MmIntEnW::new(self, 1)
    }
}
#[doc = "Memory Mapped SPI Switch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSwitchSpec;
impl crate::RegisterSpec for SpiSwitchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_switch::R`](R) reader structure"]
impl crate::Readable for SpiSwitchSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_switch::W`](W) writer structure"]
impl crate::Writable for SpiSwitchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_SWITCH to value 0"]
impl crate::Resettable for SpiSwitchSpec {
    const RESET_VALUE: u32 = 0;
}
