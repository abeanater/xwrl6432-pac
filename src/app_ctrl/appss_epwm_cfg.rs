#[doc = "Register `APPSS_EPWM_CFG` reader"]
pub type R = crate::R<AppssEpwmCfgSpec>;
#[doc = "Register `APPSS_EPWM_CFG` writer"]
pub type W = crate::W<AppssEpwmCfgSpec>;
#[doc = "Field `epwm_config` reader - 31:0\\]
bit0: SW syncin for EPWM1 bit1: SW syncin for EPWM2 bit2: SW syncin for EPWM3 bit8:9 : select bits for EPWM1 '0' : external syncin '1' : reserved '2' : sw syncin '3' : reserved bit10:11 : select bits for EPWM2 '0' : external syncin '1' : chained from EPWM1 '2' : sw syncin '3' : reserved bit12:13 : select bits for EPWM3 '0' : external syncin '1' : chained from EPWM2 '2' : sw syncin '3' : reserved bit24:TBCLKEN for EPWM1 bit25:TBCLKEN for EPWM2 bit26:TBCLKEN for EPWM3"]
pub type EpwmConfigR = crate::FieldReader<u32>;
#[doc = "Field `epwm_config` writer - 31:0\\]
bit0: SW syncin for EPWM1 bit1: SW syncin for EPWM2 bit2: SW syncin for EPWM3 bit8:9 : select bits for EPWM1 '0' : external syncin '1' : reserved '2' : sw syncin '3' : reserved bit10:11 : select bits for EPWM2 '0' : external syncin '1' : chained from EPWM1 '2' : sw syncin '3' : reserved bit12:13 : select bits for EPWM3 '0' : external syncin '1' : chained from EPWM2 '2' : sw syncin '3' : reserved bit24:TBCLKEN for EPWM1 bit25:TBCLKEN for EPWM2 bit26:TBCLKEN for EPWM3"]
pub type EpwmConfigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
bit0: SW syncin for EPWM1 bit1: SW syncin for EPWM2 bit2: SW syncin for EPWM3 bit8:9 : select bits for EPWM1 '0' : external syncin '1' : reserved '2' : sw syncin '3' : reserved bit10:11 : select bits for EPWM2 '0' : external syncin '1' : chained from EPWM1 '2' : sw syncin '3' : reserved bit12:13 : select bits for EPWM3 '0' : external syncin '1' : chained from EPWM2 '2' : sw syncin '3' : reserved bit24:TBCLKEN for EPWM1 bit25:TBCLKEN for EPWM2 bit26:TBCLKEN for EPWM3"]
    #[inline(always)]
    pub fn epwm_config(&self) -> EpwmConfigR {
        EpwmConfigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
bit0: SW syncin for EPWM1 bit1: SW syncin for EPWM2 bit2: SW syncin for EPWM3 bit8:9 : select bits for EPWM1 '0' : external syncin '1' : reserved '2' : sw syncin '3' : reserved bit10:11 : select bits for EPWM2 '0' : external syncin '1' : chained from EPWM1 '2' : sw syncin '3' : reserved bit12:13 : select bits for EPWM3 '0' : external syncin '1' : chained from EPWM2 '2' : sw syncin '3' : reserved bit24:TBCLKEN for EPWM1 bit25:TBCLKEN for EPWM2 bit26:TBCLKEN for EPWM3"]
    #[inline(always)]
    #[must_use]
    pub fn epwm_config(&mut self) -> EpwmConfigW<AppssEpwmCfgSpec> {
        EpwmConfigW::new(self, 0)
    }
}
#[doc = "APPSS_EPWM_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_epwm_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_epwm_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssEpwmCfgSpec;
impl crate::RegisterSpec for AppssEpwmCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_epwm_cfg::R`](R) reader structure"]
impl crate::Readable for AppssEpwmCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_epwm_cfg::W`](W) writer structure"]
impl crate::Writable for AppssEpwmCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_EPWM_CFG to value 0"]
impl crate::Resettable for AppssEpwmCfgSpec {
    const RESET_VALUE: u32 = 0;
}
