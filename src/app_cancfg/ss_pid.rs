#[doc = "Register `SS_PID` reader"]
pub type R = crate::R<SsPidSpec>;
#[doc = "Register `SS_PID` writer"]
pub type W = crate::W<SsPidSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Minor revision"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Minor revision"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Custom"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Custom"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Major revision"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Major revision"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - 15:11\\]
RTL revision. Will vary depending on release."]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - 15:11\\]
RTL revision. Will vary depending on release."]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MODULE_ID` reader - 27:16\\]
Module ID"]
pub type ModuleIdR = crate::FieldReader<u16>;
#[doc = "Field `MODULE_ID` writer - 27:16\\]
Module ID"]
pub type ModuleIdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BU` reader - 29:28\\]
Business Unit: 10 = Processors"]
pub type BuR = crate::FieldReader;
#[doc = "Field `BU` writer - 29:28\\]
Business Unit: 10 = Processors"]
pub type BuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
PID register scheme"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
PID register scheme"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision. Will vary depending on release."]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID"]
    #[inline(always)]
    pub fn module_id(&self) -> ModuleIdR {
        ModuleIdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Business Unit: 10 = Processors"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID register scheme"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<SsPidSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<SsPidSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<SsPidSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revision. Will vary depending on release."]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<SsPidSpec> {
        RtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID"]
    #[inline(always)]
    #[must_use]
    pub fn module_id(&mut self) -> ModuleIdW<SsPidSpec> {
        ModuleIdW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Business Unit: 10 = Processors"]
    #[inline(always)]
    #[must_use]
    pub fn bu(&mut self) -> BuW<SsPidSpec> {
        BuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID register scheme"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<SsPidSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "SS_PID\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_pid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_pid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsPidSpec;
impl crate::RegisterSpec for SsPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_pid::R`](R) reader structure"]
impl crate::Readable for SsPidSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_pid::W`](W) writer structure"]
impl crate::Writable for SsPidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_PID to value 0"]
impl crate::Resettable for SsPidSpec {
    const RESET_VALUE: u32 = 0;
}
