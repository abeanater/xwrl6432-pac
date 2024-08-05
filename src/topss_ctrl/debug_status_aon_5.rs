#[doc = "Register `DEBUG_STATUS_AON_5` reader"]
pub type R = crate::R<DebugStatusAon5Spec>;
#[doc = "Register `DEBUG_STATUS_AON_5` writer"]
pub type W = crate::W<DebugStatusAon5Spec>;
#[doc = "Field `app_mem_pscon_fsm` reader - 3:0\\]
status reg for app_mem_pscon_fsm"]
pub type AppMemPsconFsmR = crate::FieldReader;
#[doc = "Field `app_mem_pscon_fsm` writer - 3:0\\]
status reg for app_mem_pscon_fsm"]
pub type AppMemPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `app_grp1_mem_pscon_fsm` reader - 7:4\\]
status reg for app_grp1_mem_pscon_fsm"]
pub type AppGrp1MemPsconFsmR = crate::FieldReader;
#[doc = "Field `app_grp1_mem_pscon_fsm` writer - 7:4\\]
status reg for app_grp1_mem_pscon_fsm"]
pub type AppGrp1MemPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `app_grp2_mem_pscon_fsm` reader - 11:8\\]
status reg for app_grp2_mem_pscon_fsm"]
pub type AppGrp2MemPsconFsmR = crate::FieldReader;
#[doc = "Field `app_grp2_mem_pscon_fsm` writer - 11:8\\]
status reg for app_grp2_mem_pscon_fsm"]
pub type AppGrp2MemPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hwa_grp3_mem_pscon_fsm` reader - 15:12\\]
status reg for hwa_grp3_mem_pscon_fsm"]
pub type HwaGrp3MemPsconFsmR = crate::FieldReader;
#[doc = "Field `hwa_grp3_mem_pscon_fsm` writer - 15:12\\]
status reg for hwa_grp3_mem_pscon_fsm"]
pub type HwaGrp3MemPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fec_mem_pscon_fsm` reader - 19:16\\]
status reg for fec_mem_pscon_fsm"]
pub type FecMemPsconFsmR = crate::FieldReader;
#[doc = "Field `fec_mem_pscon_fsm` writer - 19:16\\]
status reg for fec_mem_pscon_fsm"]
pub type FecMemPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fec_grp4mem_pscon_fsm` reader - 23:20\\]
status reg for fec_grp4mem_pscon_fsm"]
pub type FecGrp4memPsconFsmR = crate::FieldReader;
#[doc = "Field `fec_grp4mem_pscon_fsm` writer - 23:20\\]
status reg for fec_grp4mem_pscon_fsm"]
pub type FecGrp4memPsconFsmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
status reg for app_mem_pscon_fsm"]
    #[inline(always)]
    pub fn app_mem_pscon_fsm(&self) -> AppMemPsconFsmR {
        AppMemPsconFsmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
status reg for app_grp1_mem_pscon_fsm"]
    #[inline(always)]
    pub fn app_grp1_mem_pscon_fsm(&self) -> AppGrp1MemPsconFsmR {
        AppGrp1MemPsconFsmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
status reg for app_grp2_mem_pscon_fsm"]
    #[inline(always)]
    pub fn app_grp2_mem_pscon_fsm(&self) -> AppGrp2MemPsconFsmR {
        AppGrp2MemPsconFsmR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
status reg for hwa_grp3_mem_pscon_fsm"]
    #[inline(always)]
    pub fn hwa_grp3_mem_pscon_fsm(&self) -> HwaGrp3MemPsconFsmR {
        HwaGrp3MemPsconFsmR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
status reg for fec_mem_pscon_fsm"]
    #[inline(always)]
    pub fn fec_mem_pscon_fsm(&self) -> FecMemPsconFsmR {
        FecMemPsconFsmR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
status reg for fec_grp4mem_pscon_fsm"]
    #[inline(always)]
    pub fn fec_grp4mem_pscon_fsm(&self) -> FecGrp4memPsconFsmR {
        FecGrp4memPsconFsmR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
status reg for app_mem_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn app_mem_pscon_fsm(&mut self) -> AppMemPsconFsmW<DebugStatusAon5Spec> {
        AppMemPsconFsmW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
status reg for app_grp1_mem_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn app_grp1_mem_pscon_fsm(&mut self) -> AppGrp1MemPsconFsmW<DebugStatusAon5Spec> {
        AppGrp1MemPsconFsmW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
status reg for app_grp2_mem_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn app_grp2_mem_pscon_fsm(&mut self) -> AppGrp2MemPsconFsmW<DebugStatusAon5Spec> {
        AppGrp2MemPsconFsmW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
status reg for hwa_grp3_mem_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_grp3_mem_pscon_fsm(&mut self) -> HwaGrp3MemPsconFsmW<DebugStatusAon5Spec> {
        HwaGrp3MemPsconFsmW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
status reg for fec_mem_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn fec_mem_pscon_fsm(&mut self) -> FecMemPsconFsmW<DebugStatusAon5Spec> {
        FecMemPsconFsmW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
status reg for fec_grp4mem_pscon_fsm"]
    #[inline(always)]
    #[must_use]
    pub fn fec_grp4mem_pscon_fsm(&mut self) -> FecGrp4memPsconFsmW<DebugStatusAon5Spec> {
        FecGrp4memPsconFsmW::new(self, 20)
    }
}
#[doc = "DEBUG_STATUS_AON_5\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon5Spec;
impl crate::RegisterSpec for DebugStatusAon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_5::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon5Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_5::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_5 to value 0"]
impl crate::Resettable for DebugStatusAon5Spec {
    const RESET_VALUE: u32 = 0;
}
