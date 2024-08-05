#[doc = "Register `APPSS_SPIA_CFG` reader"]
pub type R = crate::R<AppssSpiaCfgSpec>;
#[doc = "Register `APPSS_SPIA_CFG` writer"]
pub type W = crate::W<AppssSpiaCfgSpec>;
#[doc = "Field `spiasync2sen` reader - 0:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
pub type Spiasync2senR = crate::BitReader;
#[doc = "Field `spiasync2sen` writer - 0:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
pub type Spiasync2senW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spia_cs_trigsrc_en` reader - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
pub type SpiaCsTrigsrcEnR = crate::BitReader;
#[doc = "Field `spia_cs_trigsrc_en` writer - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
pub type SpiaCsTrigsrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spia_trig_gate_en` reader - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
pub type SpiaTrigGateEnR = crate::BitReader;
#[doc = "Field `spia_trig_gate_en` writer - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
pub type SpiaTrigGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spia_int_trig_polarity` reader - 24:24\\]
SPIA trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
pub type SpiaIntTrigPolarityR = crate::BitReader;
#[doc = "Field `spia_int_trig_polarity` writer - 24:24\\]
SPIA trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
pub type SpiaIntTrigPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spia_iodft_en` reader - 28:28\\]
1: Enable loop back of MOSI to MISO - Master mode Enable loop back of MISO to MOSI - Slave mode"]
pub type SpiaIodftEnR = crate::BitReader;
#[doc = "Field `spia_iodft_en` writer - 28:28\\]
1: Enable loop back of MOSI to MISO - Master mode Enable loop back of MISO to MOSI - Slave mode"]
pub type SpiaIodftEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
    #[inline(always)]
    pub fn spiasync2sen(&self) -> Spiasync2senR {
        Spiasync2senR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
    #[inline(always)]
    pub fn spia_cs_trigsrc_en(&self) -> SpiaCsTrigsrcEnR {
        SpiaCsTrigsrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
    #[inline(always)]
    pub fn spia_trig_gate_en(&self) -> SpiaTrigGateEnR {
        SpiaTrigGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
SPIA trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
    #[inline(always)]
    pub fn spia_int_trig_polarity(&self) -> SpiaIntTrigPolarityR {
        SpiaIntTrigPolarityR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
1: Enable loop back of MOSI to MISO - Master mode Enable loop back of MISO to MOSI - Slave mode"]
    #[inline(always)]
    pub fn spia_iodft_en(&self) -> SpiaIodftEnR {
        SpiaIodftEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
    #[inline(always)]
    #[must_use]
    pub fn spiasync2sen(&mut self) -> Spiasync2senW<AppssSpiaCfgSpec> {
        Spiasync2senW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn spia_cs_trigsrc_en(&mut self) -> SpiaCsTrigsrcEnW<AppssSpiaCfgSpec> {
        SpiaCsTrigsrcEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
    #[inline(always)]
    #[must_use]
    pub fn spia_trig_gate_en(&mut self) -> SpiaTrigGateEnW<AppssSpiaCfgSpec> {
        SpiaTrigGateEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
SPIA trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
    #[inline(always)]
    #[must_use]
    pub fn spia_int_trig_polarity(&mut self) -> SpiaIntTrigPolarityW<AppssSpiaCfgSpec> {
        SpiaIntTrigPolarityW::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
1: Enable loop back of MOSI to MISO - Master mode Enable loop back of MISO to MOSI - Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn spia_iodft_en(&mut self) -> SpiaIodftEnW<AppssSpiaCfgSpec> {
        SpiaIodftEnW::new(self, 28)
    }
}
#[doc = "APPSS_SPIA_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_spia_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_spia_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssSpiaCfgSpec;
impl crate::RegisterSpec for AppssSpiaCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_spia_cfg::R`](R) reader structure"]
impl crate::Readable for AppssSpiaCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_spia_cfg::W`](W) writer structure"]
impl crate::Writable for AppssSpiaCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_SPIA_CFG to value 0"]
impl crate::Resettable for AppssSpiaCfgSpec {
    const RESET_VALUE: u32 = 0;
}
