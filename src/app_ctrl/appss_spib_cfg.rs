#[doc = "Register `APPSS_SPIB_CFG` reader"]
pub type R = crate::R<AppssSpibCfgSpec>;
#[doc = "Register `APPSS_SPIB_CFG` writer"]
pub type W = crate::W<AppssSpibCfgSpec>;
#[doc = "Field `spibsync2sen` reader - 0:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
pub type Spibsync2senR = crate::BitReader;
#[doc = "Field `spibsync2sen` writer - 0:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
pub type Spibsync2senW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spib_cs_trigsrc_en` reader - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
pub type SpibCsTrigsrcEnR = crate::BitReader;
#[doc = "Field `spib_cs_trigsrc_en` writer - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
pub type SpibCsTrigsrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spib_trig_gate_en` reader - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
pub type SpibTrigGateEnR = crate::BitReader;
#[doc = "Field `spib_trig_gate_en` writer - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
pub type SpibTrigGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spib_int_trig_polarity` reader - 24:24\\]
SPIB trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
pub type SpibIntTrigPolarityR = crate::BitReader;
#[doc = "Field `spib_int_trig_polarity` writer - 24:24\\]
SPIB trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
pub type SpibIntTrigPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spib_iodft_en` reader - 28:28\\]
1: Enable loop back of MOSI to MISO - Master mode Enable loop back of MISO to MOSI - Slave mode"]
pub type SpibIodftEnR = crate::BitReader;
#[doc = "Field `spib_iodft_en` writer - 28:28\\]
1: Enable loop back of MOSI to MISO - Master mode Enable loop back of MISO to MOSI - Slave mode"]
pub type SpibIodftEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
    #[inline(always)]
    pub fn spibsync2sen(&self) -> Spibsync2senR {
        Spibsync2senR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
    #[inline(always)]
    pub fn spib_cs_trigsrc_en(&self) -> SpibCsTrigsrcEnR {
        SpibCsTrigsrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
    #[inline(always)]
    pub fn spib_trig_gate_en(&self) -> SpibTrigGateEnR {
        SpibTrigGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
SPIB trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
    #[inline(always)]
    pub fn spib_int_trig_polarity(&self) -> SpibIntTrigPolarityR {
        SpibIntTrigPolarityR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
1: Enable loop back of MOSI to MISO - Master mode Enable loop back of MISO to MOSI - Slave mode"]
    #[inline(always)]
    pub fn spib_iodft_en(&self) -> SpibIodftEnR {
        SpibIodftEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Donot touch the field. Used as Tie-off for IP-config."]
    #[inline(always)]
    #[must_use]
    pub fn spibsync2sen(&mut self) -> Spibsync2senW<AppssSpibCfgSpec> {
        Spibsync2senW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MIBSPIB CS Trigger SRC enable 1 : Use CS as trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn spib_cs_trigsrc_en(&mut self) -> SpibCsTrigsrcEnW<AppssSpibCfgSpec> {
        SpibCsTrigsrcEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
When set the TRIGGER s are un-gated only when chip-select is active"]
    #[inline(always)]
    #[must_use]
    pub fn spib_trig_gate_en(&mut self) -> SpibTrigGateEnW<AppssSpibCfgSpec> {
        SpibTrigGateEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
SPIB trigger source polarity select. 0 - Polarity 0, 1 -Polarity 1"]
    #[inline(always)]
    #[must_use]
    pub fn spib_int_trig_polarity(&mut self) -> SpibIntTrigPolarityW<AppssSpibCfgSpec> {
        SpibIntTrigPolarityW::new(self, 24)
    }
    #[doc = "Bit 28 - 28:28\\]
1: Enable loop back of MOSI to MISO - Master mode Enable loop back of MISO to MOSI - Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn spib_iodft_en(&mut self) -> SpibIodftEnW<AppssSpibCfgSpec> {
        SpibIodftEnW::new(self, 28)
    }
}
#[doc = "APPSS_SPIB_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_spib_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_spib_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssSpibCfgSpec;
impl crate::RegisterSpec for AppssSpibCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_spib_cfg::R`](R) reader structure"]
impl crate::Readable for AppssSpibCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_spib_cfg::W`](W) writer structure"]
impl crate::Writable for AppssSpibCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_SPIB_CFG to value 0"]
impl crate::Resettable for AppssSpibCfgSpec {
    const RESET_VALUE: u32 = 0;
}
