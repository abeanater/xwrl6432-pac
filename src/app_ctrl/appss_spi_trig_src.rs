#[doc = "Register `APPSS_SPI_TRIG_SRC` reader"]
pub type R = crate::R<AppssSpiTrigSrcSpec>;
#[doc = "Register `APPSS_SPI_TRIG_SRC` writer"]
pub type W = crate::W<AppssSpiTrigSrcSpec>;
#[doc = "Field `trig_spia` reader - 1:0\\]
RESERVED"]
pub type TrigSpiaR = crate::FieldReader;
#[doc = "Field `trig_spia` writer - 1:0\\]
RESERVED"]
pub type TrigSpiaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `trig_spib` reader - 26:16\\]
RESERVED"]
pub type TrigSpibR = crate::FieldReader<u16>;
#[doc = "Field `trig_spib` writer - 26:16\\]
RESERVED"]
pub type TrigSpibW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVED"]
    #[inline(always)]
    pub fn trig_spia(&self) -> TrigSpiaR {
        TrigSpiaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
RESERVED"]
    #[inline(always)]
    pub fn trig_spib(&self) -> TrigSpibR {
        TrigSpibR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn trig_spia(&mut self) -> TrigSpiaW<AppssSpiTrigSrcSpec> {
        TrigSpiaW::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn trig_spib(&mut self) -> TrigSpibW<AppssSpiTrigSrcSpec> {
        TrigSpibW::new(self, 16)
    }
}
#[doc = "APPSS_SPI_TRIG_SRC\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_spi_trig_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_spi_trig_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssSpiTrigSrcSpec;
impl crate::RegisterSpec for AppssSpiTrigSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_spi_trig_src::R`](R) reader structure"]
impl crate::Readable for AppssSpiTrigSrcSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_spi_trig_src::W`](W) writer structure"]
impl crate::Writable for AppssSpiTrigSrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_SPI_TRIG_SRC to value 0"]
impl crate::Resettable for AppssSpiTrigSrcSpec {
    const RESET_VALUE: u32 = 0;
}
