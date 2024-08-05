#[doc = "Register `PID` reader"]
pub type R = crate::R<PidSpec>;
#[doc = "Register `PID` writer"]
pub type W = crate::W<PidSpec>;
#[doc = "Field `MINOR_REVISION` reader - 5:0\\]
Minor Revision"]
pub type MinorRevisionR = crate::FieldReader;
#[doc = "Field `MINOR_REVISION` writer - 5:0\\]
Minor Revision"]
pub type MinorRevisionW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM_REVISION_FIELD` reader - 7:6\\]
Custom revision field: Not used on this version of EDMA."]
pub type CustomRevisionFieldR = crate::FieldReader;
#[doc = "Field `CUSTOM_REVISION_FIELD` writer - 7:6\\]
Custom revision field: Not used on this version of EDMA."]
pub type CustomRevisionFieldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR_REVISION` reader - 10:8\\]
Major Revision"]
pub type MajorRevisionR = crate::FieldReader;
#[doc = "Field `MAJOR_REVISION` writer - 10:8\\]
Major Revision"]
pub type MajorRevisionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL_VERSION` reader - 15:11\\]
RTL Version"]
pub type RtlVersionR = crate::FieldReader;
#[doc = "Field `RTL_VERSION` writer - 15:11\\]
RTL Version"]
pub type RtlVersionW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNCTION_INDICATES_A` reader - 27:16\\]
Function indicates a software compatible module family."]
pub type FunctionIndicatesAR = crate::FieldReader<u16>;
#[doc = "Field `FUNCTION_INDICATES_A` writer - 27:16\\]
Function indicates a software compatible module family."]
pub type FunctionIndicatesAW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PID_SCHEME` reader - 31:30\\]
PID Scheme: Used to distinguish between old ID scheme and current. Spare bit to encode future schemes EDMA uses 'new scheme' indicated with value of 0x1."]
pub type PidSchemeR = crate::FieldReader;
#[doc = "Field `PID_SCHEME` writer - 31:30\\]
PID Scheme: Used to distinguish between old ID scheme and current. Spare bit to encode future schemes EDMA uses 'new scheme' indicated with value of 0x1."]
pub type PidSchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision"]
    #[inline(always)]
    pub fn minor_revision(&self) -> MinorRevisionR {
        MinorRevisionR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision field: Not used on this version of EDMA."]
    #[inline(always)]
    pub fn custom_revision_field(&self) -> CustomRevisionFieldR {
        CustomRevisionFieldR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision"]
    #[inline(always)]
    pub fn major_revision(&self) -> MajorRevisionR {
        MajorRevisionR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version"]
    #[inline(always)]
    pub fn rtl_version(&self) -> RtlVersionR {
        RtlVersionR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function indicates a software compatible module family."]
    #[inline(always)]
    pub fn function_indicates_a(&self) -> FunctionIndicatesAR {
        FunctionIndicatesAR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID Scheme: Used to distinguish between old ID scheme and current. Spare bit to encode future schemes EDMA uses 'new scheme' indicated with value of 0x1."]
    #[inline(always)]
    pub fn pid_scheme(&self) -> PidSchemeR {
        PidSchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision"]
    #[inline(always)]
    #[must_use]
    pub fn minor_revision(&mut self) -> MinorRevisionW<PidSpec> {
        MinorRevisionW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision field: Not used on this version of EDMA."]
    #[inline(always)]
    #[must_use]
    pub fn custom_revision_field(&mut self) -> CustomRevisionFieldW<PidSpec> {
        CustomRevisionFieldW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision"]
    #[inline(always)]
    #[must_use]
    pub fn major_revision(&mut self) -> MajorRevisionW<PidSpec> {
        MajorRevisionW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version"]
    #[inline(always)]
    #[must_use]
    pub fn rtl_version(&mut self) -> RtlVersionW<PidSpec> {
        RtlVersionW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function indicates a software compatible module family."]
    #[inline(always)]
    #[must_use]
    pub fn function_indicates_a(&mut self) -> FunctionIndicatesAW<PidSpec> {
        FunctionIndicatesAW::new(self, 16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID Scheme: Used to distinguish between old ID scheme and current. Spare bit to encode future schemes EDMA uses 'new scheme' indicated with value of 0x1."]
    #[inline(always)]
    #[must_use]
    pub fn pid_scheme(&mut self) -> PidSchemeW<PidSpec> {
        PidSchemeW::new(self, 30)
    }
}
#[doc = "Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidSpec;
impl crate::RegisterSpec for PidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid::R`](R) reader structure"]
impl crate::Readable for PidSpec {}
#[doc = "`write(|w| ..)` method takes [`pid::W`](W) writer structure"]
impl crate::Writable for PidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PidSpec {
    const RESET_VALUE: u32 = 0;
}
