#[doc = "Register `DBRED_DBFED` reader"]
pub type R = crate::R<DbredDbfedSpec>;
#[doc = "Register `DBRED_DBFED` writer"]
pub type W = crate::W<DbredDbfedSpec>;
#[doc = "Field `DBRED_DEL` reader - 9:0\\]
Rising Edge Delay Count. 10-bit counter"]
pub type DbredDelR = crate::FieldReader<u16>;
#[doc = "Field `DBRED_DEL` writer - 9:0\\]
Rising Edge Delay Count. 10-bit counter"]
pub type DbredDelW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `Reserved2` reader - 15:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 15:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DBFED_DEL` reader - 25:16\\]
Falling Edge Delay Count. 10-bit counter"]
pub type DbfedDelR = crate::FieldReader<u16>;
#[doc = "Field `DBFED_DEL` writer - 25:16\\]
Falling Edge Delay Count. 10-bit counter"]
pub type DbfedDelW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `Reserved1` reader - 31:26\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 31:26\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Rising Edge Delay Count. 10-bit counter"]
    #[inline(always)]
    pub fn dbred_del(&self) -> DbredDelR {
        DbredDelR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Falling Edge Delay Count. 10-bit counter"]
    #[inline(always)]
    pub fn dbfed_del(&self) -> DbfedDelR {
        DbfedDelR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Rising Edge Delay Count. 10-bit counter"]
    #[inline(always)]
    #[must_use]
    pub fn dbred_del(&mut self) -> DbredDelW<DbredDbfedSpec> {
        DbredDelW::new(self, 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<DbredDbfedSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Falling Edge Delay Count. 10-bit counter"]
    #[inline(always)]
    #[must_use]
    pub fn dbfed_del(&mut self) -> DbfedDelW<DbredDbfedSpec> {
        DbfedDelW::new(self, 16)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DbredDbfedSpec> {
        Reserved1W::new(self, 26)
    }
}
#[doc = "Dead-Band Generator Rising Edge Delay Count Register/ Dead-Band Generator Falling Edge Delay Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbred_dbfed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbred_dbfed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbredDbfedSpec;
impl crate::RegisterSpec for DbredDbfedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbred_dbfed::R`](R) reader structure"]
impl crate::Readable for DbredDbfedSpec {}
#[doc = "`write(|w| ..)` method takes [`dbred_dbfed::W`](W) writer structure"]
impl crate::Writable for DbredDbfedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBRED_DBFED to value 0"]
impl crate::Resettable for DbredDbfedSpec {
    const RESET_VALUE: u32 = 0;
}
