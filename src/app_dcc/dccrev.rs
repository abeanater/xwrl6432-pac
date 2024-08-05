#[doc = "Register `DCCREV` reader"]
pub type R = crate::R<DccrevSpec>;
#[doc = "Register `DCCREV` writer"]
pub type W = crate::W<DccrevSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Minor revision number. - (RO )"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Minor revision number. - (RO )"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Indicates a special version of the module. May not be supported by standard software - (RO )"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Indicates a special version of the module. May not be supported by standard software - (RO )"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Major Revision Number - (RO )"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Major Revision Number - (RO )"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - 15:11\\]
Design Release Number - (RO )"]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - 15:11\\]
Design Release Number - (RO )"]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
Functional release number - (RO )"]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
Functional release number - (RO )"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NU1` reader - 29:28\\]
Reserved"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 29:28\\]
Reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
SCHEME. - (RO )"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
SCHEME. - (RO )"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number. - (RO )"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates a special version of the module. May not be supported by standard software - (RO )"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision Number - (RO )"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Design Release Number - (RO )"]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Functional release number - (RO )"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
SCHEME. - (RO )"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision number. - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<DccrevSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates a special version of the module. May not be supported by standard software - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<DccrevSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision Number - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<DccrevSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Design Release Number - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<DccrevSpec> {
        RtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Functional release number - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<DccrevSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<DccrevSpec> {
        Nu1W::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
SCHEME. - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<DccrevSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "Module version\n\nYou can [`read`](crate::Reg::read) this register and get [`dccrev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccrev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DccrevSpec;
impl crate::RegisterSpec for DccrevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccrev::R`](R) reader structure"]
impl crate::Readable for DccrevSpec {}
#[doc = "`write(|w| ..)` method takes [`dccrev::W`](W) writer structure"]
impl crate::Writable for DccrevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCCREV to value 0"]
impl crate::Resettable for DccrevSpec {
    const RESET_VALUE: u32 = 0;
}
