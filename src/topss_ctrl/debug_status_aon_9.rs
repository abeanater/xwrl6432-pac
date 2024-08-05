#[doc = "Register `DEBUG_STATUS_AON_9` reader"]
pub type R = crate::R<DebugStatusAon9Spec>;
#[doc = "Register `DEBUG_STATUS_AON_9` writer"]
pub type W = crate::W<DebugStatusAon9Spec>;
#[doc = "Field `app_pd_mem_dftrtaon` reader - 2:0\\]
status reg for app_pd_mem_dftrtaon"]
pub type AppPdMemDftrtaonR = crate::FieldReader;
#[doc = "Field `app_pd_mem_dftrtaon` writer - 2:0\\]
status reg for app_pd_mem_dftrtaon"]
pub type AppPdMemDftrtaonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_pd_mem_dftrtagood` reader - 5:3\\]
status reg for app_pd_mem_dftrtagood"]
pub type AppPdMemDftrtagoodR = crate::FieldReader;
#[doc = "Field `app_pd_mem_dftrtagood` writer - 5:3\\]
status reg for app_pd_mem_dftrtagood"]
pub type AppPdMemDftrtagoodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_pd_mem_grp1_dftrtaon` reader - 7:6\\]
status reg for app_pd_mem_grp1_dftrtaon"]
pub type AppPdMemGrp1DftrtaonR = crate::FieldReader;
#[doc = "Field `app_pd_mem_grp1_dftrtaon` writer - 7:6\\]
status reg for app_pd_mem_grp1_dftrtaon"]
pub type AppPdMemGrp1DftrtaonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `app_pd_mem_grp1_dftrtagood` reader - 9:8\\]
status reg for app_pd_mem_grp1_dftrtagood"]
pub type AppPdMemGrp1DftrtagoodR = crate::FieldReader;
#[doc = "Field `app_pd_mem_grp1_dftrtagood` writer - 9:8\\]
status reg for app_pd_mem_grp1_dftrtagood"]
pub type AppPdMemGrp1DftrtagoodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `app_pd_mem_grp2_dftrtaon` reader - 10:10\\]
status reg for app_pd_mem_grp2_dftrtaon"]
pub type AppPdMemGrp2DftrtaonR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_dftrtaon` writer - 10:10\\]
status reg for app_pd_mem_grp2_dftrtaon"]
pub type AppPdMemGrp2DftrtaonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp2_dftrtagood` reader - 11:11\\]
status reg for app_pd_mem_grp2_dftrtagood"]
pub type AppPdMemGrp2DftrtagoodR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_dftrtagood` writer - 11:11\\]
status reg for app_pd_mem_grp2_dftrtagood"]
pub type AppPdMemGrp2DftrtagoodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
status reg for app_pd_mem_dftrtaon"]
    #[inline(always)]
    pub fn app_pd_mem_dftrtaon(&self) -> AppPdMemDftrtaonR {
        AppPdMemDftrtaonR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
status reg for app_pd_mem_dftrtagood"]
    #[inline(always)]
    pub fn app_pd_mem_dftrtagood(&self) -> AppPdMemDftrtagoodR {
        AppPdMemDftrtagoodR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
status reg for app_pd_mem_grp1_dftrtaon"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_dftrtaon(&self) -> AppPdMemGrp1DftrtaonR {
        AppPdMemGrp1DftrtaonR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
status reg for app_pd_mem_grp1_dftrtagood"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_dftrtagood(&self) -> AppPdMemGrp1DftrtagoodR {
        AppPdMemGrp1DftrtagoodR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
status reg for app_pd_mem_grp2_dftrtaon"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_dftrtaon(&self) -> AppPdMemGrp2DftrtaonR {
        AppPdMemGrp2DftrtaonR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for app_pd_mem_grp2_dftrtagood"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_dftrtagood(&self) -> AppPdMemGrp2DftrtagoodR {
        AppPdMemGrp2DftrtagoodR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
status reg for app_pd_mem_dftrtaon"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_dftrtaon(&mut self) -> AppPdMemDftrtaonW<DebugStatusAon9Spec> {
        AppPdMemDftrtaonW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
status reg for app_pd_mem_dftrtagood"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_dftrtagood(&mut self) -> AppPdMemDftrtagoodW<DebugStatusAon9Spec> {
        AppPdMemDftrtagoodW::new(self, 3)
    }
    #[doc = "Bits 6:7 - 7:6\\]
status reg for app_pd_mem_grp1_dftrtaon"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_dftrtaon(&mut self) -> AppPdMemGrp1DftrtaonW<DebugStatusAon9Spec> {
        AppPdMemGrp1DftrtaonW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
status reg for app_pd_mem_grp1_dftrtagood"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_dftrtagood(&mut self) -> AppPdMemGrp1DftrtagoodW<DebugStatusAon9Spec> {
        AppPdMemGrp1DftrtagoodW::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
status reg for app_pd_mem_grp2_dftrtaon"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_dftrtaon(&mut self) -> AppPdMemGrp2DftrtaonW<DebugStatusAon9Spec> {
        AppPdMemGrp2DftrtaonW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for app_pd_mem_grp2_dftrtagood"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_dftrtagood(&mut self) -> AppPdMemGrp2DftrtagoodW<DebugStatusAon9Spec> {
        AppPdMemGrp2DftrtagoodW::new(self, 11)
    }
}
#[doc = "DEBUG_STATUS_AON_9\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon9Spec;
impl crate::RegisterSpec for DebugStatusAon9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_9::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon9Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_9::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_9 to value 0"]
impl crate::Resettable for DebugStatusAon9Spec {
    const RESET_VALUE: u32 = 0;
}
