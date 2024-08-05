#[doc = "Register `DEBUG_STATUS_AON_11` reader"]
pub type R = crate::R<DebugStatusAon11Spec>;
#[doc = "Register `DEBUG_STATUS_AON_11` writer"]
pub type W = crate::W<DebugStatusAon11Spec>;
#[doc = "Field `fec_pd_mem_aonout` reader - 0:0\\]
status reg for fec_pd_mem_aonout"]
pub type FecPdMemAonoutR = crate::BitReader;
#[doc = "Field `fec_pd_mem_aonout` writer - 0:0\\]
status reg for fec_pd_mem_aonout"]
pub type FecPdMemAonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_agoodout` reader - 1:1\\]
status reg for fec_pd_mem_agoodout"]
pub type FecPdMemAgoodoutR = crate::BitReader;
#[doc = "Field `fec_pd_mem_agoodout` writer - 1:1\\]
status reg for fec_pd_mem_agoodout"]
pub type FecPdMemAgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_aonin` reader - 2:2\\]
status reg for fec_pd_mem_aonin"]
pub type FecPdMemAoninR = crate::BitReader;
#[doc = "Field `fec_pd_mem_aonin` writer - 2:2\\]
status reg for fec_pd_mem_aonin"]
pub type FecPdMemAoninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_agoodin` reader - 3:3\\]
status reg for fec_pd_mem_agoodin"]
pub type FecPdMemAgoodinR = crate::BitReader;
#[doc = "Field `fec_pd_mem_agoodin` writer - 3:3\\]
status reg for fec_pd_mem_agoodin"]
pub type FecPdMemAgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_grp4_aonout` reader - 5:4\\]
status reg for fec_pd_mem_grp4_aonout"]
pub type FecPdMemGrp4AonoutR = crate::FieldReader;
#[doc = "Field `fec_pd_mem_grp4_aonout` writer - 5:4\\]
status reg for fec_pd_mem_grp4_aonout"]
pub type FecPdMemGrp4AonoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fec_pd_mem_grp4_agoodout` reader - 7:6\\]
status reg for fec_pd_mem_grp4_agoodout"]
pub type FecPdMemGrp4AgoodoutR = crate::FieldReader;
#[doc = "Field `fec_pd_mem_grp4_agoodout` writer - 7:6\\]
status reg for fec_pd_mem_grp4_agoodout"]
pub type FecPdMemGrp4AgoodoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fec_pd_mem_grp4_aonin` reader - 9:8\\]
status reg for fec_pd_mem_grp4_aonin"]
pub type FecPdMemGrp4AoninR = crate::FieldReader;
#[doc = "Field `fec_pd_mem_grp4_aonin` writer - 9:8\\]
status reg for fec_pd_mem_grp4_aonin"]
pub type FecPdMemGrp4AoninW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fec_pd_mem_grp4_agoodin` reader - 11:10\\]
status reg for fec_pd_mem_grp4_agoodin"]
pub type FecPdMemGrp4AgoodinR = crate::FieldReader;
#[doc = "Field `fec_pd_mem_grp4_agoodin` writer - 11:10\\]
status reg for fec_pd_mem_grp4_agoodin"]
pub type FecPdMemGrp4AgoodinW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fec_pd_mem_dftrtaon` reader - 12:12\\]
status reg for fec_pd_mem_dftrtaon"]
pub type FecPdMemDftrtaonR = crate::BitReader;
#[doc = "Field `fec_pd_mem_dftrtaon` writer - 12:12\\]
status reg for fec_pd_mem_dftrtaon"]
pub type FecPdMemDftrtaonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_dftrtagood` reader - 13:13\\]
status reg for fec_pd_mem_dftrtagood"]
pub type FecPdMemDftrtagoodR = crate::BitReader;
#[doc = "Field `fec_pd_mem_dftrtagood` writer - 13:13\\]
status reg for fec_pd_mem_dftrtagood"]
pub type FecPdMemDftrtagoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_mem_grp4_dftrtaon` reader - 15:14\\]
status reg for fec_pd_mem_grp4_dftrtaon"]
pub type FecPdMemGrp4DftrtaonR = crate::FieldReader;
#[doc = "Field `fec_pd_mem_grp4_dftrtaon` writer - 15:14\\]
status reg for fec_pd_mem_grp4_dftrtaon"]
pub type FecPdMemGrp4DftrtaonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fec_pd_mem_grp4_dftrtagood` reader - 17:16\\]
status reg for fec_pd_mem_grp4_dftrtagood"]
pub type FecPdMemGrp4DftrtagoodR = crate::FieldReader;
#[doc = "Field `fec_pd_mem_grp4_dftrtagood` writer - 17:16\\]
status reg for fec_pd_mem_grp4_dftrtagood"]
pub type FecPdMemGrp4DftrtagoodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
status reg for fec_pd_mem_aonout"]
    #[inline(always)]
    pub fn fec_pd_mem_aonout(&self) -> FecPdMemAonoutR {
        FecPdMemAonoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for fec_pd_mem_agoodout"]
    #[inline(always)]
    pub fn fec_pd_mem_agoodout(&self) -> FecPdMemAgoodoutR {
        FecPdMemAgoodoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for fec_pd_mem_aonin"]
    #[inline(always)]
    pub fn fec_pd_mem_aonin(&self) -> FecPdMemAoninR {
        FecPdMemAoninR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for fec_pd_mem_agoodin"]
    #[inline(always)]
    pub fn fec_pd_mem_agoodin(&self) -> FecPdMemAgoodinR {
        FecPdMemAgoodinR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
status reg for fec_pd_mem_grp4_aonout"]
    #[inline(always)]
    pub fn fec_pd_mem_grp4_aonout(&self) -> FecPdMemGrp4AonoutR {
        FecPdMemGrp4AonoutR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
status reg for fec_pd_mem_grp4_agoodout"]
    #[inline(always)]
    pub fn fec_pd_mem_grp4_agoodout(&self) -> FecPdMemGrp4AgoodoutR {
        FecPdMemGrp4AgoodoutR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
status reg for fec_pd_mem_grp4_aonin"]
    #[inline(always)]
    pub fn fec_pd_mem_grp4_aonin(&self) -> FecPdMemGrp4AoninR {
        FecPdMemGrp4AoninR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
status reg for fec_pd_mem_grp4_agoodin"]
    #[inline(always)]
    pub fn fec_pd_mem_grp4_agoodin(&self) -> FecPdMemGrp4AgoodinR {
        FecPdMemGrp4AgoodinR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for fec_pd_mem_dftrtaon"]
    #[inline(always)]
    pub fn fec_pd_mem_dftrtaon(&self) -> FecPdMemDftrtaonR {
        FecPdMemDftrtaonR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
status reg for fec_pd_mem_dftrtagood"]
    #[inline(always)]
    pub fn fec_pd_mem_dftrtagood(&self) -> FecPdMemDftrtagoodR {
        FecPdMemDftrtagoodR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
status reg for fec_pd_mem_grp4_dftrtaon"]
    #[inline(always)]
    pub fn fec_pd_mem_grp4_dftrtaon(&self) -> FecPdMemGrp4DftrtaonR {
        FecPdMemGrp4DftrtaonR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
status reg for fec_pd_mem_grp4_dftrtagood"]
    #[inline(always)]
    pub fn fec_pd_mem_grp4_dftrtagood(&self) -> FecPdMemGrp4DftrtagoodR {
        FecPdMemGrp4DftrtagoodR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
status reg for fec_pd_mem_aonout"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_aonout(&mut self) -> FecPdMemAonoutW<DebugStatusAon11Spec> {
        FecPdMemAonoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for fec_pd_mem_agoodout"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_agoodout(&mut self) -> FecPdMemAgoodoutW<DebugStatusAon11Spec> {
        FecPdMemAgoodoutW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for fec_pd_mem_aonin"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_aonin(&mut self) -> FecPdMemAoninW<DebugStatusAon11Spec> {
        FecPdMemAoninW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for fec_pd_mem_agoodin"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_agoodin(&mut self) -> FecPdMemAgoodinW<DebugStatusAon11Spec> {
        FecPdMemAgoodinW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
status reg for fec_pd_mem_grp4_aonout"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_grp4_aonout(&mut self) -> FecPdMemGrp4AonoutW<DebugStatusAon11Spec> {
        FecPdMemGrp4AonoutW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
status reg for fec_pd_mem_grp4_agoodout"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_grp4_agoodout(&mut self) -> FecPdMemGrp4AgoodoutW<DebugStatusAon11Spec> {
        FecPdMemGrp4AgoodoutW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
status reg for fec_pd_mem_grp4_aonin"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_grp4_aonin(&mut self) -> FecPdMemGrp4AoninW<DebugStatusAon11Spec> {
        FecPdMemGrp4AoninW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
status reg for fec_pd_mem_grp4_agoodin"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_grp4_agoodin(&mut self) -> FecPdMemGrp4AgoodinW<DebugStatusAon11Spec> {
        FecPdMemGrp4AgoodinW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for fec_pd_mem_dftrtaon"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_dftrtaon(&mut self) -> FecPdMemDftrtaonW<DebugStatusAon11Spec> {
        FecPdMemDftrtaonW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
status reg for fec_pd_mem_dftrtagood"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_dftrtagood(&mut self) -> FecPdMemDftrtagoodW<DebugStatusAon11Spec> {
        FecPdMemDftrtagoodW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
status reg for fec_pd_mem_grp4_dftrtaon"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_grp4_dftrtaon(&mut self) -> FecPdMemGrp4DftrtaonW<DebugStatusAon11Spec> {
        FecPdMemGrp4DftrtaonW::new(self, 14)
    }
    #[doc = "Bits 16:17 - 17:16\\]
status reg for fec_pd_mem_grp4_dftrtagood"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_mem_grp4_dftrtagood(&mut self) -> FecPdMemGrp4DftrtagoodW<DebugStatusAon11Spec> {
        FecPdMemGrp4DftrtagoodW::new(self, 16)
    }
}
#[doc = "DEBUG_STATUS_AON_11\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon11Spec;
impl crate::RegisterSpec for DebugStatusAon11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_11::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon11Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_11::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_11 to value 0"]
impl crate::Resettable for DebugStatusAon11Spec {
    const RESET_VALUE: u32 = 0;
}
