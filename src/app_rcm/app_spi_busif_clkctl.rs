#[doc = "Register `APP_SPI_BUSIF_CLKCTL` reader"]
pub type R = crate::R<AppSpiBusifClkctlSpec>;
#[doc = "Register `APP_SPI_BUSIF_CLKCTL` writer"]
pub type W = crate::W<AppSpiBusifClkctlSpec>;
#[doc = "Field `divr` reader - 11:0\\]
Divide value 0x0 : div1 0x1 : div2 0x2 : div3 . . 0xF = div16 Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register."]
pub type DivrR = crate::FieldReader<u16>;
#[doc = "Field `divr` writer - 11:0\\]
Divide value 0x0 : div1 0x1 : div2 0x2 : div3 . . 0xF = div16 Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register."]
pub type DivrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Divide value 0x0 : div1 0x1 : div2 0x2 : div3 . . 0xF = div16 Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register."]
    #[inline(always)]
    pub fn divr(&self) -> DivrR {
        DivrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Divide value 0x0 : div1 0x1 : div2 0x2 : div3 . . 0xF = div16 Data should be loaded as multibit. For example: if divider value of '0x8' should be selected then '0x888' should be configured to the register."]
    #[inline(always)]
    #[must_use]
    pub fn divr(&mut self) -> DivrW<AppSpiBusifClkctlSpec> {
        DivrW::new(self, 0)
    }
}
#[doc = "APP_SPI_BUSIF_CLKCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_spi_busif_clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_spi_busif_clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppSpiBusifClkctlSpec;
impl crate::RegisterSpec for AppSpiBusifClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_spi_busif_clkctl::R`](R) reader structure"]
impl crate::Readable for AppSpiBusifClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`app_spi_busif_clkctl::W`](W) writer structure"]
impl crate::Writable for AppSpiBusifClkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_SPI_BUSIF_CLKCTL to value 0"]
impl crate::Resettable for AppSpiBusifClkctlSpec {
    const RESET_VALUE: u32 = 0;
}
