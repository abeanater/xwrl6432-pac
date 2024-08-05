#[doc = "Register `dft_proc_dmled_exec` reader"]
pub type R = crate::R<DftProcDmledExecSpec>;
#[doc = "Register `dft_proc_dmled_exec` writer"]
pub type W = crate::W<DftProcDmledExecSpec>;
#[doc = "Field `cm4_exec` reader - 2:0\\]
dft_proc_dmled_cm4_exec"]
pub type Cm4ExecR = crate::FieldReader;
#[doc = "Field `cm4_exec` writer - 2:0\\]
dft_proc_dmled_cm4_exec"]
pub type Cm4ExecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cm3_exec` reader - 5:3\\]
dft_proc_dmled_cm3_exec"]
pub type Cm3ExecR = crate::FieldReader;
#[doc = "Field `cm3_exec` writer - 5:3\\]
dft_proc_dmled_cm3_exec"]
pub type Cm3ExecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `obs_exec` reader - 8:6\\]
dft_proc_dmled_obs_exec"]
pub type ObsExecR = crate::FieldReader;
#[doc = "Field `obs_exec` writer - 8:6\\]
dft_proc_dmled_obs_exec"]
pub type ObsExecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cm4_status` reader - 11:9\\]
dft_proc_dmled_cm4_status"]
pub type Cm4StatusR = crate::FieldReader;
#[doc = "Field `cm4_status` writer - 11:9\\]
dft_proc_dmled_cm4_status"]
pub type Cm4StatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cm3_status` reader - 14:12\\]
dft_proc_dmled_cm3_status"]
pub type Cm3StatusR = crate::FieldReader;
#[doc = "Field `cm3_status` writer - 14:12\\]
dft_proc_dmled_cm3_status"]
pub type Cm3StatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `obs_status` reader - 17:15\\]
dft_proc_dmled_obs_status"]
pub type ObsStatusR = crate::FieldReader;
#[doc = "Field `obs_status` writer - 17:15\\]
dft_proc_dmled_obs_status"]
pub type ObsStatusW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
dft_proc_dmled_cm4_exec"]
    #[inline(always)]
    pub fn cm4_exec(&self) -> Cm4ExecR {
        Cm4ExecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
dft_proc_dmled_cm3_exec"]
    #[inline(always)]
    pub fn cm3_exec(&self) -> Cm3ExecR {
        Cm3ExecR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
dft_proc_dmled_obs_exec"]
    #[inline(always)]
    pub fn obs_exec(&self) -> ObsExecR {
        ObsExecR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
dft_proc_dmled_cm4_status"]
    #[inline(always)]
    pub fn cm4_status(&self) -> Cm4StatusR {
        Cm4StatusR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
dft_proc_dmled_cm3_status"]
    #[inline(always)]
    pub fn cm3_status(&self) -> Cm3StatusR {
        Cm3StatusR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - 17:15\\]
dft_proc_dmled_obs_status"]
    #[inline(always)]
    pub fn obs_status(&self) -> ObsStatusR {
        ObsStatusR::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
dft_proc_dmled_cm4_exec"]
    #[inline(always)]
    #[must_use]
    pub fn cm4_exec(&mut self) -> Cm4ExecW<DftProcDmledExecSpec> {
        Cm4ExecW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
dft_proc_dmled_cm3_exec"]
    #[inline(always)]
    #[must_use]
    pub fn cm3_exec(&mut self) -> Cm3ExecW<DftProcDmledExecSpec> {
        Cm3ExecW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
dft_proc_dmled_obs_exec"]
    #[inline(always)]
    #[must_use]
    pub fn obs_exec(&mut self) -> ObsExecW<DftProcDmledExecSpec> {
        ObsExecW::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
dft_proc_dmled_cm4_status"]
    #[inline(always)]
    #[must_use]
    pub fn cm4_status(&mut self) -> Cm4StatusW<DftProcDmledExecSpec> {
        Cm4StatusW::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
dft_proc_dmled_cm3_status"]
    #[inline(always)]
    #[must_use]
    pub fn cm3_status(&mut self) -> Cm3StatusW<DftProcDmledExecSpec> {
        Cm3StatusW::new(self, 12)
    }
    #[doc = "Bits 15:17 - 17:15\\]
dft_proc_dmled_obs_status"]
    #[inline(always)]
    #[must_use]
    pub fn obs_status(&mut self) -> ObsStatusW<DftProcDmledExecSpec> {
        ObsStatusW::new(self, 15)
    }
}
#[doc = "dft_proc_dmled_exec\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_proc_dmled_exec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_proc_dmled_exec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftProcDmledExecSpec;
impl crate::RegisterSpec for DftProcDmledExecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_proc_dmled_exec::R`](R) reader structure"]
impl crate::Readable for DftProcDmledExecSpec {}
#[doc = "`write(|w| ..)` method takes [`dft_proc_dmled_exec::W`](W) writer structure"]
impl crate::Writable for DftProcDmledExecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dft_proc_dmled_exec to value 0"]
impl crate::Resettable for DftProcDmledExecSpec {
    const RESET_VALUE: u32 = 0;
}
