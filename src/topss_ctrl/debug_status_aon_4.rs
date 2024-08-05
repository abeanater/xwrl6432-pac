#[doc = "Register `DEBUG_STATUS_AON_4` reader"]
pub type R = crate::R<DebugStatusAon4Spec>;
#[doc = "Register `DEBUG_STATUS_AON_4` writer"]
pub type W = crate::W<DebugStatusAon4Spec>;
#[doc = "Field `app_logic_pscon_fsm` reader - 4:0\\]
status reg for app_logic_pscon_fsm"]
pub type AppLogicPsconFsmR = crate::FieldReader;
#[doc = "Field `app_logic_pscon_fsm` writer - 4:0\\]
status reg for app_logic_pscon_fsm"]
pub type AppLogicPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `fec_logic_pscon_fsm` reader - 9:5\\]
status reg for fec_logic_pscon_fsm"]
pub type FecLogicPsconFsmR = crate::FieldReader;
#[doc = "Field `fec_logic_pscon_fsm` writer - 9:5\\]
status reg for fec_logic_pscon_fsm"]
pub type FecLogicPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hwa_logic_pscon_fsm` reader - 14:10\\]
status reg for hwa_logic_pscon_fsm"]
pub type HwaLogicPsconFsmR = crate::FieldReader;
#[doc = "Field `hwa_logic_pscon_fsm` writer - 14:10\\]
status reg for hwa_logic_pscon_fsm"]
pub type HwaLogicPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `testdbg_logic_pscon_fsm` reader - 19:15\\]
status reg for testdbg_logic_pscon_fsm"]
pub type TestdbgLogicPsconFsmR = crate::FieldReader;
#[doc = "Field `testdbg_logic_pscon_fsm` writer - 19:15\\]
status reg for testdbg_logic_pscon_fsm"]
pub type TestdbgLogicPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
status reg for app_logic_pscon_fsm"]
    #[inline(always)]
    pub fn app_logic_pscon_fsm(&self) -> AppLogicPsconFsmR {
        AppLogicPsconFsmR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
status reg for fec_logic_pscon_fsm"]
    #[inline(always)]
    pub fn fec_logic_pscon_fsm(&self) -> FecLogicPsconFsmR {
        FecLogicPsconFsmR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 14:10\\]
status reg for hwa_logic_pscon_fsm"]
    #[inline(always)]
    pub fn hwa_logic_pscon_fsm(&self) -> HwaLogicPsconFsmR {
        HwaLogicPsconFsmR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
status reg for testdbg_logic_pscon_fsm"]
    #[inline(always)]
    pub fn testdbg_logic_pscon_fsm(&self) -> TestdbgLogicPsconFsmR {
        TestdbgLogicPsconFsmR::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
status reg for app_logic_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn app_logic_pscon_fsm(&mut self) -> AppLogicPsconFsmW<DebugStatusAon4Spec> {
        AppLogicPsconFsmW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
status reg for fec_logic_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn fec_logic_pscon_fsm(&mut self) -> FecLogicPsconFsmW<DebugStatusAon4Spec> {
        FecLogicPsconFsmW::new(self, 5)
    }
    #[doc = "Bits 10:14 - 14:10\\]
status reg for hwa_logic_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_logic_pscon_fsm(&mut self) -> HwaLogicPsconFsmW<DebugStatusAon4Spec> {
        HwaLogicPsconFsmW::new(self, 10)
    }
    #[doc = "Bits 15:19 - 19:15\\]
status reg for testdbg_logic_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn testdbg_logic_pscon_fsm(&mut self) -> TestdbgLogicPsconFsmW<DebugStatusAon4Spec> {
        TestdbgLogicPsconFsmW::new(self, 15)
    }
}
#[doc = "DEBUG_STATUS_AON_4\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon4Spec;
impl crate::RegisterSpec for DebugStatusAon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_4::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon4Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_4::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_4 to value 0"]
impl crate::Resettable for DebugStatusAon4Spec {
    const RESET_VALUE: u32 = 0;
}
