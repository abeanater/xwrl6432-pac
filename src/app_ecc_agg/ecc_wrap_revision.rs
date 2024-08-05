#[doc = "Register `ECC_WRAP_REVISION` reader"]
pub type R = crate::R<EccWrapRevisionSpec>;
#[doc = "Register `ECC_WRAP_REVISION` writer"]
pub type W = crate::W<EccWrapRevisionSpec>;
#[doc = "Field `REVMIN` reader - 5:0\\]
Minor version - (RO )"]
pub type RevminR = crate::FieldReader;
#[doc = "Field `REVMIN` writer - 5:0\\]
Minor version - (RO )"]
pub type RevminW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Custom version - (RO )"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Custom version - (RO )"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REVMAJ` reader - 10:8\\]
Major version - (RO )"]
pub type RevmajR = crate::FieldReader;
#[doc = "Field `REVMAJ` writer - 10:8\\]
Major version - (RO )"]
pub type RevmajW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REVRTL` reader - 15:11\\]
RTL version - (RO )"]
pub type RevrtlR = crate::FieldReader;
#[doc = "Field `REVRTL` writer - 15:11\\]
RTL version - (RO )"]
pub type RevrtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MODULE_ID` reader - 27:16\\]
Module ID - (RO )"]
pub type ModuleIdR = crate::FieldReader<u16>;
#[doc = "Field `MODULE_ID` writer - 27:16\\]
Module ID - (RO )"]
pub type ModuleIdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BU` reader - 29:28\\]
bu - (RO )"]
pub type BuR = crate::FieldReader;
#[doc = "Field `BU` writer - 29:28\\]
bu - (RO )"]
pub type BuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
Scheme - (RO )"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
Scheme - (RO )"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor version - (RO )"]
    #[inline(always)]
    pub fn revmin(&self) -> RevminR {
        RevminR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom version - (RO )"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major version - (RO )"]
    #[inline(always)]
    pub fn revmaj(&self) -> RevmajR {
        RevmajR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version - (RO )"]
    #[inline(always)]
    pub fn revrtl(&self) -> RevrtlR {
        RevrtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID - (RO )"]
    #[inline(always)]
    pub fn module_id(&self) -> ModuleIdR {
        ModuleIdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
bu - (RO )"]
    #[inline(always)]
    pub fn bu(&self) -> BuR {
        BuR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme - (RO )"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor version - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn revmin(&mut self) -> RevminW<EccWrapRevisionSpec> {
        RevminW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom version - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<EccWrapRevisionSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major version - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn revmaj(&mut self) -> RevmajW<EccWrapRevisionSpec> {
        RevmajW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL version - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn revrtl(&mut self) -> RevrtlW<EccWrapRevisionSpec> {
        RevrtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn module_id(&mut self) -> ModuleIdW<EccWrapRevisionSpec> {
        ModuleIdW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
bu - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn bu(&mut self) -> BuW<EccWrapRevisionSpec> {
        BuW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<EccWrapRevisionSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "Revision parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_wrap_revision::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_wrap_revision::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccWrapRevisionSpec;
impl crate::RegisterSpec for EccWrapRevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_wrap_revision::R`](R) reader structure"]
impl crate::Readable for EccWrapRevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_wrap_revision::W`](W) writer structure"]
impl crate::Writable for EccWrapRevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_WRAP_REVISION to value 0"]
impl crate::Resettable for EccWrapRevisionSpec {
    const RESET_VALUE: u32 = 0;
}
