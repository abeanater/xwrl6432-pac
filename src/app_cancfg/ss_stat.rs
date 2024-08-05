#[doc = "Register `SS_STAT` reader"]
pub type R = crate::R<SsStatSpec>;
#[doc = "Register `SS_STAT` writer"]
pub type W = crate::W<SsStatSpec>;
#[doc = "Field `NU` reader - 0:0\\]
Reserved"]
pub type NuR = crate::BitReader;
#[doc = "Field `NU` writer - 0:0\\]
Reserved"]
pub type NuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMI_DONE` reader - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
pub type MmiDoneR = crate::BitReader;
#[doc = "Field `MMI_DONE` writer - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
pub type MmiDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FDOE` reader - 2:2\\]
Reflects the value of mcanss_enable_fdoe configuration port x=mcanss_enable_fdoe"]
pub type EnFdoeR = crate::BitReader;
#[doc = "Field `EN_FDOE` writer - 2:2\\]
Reflects the value of mcanss_enable_fdoe configuration port x=mcanss_enable_fdoe"]
pub type EnFdoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 31:3\\]
Reserved"]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - 31:3\\]
Reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
    #[inline(always)]
    pub fn mmi_done(&self) -> MmiDoneR {
        MmiDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reflects the value of mcanss_enable_fdoe configuration port x=mcanss_enable_fdoe"]
    #[inline(always)]
    pub fn en_fdoe(&self) -> EnFdoeR {
        EnFdoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<SsStatSpec> {
        NuW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0:Memory Initialization is in progress, 1:Memory Intialization Done"]
    #[inline(always)]
    #[must_use]
    pub fn mmi_done(&mut self) -> MmiDoneW<SsStatSpec> {
        MmiDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Reflects the value of mcanss_enable_fdoe configuration port x=mcanss_enable_fdoe"]
    #[inline(always)]
    #[must_use]
    pub fn en_fdoe(&mut self) -> EnFdoeW<SsStatSpec> {
        EnFdoeW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<SsStatSpec> {
        Nu1W::new(self, 3)
    }
}
#[doc = "SS_STAT\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsStatSpec;
impl crate::RegisterSpec for SsStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_stat::R`](R) reader structure"]
impl crate::Readable for SsStatSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_stat::W`](W) writer structure"]
impl crate::Writable for SsStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_STAT to value 0"]
impl crate::Resettable for SsStatSpec {
    const RESET_VALUE: u32 = 0;
}
