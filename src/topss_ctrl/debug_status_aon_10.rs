#[doc = "Register `DEBUG_STATUS_AON_10` reader"]
pub type R = crate::R<DebugStatusAon10Spec>;
#[doc = "Register `DEBUG_STATUS_AON_10` writer"]
pub type W = crate::W<DebugStatusAon10Spec>;
#[doc = "Field `hwa_pd_mem_grp3_aonout` reader - 2:0\\]
status reg for hwa_pd_mem_grp3_aonout"]
pub type HwaPdMemGrp3AonoutR = crate::FieldReader;
#[doc = "Field `hwa_pd_mem_grp3_aonout` writer - 2:0\\]
status reg for hwa_pd_mem_grp3_aonout"]
pub type HwaPdMemGrp3AonoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwa_pd_mem_grp3_agoodout` reader - 5:3\\]
status reg for hwa_pd_mem_grp3_agoodout"]
pub type HwaPdMemGrp3AgoodoutR = crate::FieldReader;
#[doc = "Field `hwa_pd_mem_grp3_agoodout` writer - 5:3\\]
status reg for hwa_pd_mem_grp3_agoodout"]
pub type HwaPdMemGrp3AgoodoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwa_pd_mem_grp3_aonin` reader - 8:6\\]
status reg for hwa_pd_mem_grp3_aonin"]
pub type HwaPdMemGrp3AoninR = crate::FieldReader;
#[doc = "Field `hwa_pd_mem_grp3_aonin` writer - 8:6\\]
status reg for hwa_pd_mem_grp3_aonin"]
pub type HwaPdMemGrp3AoninW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwa_pd_mem_grp3_agoodin` reader - 11:9\\]
status reg for hwa_pd_mem_grp3_agoodin"]
pub type HwaPdMemGrp3AgoodinR = crate::FieldReader;
#[doc = "Field `hwa_pd_mem_grp3_agoodin` writer - 11:9\\]
status reg for hwa_pd_mem_grp3_agoodin"]
pub type HwaPdMemGrp3AgoodinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwa_pd_mem_grp3_dftrtaon` reader - 14:12\\]
status reg for hwa_pd_mem_grp3_dftrtaon"]
pub type HwaPdMemGrp3DftrtaonR = crate::FieldReader;
#[doc = "Field `hwa_pd_mem_grp3_dftrtaon` writer - 14:12\\]
status reg for hwa_pd_mem_grp3_dftrtaon"]
pub type HwaPdMemGrp3DftrtaonW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwa_pd_mem_grp3_dftrtagood` reader - 17:15\\]
status reg for hwa_pd_mem_grp3_dftrtagood"]
pub type HwaPdMemGrp3DftrtagoodR = crate::FieldReader;
#[doc = "Field `hwa_pd_mem_grp3_dftrtagood` writer - 17:15\\]
status reg for hwa_pd_mem_grp3_dftrtagood"]
pub type HwaPdMemGrp3DftrtagoodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
status reg for hwa_pd_mem_grp3_aonout"]
    #[inline(always)]
    pub fn hwa_pd_mem_grp3_aonout(&self) -> HwaPdMemGrp3AonoutR {
        HwaPdMemGrp3AonoutR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
status reg for hwa_pd_mem_grp3_agoodout"]
    #[inline(always)]
    pub fn hwa_pd_mem_grp3_agoodout(&self) -> HwaPdMemGrp3AgoodoutR {
        HwaPdMemGrp3AgoodoutR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
status reg for hwa_pd_mem_grp3_aonin"]
    #[inline(always)]
    pub fn hwa_pd_mem_grp3_aonin(&self) -> HwaPdMemGrp3AoninR {
        HwaPdMemGrp3AoninR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
status reg for hwa_pd_mem_grp3_agoodin"]
    #[inline(always)]
    pub fn hwa_pd_mem_grp3_agoodin(&self) -> HwaPdMemGrp3AgoodinR {
        HwaPdMemGrp3AgoodinR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
status reg for hwa_pd_mem_grp3_dftrtaon"]
    #[inline(always)]
    pub fn hwa_pd_mem_grp3_dftrtaon(&self) -> HwaPdMemGrp3DftrtaonR {
        HwaPdMemGrp3DftrtaonR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - 17:15\\]
status reg for hwa_pd_mem_grp3_dftrtagood"]
    #[inline(always)]
    pub fn hwa_pd_mem_grp3_dftrtagood(&self) -> HwaPdMemGrp3DftrtagoodR {
        HwaPdMemGrp3DftrtagoodR::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
status reg for hwa_pd_mem_grp3_aonout"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_grp3_aonout(&mut self) -> HwaPdMemGrp3AonoutW<DebugStatusAon10Spec> {
        HwaPdMemGrp3AonoutW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
status reg for hwa_pd_mem_grp3_agoodout"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_grp3_agoodout(&mut self) -> HwaPdMemGrp3AgoodoutW<DebugStatusAon10Spec> {
        HwaPdMemGrp3AgoodoutW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
status reg for hwa_pd_mem_grp3_aonin"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_grp3_aonin(&mut self) -> HwaPdMemGrp3AoninW<DebugStatusAon10Spec> {
        HwaPdMemGrp3AoninW::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
status reg for hwa_pd_mem_grp3_agoodin"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_grp3_agoodin(&mut self) -> HwaPdMemGrp3AgoodinW<DebugStatusAon10Spec> {
        HwaPdMemGrp3AgoodinW::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
status reg for hwa_pd_mem_grp3_dftrtaon"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_grp3_dftrtaon(&mut self) -> HwaPdMemGrp3DftrtaonW<DebugStatusAon10Spec> {
        HwaPdMemGrp3DftrtaonW::new(self, 12)
    }
    #[doc = "Bits 15:17 - 17:15\\]
status reg for hwa_pd_mem_grp3_dftrtagood"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_mem_grp3_dftrtagood(&mut self) -> HwaPdMemGrp3DftrtagoodW<DebugStatusAon10Spec> {
        HwaPdMemGrp3DftrtagoodW::new(self, 15)
    }
}
#[doc = "DEBUG_STATUS_AON_10\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon10Spec;
impl crate::RegisterSpec for DebugStatusAon10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_10::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon10Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_10::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_10 to value 0"]
impl crate::Resettable for DebugStatusAon10Spec {
    const RESET_VALUE: u32 = 0;
}
