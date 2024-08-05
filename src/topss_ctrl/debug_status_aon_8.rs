#[doc = "Register `DEBUG_STATUS_AON_8` reader"]
pub type R = crate::R<DebugStatusAon8Spec>;
#[doc = "Register `DEBUG_STATUS_AON_8` writer"]
pub type W = crate::W<DebugStatusAon8Spec>;
#[doc = "Field `app_pd_mem_aonout` reader - 2:0\\]
status reg for app_pd_mem_aonout"]
pub type AppPdMemAonoutR = crate::FieldReader;
#[doc = "Field `app_pd_mem_aonout` writer - 2:0\\]
status reg for app_pd_mem_aonout"]
pub type AppPdMemAonoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_pd_mem_agoodout` reader - 5:3\\]
status reg for app_pd_mem_agoodout"]
pub type AppPdMemAgoodoutR = crate::FieldReader;
#[doc = "Field `app_pd_mem_agoodout` writer - 5:3\\]
status reg for app_pd_mem_agoodout"]
pub type AppPdMemAgoodoutW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_pd_mem_aonin` reader - 8:6\\]
status reg for app_pd_mem_aonin"]
pub type AppPdMemAoninR = crate::FieldReader;
#[doc = "Field `app_pd_mem_aonin` writer - 8:6\\]
status reg for app_pd_mem_aonin"]
pub type AppPdMemAoninW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_pd_mem_agoodin` reader - 11:9\\]
status reg for app_pd_mem_agoodin"]
pub type AppPdMemAgoodinR = crate::FieldReader;
#[doc = "Field `app_pd_mem_agoodin` writer - 11:9\\]
status reg for app_pd_mem_agoodin"]
pub type AppPdMemAgoodinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `app_pd_mem_grp1_aonout` reader - 13:12\\]
status reg for app_pd_mem_grp1_aonout"]
pub type AppPdMemGrp1AonoutR = crate::FieldReader;
#[doc = "Field `app_pd_mem_grp1_aonout` writer - 13:12\\]
status reg for app_pd_mem_grp1_aonout"]
pub type AppPdMemGrp1AonoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `app_pd_mem_grp1_agoodout` reader - 15:14\\]
status reg for app_pd_mem_grp1_agoodout"]
pub type AppPdMemGrp1AgoodoutR = crate::FieldReader;
#[doc = "Field `app_pd_mem_grp1_agoodout` writer - 15:14\\]
status reg for app_pd_mem_grp1_agoodout"]
pub type AppPdMemGrp1AgoodoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `app_pd_mem_grp1_aonin` reader - 17:16\\]
status reg for app_pd_mem_grp1_aonin"]
pub type AppPdMemGrp1AoninR = crate::FieldReader;
#[doc = "Field `app_pd_mem_grp1_aonin` writer - 17:16\\]
status reg for app_pd_mem_grp1_aonin"]
pub type AppPdMemGrp1AoninW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `app_pd_mem_grp1_agoodin` reader - 19:18\\]
status reg for app_pd_mem_grp1_agoodin"]
pub type AppPdMemGrp1AgoodinR = crate::FieldReader;
#[doc = "Field `app_pd_mem_grp1_agoodin` writer - 19:18\\]
status reg for app_pd_mem_grp1_agoodin"]
pub type AppPdMemGrp1AgoodinW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `app_pd_mem_grp2_aonout` reader - 20:20\\]
status reg for app_pd_mem_grp2_aonout"]
pub type AppPdMemGrp2AonoutR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_aonout` writer - 20:20\\]
status reg for app_pd_mem_grp2_aonout"]
pub type AppPdMemGrp2AonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp2_agoodout` reader - 21:21\\]
status reg for app_pd_mem_grp2_agoodout"]
pub type AppPdMemGrp2AgoodoutR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_agoodout` writer - 21:21\\]
status reg for app_pd_mem_grp2_agoodout"]
pub type AppPdMemGrp2AgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp2_aonin` reader - 22:22\\]
status reg for app_pd_mem_grp2_aonin"]
pub type AppPdMemGrp2AoninR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_aonin` writer - 22:22\\]
status reg for app_pd_mem_grp2_aonin"]
pub type AppPdMemGrp2AoninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_mem_grp2_agoodin` reader - 23:23\\]
status reg for app_pd_mem_grp2_agoodin"]
pub type AppPdMemGrp2AgoodinR = crate::BitReader;
#[doc = "Field `app_pd_mem_grp2_agoodin` writer - 23:23\\]
status reg for app_pd_mem_grp2_agoodin"]
pub type AppPdMemGrp2AgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
status reg for app_pd_mem_aonout"]
    #[inline(always)]
    pub fn app_pd_mem_aonout(&self) -> AppPdMemAonoutR {
        AppPdMemAonoutR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
status reg for app_pd_mem_agoodout"]
    #[inline(always)]
    pub fn app_pd_mem_agoodout(&self) -> AppPdMemAgoodoutR {
        AppPdMemAgoodoutR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
status reg for app_pd_mem_aonin"]
    #[inline(always)]
    pub fn app_pd_mem_aonin(&self) -> AppPdMemAoninR {
        AppPdMemAoninR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
status reg for app_pd_mem_agoodin"]
    #[inline(always)]
    pub fn app_pd_mem_agoodin(&self) -> AppPdMemAgoodinR {
        AppPdMemAgoodinR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
status reg for app_pd_mem_grp1_aonout"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_aonout(&self) -> AppPdMemGrp1AonoutR {
        AppPdMemGrp1AonoutR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
status reg for app_pd_mem_grp1_agoodout"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_agoodout(&self) -> AppPdMemGrp1AgoodoutR {
        AppPdMemGrp1AgoodoutR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
status reg for app_pd_mem_grp1_aonin"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_aonin(&self) -> AppPdMemGrp1AoninR {
        AppPdMemGrp1AoninR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
status reg for app_pd_mem_grp1_agoodin"]
    #[inline(always)]
    pub fn app_pd_mem_grp1_agoodin(&self) -> AppPdMemGrp1AgoodinR {
        AppPdMemGrp1AgoodinR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
status reg for app_pd_mem_grp2_aonout"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_aonout(&self) -> AppPdMemGrp2AonoutR {
        AppPdMemGrp2AonoutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
status reg for app_pd_mem_grp2_agoodout"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_agoodout(&self) -> AppPdMemGrp2AgoodoutR {
        AppPdMemGrp2AgoodoutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
status reg for app_pd_mem_grp2_aonin"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_aonin(&self) -> AppPdMemGrp2AoninR {
        AppPdMemGrp2AoninR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
status reg for app_pd_mem_grp2_agoodin"]
    #[inline(always)]
    pub fn app_pd_mem_grp2_agoodin(&self) -> AppPdMemGrp2AgoodinR {
        AppPdMemGrp2AgoodinR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
status reg for app_pd_mem_aonout"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_aonout(&mut self) -> AppPdMemAonoutW<DebugStatusAon8Spec> {
        AppPdMemAonoutW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
status reg for app_pd_mem_agoodout"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_agoodout(&mut self) -> AppPdMemAgoodoutW<DebugStatusAon8Spec> {
        AppPdMemAgoodoutW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
status reg for app_pd_mem_aonin"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_aonin(&mut self) -> AppPdMemAoninW<DebugStatusAon8Spec> {
        AppPdMemAoninW::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
status reg for app_pd_mem_agoodin"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_agoodin(&mut self) -> AppPdMemAgoodinW<DebugStatusAon8Spec> {
        AppPdMemAgoodinW::new(self, 9)
    }
    #[doc = "Bits 12:13 - 13:12\\]
status reg for app_pd_mem_grp1_aonout"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_aonout(&mut self) -> AppPdMemGrp1AonoutW<DebugStatusAon8Spec> {
        AppPdMemGrp1AonoutW::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
status reg for app_pd_mem_grp1_agoodout"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_agoodout(&mut self) -> AppPdMemGrp1AgoodoutW<DebugStatusAon8Spec> {
        AppPdMemGrp1AgoodoutW::new(self, 14)
    }
    #[doc = "Bits 16:17 - 17:16\\]
status reg for app_pd_mem_grp1_aonin"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_aonin(&mut self) -> AppPdMemGrp1AoninW<DebugStatusAon8Spec> {
        AppPdMemGrp1AoninW::new(self, 16)
    }
    #[doc = "Bits 18:19 - 19:18\\]
status reg for app_pd_mem_grp1_agoodin"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp1_agoodin(&mut self) -> AppPdMemGrp1AgoodinW<DebugStatusAon8Spec> {
        AppPdMemGrp1AgoodinW::new(self, 18)
    }
    #[doc = "Bit 20 - 20:20\\]
status reg for app_pd_mem_grp2_aonout"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_aonout(&mut self) -> AppPdMemGrp2AonoutW<DebugStatusAon8Spec> {
        AppPdMemGrp2AonoutW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
status reg for app_pd_mem_grp2_agoodout"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_agoodout(&mut self) -> AppPdMemGrp2AgoodoutW<DebugStatusAon8Spec> {
        AppPdMemGrp2AgoodoutW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
status reg for app_pd_mem_grp2_aonin"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_aonin(&mut self) -> AppPdMemGrp2AoninW<DebugStatusAon8Spec> {
        AppPdMemGrp2AoninW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
status reg for app_pd_mem_grp2_agoodin"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_mem_grp2_agoodin(&mut self) -> AppPdMemGrp2AgoodinW<DebugStatusAon8Spec> {
        AppPdMemGrp2AgoodinW::new(self, 23)
    }
}
#[doc = "DEBUG_STATUS_AON_8\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon8Spec;
impl crate::RegisterSpec for DebugStatusAon8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_8::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon8Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_8::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_8 to value 0"]
impl crate::Resettable for DebugStatusAon8Spec {
    const RESET_VALUE: u32 = 0;
}
